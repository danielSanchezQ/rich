use segment::Segment;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

lazy_static::lazy_static! {
pub static ref STRIP_CONTROL_CODES : HashSet<char> = [8u8, 11, 12, 13].iter().map(|code| *code as char).collect();
}

/// A renderable that inserts a control code (non printable but may move cursor)
pub struct Control {
    control_codes: Segment,
}

impl Control {
    pub fn new(control_codes: &str) -> Self {
        Self {
            control_codes: Segment::control(control_codes, None),
        }
    }
}

pub fn strip_control_codes(text: &str, codes_set: &HashSet<char>) -> String {
    text.chars().filter(|c| !codes_set.contains(c)).collect()
}

impl Display for Control {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.control_codes.text())
    }
}
