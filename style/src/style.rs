use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::BitAnd;
use std::option::Option::Some;

use lazy_static::lazy_static;

use color::{blend_rgb, terminal_theme::TerminalTheme, Color, ColorSystem};

lazy_static! {
    static ref STYLE_MAP: [&'static str; 13] = {
        [
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "21", "51", "52", "53",
        ]
    };
    static ref STYLE_ATTRIBUTES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::with_capacity(22);
        m.insert("dim", "dim");
        m.insert("d", "dim");
        m.insert("bold", "bold");
        m.insert("b", "bold");
        m.insert("italic", "italic");
        m.insert("i", "italic");
        m.insert("underline", "underline");
        m.insert("u", "underline");
        m.insert("blink", "blink");
        m.insert("blink2", "blink2");
        m.insert("reverse", "reverse");
        m.insert("r", "reverse");
        m.insert("conceal", "conceal");
        m.insert("c", "conceal");
        m.insert("strike", "strike");
        m.insert("s", "strike");
        m.insert("underline2", "underline2");
        m.insert("uu", "underline2");
        m.insert("frame", "frame");
        m.insert("encircle", "encircle");
        m.insert("overline", "overline");
        m.insert("o", "overline");
        m
    };
    pub static ref NULL_STYLE: Style = Style::default();
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    SyntaxError(String),
    #[error("{0}")]
    ColorParseError(#[from] color::Error),
}

bitflags::bitflags! {
    #[derive(Default)]
    pub struct StyleAttribute: u32 {
        const BOLD = 1;
        const DIM = 2;
        const ITALIC = 4;
        const UNDERLINE = 8;
        const BLINK = 16;
        const BLINK2 = 32;
        const REVERSE = 64;
        const CONCEAL = 128;
        const STRIKE = 256;
        const UNDERLINE2 = 512;
        const FRAME = 1024;
        const ENCIRCLE = 2048;
        const OVERLINE = 4096;
    }
}

impl StyleAttribute {
    pub fn enabled(&self, flag: StyleAttribute) -> bool {
        self.bitand(flag).bits == flag.bits
    }

    pub fn all_flags() -> [StyleAttribute; 13] {
        [
            StyleAttribute::BOLD,
            StyleAttribute::DIM,
            StyleAttribute::ITALIC,
            StyleAttribute::UNDERLINE,
            StyleAttribute::BLINK,
            StyleAttribute::BLINK2,
            StyleAttribute::REVERSE,
            StyleAttribute::CONCEAL,
            StyleAttribute::STRIKE,
            StyleAttribute::UNDERLINE2,
            StyleAttribute::FRAME,
            StyleAttribute::ENCIRCLE,
            StyleAttribute::OVERLINE,
        ]
    }
}

/// A terminal style.
/// A terminal style consists of a color (`color`), a background color (`bgcolor`), and a number of attributes, such
/// as bold, italic etc. The attributes have 3 states: they can either be on
/// (``True``), off (``False``), or not set (``None``).
#[derive(Debug, Eq)]
pub struct Style {
    ansi: String,
    style_definition: String,
    /// Color of terminal text. Defaults to None.
    color: Option<Color>,
    /// Color of terminal background. Defaults to None.
    background_color: Option<Color>,
    set_attributes: StyleAttribute,
    attributes: StyleAttribute,
    /// Link URL. Defaults to None.
    link: Option<String>,
    link_id: String,
    null: bool,
}

#[derive(Clone)]
struct StyleBuilder {
    color: Option<Color>,
    background_color: Option<Color>,
    attributes_set: HashSet<StyleAttribute>,
    attributes: StyleAttribute,
    link: Option<String>,
}

struct StyleStack(VecDeque<Style>);

impl Default for Style {
    fn default() -> Self {
        Self {
            ansi: "".to_string(),
            style_definition: "none".to_string(),
            color: None,
            background_color: None,
            set_attributes: Default::default(),
            attributes: Default::default(),
            link: None,
            link_id: "".to_string(),
            null: true,
        }
    }
}

impl Default for StyleBuilder {
    fn default() -> Self {
        Self {
            color: None,
            background_color: None,
            attributes_set: HashSet::with_capacity(13),
            attributes: Default::default(),
            link: None,
        }
    }
}

