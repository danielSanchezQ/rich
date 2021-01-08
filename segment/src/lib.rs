use cells::{cell_len, set_cell_size, DEFAULT_CELL_LEN_CACHE};
use style::Style;

/// A piece of text with associated style
#[derive(Clone)]
pub struct Segment {
    /// Raw text
    text: String,
    /// An optional style
    style: Option<Style>,
    /// `true` if the segment contains control codes, `false` otherwise
    is_control: bool,
}

impl Segment {
    pub fn new(text: &str, style: Option<Style>, is_control: bool) -> Self {
        Self {
            text: text.to_string(),
            style,
            is_control,
        }
    }

    /// Create a Segment with control codes
    pub fn control(text: &str, style: Option<Style>) -> Self {
        Self {
            text: text.to_string(),
            style,
            is_control: true,
        }
    }

    /// Get segment attributes packed in a tuple
    pub fn as_tuple(&self) -> (&str, &Option<Style>, bool) {
        (&self.text, &self.style, self.is_control)
    }

    pub fn as_bool(&self) -> bool {
        !self.text.is_empty()
    }

    /// Get cell length of segment
    pub fn cell_len(&self) -> usize {
        if self.is_control {
            0
        } else {
            cell_len(&self.text, &mut DEFAULT_CELL_LEN_CACHE.lock().unwrap())
        }
    }

    pub fn make_control<Segments>(segments: Segments) -> Vec<Segment>
    where
        Segments: IntoIterator<Item = Segment>,
    {
        segments
            .into_iter()
            .map(|s| Segment::new(&s.text, s.style.clone(), true))
            .collect()
    }

    pub fn line(is_control: Option<bool>) -> Segment {
        Self::new("\n", None, is_control.unwrap_or(false))
    }

    /// Apply a style to an iterable of segments
    pub fn apply_style<Segments>(
        segments: Segments,
        style: Option<Style>,
    ) -> impl Iterator<Item = Segment>
    where
        Segments: IntoIterator<Item = Segment>,
    {
        segments.into_iter().map(move |s: Segment| {
            if let Some(style) = &style {
                Self::new(
                    &s.text,
                    if s.is_control {
                        None
                    } else {
                        Some(style.combine(s.style.as_ref()))
                    },
                    s.is_control,
                )
            } else {
                s
            }
        })
    }

    /// Filter segments by ``is_control`` attribute
    pub fn filter_control<Segments>(
        segments: Segments,
        is_control: bool,
    ) -> impl Iterator<Item = Segment>
    where
        Segments: IntoIterator<Item = Segment>,
    {
        segments
            .into_iter()
            .filter(move |s| s.is_control == is_control)
    }

    /// Split a sequence of segments in to a list of lines
    pub fn split_lines<Segments>(segments: Segments) -> impl Iterator<Item = Segment>
    where
        Segments: IntoIterator<Item = Segment>,
    {
        segments.into_iter()
    }
}
