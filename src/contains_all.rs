/// Returns `true` when every value in `subset` occurs in `collection`.
///
/// Membership is checked by value and duplicate values in `subset` do not
/// require duplicate occurrences in `collection`. An empty subset matches.
///
/// **Time Complexity:** O(n * m), where `n` is the collection length and `m` is
/// the subset length.
/// **Space Complexity:** O(1) auxiliary space.
///
/// # Examples
/// ```rust
/// use lowdash::contains_all;
///
/// assert!(contains_all(&[1, 2, 3, 4], &[2, 4]));
/// assert!(!contains_all(&[1, 2, 3, 4], &[2, 5]));
/// ```
pub fn contains_all<T>(collection: &[T], subset: &[T]) -> bool
where
    T: PartialEq,
{
    subset.iter().all(|item| collection.contains(item))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requires_every_subset_value() {
        assert!(contains_all(&[1, 2, 3, 4], &[2, 4]));
        assert!(!contains_all(&[1, 2, 3, 4], &[2, 5]));
    }

    #[test]
    fn ignores_duplicate_subset_values() {
        assert!(contains_all(&[1, 2], &[1, 1, 2, 2]));
    }

    #[test]
    fn an_empty_subset_always_matches() {
        assert!(contains_all::<i32>(&[], &[]));
        assert!(contains_all(&[1, 2], &[]));
    }

    #[test]
    fn accepts_non_clone_values() {
        #[derive(PartialEq)]
        struct Item(u8);

        let collection = [Item(1), Item(2)];
        let subset = [Item(2)];

        assert!(contains_all(&collection, &subset));
    }

    #[test]
    fn follows_partial_eq_for_nan() {
        assert!(!contains_all(&[f64::NAN], &[f64::NAN]));
    }
}