impl StyleBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn with_background_color(mut self, color: Color) -> Self {
        self.background_color = Some(color);
        self
    }

    pub fn with_attribute(mut self, flag: StyleAttribute, value: bool) -> Self {
        self.attributes_set.insert(flag);
        self.attributes.set(flag, value);
        self
    }

    pub fn with_link(mut self, link: &str) -> Self {
        self.link = Some(link.to_string());
        self
    }

    pub fn attribute_from_str(self, attribute: &str, value: bool) -> Self {
        let attr = STYLE_ATTRIBUTES.get(attribute);
        match *attr.unwrap_or(&"") {
            "bold" => self.with_attribute(StyleAttribute::BOLD, value),
            "dim" => self.with_attribute(StyleAttribute::DIM, value),
            "italic" => self.with_attribute(StyleAttribute::ITALIC, value),
            "underline" => self.with_attribute(StyleAttribute::UNDERLINE, value),
            "blink" => self.with_attribute(StyleAttribute::BLINK, value),
            "blink2" => self.with_attribute(StyleAttribute::BLINK2, value),
            "reverse" => self.with_attribute(StyleAttribute::REVERSE, value),
            "conceal" => self.with_attribute(StyleAttribute::CONCEAL, value),
            "strike" => self.with_attribute(StyleAttribute::STRIKE, value),
            "underline2" => self.with_attribute(StyleAttribute::UNDERLINE2, value),
            "frame" => self.with_attribute(StyleAttribute::FRAME, value),
            "encircle" => self.with_attribute(StyleAttribute::ENCIRCLE, value),
            "overline" => self.with_attribute(StyleAttribute::OVERLINE, value),
            _ => self,
        }
    }

    pub fn build(self) -> Style {
        let attributes: Vec<(StyleAttribute, bool)> = self
            .attributes_set
            .iter()
            .cloned()
            .map(|flag| (flag, self.attributes.enabled(flag)))
            .collect();
        Style::new(self.color, self.background_color, &attributes, self.link)
    }
}

impl Style {
    pub fn new(
        color: Option<Color>,
        background_color: Option<Color>,
        attributes: &[(StyleAttribute, bool)],
        link: Option<String>,
    ) -> Self {
        let set_attributes: StyleAttribute = attributes
            .iter()
            .map(|(flag, _)| flag)
            .fold(StyleAttribute::default(), |f1, f2| f1 | *f2);

        let attributes: StyleAttribute = if set_attributes.bits > 0 {
            attributes
                .iter()
                .fold(StyleAttribute::default(), |accum, (flag, value)| {
                    if *value {
                        accum | *flag
                    } else {
                        accum
                    }
                })
        } else {
            StyleAttribute::default()
        };

        let null = !(set_attributes.bits > 0
            || color.is_some()
            || background_color.is_some()
            || link.is_some());

        let mut obj = Self {
            color,
            background_color,
            set_attributes,
            attributes,
            link: link.clone(),
            link_id: link
                .map(|_| uuid::Uuid::new_v4().to_string())
                .unwrap_or(Default::default()),
            null,
            ..Default::default()
        };
        obj.load_style_definition();
        obj
    }

    pub fn null() -> Self {
        Self::default()
    }

    pub fn from_color(color: Option<Color>, background_color: Option<Color>) -> Self {
        let null = !(color.is_some() || background_color.is_some());
        Self {
            ansi: "".to_string(),
            style_definition: "none".to_string(),
            color,
            background_color,
            set_attributes: StyleAttribute::default(),
            attributes: StyleAttribute::default(),
            link: None,
            link_id: "".to_string(),
            null,
        }
    }

    #[inline]
    fn flag_value(&self, flag: StyleAttribute) -> Option<bool> {
        if self.set_attributes.enabled(flag) {
            Some(self.attributes.enabled(flag))
        } else {
            None
        }
    }

    /// The foreground color or None if it is not set
    pub fn color(&self) -> Option<&Color> {
        self.color.as_ref()
    }

