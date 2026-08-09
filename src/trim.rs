/// Removes leading and trailing values that occur in `cutset`.
///
/// The cutset contains individual values, not a sequence. The returned slice
/// borrows from `collection`, so this operation does not allocate.
///
/// **Time Complexity:** O(n * m), where `n` is the collection length and `m` is
/// the cutset length.
/// **Space Complexity:** O(1) auxiliary space.
///
/// # Examples
/// ```rust
/// use lowdash::trim;
///
/// assert_eq!(trim(&[0, 1, 2, 0, 3, 0], &[0, 1]), &[2, 0, 3][..]);
/// ```
pub fn trim<'a, T>(collection: &'a [T], cutset: &[T]) -> &'a [T]
where
    T: PartialEq,
{
    let start = collection
        .iter()
        .position(|item| !cutset.contains(item))
        .unwrap_or(collection.len());
    let end = collection
        .iter()
        .rposition(|item| !cutset.contains(item))
        .map_or(0, |index| index + 1);

    if start >= end {
        &collection[..0]
    } else {
        &collection[start..end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_both_ends_using_a_cutset() {
        assert_eq!(trim(&[0, 1, 2, 0, 3, 0], &[0, 1]), &[2, 0, 3][..]);
    }

    #[test]
    fn keeps_values_in_the_middle() {
        assert_eq!(trim(&[0, 1, 2, 1, 0], &[0, 1]), &[2][..]);
    }

    #[test]
    fn handles_empty_cutsets_and_all_trimmed_inputs() {
        assert_eq!(trim(&[1, 2, 3], &[]), &[1, 2, 3][..]);
        assert_eq!(trim(&[1, 1], &[1]), &[][..]);
        assert_eq!(trim::<i32>(&[], &[1]), &[][..]);
    }

    #[test]
    fn accepts_non_clone_values() {
        #[derive(Debug, PartialEq)]
        struct Item(u8);

        let values = [Item(0), Item(1), Item(2), Item(0)];

        assert_eq!(trim(&values, &[Item(0), Item(1)]), &[Item(2)][..]);
    }

    #[test]
    fn follows_partial_eq_for_nan() {
        let values = [f64::NAN, 1.0, f64::NAN];
        let result = trim(&values, &[f64::NAN]);

        assert_eq!(result.len(), values.len());
        assert!(result[0].is_nan());
        assert_eq!(result[1], 1.0);
        assert!(result[2].is_nan());
    }
}
