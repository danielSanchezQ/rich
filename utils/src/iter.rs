use std::iter;
use std::iter::Peekable;

pub fn loop_first<Values, T>(values: Values) -> impl Iterator<Item = (bool, T)>
where
    Values: Iterator<Item = T>,
{
    values
        .zip([true].iter().cloned().chain(iter::repeat(false)))
        .map(|(a, b)| (b, a))
}

struct LoopLastIterator<T, Values: Iterator<Item = T>> {
    inner: Peekable<Values>,
}

impl<T, Values: Iterator<Item = T>> LoopLastIterator<T, Values> {
    fn new(values: Values) -> Self {
        Self {
            inner: values.peekable(),
        }
    }
}

impl<T, Values: Iterator<Item = T>> Iterator for LoopLastIterator<T, Values> {
    type Item = (bool, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.peek().is_none() {
            Some((true, self.inner.next().unwrap()))
        } else if let Some(value) = self.inner.next() {
            Some((false, value))
        } else {
            None
        }
    }
}

pub fn loop_last<Values, T>(values: Values) -> impl Iterator<Item = (bool, T)>
where
    Values: Iterator<Item = T>,
{
    LoopLastIterator::new(values)
}

pub fn loop_first_last<Values, T>(values: Values) -> impl Iterator<Item = (bool, T)>
where
    Values: Iterator<Item = T>,
{
    LoopLastIterator::new(values)
        .zip([true].iter().cloned().chain(iter::repeat(false)))
        .map(|((flag1, value), flag2)| (flag1 || flag2, value))
}
