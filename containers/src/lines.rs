use std::ops::{Deref, DerefMut};

use console::{
    options::{JustifyMethod, OverflowMethod},
    Console,
};
use text::Text;

pub struct Lines {
    inner: Vec<Text>,
}

impl Lines {
    pub fn new<'a, L>(lines: L) -> Self
    where
        L: IntoIterator<Item = &'a Text>,
    {
        Self {
            inner: lines.into_iter().cloned().collect(),
        }
    }

    pub fn lines(&self) -> &[Text] {
        &self.inner
    }

    pub fn justify(
        &mut self,
        console: Console,
        width: usize,
        justify: Option<JustifyMethod>,
        overflow: Option<OverflowMethod>,
    ) {
        let justify = justify.unwrap_or(JustifyMethod::Left);
        let overflow = overflow.unwrap_or(OverflowMethod::Fold);
        match justify {
            JustifyMethod::Full => {}
            JustifyMethod::Left => {}
            JustifyMethod::Center => {}
            JustifyMethod::Right => {}
        }
        unimplemented!()
    }
}