    /// The background color or None if it is not set
    pub fn background_color(&self) -> Option<&Color> {
        self.background_color.as_ref()
    }

    pub fn link(&self) -> &Option<String> {
        &self.link
    }

    /// bold text flag
    pub fn bold(&self) -> Option<bool> {
        self.flag_value(StyleAttribute::BOLD)
    }

    /// dim text flag
    pub fn dim(&self) -> Option<bool> {
        self.flag_value(StyleAttribute::DIM)
    }

    /// italic text flag
    pub fn italic(&self) -> Option<bool> {
        self.flag_value(StyleAttribute::ITALIC)
    }

    /// underlined text flag
    pub fn underline(&self) -> Option<bool> {
        self.flag_value(StyleAttribute::UNDERLINE)
    }

    /// blinking text flag
    pub fn blink(&self) -> Option<bool> {
        self.flag_value(StyleAttribute::BLINK)
    }

    /// fast blinking text
    pub fn blink2(&self) -> Option<bool> {
        self.flag_value(StyleAttribute::BLINK2)
    }

    /// reverse text flag
    pub fn reverse(&self) -> Option<bool> {
        self.flag_value(StyleAttribute::REVERSE)
    }

    /// concealed text flag
    pub fn conceal(&self) -> Option<bool> {
        self.flag_value(StyleAttribute::CONCEAL)
    }

    /// strikethrough text flag
    pub fn strike(&self) -> Option<bool> {
        self.flag_value(StyleAttribute::STRIKE)
    }

    /// doubly underlined text flag
    pub fn underline2(&self) -> Option<bool> {
        self.flag_value(StyleAttribute::UNDERLINE2)
    }

    /// framed text flag
    pub fn frame(&self) -> Option<bool> {
        self.flag_value(StyleAttribute::FRAME)
    }

    /// encircled text flag
    pub fn encircle(&self) -> Option<bool> {
        self.flag_value(StyleAttribute::ENCIRCLE)
    }

    /// overlined text flag
    pub fn overline(&self) -> Option<bool> {
        self.flag_value(StyleAttribute::OVERLINE)
    }

    /// Get a link id, used in ansi code for links
    pub fn link_id(&self) -> &str {
        &self.link_id
    }

    /// A Style is false if it has no attributes, colors, or links
    pub fn as_bool(&self) -> bool {
        !self.null
    }

    /// Check if the style specified a transparent background
    pub fn transparent_background(&self) -> bool {
        if let Some(color) = &self.background_color {
            color.is_default()
        } else {
            false
        }
    }

    /// A Style with background only
    pub fn background_style(&self) -> Style {
        if let Some(color) = &self.background_color {
            StyleBuilder::new()
                .with_background_color(color.clone())
                .build()
        } else {
            Self::default()
        }
    }

    fn load_style_definition(&mut self) {
        // calculate, store and return
        let mut attributes: Vec<&str> = Vec::new();

        // TODO: maybe use some loop here?
        if let Some(bold) = self.bold() {
            attributes.push(if bold { "bold" } else { "not bold" });
        }

        if let Some(dim) = self.dim() {
            attributes.push(if dim { "dim" } else { "not dim" });
        }

        if let Some(italic) = self.italic() {
            attributes.push(if italic { "italic" } else { "not italic" });
        }

        if let Some(underline) = self.underline() {
            attributes.push(if underline {
                "underline"
            } else {
                "not underline"
            });
        }

        if let Some(blink) = self.blink() {
            attributes.push(if blink { "blink" } else { "not blink" });
        }

        if let Some(blink2) = self.blink2() {
            attributes.push(if blink2 { "blink2" } else { "not blink2" });
        }

        if let Some(reverse) = self.reverse() {
            attributes.push(if reverse { "reverse" } else { "not reverse" });
        }

        if let Some(conceal) = self.conceal() {
            attributes.push(if conceal { "conceal" } else { "not conceal" });
        }

        if let Some(strike) = self.strike() {
            attributes.push(if strike { "strike" } else { "not strike" });
        }

        if let Some(underline2) = self.underline2() {
            attributes.push(if underline2 {
                "underline2"
            } else {
                "not underline2"
            });
        }

        if let Some(frame) = self.frame() {
            attributes.push(if frame { "frame" } else { "not frame" });
        }

        if let Some(encircle) = self.encircle() {
            attributes.push(if encircle { "encircle" } else { "not encircle" });
        }

        if let Some(overline) = self.overline() {
            attributes.push(if overline { "overline" } else { "not overline" });
        }

        if let Some(color) = self.color() {
            attributes.push(color.name.as_str());
        }

        if let Some(color) = self.background_color() {
            attributes.push("on");
            attributes.push(color.name.as_str());
        }

        if let Some(link) = self.link() {
            attributes.push("link");
            attributes.push(link.as_str());
        }

        let mut res: String = attributes.join(" ");
        if res.is_empty() {
            res = "none".to_string();
        }
        self.style_definition = res;
    }

