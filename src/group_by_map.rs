use std::collections::HashMap;
use std::hash::Hash;

/// Maps each item to a key-value pair and groups the values by key.
///
/// Values within each group keep input order. The transform runs once per
/// item, and neither the input item nor the key needs to be cloned.
///
/// **Time Complexity:** O(n) average case, where `n` is the collection length.
/// **Space Complexity:** O(n) for the grouped values.
///
/// # Examples
/// ```rust
/// use lowdash::group_by_map;
///
/// let numbers = [1, 2, 3, 4];
/// let grouped = group_by_map(&numbers, |number| (number % 2, number * 10));
///
/// assert_eq!(grouped.get(&0), Some(&vec![20, 40]));
/// assert_eq!(grouped.get(&1), Some(&vec![10, 30]));
/// ```
pub fn group_by_map<T, K, V, F>(collection: &[T], transform: F) -> HashMap<K, Vec<V>>
where
    K: Eq + Hash,
    F: Fn(&T) -> (K, V),
{
    let mut result: HashMap<K, Vec<V>> = HashMap::new();

    for item in collection {
        let (key, value) = transform(item);
        result.entry(key).or_default().push(value);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn groups_transformed_values_in_input_order() {
        let values = [1, 2, 3, 4, 5];
        let grouped = group_by_map(&values, |value| (value % 2, value * 10));

        assert_eq!(grouped.get(&0), Some(&vec![20, 40]));
        assert_eq!(grouped.get(&1), Some(&vec![10, 30, 50]));
    }

    #[test]
    fn calls_transform_once_per_item() {
        let calls = Cell::new(0);

        let grouped = group_by_map(&[1, 2, 3], |value| {
            calls.set(calls.get() + 1);
            (value % 2, *value)
        });

        assert_eq!(calls.get(), 3);
        assert_eq!(grouped.get(&0), Some(&vec![2]));
        assert_eq!(grouped.get(&1), Some(&vec![1, 3]));
    }

    #[test]
    fn accepts_non_clone_keys() {
        #[derive(Debug, Eq, Hash, PartialEq)]
        struct Key(u8);

        let grouped = group_by_map(&[1, 2, 1], |value| (Key(*value), *value));

        assert_eq!(grouped.get(&Key(1)), Some(&vec![1, 1]));
        assert_eq!(grouped.get(&Key(2)), Some(&vec![2]));
    }

    #[test]
    fn returns_an_empty_map_for_empty_input() {
        let grouped = group_by_map::<i32, i32, i32, _>(&[], |value| (*value, *value));

        assert!(grouped.is_empty());
    }
}
