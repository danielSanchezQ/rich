use console::traits::Renderable;
use console::Console;

pub trait Measure {
    fn measure(&self, console: &Console, max_width: usize) -> Measurement;
}

/// Stores the minimum and maximum widths (in characters) required to render an object
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Measurement {
    /// Minimum number of cells required to render
    pub minimum: usize,
    /// Maximum number of cells required to render
    pub maximum: usize,
}

impl Measurement {
    pub fn new(minimum: usize, maximum: usize) -> Self {
        Self { minimum, maximum }
    }
    /// Get difference between maximum and minimum
    pub fn span(&self) -> i32 {
        self.maximum as i32 - self.minimum as i32
    }

    /// Get a `(minimum, maximum)` tuple
    pub fn as_tuple(&self) -> (usize, usize) {
        (self.minimum, self.maximum)
    }

    /// Get measurement that ensures that minimum <= maximum and minimum >= 0
    pub fn normalized(&self) -> Self {
        let (mut min, max) = self.as_tuple();
        min = 0.max(min).min(max);
        Self {
            minimum: 0.max(min),
            maximum: 0.max(min.max(max)),
        }
    }

    /// Get a Measurement where the widths are <= width
    pub fn with_maximum(&self, width: usize) -> Self {
        let (min, max) = self.as_tuple();
        Self {
            minimum: min.min(width),
            maximum: max.min(width),
        }
    }

    /// Get a Measurement where the widths are >= width
    pub fn with_minimum(&self, width: usize) -> Self {
        let (min, max) = self.as_tuple();
        let width = 0.max(width);
        Self {
            minimum: min.max(width),
            maximum: max.max(width),
        }
    }

    /// Clamp a measurement within the specified range
    pub fn clamp(&self, min_width: Option<usize>, max_width: Option<usize>) -> Self {
        let mut measurement = self.clone();
        if let Some(min_width) = min_width {
            measurement = measurement.with_minimum(min_width);
        }
        if let Some(max_width) = max_width {
            measurement = measurement.with_maximum(max_width);
        }
        measurement
    }

    pub fn get<R>(console: &Console, rendereable: R, max_width: Option<usize>) -> Self
    where
        R: Renderable,
    {
        // TODO: implement this when console is ready
        unimplemented!()
    }
}

/// Get a measurement that would fit a number of renderables
pub fn measure_renderables<Renderables>(
    console: &Console,
    renderables: Renderables,
    max_width: usize,
) -> Measurement
where
    Renderables: IntoIterator<Item = dyn Renderable>,
{
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use crate::Measurement;

    #[test]
    fn test_span() {
        let measurement = Measurement::new(10, 100);
        assert_eq!(measurement.span(), 90);
    }

    #[test]
    fn test_clamp() {
        let measurement = Measurement::new(20, 100);
        assert_eq!(
            measurement.clamp(Some(10), Some(50)),
            Measurement::new(20, 50)
        );
        assert_eq!(
            measurement.clamp(Some(30), Some(50)),
            Measurement::new(30, 50)
        );
        assert_eq!(measurement.clamp(None, Some(50)), Measurement::new(20, 50));
        assert_eq!(measurement.clamp(Some(30), None), Measurement::new(30, 100));
        assert_eq!(measurement.clamp(None, None), Measurement::new(20, 100));
    }
}
