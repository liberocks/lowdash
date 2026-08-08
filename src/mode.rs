use std::hash::Hash;

/// Returns the first encountered value with the highest frequency.
///
/// Ties follow first appearance order, matching [`crate::modes`]. Empty input
/// returns `None`.
///
/// **Time Complexity:** O(n) expected time and O(n) additional space.
///
/// # Examples
/// ```rust
/// use lowdash::mode;
///
/// assert_eq!(mode(&[1, 2, 2, 3, 3]), Some(2));
/// assert_eq!(mode::<i32>(&[]), None);
/// ```
pub fn mode<T>(values: &[T]) -> Option<T>
where
    T: Eq + Hash + Clone,
{
    crate::modes::modes(values).into_iter().next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_the_most_common_value() {
        assert_eq!(mode(&[1, 2, 2, 3, 3, 3]), Some(3));
    }

    #[test]
    fn returns_the_first_value_on_a_tie() {
        assert_eq!(mode(&[3, 1, 2, 3, 2, 1]), Some(3));
    }

    #[test]
    fn handles_empty_singleton_and_unique_inputs() {
        assert_eq!(mode::<i32>(&[]), None);
        assert_eq!(mode(&[7]), Some(7));
        assert_eq!(mode(&[1, 2, 3]), Some(1));
    }

    #[test]
    fn preserves_duplicate_counting() {
        assert_eq!(mode(&[4, 4, 2, 2, 2]), Some(2));
    }

    #[test]
    fn supports_custom_hashable_types() {
        #[derive(Debug, PartialEq, Eq, Hash, Clone)]
        struct Tag(&'static str);

        let tags = [Tag("red"), Tag("blue"), Tag("blue"), Tag("red"), Tag("red")];

        assert_eq!(mode(&tags), Some(Tag("red")));
    }
}
