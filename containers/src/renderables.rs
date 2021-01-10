use console::traits::Renderable;
use console::Console;
use measure::{Measure, Measurement};
use std::ops::Deref;

pub struct Renderables<T>(Vec<T>)
where
    T: Renderable + Clone;

impl<T: Renderable + Clone> Renderables<T> {
    pub fn from_slice(renderables: &[T]) -> Self {
        Self(renderables.to_vec())
    }

    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn append(&mut self, renderable: T) {
        self.0.push(renderable)
    }
}

impl<T: Renderable + Clone> IntoIterator for Renderables<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T: Renderable + Clone> Deref for Renderables<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<T: Renderable + Clone> Measure for Renderables<T> {
    fn measure(&self, console: &Console, max_width: usize) -> Measurement {
        let dimensions: Vec<Measurement> = self
            .0
            .iter()
            .map(|r| Measurement::get(console, r, Some(max_width)))
            .collect();

        if dimensions.is_empty() {
            return Measurement::new(1, 1);
        }

        let min = dimensions.iter().map(|m| m.minimum).min().unwrap();
        let max = dimensions.iter().map(|m| m.maximum).max().unwrap();
        Measurement::new(min, max)
    }
}
