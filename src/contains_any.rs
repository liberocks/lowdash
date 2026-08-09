/// Returns `true` when at least one value in `subset` occurs in `collection`.
///
/// An empty subset does not match. Membership is checked by value, and the
/// search stops at the first match.
///
/// **Time Complexity:** O(n * m), where `n` is the collection length and `m` is
/// the subset length.
/// **Space Complexity:** O(1) auxiliary space.
///
/// # Examples
/// ```rust
/// use lowdash::contains_any;
///
/// assert!(contains_any(&[1, 2, 3, 4], &[0, 2]));
/// assert!(!contains_any(&[1, 2, 3, 4], &[0, 5]));
/// ```
pub fn contains_any<T>(collection: &[T], subset: &[T]) -> bool
where
    T: PartialEq,
{
    subset.iter().any(|item| collection.contains(item))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_when_any_subset_value_is_present() {
        assert!(contains_any(&[1, 2, 3, 4], &[0, 2]));
        assert!(!contains_any(&[1, 2, 3, 4], &[0, 5]));
    }

    #[test]
    fn an_empty_subset_does_not_match() {
        assert!(!contains_any::<i32>(&[], &[]));
        assert!(!contains_any(&[1, 2], &[]));
    }

    #[test]
    fn accepts_non_clone_values() {
        #[derive(PartialEq)]
        struct Item(u8);

        let collection = [Item(1), Item(2)];
        let subset = [Item(3), Item(2)];

        assert!(contains_any(&collection, &subset));
    }

    #[test]
    fn follows_partial_eq_for_nan() {
        assert!(!contains_any(&[f64::NAN], &[f64::NAN]));
    }
}
