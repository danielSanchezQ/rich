use console::traits::Renderable;
use style::Style;

pub enum AlignMethod {
    Left,
    Center,
    Right,
}

/// Align a renderable by adding spaces if necessary
struct Align {
    inner_renderable: Box<dyn Renderable>,
    method: AlignMethod,
    style: Option<Style>,
    padding: bool,
    width: Option<usize>,
}

impl Align {
    pub fn new(
        renderable: Box<impl Renderable + 'static>,
        method: AlignMethod,
        style: Option<Style>,
        padding: Option<bool>,
        width: Option<usize>,
    ) -> Self {
        Self {
            inner_renderable: renderable,
            method,
            style,
            padding: padding.unwrap_or(true),
            width,
        }
    }

    pub fn left(
        renderable: Box<impl Renderable + 'static>,
        style: Option<Style>,
        padding: Option<bool>,
        width: Option<usize>,
    ) -> Self {
        Self::new(renderable, AlignMethod::Left, style, padding, width)
    }

    pub fn center(
        renderable: Box<impl Renderable + 'static>,
        style: Option<Style>,
        padding: Option<bool>,
        width: Option<usize>,
    ) -> Self {
        Self::new(renderable, AlignMethod::Center, style, padding, width)
    }

    pub fn right(
        renderable: Box<impl Renderable + 'static>,
        style: Option<Style>,
        padding: Option<bool>,
        width: Option<usize>,
    ) -> Self {
        Self::new(renderable, AlignMethod::Right, style, padding, width)
    }
}
