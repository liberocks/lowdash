/// Removes leading values that occur in `cutset`.
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
/// use lowdash::trim_start;
///
/// assert_eq!(trim_start(&[0, 1, 2, 0, 3], &[0, 1]), &[2, 0, 3][..]);
/// ```
pub fn trim_start<'a, T>(collection: &'a [T], cutset: &[T]) -> &'a [T]
where
    T: PartialEq,
{
    let start = collection
        .iter()
        .position(|item| !cutset.contains(item))
        .unwrap_or(collection.len());

    &collection[start..]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_only_leading_values() {
        assert_eq!(trim_start(&[0, 1, 2, 0, 3], &[0, 1]), &[2, 0, 3][..]);
    }

    #[test]
    fn keeps_trailing_cutset_values() {
        assert_eq!(trim_start(&[0, 1, 2, 1, 0], &[0, 1]), &[2, 1, 0][..]);
    }

    #[test]
    fn handles_empty_cutsets_and_all_trimmed_inputs() {
        assert_eq!(trim_start(&[1, 2, 3], &[]), &[1, 2, 3][..]);
        assert_eq!(trim_start(&[1, 1], &[1]), &[][..]);
        assert_eq!(trim_start::<i32>(&[], &[1]), &[][..]);
    }

    #[test]
    fn accepts_non_clone_values() {
        #[derive(Debug, PartialEq)]
        struct Item(u8);

        let values = [Item(0), Item(1), Item(2)];

        assert_eq!(trim_start(&values, &[Item(0), Item(1)]), &[Item(2)][..]);
    }
}
