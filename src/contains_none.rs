/// Returns `true` when no value in `subset` occurs in `collection`.
///
/// An empty subset matches. Membership is checked by value, and the search
/// stops at the first match.
///
/// **Time Complexity:** O(n * m), where `n` is the collection length and `m` is
/// the subset length.
/// **Space Complexity:** O(1) auxiliary space.
///
/// # Examples
/// ```rust
/// use lowdash::contains_none;
///
/// assert!(contains_none(&[1, 2, 3, 4], &[0, 5]));
/// assert!(!contains_none(&[1, 2, 3, 4], &[0, 2]));
/// ```
pub fn contains_none<T>(collection: &[T], subset: &[T]) -> bool
where
    T: PartialEq,
{
    !crate::contains_any(collection, subset)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_when_no_subset_value_is_present() {
        assert!(contains_none(&[1, 2, 3, 4], &[0, 5]));
        assert!(!contains_none(&[1, 2, 3, 4], &[0, 2]));
    }

    #[test]
    fn an_empty_subset_always_matches() {
        assert!(contains_none::<i32>(&[], &[]));
        assert!(contains_none(&[1, 2], &[]));
    }

    #[test]
    fn accepts_non_clone_values() {
        #[derive(PartialEq)]
        struct Item(u8);

        let collection = [Item(1), Item(2)];
        let subset = [Item(3)];

        assert!(contains_none(&collection, &subset));
    }

    #[test]
    fn follows_partial_eq_for_nan() {
        assert!(contains_none(&[f64::NAN], &[f64::NAN]));
    }
}
