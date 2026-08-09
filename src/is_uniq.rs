/// Returns `true` when every item occurs at most once.
///
/// Equality uses `PartialEq`, so incomparable values such as `NaN` follow the
/// normal `PartialEq` behavior.
///
/// **Time Complexity:** O(n²), where `n` is the collection length.
/// **Space Complexity:** O(1) auxiliary space.
///
/// # Examples
/// ```rust
/// use lowdash::is_uniq;
///
/// assert!(is_uniq(&[1, 2, 3]));
/// assert!(!is_uniq(&[1, 2, 1]));
/// ```
pub fn is_uniq<T>(collection: &[T]) -> bool
where
    T: PartialEq,
{
    for (index, item) in collection.iter().enumerate() {
        if collection[..index].contains(item) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_true_for_empty_and_unique_collections() {
        assert!(is_uniq::<i32>(&[]));
        assert!(is_uniq(&[1, 2, 3]));
    }

    #[test]
    fn stops_when_a_duplicate_is_found() {
        assert!(!is_uniq(&[1, 2, 3, 2, 4]));
    }

    #[test]
    fn does_not_require_clone() {
        #[derive(PartialEq)]
        struct NonClone(u8);

        let values = [NonClone(1), NonClone(2), NonClone(1)];

        assert!(!is_uniq(&values));
    }

    #[test]
    fn follows_partial_eq_for_nan() {
        assert!(is_uniq(&[f64::NAN, f64::NAN]));
    }
}
