/// Pick the first non-none bool or return the last value
pub fn pick_bool<'a, Values>(values: Values) -> bool
where
    Values: IntoIterator<Item = &'a Option<bool>>,
{
    // original algorithm asserts lenght: https://github.com/danielSanchezQ/rich-1/blob/master/rich/_pick.py
    // but we will denote an empty one as false
    // let mut peekable = values.into_iter().peekable();
    // assert!(peekable.peek().is_some(), "1 or more values required");
    for e in values.into_iter() {
        if let Some(value) = e {
            return *value;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::pick_bool;
    #[test]
    fn test_pick_bool() {
        assert_eq!(pick_bool(&[]), false);
        assert_eq!(pick_bool(&[Some(true)]), true);
        assert_eq!(pick_bool(&[Some(false), Some(false)]), false);
    }
}