    /// Re-generate style definition from attributes
    pub fn style_definition(&self) -> &str {
        &self.style_definition
    }

    /// Generate ANSI codes for this style
    fn ansi_codes(&self, color_system: ColorSystem) -> String {
        let mut ansi_codes: Vec<String> = Vec::new();
        for (i, flag) in StyleAttribute::all_flags().iter().enumerate() {
            if matches!(self.flag_value(*flag), Some(true)) {
                ansi_codes.push(STYLE_MAP[i as usize].to_string());
            }
        }
        if let Some(color) = self.color() {
            ansi_codes.extend(
                color
                    .downgrade(color_system)
                    .get_ansi_codes(None)
                    .iter()
                    .cloned(),
            );
        }
        if let Some(color) = self.background_color() {
            ansi_codes.extend(
                color
                    .downgrade(color_system)
                    .get_ansi_codes(Some(false))
                    .iter()
                    .cloned(),
            );
        }
        ansi_codes.join(";")
    }

    pub fn update_link(&self, link: Option<&str>) -> Self {
        let mut ret = self.clone();
        ret.link = link.map(str::to_string);
        ret
    }

    pub fn combine(&self, style2: Option<&Self>) -> Self {
        match (self, style2) {
            (style, None) => style.clone(),
            (style, Some(style2)) => {
                if style2.null {
                    return style.clone();
                }
                if style.null {
                    return style2.clone();
                }
                let mut new_style = style.clone();
                new_style.color = match (&style.color, &style2.color) {
                    (Some(color), _) => Some(color.clone()),
                    (None, other) => other.clone(),
                };
                new_style.background_color =
                    match (&style.background_color, &style2.background_color) {
                        (Some(color), _) => Some(color.clone()),
                        (None, other) => other.clone(),
                    };
                new_style.attributes = (style.attributes & !style.set_attributes)
                    | (style2.attributes & style2.set_attributes);
                new_style.set_attributes = style.set_attributes | style2.set_attributes;
                new_style.link = match (&style.link, &style2.link) {
                    (Some(link), _) => Some(link.clone()),
                    (None, other) => other.clone(),
                };
                new_style.link_id = match (style.link_id.as_str(), style2.link_id.as_str()) {
                    ("", "") => "".to_string(),
                    ("", id) => id.to_string(),
                    (id, "") => id.to_string(),
                    _ => unreachable!(),
                };
                new_style.null = style.null || style2.null;
                new_style.load_style_definition();
                new_style
            }
        }
    }

    pub fn chain<'a, Styles>(styles: Styles) -> Style
    where
        Styles: IntoIterator<Item = &'a Option<&'a Style>> + Copy,
    {
        let mut ret_style: Style = Style::default();
        for style in styles {
            ret_style = ret_style.combine(*style);
        }
        ret_style
    }

