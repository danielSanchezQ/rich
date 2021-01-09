use cells::{cell_len, set_cell_size, DEFAULT_CELL_LEN_CACHE};
use itertools::{EitherOrBoth, Itertools};
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
    pub fn filter_control<'a, Segments>(
        segments: Segments,
        is_control: bool,
    ) -> impl Iterator<Item = &'a Segment> + 'a
    where
        Segments: IntoIterator<Item = &'a Segment> + 'a,
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

    /// Get the length of list of segments
    pub fn get_line_length(line: &[Segment]) -> usize {
        line.iter().map(|s: &Segment| s.cell_len()).sum()
    }

    /// Get the shape (enclosing rectangle) of a list of lines
    pub fn get_shape(lines: &[&[Segment]]) -> (usize, usize) {
        let max_width = lines
            .iter()
            .map(|s| Segment::get_line_length(s))
            .max()
            .unwrap_or(0);
        (max_width, lines.len())
    }

    /// Set the shape of a list of lines (enclosing rectangle)
    pub fn set_shape(
        lines: &[&[Segment]],
        width: usize,
        height: Option<usize>,
        style: Option<Style>,
    ) -> Vec<Vec<Segment>> {
        use itertools::EitherOrBoth::{Both, Left, Right};
        let height = height.unwrap_or(lines.len());
        let pad_line = [Segment::new(&" ".repeat(width), style.clone(), false)];
        lines
            .iter()
            .zip_longest((0..height).into_iter())
            .map(|e| match e {
                Both(line, _) | Left(line) => {
                    Segment::adjust_line_length(line, width, style.clone(), None)
                }
                Right(_) => pad_line.to_vec(),
            })
            .collect()
    }

    /// Simplify an iterable of segments by combining contiguous segments with the same style
    pub fn simplify<'a, Segments>(segments: Segments) -> impl Iterator<Item = Segment> + 'a
    where
        Segments: IntoIterator<Item = &'a Segment> + 'a,
    {
        SimplifiedSegments::new(segments.into_iter())
    }

    /// Remove all links from an iterable of styles
    pub fn strip_links<'a, Segments>(segments: Segments) -> impl Iterator<Item = Segment> + 'a
    where
        Segments: IntoIterator<Item = &'a Segment> + 'a,
    {
        segments.into_iter().map(|segment: &Segment| {
            if segment.is_control || segment.style.is_none() {
                segment.clone()
            } else {
                Segment::new(
                    &segment.text,
                    segment.style.clone().map(|style| style.update_link(None)),
                    false,
                )
            }
        })
    }

    /// Remove all styles from an iterable of segments
    pub fn strip_styles<'a, Segments>(segments: Segments) -> impl Iterator<Item = Segment> + 'a
    where
        Segments: IntoIterator<Item = &'a Segment> + 'a,
    {
        segments
            .into_iter()
            .map(|segment: &Segment| Segment::new(&segment.text, None, segment.is_control))
    }
}

struct SimplifiedSegments<'a, Segments: Iterator<Item = &'a Segment>> {
    inner: Segments,
    last_segment: Option<Segment>,
}

impl<'a, Segments: Iterator<Item = &'a Segment>> SimplifiedSegments<'a, Segments> {
    fn new(segments: Segments) -> Self {
        Self {
            inner: segments,
            last_segment: None,
        }
    }
}

