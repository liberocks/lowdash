/// Splits a collection around the first occurrence of `separator`.
///
/// The separator is omitted from the returned slices. An empty separator
/// matches at the beginning. The returned slices borrow from `collection`, so
/// this operation does not allocate.
///
/// **Time Complexity:** O(n * m), where `n` is the collection length and `m` is
/// the separator length.
/// **Space Complexity:** O(1) auxiliary space.
///
/// # Examples
/// ```rust
/// use lowdash::cut;
///
/// let result = cut(&[1, 2, 3, 4], &[2, 3]);
/// assert_eq!(result, Some((&[1][..], &[4][..])));
/// ```
pub fn cut<'a, T>(collection: &'a [T], separator: &[T]) -> Option<(&'a [T], &'a [T])>
where
    T: PartialEq,
{
    if separator.is_empty() {
        return Some((&collection[..0], collection));
    }

    collection
        .windows(separator.len())
        .position(|window| window == separator)
        .map(|index| (&collection[..index], &collection[index + separator.len()..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_at_the_first_separator() {
        assert_eq!(
            cut(&[1, 2, 3, 2, 4], &[2]),
            Some((&[1][..], &[3, 2, 4][..]))
        );
    }

    #[test]
    fn omits_a_multi_item_separator() {
        assert_eq!(cut(&[1, 2, 3, 4], &[2, 3]), Some((&[1][..], &[4][..])));
    }

    #[test]
    fn returns_none_when_separator_is_missing_or_too_large() {
        assert_eq!(cut(&[1, 2, 3], &[4]), None);
        assert_eq!(cut(&[1, 2], &[1, 2, 3]), None);
    }

    #[test]
    fn empty_separator_matches_at_the_beginning() {
        assert_eq!(cut(&[1, 2], &[]), Some((&[][..], &[1, 2][..])));
    }

    #[test]
    fn accepts_non_clone_values() {
        #[derive(Debug, PartialEq)]
        struct Item(u8);

        let values = [Item(1), Item(2), Item(3)];
        let (before, after) = cut(&values, &[Item(2)]).unwrap();

        assert_eq!(before, &[Item(1)]);
        assert_eq!(after, &[Item(3)]);
    }
}