    /// Parse a style definition
    pub fn parse(style_definition: &str) -> Result<Style, Error> {
        if style_definition.trim() == "none" {
            return Ok(Style::null());
        }
        let mut style_builder = StyleBuilder::new();
        let mut words = style_definition.split_ascii_whitespace().into_iter();
        while let Some(original_word) = words.next() {
            let word = original_word.to_lowercase();
            match word.as_str() {
                "on" => {
                    let color_word = words
                        .next()
                        .ok_or(Error::SyntaxError("color expected after 'on'".to_string()))?;
                    let color = Color::parse(color_word)?;
                    style_builder = style_builder.with_background_color(color);
                }
                "not" => {
                    let word = words.next().unwrap_or("");
                    let attr = STYLE_ATTRIBUTES.get(word).cloned().ok_or_else(|| {
                        Error::SyntaxError(format!(
                            "style attribute expected after 'not', found: {}",
                            word
                        ))
                    })?;
                    style_builder = style_builder.attribute_from_str(attr, false);
                }
                "link" => {
                    let link = words
                        .next()
                        .ok_or(Error::SyntaxError("URL expected after 'link'".to_string()))?;
                    style_builder = style_builder.with_link(link);
                }
                attribute if STYLE_ATTRIBUTES.contains_key(attribute) => {
                    style_builder = style_builder.attribute_from_str(attribute, true);
                }
                word => {
                    let color = Color::parse(word)?;
                    style_builder = style_builder.with_color(color);
                }
            }
        }
        Ok(style_builder.build())
    }

    // Get a CSS style rule
    pub fn get_html_style(&self, theme: Option<TerminalTheme>) -> String {
        let theme = theme.unwrap_or(Default::default());
        let mut css: Vec<String> = Vec::new();
        let (mut color, mut background_color) =
            (self.color().cloned(), self.background_color().cloned());

        if self.reverse().unwrap_or(false) {
            color = self.background_color().cloned();
            background_color = self.color().cloned();
        }

        if self.dim().unwrap_or(false) {
            let foreground_color = if color.is_none() {
                theme.foreground_color
            } else {
                color.unwrap().get_true_color(Some(&theme), None)
            };
            color = Some(Color::from_triplet(blend_rgb(
                foreground_color,
                theme.background_color,
                Some(0.5),
            )));
        }

        if let Some(color) = color {
            let theme_color = color.get_true_color(Some(&theme), None);
            css.push(format!("color: {}", theme_color.hex()));
        }

        if let Some(background_color) = background_color {
            let theme_color = background_color.get_true_color(Some(&theme), Some(false));
            css.push(format!("background-color: {}", theme_color.hex()));
        }

        if self.bold().unwrap_or(false) {
            css.push("font-weight: bold".to_string());
        }

        if self.italic().unwrap_or(false) {
            css.push("font-style: italic".to_string());
        }

        if self.underline().unwrap_or(false) {
            css.push("text-decoration: underline".to_string());
        }

        if self.strike().unwrap_or(false) {
            css.push("text-decoration: line-through".to_string());
        }

        if self.overline().unwrap_or(false) {
            css.push("text-decoration: overline".to_string());
        }

        css.join("; ")
    }

    /// Render the ANSI codes for the style
    pub fn render(
        &self,
        text: &str,
        color_system: Option<ColorSystem>,
        legacy_windows: Option<bool>,
    ) -> String {
        if text.is_empty() || color_system.is_none() {
            return text.to_string();
        }

        let attrs = self.ansi_codes(color_system.unwrap_or(ColorSystem::TrueColor));
        let rendered = if !attrs.is_empty() {
            format!("\x1b[{attrs}m{text}\x1b[0m", attrs = attrs, text = text)
        } else {
            text.to_string()
        };
        match (self.link(), !legacy_windows.unwrap_or(false)) {
            (Some(link), true) => {
                format!(
                    "\x1b]8;id={};{}\x1b\\{}\x1b]8;;\x1b\\",
                    self.link_id(),
                    link,
                    rendered
                )
            }
            _ => rendered,
        }
    }

    /// Normalize a style definition so that styles with the same effect have the same String representation
    pub fn normalize(style: &str) -> String {
        Self::parse(style)
            .map(|style| style.to_string())
            .unwrap_or(style.trim().to_lowercase())
    }

    pub fn pick_first<'a, Styles>(styles: Styles) -> Option<Style>
    where
        Styles: IntoIterator<Item = Option<&'a Style>>,
    {
        styles
            .into_iter()
            .filter(Option::is_some)
            .next()
            .unwrap_or(None)
            .cloned()
    }
}

