use std::iter;
use std::iter::Peekable;

pub fn loop_first<Values, T>(values: Values) -> impl Iterator<Item = (bool, T)>
where
    Values: IntoIterator<Item = T>,
{
    values
        .into_iter()
        .zip([true].iter().cloned().chain(iter::repeat(false)))
        .map(|(a, b)| (b, a))
}

struct LoopLastIterator<T, Values: Iterator<Item = T>> {
    inner: Peekable<Values>,
}

impl<T, Values: Iterator<Item = T>> LoopLastIterator<T, Values> {
    fn new(values: Values) -> Self {
        Self {
            inner: values.into_iter().peekable(),
        }
    }
}

impl<T, Values: Iterator<Item = T>> Iterator for LoopLastIterator<T, Values> {
    type Item = (bool, T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.inner.next() {
            if self.inner.peek().is_none() {
                Some((true, value))
            } else {
                Some((false, value))
            }
        } else {
            None
        }
    }
}

pub fn loop_last<Values, T>(values: Values) -> impl Iterator<Item = (bool, T)>
where
    Values: IntoIterator<Item = T>,
{
    LoopLastIterator::new(values.into_iter())
}

pub fn loop_first_last<Values, T>(values: Values) -> impl Iterator<Item = (bool, T)>
where
    Values: IntoIterator<Item = T>,
{
    LoopLastIterator::new(values.into_iter())
        .zip([true].iter().cloned().chain(iter::repeat(false)))
        .map(|((flag1, value), flag2)| (flag1 || flag2, value))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ITERABLE: [&'static str; 4] = ["a", "b", "c", "d"];

    #[test]
    fn test_loop_first() {
        let empty_vec: Vec<i32> = Vec::new();
        assert_eq!(loop_first(&empty_vec).collect::<Vec<_>>(), vec![]);
        let mut iter = loop_first(&TEST_ITERABLE);
        assert_eq!(iter.next().unwrap(), (true, &TEST_ITERABLE[0]));
        assert_eq!(iter.next().unwrap(), (false, &TEST_ITERABLE[1]));
        assert_eq!(iter.next().unwrap(), (false, &TEST_ITERABLE[2]));
        assert_eq!(iter.next().unwrap(), (false, &TEST_ITERABLE[3]));
    }

    #[test]
    fn test_loop_last() {
        let empty_vec: Vec<i32> = Vec::new();
        assert_eq!(loop_last(&empty_vec).collect::<Vec<_>>(), vec![]);
        let mut iter = loop_last(&TEST_ITERABLE);
        assert_eq!(iter.next().unwrap(), (false, &TEST_ITERABLE[0]));
        assert_eq!(iter.next().unwrap(), (false, &TEST_ITERABLE[1]));
        assert_eq!(iter.next().unwrap(), (false, &TEST_ITERABLE[2]));
        assert_eq!(iter.next().unwrap(), (true, &TEST_ITERABLE[3]));
    }

    #[test]
    fn test_loop_first_last() {
        let empty_vec: Vec<i32> = Vec::new();
        assert_eq!(loop_first_last(&empty_vec).collect::<Vec<_>>(), vec![]);
        let mut iter = loop_first_last(&TEST_ITERABLE);
        assert_eq!(iter.next().unwrap(), (true, &TEST_ITERABLE[0]));
        assert_eq!(iter.next().unwrap(), (false, &TEST_ITERABLE[1]));
        assert_eq!(iter.next().unwrap(), (false, &TEST_ITERABLE[2]));
        assert_eq!(iter.next().unwrap(), (true, &TEST_ITERABLE[3]));
    }
}
