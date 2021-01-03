use color::Color;

struct Bit(u32);

#[inline]
fn _to_bool(n: u32) -> bool {
    n.count_ones() > 0
}

/// A helper to get/set a style attribute bit
impl Bit {
    pub fn new(bit: u32) -> Self {
        Self(1 << bit)
    }

    pub fn get(&self, style: &Style) -> Option<bool> {
        let res: u32 = &style.set_attributes & &self.0;
        if _to_bool(res) {
            Some(res != 0)
        } else {
            None
        }
    }
}

/// A terminal style.
/// A terminal style consists of a color (`color`), a background color (`bgcolor`), and a number of attributes, such
/// as bold, italic etc. The attributes have 3 states: they can either be on
/// (``True``), off (``False``), or not set (``None``).
struct Style {
    ansi: Option<String>,
    style_definition: Option<String>,
    color: Option<Color>,
    background_color: Option<Color>,
    set_attributes: u32,
    attributes: u32,
    link: Option<String>,
    link_id: String,
    null: bool,
}

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
            ansi: None,
            style_definition: None,
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

    pub fn with_color(&mut self, color: Color) -> &mut Self {
        self.color = Some(color);
        self
    }

    pub fn with_background_color(&mut self, color: Color) -> &mut Self {
        self.background_color = Some(color);
        self
    }

    pub fn bold(&mut self) -> &mut Self {
        self.bold = true;
        self
    }

    pub fn dim(&mut self) -> &mut Self {
        self.dim = true;
        self
    }

    pub fn italic(&mut self) -> &mut Self {
        self.italic = true;
        self
    }

    pub fn underline(&mut self) -> &mut Self {
        self.underline = true;
        self
    }

    pub fn blink(&mut self) -> &mut Self {
        self.blink = true;
        self
    }

    pub fn blink2(&mut self) -> &mut Self {
        self.blink2 = true;
        self
    }

    pub fn reverse(&mut self) -> &mut Self {
        self.reverse = true;
        self
    }

    pub fn conceal(&mut self) -> &mut Self {
        self.conceal = true;
        self
    }

    pub fn strike(&mut self) -> &mut Self {
        self.strike = true;
        self
    }

    pub fn underline2(&mut self) -> &mut Self {
        self.underline2 = true;
        self
    }

    pub fn frame(&mut self) -> &mut Self {
        self.frame = true;
        self
    }

    pub fn encircle(&mut self) -> &mut Self {
        self.encircle = true;
        self
    }

    pub fn overline(&mut self) -> &mut Self {
        self.overline = true;
        self
    }

    pub fn with_link(&mut self, link: String) -> &mut Self {
        self.link = Some(link);
        self
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

        Self {
            ansi: None,
            style_definition: None,
            color,
            background_color,
            set_attributes,
            attributes,
            link: link.clone(),
            link_id: link
                .map(|_| uuid::Uuid::new_v4().to_string())
                .unwrap_or("".to_string()),
            null,
        }
    }

    pub fn null() -> Self {
        Self::default()
    }
}