impl PartialEq for Style {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && self.background_color == self.background_color
            && self.set_attributes == other.set_attributes
            && self.attributes == other.attributes
            && self.link == other.link
    }
}

impl Hash for Style {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.color.hash(state);
        self.background_color.hash(state);
        self.attributes.hash(state);
        self.set_attributes.hash(state);
        self.link.hash(state);
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.style_definition())
    }
}

impl Clone for Style {
    fn clone(&self) -> Self {
        Self {
            ansi: self.ansi.clone(),
            style_definition: self.style_definition.clone(),
            color: self.color.clone(),
            background_color: self.background_color.clone(),
            set_attributes: self.set_attributes,
            attributes: self.attributes,
            link: self.link.clone(),
            link_id: self
                .link
                .as_ref()
                .map(|_| uuid::Uuid::new_v4().to_string())
                .unwrap_or(Default::default()),
            null: false,
        }
    }
}

impl StyleStack {
    pub fn new(default_style: Style) -> Self {
        Self(VecDeque::from(vec![default_style]))
    }

    pub fn current(&self) -> &Style {
        // we can safely unwrap, we will check so it is never empty
        self.0.get(self.0.len() - 1).unwrap()
    }

    pub fn push(&mut self, new_style: Style) {
        self.0.push_back(self.current().combine(Some(&new_style)));
    }

