use cells::{cell_len, set_cell_size, DEFAULT_CELL_LEN_CACHE};
use style::{Style, StyleBuilder};

/// A piece of text with associated style
#[derive(Clone, Eq, PartialEq, Debug)]
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
    pub fn apply_style<'a, Segments>(
        segments: Segments,
        style: Option<Style>,
    ) -> impl Iterator<Item = Segment> + 'a
    where
        Segments: IntoIterator<Item = &'a Segment> + 'a,
    {
        segments.into_iter().map(move |s: &Segment| {
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
                s.clone()
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

    /// Adjust a line to a given width (cropping or padding as required)
    pub fn adjust_line_length(
        line: &[Segment],
        length: usize,
        style: Option<Style>,
        padding: Option<bool>,
    ) -> Vec<Segment> {
        let padding = padding.unwrap_or(true);
        let line_length: usize = line.iter().map(|s| s.cell_len()).sum();
        if line_length < length {
            if padding {
                line.iter()
                    .chain(
                        [Segment::new(
                            &" ".repeat(length - line_length),
                            style,
                            false,
                        )]
                        .iter(),
                    )
                    .cloned()
                    .collect()
            } else {
                line.iter().cloned().collect()
            }
        } else if line_length > length {
            let mut new_line: Vec<Segment> = Vec::new();
            let mut line_length = 0;
            for segment in line {
                let segment_length = segment.cell_len();
                if (line_length + segment_length) < length || segment.is_control {
                    new_line.push(segment.clone());
                    line_length += segment_length;
                } else {
                    let (text, segment_style, _) = segment.as_tuple();
                    let text = set_cell_size(text, length - line_length);
                    new_line.push(Segment::new(&text, segment_style.clone(), false));
                }
            }
            new_line
        } else {
            line.iter().cloned().collect()
        }
    }

    /// Split a sequence of segments in to a list of lines
    pub fn split_lines<'a, Segments>(segments: Segments) -> Vec<Vec<Segment>>
    where
        Segments: IntoIterator<Item = &'a Segment>,
    {
        let mut res: Vec<Vec<Segment>> = Vec::new();
        let mut line: Vec<Segment> = Vec::new();
        for segment in segments {
            if segment.text.contains('\n') && !segment.is_control {
                let (mut text, style, _) = segment.as_tuple();
                while !text.is_empty() {
                    match text.splitn(2, '\n').collect::<Vec<&str>>().as_slice() {
                        [_text, next] => {
                            line.push(Segment::new(_text, style.clone(), false));
                            res.push(line);
                            line = Vec::new();
                            text = next;
                        }
                        [_text] => {
                            text = "";
                            line.push(Segment::new(_text, style.clone(), false));
                        }
                        _ => unreachable!(),
                    }
                }
            } else {
                line.push(segment.clone());
            }
        }
        if !line.is_empty() {
            res.push(line);
        }
        res
    }

    /// Split segments in to lines, and crop lines greater than a given length
    pub fn split_and_crop_lines<'a, Segments>(
        segments: Segments,
        length: usize,
        style: Option<Style>,
        padding: Option<bool>,
        include_new_lines: Option<bool>,
    ) -> Vec<Vec<Segment>>
    where
        Segments: IntoIterator<Item = &'a Segment>,
    {
        let include_new_lines = include_new_lines.unwrap_or(true);
        let mut res: Vec<Vec<Segment>> = Vec::new();
        let mut line: Vec<Segment> = Vec::new();
        let new_line_segment = Segment::line(None);

        for segment in segments {
            if segment.text.contains('\n') && !segment.is_control {
                let (mut text, style, _) = segment.as_tuple();
                while !text.is_empty() {
                    match text.splitn(2, '\n').collect::<Vec<&str>>().as_slice() {
                        [_text, next] => {
                            line.push(Segment::new(_text, style.clone(), false));
                            let mut cropped_line =
                                Segment::adjust_line_length(&line, length, style.clone(), padding);
                            if include_new_lines {
                                cropped_line.push(new_line_segment.clone());
                            }
                            res.push(cropped_line);
                            line = Vec::new();
                            text = next;
                        }
                        [_text] => {
                            line.push(Segment::new(_text, style.clone(), false));
                            text = "";
                        }
                        _ => unreachable!(),
                    }
                }
            } else {
                line.push(segment.clone());
            }
        }
        if !line.is_empty() {
            res.push(Segment::adjust_line_length(&line, length, style, padding));
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::Segment;
    use style::{Style, StyleAttribute, StyleBuilder};

    #[test]
    fn test_line() {
        assert_eq!(Segment::line(None), Segment::new("\n", None, false))
    }

    #[test]
    fn test_apply_style() {
        let bold = StyleBuilder::new()
            .with_attribute(StyleAttribute::BOLD, true)
            .build();

        let segments = [
            Segment::new("foo", None, false),
            Segment::new("bar", Some(bold.clone()), false),
        ];
        let computed: Vec<Segment> = Segment::apply_style(&segments, None).collect();
        assert_eq!(computed, segments);

        let italic = StyleBuilder::new()
            .with_attribute(StyleAttribute::ITALIC, true)
            .build();

        let italic_bold = StyleBuilder::new()
            .with_attribute(StyleAttribute::ITALIC, true)
            .with_attribute(StyleAttribute::BOLD, true)
            .build();

        let expected = [
            Segment::new("foo", Some(italic.clone()), false),
            Segment::new("bar", Some(italic_bold), false),
        ];
        let computed: Vec<Segment> = Segment::apply_style(&segments, Some(italic)).collect();
        assert_eq!(computed, expected);
    }

    #[test]
    fn test_split_lines() {
        let lines = [Segment::new("Hello\nWorld", None, false)];
        let expected = [
            [Segment::new("Hello", None, false)],
            [Segment::new("World", None, false)],
        ];
        let computed: Vec<Vec<Segment>> = Segment::split_lines(&lines);
        assert_eq!(computed, expected);
    }

    #[test]
    fn test_adjust_line_length() {
        let strike = StyleBuilder::new()
            .with_attribute(StyleAttribute::STRIKE, true)
            .build();

        let bold = StyleBuilder::new()
            .with_attribute(StyleAttribute::BOLD, true)
            .build();

        let line = [Segment::new("Hello", Some(strike.clone()), false)];
        let expected = [
            Segment::new("Hello", Some(strike.clone()), false),
            Segment::new("     ", Some(bold.clone()), false),
        ];
        assert_eq!(
            Segment::adjust_line_length(&line, 10, Some(bold), None),
            expected
        );

        let line = [
            Segment::new("H", None, false),
            Segment::new("ello, World!", None, false),
        ];
        let expected = [
            Segment::new("H", None, false),
            Segment::new("ello", None, false),
        ];
        assert_eq!(Segment::adjust_line_length(&line, 5, None, None), expected);

        let line = [Segment::new("Hello", None, false)];
        assert_eq!(Segment::adjust_line_length(&line, 5, None, None), line);
    }

    #[test]
    fn test_split_and_crop_lines() {
        let original = [
            Segment::new("Hello\nWorld!\n", None, false),
            Segment::new("foo", None, false),
        ];
        let result = Segment::split_and_crop_lines(&original, 4, None, None, None);
        let expected = [
            [Segment::new("Hell", None, false), Segment::line(None)],
            [Segment::new("Worl", None, false), Segment::line(None)],
            [
                Segment::new("foo", None, false),
                Segment::new(" ", None, false),
            ],
        ];
        assert_eq!(result, expected);
    }
}
