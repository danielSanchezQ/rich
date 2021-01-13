use style::Style;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Span {
    start: usize,
    end: usize,
    style: Style,
}

#[derive(Clone)]
pub struct Text {}

impl Span {
    pub fn new(start: usize, end: usize, style: Style) -> Self {
        Self { start, end, style }
    }

    pub fn as_tuple(&self) -> (usize, usize, &Style) {
        (self.start, self.end, &self.style)
    }

    pub fn as_bool(&self) -> bool {
        self.end > self.start
    }

    /// Split a span in to 2 from a given offset
    pub fn split(&self, offset: usize) -> (Span, Option<Span>) {
        if offset < self.start {
            return (self.clone(), None);
        }
        if offset >= self.end {
            return (self.clone(), None);
        }
        let (start, end, style) = self.as_tuple();
        let span1 = Span::new(start, end.min(offset), style.clone());
        let span2 = Span::new(span1.end, end, style.clone());
        (span1, Some(span2))
    }

    /// Move start and end by a given offset
    pub fn with_offset(&self, offset: usize) -> Span {
        Span::new(self.start + offset, self.end + offset, self.style.clone())
    }

    /// Crop the span at the given offset
    pub fn right_crop(&self, offset: usize) -> Span {
        if offset >= self.end {
            return self.clone();
        }
        Span::new(self.start, offset.min(self.end), self.style.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::Span;
    use style::Style;

    #[test]
    fn test_span() {
        let span = Span::new(10, 11, Style::default());
        assert!(span.as_bool());
        assert_eq!(span.as_tuple(), (10, 11, &Style::default()))
    }

    #[test]
    fn test_span_split() {
        assert_eq!(
            Span::new(5, 10, Style::default()).split(2),
            (Span::new(5, 10, Style::default()), None)
        );

        assert_eq!(
            Span::new(5, 10, Style::default()).split(15),
            (Span::new(5, 10, Style::default()), None)
        );

        assert_eq!(
            Span::new(0, 10, Style::default()).split(5),
            (
                Span::new(0, 5, Style::default()),
                Some(Span::new(5, 10, Style::default()))
            )
        );
    }

    #[test]
    fn test_span_with_offset() {
        assert_eq!(
            Span::new(5, 10, Style::default()).with_offset(2),
            Span::new(7, 12, Style::default())
        );
    }

    #[test]
    fn test_span_right_crop() {
        assert_eq!(
            Span::new(5, 10, Style::default()).right_crop(15),
            Span::new(5, 10, Style::default())
        );

        assert_eq!(
            Span::new(5, 10, Style::default()).right_crop(7),
            Span::new(5, 7, Style::default())
        );
    }
}
