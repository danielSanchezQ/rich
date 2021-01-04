use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::option::Option::Some;

use lazy_static::lazy_static;
use thiserror::Error;

use color::{
    blend_rgb,
    terminal_theme::{TerminalTheme, DEFAULT_TERMINAL_THEME},
    Color, ColorSystem,
};

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
    pub static ref NULL_STYLE: Style = { Style::default() };
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    SyntaxError(String),
    #[error("{0}")]
    ColorParseError(#[from] color::Error),
}

/// A terminal style.
/// A terminal style consists of a color (`color`), a background color (`bgcolor`), and a number of attributes, such
/// as bold, italic etc. The attributes have 3 states: they can either be on
/// (``True``), off (``False``), or not set (``None``).
pub struct Style {
    ansi: String,
    style_definition: String,
    /// Color of terminal text. Defaults to None.
    color: Option<Color>,
    /// Color of terminal background. Defaults to None.
    background_color: Option<Color>,
    set_attributes: u32,
    attributes: u32,
    /// Link URL. Defaults to None.
    link: Option<String>,
    link_id: String,
    null: bool,
}

// TODO: Maybe move this to use bitflags crate? https://docs.rs/bitflags
struct StyleBuilder {
    color: Option<Color>,
    background_color: Option<Color>,
    bold: bool,
    dim: bool,
    italic: bool,
    underline: bool,
    blink: bool,
    blink2: bool,
    reverse: bool,
    conceal: bool,
    strike: bool,
    underline2: bool,
    frame: bool,
    encircle: bool,
    overline: bool,
    link: Option<String>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            ansi: "".to_string(),
            style_definition: "none".to_string(),
            color: None,
            background_color: None,
            set_attributes: 0,
            attributes: 0,
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
            bold: false,
            dim: false,
            italic: false,
            underline: false,
            blink: false,
            blink2: false,
            reverse: false,
            conceal: false,
            strike: false,
            underline2: false,
            frame: false,
            encircle: false,
            overline: false,
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

    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    pub fn dim(mut self) -> Self {
        self.dim = true;
        self
    }

    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    pub fn blink(mut self) -> Self {
        self.blink = true;
        self
    }

    pub fn blink2(mut self) -> Self {
        self.blink2 = true;
        self
    }

    pub fn reverse(mut self) -> Self {
        self.reverse = true;
        self
    }

    pub fn conceal(mut self) -> Self {
        self.conceal = true;
        self
    }

    pub fn strike(mut self) -> Self {
        self.strike = true;
        self
    }

    pub fn underline2(mut self) -> Self {
        self.underline2 = true;
        self
    }

    pub fn frame(mut self) -> Self {
        self.frame = true;
        self
    }

    pub fn encircle(mut self) -> Self {
        self.encircle = true;
        self
    }

    pub fn overline(mut self) -> Self {
        self.overline = true;
        self
    }

    pub fn with_link(mut self, link: String) -> Self {
        self.link = Some(link);
        self
    }

    pub fn attribute_from_str(mut self, attribute: &str) -> Self {
        let attr = STYLE_ATTRIBUTES.get(attribute);
        match *attr.unwrap_or(&"") {
            "bold" => self.bold(),
            "dim" => self.dim(),
            "italic" => self.italic(),
            "underline" => self.underline(),
            "blink" => self.blink(),
            "blink2" => self.blink2(),
            "reverse" => self.reverse(),
            "conceal" => self.conceal(),
            "strike" => self.strike(),
            "underline2" => self.underline2(),
            "frame" => self.frame(),
            "encircle" => self.encircle(),
            "overline" => self.overline(),
            _ => self,
        }
    }

    pub fn build(self) -> Style {
        Style::new(
            self.color,
            self.background_color,
            self.bold,
            self.dim,
            self.italic,
            self.underline,
            self.blink,
            self.blink2,
            self.reverse,
            self.conceal,
            self.strike,
            self.underline2,
            self.frame,
            self.encircle,
            self.overline,
            self.link,
        )
    }
}

