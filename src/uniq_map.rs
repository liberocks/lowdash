/// Maps each item and keeps only the first occurrence of every mapped value.
///
/// The transform receives the item and its zero-based index. Equality uses
/// `PartialEq`, and the mapped value is stored directly without requiring
/// `Clone`.
///
/// **Time Complexity:** O(n²), where `n` is the collection length.
/// **Space Complexity:** O(n) for the result.
///
/// # Examples
/// ```rust
/// use lowdash::uniq_map;
///
/// let names = ["Ada", "Ada", "Linus"];
/// assert_eq!(uniq_map(&names, |name, _| name.len()), vec![3, 5]);
/// ```
pub fn uniq_map<T, U, F>(collection: &[T], transform: F) -> Vec<U>
where
    U: PartialEq,
    F: Fn(&T, usize) -> U,
{
    let mut result = Vec::with_capacity(collection.len());

    for (index, item) in collection.iter().enumerate() {
        let value = transform(item, index);
        if !result.contains(&value) {
            result.push(value);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn maps_and_deduplicates_in_first_occurrence_order() {
        let values = [1, 2, 3, 4, 5];

        assert_eq!(uniq_map(&values, |value, _| value % 3), vec![1, 2, 0]);
    }

    #[test]
    fn passes_zero_based_indexes() {
        let values = [10, 20, 30];

        assert_eq!(
            uniq_map(&values, |value, index| value + index),
            vec![10, 21, 32]
        );
    }

    #[test]
    fn transforms_every_item() {
        let calls = Cell::new(0);

        assert_eq!(
            uniq_map(&[1, 1, 2], |value, _| {
                calls.set(calls.get() + 1);
                *value
            }),
            vec![1, 2]
        );
        assert_eq!(calls.get(), 3);
    }

    #[test]
    fn accepts_non_clone_mapped_values() {
        #[derive(Debug, PartialEq)]
        struct Key(u8);

        let result = uniq_map(&[1, 2, 1], |value, _| Key(*value));

        assert_eq!(result, vec![Key(1), Key(2)]);
    }

    #[test]
    fn follows_partial_eq_for_nan() {
        let result = uniq_map(&[1, 2], |_, _| f64::NAN);

        assert_eq!(result.len(), 2);
        assert!(result.iter().all(|value| value.is_nan()));
    }
}
