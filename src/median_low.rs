/// Returns the lower middle value after sorting a copy of `values`.
///
/// For an odd number of values this is the ordinary middle item. For an even
/// number it is the item immediately below the midpoint. Empty input returns
/// `None`; the input is not changed.
///
/// **Time Complexity:** O(n log n) time and O(n) space.
///
/// # Examples
/// ```rust
/// use lowdash::median_low;
///
/// assert_eq!(median_low(&[1, 4, 2, 3]), Some(2));
/// assert_eq!(median_low(&[1, 3, 2]), Some(2));
/// ```
pub fn median_low<T>(values: &[T]) -> Option<T>
where
    T: Ord + Clone,
{
    if values.is_empty() {
        return None;
    }

    let mut sorted = values.to_vec();
    sorted.sort();
    Some(sorted[(sorted.len() - 1) / 2].clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_lower_middle_for_even_and_middle_for_odd_lengths() {
        assert_eq!(median_low(&[1, 4, 2, 3]), Some(2));
        assert_eq!(median_low(&[1, 3, 2]), Some(2));
    }

    #[test]
    fn handles_empty_and_singleton_inputs() {
        assert_eq!(median_low::<i32>(&[]), None);
        assert_eq!(median_low(&[42]), Some(42));
    }

    #[test]
    fn preserves_duplicates_and_negative_ordering() {
        assert_eq!(median_low(&[-5, -1, -3, -3]), Some(-3));
        assert_eq!(median_low(&[1, 2, 2, 2]), Some(2));
    }

    #[test]
    fn does_not_change_the_input() {
        let values = [4, 1, 3, 2];

        assert_eq!(median_low(&values), Some(2));
        assert_eq!(values, [4, 1, 3, 2]);
    }

    #[test]
    fn supports_custom_ordered_types() {
        #[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
        struct Person {
            age: u8,
            name: &'static str,
        }

        let people = [
            Person { age: 30, name: "a" },
            Person { age: 20, name: "b" },
            Person { age: 40, name: "c" },
            Person { age: 10, name: "d" },
        ];

        assert_eq!(median_low(&people), Some(Person { age: 20, name: "b" }));
    }
}
