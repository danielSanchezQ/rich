#[derive(PartialEq)]
pub struct Encoding(&'static str);
pub struct JustifyMethod(&'static str);
pub struct OverflowMethod(&'static str);

impl Encoding {
    pub fn new(encoding: &'static str) -> Self {
        Self(encoding)
    }
}
/// Size of the terminal
pub struct ConsoleDimensions {
    width: usize,
    height: usize,
}

/// Options for `rich_console` method
pub struct ConsoleOptions {
    /// flag for legacy windows
    pub legacy_windows: bool,
    /// Minimum width of renderable
    pub min_width: usize,
    /// Maximum width of renderable
    pub max_width: usize,
    /// True if the target is a terminal, otherwise False
    pub is_terminal: bool,
    /// Encoding of terminal
    pub encoding: Encoding,
    /// Justify value override for renderable
    pub justify: Option<JustifyMethod>,
    /// Overflow value override for renderable
    pub overflow: Option<OverflowMethod>,
    // Deisable wrapping for text
    pub no_wrap: Option<bool>,
    /// Highlight override for render_str
    pub highlight: Option<bool>,
}

#[derive(Default)]
pub struct UpdateConsoleOptions {
    width: Option<usize>,
    min_width: Option<usize>,
    max_width: Option<usize>,
    justify: Option<JustifyMethod>,
    overflow: Option<OverflowMethod>,
    no_wrap: Option<bool>,
    highlight: Option<bool>,
}

impl ConsoleOptions {
    pub fn ascii_only(&self) -> bool {
        // TODO: actually check on encodings when they are implemented
        self.encoding != Encoding("utf-8")
    }

    /// Update ConsoleOptions values
    pub fn update(&mut self, other: UpdateConsoleOptions) {
        if let Some(width) = other.width {
            self.min_width = width;
            self.max_width = width;
        }
        if let Some(min_width) = other.min_width {
            self.min_width = min_width;
        }
        if let Some(max_width) = other.max_width {
            self.max_width = max_width;
        }
        if let Some(justify) = other.justify {
            self.justify = Some(justify);
        }
        if let Some(overflow) = other.overflow {
            self.overflow = Some(overflow);
        }
        if let Some(no_wrap) = other.no_wrap {
            self.no_wrap = Some(no_wrap);
        }
        if let Some(highlight) = other.highlight {
            self.highlight = Some(highlight);
        }
    }
}