impl Style {
    pub fn new(
        color: Option<Color>,
        background_color: Option<Color>,
        bold: bool,
        dim: bool,
        italic: bool,
        underline: bool,
        blink: bool,
        blink2: bool,
        reverse: bool,
        conceal: bool,
        strike: bool,
        underline2: bool,
        frame: bool,
        encircle: bool,
        overline: bool,
        link: Option<String>,
    ) -> Self {
        let attrs = [
            bold, dim, italic, underline, blink, blink2, reverse, conceal, strike, underline2,
            frame, encircle, overline,
        ];
        let set_attributes: u32 = attrs[1..]
            .iter()
            .enumerate()
            .filter_map(|(i, flag)| {
                if *flag {
                    Some(2u32.pow(i as u32))
                } else {
                    None
                }
            })
            .sum::<u32>()
            + if bold { 1 } else { 0 };

        let attributes: u32 = if set_attributes > 0 {
            attrs
                .iter()
                .enumerate()
                .filter_map(|(i, flag)| {
                    if *flag {
                        Some(2u32.pow(i as u32))
                    } else {
                        None
                    }
                })
                .sum()
        } else {
            0 as u32
        };

        let null = !(set_attributes > 0
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
            set_attributes: 0,
            attributes: 0,
            link: None,
            link_id: "".to_string(),
            null,
        }
    }

    #[inline]
    fn bit_flag(&self, bit: u32) -> Option<bool> {
        let bit: u32 = (1 << bit);
        let res: u32 = &self.set_attributes & bit;
        if res.count_ones() > 0 {
            Some((self.attributes & bit) != 0)
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
        self.bit_flag(0)
    }

    /// dim text flag
    pub fn dim(&self) -> Option<bool> {
        self.bit_flag(1)
    }

    /// italic text flag
    pub fn italic(&self) -> Option<bool> {
        self.bit_flag(2)
    }

    /// underlined text flag
    pub fn underline(&self) -> Option<bool> {
        self.bit_flag(3)
    }

    /// blinking text flag
    pub fn blink(&self) -> Option<bool> {
        self.bit_flag(4)
    }

    /// fast blinking text
    pub fn blink2(&self) -> Option<bool> {
        self.bit_flag(5)
    }

    /// reverse text flag
    pub fn reverse(&self) -> Option<bool> {
        self.bit_flag(6)
    }

    /// concealed text flag
    pub fn conceal(&self) -> Option<bool> {
        self.bit_flag(7)
    }

    /// strikethrough text flag
    pub fn strike(&self) -> Option<bool> {
        self.bit_flag(8)
    }

    /// doubly underlined text flag
    pub fn underline2(&self) -> Option<bool> {
        self.bit_flag(9)
    }

    /// framed text flag
    pub fn frame(&self) -> Option<bool> {
        self.bit_flag(10)
    }

    /// encircled text flag
    pub fn encircle(&self) -> Option<bool> {
        self.bit_flag(11)
    }

    /// overlined text flag
    pub fn overline(&self) -> Option<bool> {
        self.bit_flag(12)
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
            attributes.push(if blink2 { "blink" } else { "not blink" });
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

    // TODO: Do not like the mut ref here...think how to improve this api
    /// Re-generate style definition from attributes
    pub fn style_definition(&self) -> &str {
        &self.style_definition
    }

    /// Generate ANSI codes for this style
    fn ansi_codes(&self, color_system: ColorSystem) -> String {
        let mut ansi_codes: Vec<String> = Vec::new();
        for i in 0..13 {
            if matches!(self.bit_flag(i), Some(true)) {
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
        Styles: IntoIterator<Item = Option<&'a Style>> + Copy,
    {
        let mut iter = styles.into_iter().filter_map(|style| style);
        let mut ret_style: Style = Style::default();
        for style in styles {
            ret_style = ret_style.combine(style);
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
                    // we skip since attributes are false by default
                    words.next();
                }
                "link" => {
                    let link = words
                        .next()
                        .ok_or(Error::SyntaxError("URL expected after 'link'".to_string()))?;
                    style_builder = style_builder.with_link(link.to_string());
                }
                attribute if STYLE_ATTRIBUTES.contains_key(attribute) => {
                    style_builder = style_builder.attribute_from_str(attribute);
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
            css.push("font-weight: italic".to_string());
        }

        if self.underline().unwrap_or(false) {
            css.push("font-weight: underline".to_string());
        }

        if self.strike().unwrap_or(false) {
            css.push("font-weight: line-through".to_string());
        }

        if self.overline().unwrap_or(false) {
            css.push("font-weight: overline".to_string());
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
            return String::default();
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
        write!(f, "Style.parse({})", self.style_definition())
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