    pub fn pop(&mut self) -> Style {
        if self.0.len() == 1 {
            return self.current().clone();
        }
        // safe to unwrap here since we always will have at least one extra
        self.0.pop_back().unwrap()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use color::ColorType;

    #[test]
    fn test_str() {
        assert_eq!(
            StyleBuilder::new()
                .with_attribute(StyleAttribute::BOLD, false)
                .build()
                .to_string(),
            "not bold"
        );

        assert_eq!(
            StyleBuilder::new()
                .with_color(Color::parse("red").unwrap())
                .with_attribute(StyleAttribute::BOLD, false)
                .build()
                .to_string(),
            "not bold red"
        );

        assert_eq!(
            StyleBuilder::new()
                .with_color(Color::parse("red").unwrap())
                .with_attribute(StyleAttribute::BOLD, false)
                .with_attribute(StyleAttribute::ITALIC, true)
                .build()
                .to_string(),
            "not bold italic red"
        );

        assert_eq!(Style::null().to_string(), "none");

        assert_eq!(
            StyleBuilder::new()
                .with_attribute(StyleAttribute::BOLD, true)
                .build()
                .to_string(),
            "bold"
        );

        assert_eq!(
            StyleBuilder::new()
                .with_color(Color::parse("red").unwrap())
                .with_attribute(StyleAttribute::BOLD, true)
                .build()
                .to_string(),
            "bold red"
        );

        assert_eq!(
            StyleBuilder::new()
                .with_color(Color::parse("red").unwrap())
                .with_background_color(Color::parse("black").unwrap())
                .with_attribute(StyleAttribute::BOLD, true)
                .build()
                .to_string(),
            "bold red on black"
        );

        let all_styles_builder = StyleBuilder::new()
            .with_color(Color::parse("red").unwrap())
            .with_background_color(Color::parse("black").unwrap());
        let all_styles_builder = StyleAttribute::all_flags()
            .iter()
            .fold(all_styles_builder, |builder, flag| {
                builder.with_attribute(*flag, true)
            });
        let all_styles_expected = "bold dim italic underline blink blink2 reverse conceal strike underline2 frame encircle overline red on black";
        assert_eq!(all_styles_builder.build().to_string(), all_styles_expected);

        assert_eq!(
            StyleBuilder::new().with_link("foo").build().to_string(),
            "link foo"
        );
    }

    #[test]
    fn test_ansi_codes() {
        let all_styles_builder = StyleBuilder::new()
            .with_color(Color::parse("red").unwrap())
            .with_background_color(Color::parse("black").unwrap());
        let all_styles_builder = StyleAttribute::all_flags()
            .iter()
            .fold(all_styles_builder, |builder, flag| {
                builder.with_attribute(*flag, true)
            });

        let expected_ansi_codes = "1;2;3;4;5;6;7;8;9;21;51;52;53;31;40";

        assert_eq!(
            all_styles_builder
                .build()
                .ansi_codes(ColorSystem::TrueColor),
            expected_ansi_codes
        );
    }

    #[test]
    fn test_eq() {
        let red_builder = StyleBuilder::new()
            .with_attribute(StyleAttribute::BOLD, true)
            .with_color(Color::parse("red").unwrap());
        let green_builder = StyleBuilder::new()
            .with_attribute(StyleAttribute::BOLD, true)
            .with_color(Color::parse("green").unwrap());
        assert_eq!(red_builder.clone().build(), red_builder.clone().build());
        assert_ne!(red_builder.build(), green_builder.build());
    }

    #[test]
    fn test_hash() {
        let style_null = Style::null();
        let other_style = StyleBuilder::new()
            .with_color(Color::parse("red").unwrap())
            .build();
        let mut set = HashSet::new();
        set.insert(style_null.clone());
        set.insert(other_style);
        set.insert(style_null);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_empty() {
        assert_eq!(Style::null(), Style::default());
    }

    #[test]
    fn test_bool() {
        assert_eq!(Style::null().as_bool(), false);
        assert_eq!(
            StyleBuilder::new()
                .with_attribute(StyleAttribute::BOLD, true)
                .build()
                .as_bool(),
            true
        );
        assert_eq!(
            StyleBuilder::new()
                .with_color(Color::parse("red").unwrap())
                .build()
                .as_bool(),
            true
        );
        assert_eq!(Style::parse("").unwrap().as_bool(), false);
    }

    #[test]
    fn test_color_property() {
        assert_eq!(
            *StyleBuilder::new()
                .with_color(Color::parse("red").unwrap())
                .build()
                .color()
                .unwrap(),
            Color {
                name: "red".to_string(),
                color_type: ColorType::Standard,
                number: Some(1),
                triplet: None
            }
        )
    }

    #[test]
    fn test_background_color_property() {
        assert_eq!(
            *StyleBuilder::new()
                .with_background_color(Color::parse("black").unwrap())
                .build()
                .background_color()
                .unwrap(),
            Color {
                name: "black".to_string(),
                color_type: ColorType::Standard,
                number: Some(0),
                triplet: None
            }
        )
    }

    #[test]
    fn test_parse_success() {
        assert_eq!(
            Style::parse("").expect("a 'null' style was expected"),
            Style::null()
        );

        assert_eq!(
            Style::parse("red").expect("a 'red' only style"),
            Style::new(Color::parse("red").ok(), None, &[], None)
        );

        assert_eq!(
            Style::parse("not bold").expect("a 'not bold' style"),
            StyleBuilder::new()
                .with_attribute(StyleAttribute::BOLD, false)
                .build()
        );

        assert_eq!(
            Style::parse("bold red on black").expect("a 'bold red on black' style"),
            StyleBuilder::new()
                .with_color(Color::parse("red").unwrap())
                .with_background_color(Color::parse("black").unwrap())
                .with_attribute(StyleAttribute::BOLD, true)
                .build()
        );

        assert_eq!(
            Style::parse("bold link https://example.org").expect("a style with a bold link"),
            StyleBuilder::new()
                .with_attribute(StyleAttribute::BOLD, true)
                .with_link("https://example.org")
                .build()
        )
    }

    #[test]
    fn test_parse_fails() {
        assert!(Style::parse("on").is_err());
        assert!(Style::parse("on nothing").is_err());
        assert!(Style::parse("rgb(999,999,999)").is_err());
        assert!(Style::parse("not monkey").is_err());
        assert!(Style::parse("link").is_err());
    }

    #[test]
    fn test_background_style() {
        assert_eq!(
            StyleBuilder::new()
                .with_background_color(Color::parse("red").expect("a red color"))
                .with_color(Color::parse("yellow").expect("a yellow color"))
                .with_attribute(StyleAttribute::BOLD, true)
                .build()
                .background_style(),
            StyleBuilder::new()
                .with_background_color(Color::parse("red").expect("a red color"))
                .build()
        );
    }

    #[test]
    fn test_link_id() {
        assert_eq!(Style::null().link_id, "");
        assert_eq!(Style::parse("").expect("null style expected").link_id(), "");
        assert_eq!(Style::parse("red").expect("a red only style").link_id(), "");
        assert!(
            Style::parse("red link https://example.org")
                .expect("a red link style")
                .link_id()
                .len()
                > 1
        );
    }

    #[test]
    fn test_get_html_style() {
        let expected = "color: #7f7fbf; background-color: #800000; font-weight: bold; font-style: italic; text-decoration: underline; text-decoration: line-through; text-decoration: overline";
        let style = Style::new(
            Color::parse("red").ok(),
            Color::parse("blue").ok(),
            &[
                (StyleAttribute::REVERSE, true),
                (StyleAttribute::DIM, true),
                (StyleAttribute::BOLD, true),
                (StyleAttribute::ITALIC, true),
                (StyleAttribute::UNDERLINE, true),
                (StyleAttribute::STRIKE, true),
                (StyleAttribute::OVERLINE, true),
            ],
            None,
        );
        assert_eq!(style.get_html_style(None), expected);
    }

    #[test]
    fn test_chain() {
        let red = StyleBuilder::new()
            .with_color(Color::parse("red").expect("a red color"))
            .build();
        let bold = StyleBuilder::new()
            .with_attribute(StyleAttribute::BOLD, true)
            .build();
        let expected = StyleBuilder::new()
            .with_color(Color::parse("red").expect("a red color"))
            .with_attribute(StyleAttribute::BOLD, true)
            .build();
        let styles = [Some(&red), Some(&bold)];
        assert_eq!(Style::chain(&styles), expected);
    }

    #[test]
    fn test_copy() {
        let style = StyleBuilder::new()
            .with_color(Color::parse("red").expect("red color"))
            .with_background_color(Color::parse("black").expect("black color"))
            .with_attribute(StyleAttribute::ITALIC, true)
            .with_link("https://foo.bar")
            .build();
        assert_eq!(style.clone(), style.clone());
        assert_ne!(style.clone().link_id, style.link_id);
    }

    #[test]
    fn test_render() {
        assert_eq!(
            StyleBuilder::new()
                .with_color(Color::parse("red").expect("a red color"))
                .build()
                .render("foo1", None, None),
            "foo1"
        );

        assert_eq!(
            StyleBuilder::new()
                .with_color(Color::parse("red").expect("a red color"))
                .with_background_color(Color::parse("black").expect("a black color"))
                .with_attribute(StyleAttribute::BOLD, true)
                .build()
                .render("foo2", Some(ColorSystem::TrueColor), None),
            "\x1b[1;31;40mfoo2\x1b[0m"
        );

        assert_eq!(Style::null().render("foo3", None, None), "foo3");
    }

    #[test]
    fn test_combine() {
        let red = StyleBuilder::new()
            .with_color(Color::parse("red").expect("a red color"))
            .build();
        let bold = StyleBuilder::new()
            .with_attribute(StyleAttribute::BOLD, true)
            .build();
        let expected = StyleBuilder::new()
            .with_color(Color::parse("red").expect("a red color"))
            .with_attribute(StyleAttribute::BOLD, true)
            .build();
        assert_eq!(red.combine(None), red);
        assert_eq!(red.combine(Some(&bold)), expected)
    }

    #[test]
    fn test_pick_first() {
        let void: Vec<Option<&Style>> = vec![];
        assert!(Style::pick_first(void).is_none());
    }

    #[test]
    fn test_style_stack() {
        let red = StyleBuilder::new()
            .with_color(Color::parse("red").expect("a red color"))
            .build();
        let bold = StyleBuilder::new()
            .with_attribute(StyleAttribute::BOLD, true)
            .build();
        let expected = StyleBuilder::new()
            .with_color(Color::parse("red").expect("a red color"))
            .with_attribute(StyleAttribute::BOLD, true)
            .build();

        let mut stack = StyleStack::new(red.clone());
        assert_eq!(*stack.current(), red.clone());

        stack.push(bold.clone());
        assert_eq!(*stack.current(), expected);

        stack.pop();
        assert_eq!(*stack.current(), red);
    }
}