impl<'a, Segments: Iterator<Item = &'a Segment>> Iterator for SimplifiedSegments<'a, Segments> {
    type Item = Segment;

    fn next(&mut self) -> Option<Self::Item> {
        let last_segment = if self.last_segment.is_none() {
            self.inner.next().cloned()
        } else {
            let last = self.last_segment.clone();
            self.last_segment = None;
            last
        };

        if let Some(last_segment) = last_segment {
            let mut last_segment = last_segment.clone();
            while let Some(segment) = self.inner.next() {
                if last_segment.style == segment.style && !segment.is_control {
                    last_segment = Segment::new(
                        &format!("{}{}", last_segment.text, segment.text),
                        last_segment.style.clone(),
                        false,
                    );
                } else {
                    self.last_segment = Some(segment.clone());
                    return Some(last_segment);
                }
            }
            Some(last_segment.clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Segment;
    use color::Color;
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

    #[test]
    fn test_get_line_length() {
        let lines = [
            Segment::new("foo", None, false),
            Segment::new("bar", None, false),
        ];
        assert_eq!(Segment::get_line_length(&lines), 6)
    }

    #[test]
    fn test_get_shape() {
        assert_eq!(
            Segment::get_shape(&[&[Segment::new("Hello", None, false)]]),
            (5, 1)
        );
        assert_eq!(
            Segment::get_shape(&[
                &[Segment::new("Hello", None, false)],
                &[Segment::new("World!", None, false)]
            ]),
            (6, 2)
        );
    }

    #[test]
    fn test_set_shape() {
        let segment = Segment::new("Hello", None, false);
        let gap_segment = Segment::new("     ", None, false);
        assert_eq!(
            Segment::set_shape(&[&[segment.clone()]], 10, None, None),
            [[segment.clone(), gap_segment.clone()]]
        );

        assert_eq!(
            Segment::set_shape(&[&[segment.clone()]], 10, Some(2), None),
            [
                vec![segment.clone(), gap_segment.clone()],
                vec![Segment::new(&" ".repeat(10), None, false)]
            ]
        );
    }

    #[test]
    fn test_simplify() {
        let segments = [
            Segment::new("Hello", None, false),
            Segment::new(" ", None, false),
            Segment::new("World!", None, false),
        ];
        let expected = [Segment::new("Hello World!", None, false)];
        assert_eq!(
            Segment::simplify(&segments).collect::<Vec<Segment>>(),
            expected
        );

        let red_style = StyleBuilder::new()
            .with_color(Color::parse("red").unwrap())
            .build();
        let blue_style = StyleBuilder::new()
            .with_color(Color::parse("blue").unwrap())
            .build();
        let segments = [
            Segment::new("Hello", Some(red_style.clone()), false),
            Segment::new(" ", Some(red_style.clone()), false),
            Segment::new("World!", Some(blue_style.clone()), false),
        ];
        let expected = [
            Segment::new("Hello ", Some(red_style.clone()), false),
            Segment::new("World!", Some(blue_style.clone()), false),
        ];
        assert_eq!(
            Segment::simplify(&segments).collect::<Vec<Segment>>(),
            expected
        );

        assert_eq!(Segment::simplify(&[]).collect::<Vec<Segment>>(), []);
    }

    #[test]
    fn test_filter_control() {
        let segments = [
            Segment::new("foo", None, false),
            Segment::new("bar", None, true),
        ];

        assert_eq!(
            Segment::filter_control(&segments, false)
                .cloned()
                .collect::<Vec<Segment>>(),
            [Segment::new("foo", None, false)]
        );

        assert_eq!(
            Segment::filter_control(&segments, true)
                .cloned()
                .collect::<Vec<Segment>>(),
            [Segment::new("bar", None, true)]
        )
    }

    #[test]
    fn test_strip_styles() {
        let segments = [Segment::new(
            "foo",
            Some(
                StyleBuilder::new()
                    .with_attribute(StyleAttribute::BOLD, true)
                    .build(),
            ),
            false,
        )];

        assert_eq!(
            Segment::strip_styles(&segments).collect::<Vec<Segment>>(),
            [Segment::new("foo", None, false)]
        );
    }

    #[test]
    fn test_strip_links() {
        let segments = [Segment::new(
            "foo",
            Some(
                StyleBuilder::new()
                    .with_attribute(StyleAttribute::BOLD, true)
                    .with_link("https://www.example.org")
                    .build(),
            ),
            false,
        )];

        assert_eq!(
            Segment::strip_links(&segments).collect::<Vec<Segment>>(),
            [Segment::new(
                "foo",
                Some(
                    StyleBuilder::new()
                        .with_attribute(StyleAttribute::BOLD, true)
                        .build()
                ),
                false
            )]
        );
    }
}
