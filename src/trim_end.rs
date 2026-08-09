/// Removes trailing values that occur in `cutset`.
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
/// use lowdash::trim_end;
///
/// assert_eq!(trim_end(&[0, 1, 2, 0, 3, 0], &[0, 3]), &[0, 1, 2][..]);
/// ```
pub fn trim_end<'a, T>(collection: &'a [T], cutset: &[T]) -> &'a [T]
where
    T: PartialEq,
{
    let mut end = collection.len();
    while end > 0 && cutset.contains(&collection[end - 1]) {
        end -= 1;
    }

    &collection[..end]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_only_trailing_values() {
        assert_eq!(trim_end(&[0, 1, 2, 0, 3, 0], &[0, 3]), &[0, 1, 2][..]);
    }

    #[test]
    fn keeps_leading_cutset_values() {
        assert_eq!(trim_end(&[0, 1, 2, 1, 0], &[0, 1]), &[0, 1, 2][..]);
    }

    #[test]
    fn handles_empty_cutsets_and_all_trimmed_inputs() {
        assert_eq!(trim_end(&[1, 2, 3], &[]), &[1, 2, 3][..]);
        assert_eq!(trim_end(&[1, 1], &[1]), &[][..]);
        assert_eq!(trim_end::<i32>(&[], &[1]), &[][..]);
    }

    #[test]
    fn accepts_non_clone_values() {
        #[derive(Debug, PartialEq)]
        struct Item(u8);

        let values = [Item(2), Item(0), Item(1)];

        assert_eq!(trim_end(&values, &[Item(0), Item(1)]), &[Item(2)][..]);
    }
}
