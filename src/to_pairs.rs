use crate::entries;
use crate::entries::Entry;
use std::collections::HashMap;

/// Collects all entries from a map into a vector of `Entry` structs.
///
/// This function is an alias for the `entries` function and provides the same functionality.
/// It exists for semantic clarity in contexts where representing map entries as pairs is more intuitive.
///
/// # Arguments
/// * `map` - The input map from which to collect entries.
///
/// # Returns
/// * `Vec<Entry<K, V>>` - A vector containing all key-value pairs as `Entry` structs.
///
/// # Examples
/// ```rust
/// use lowdash::{Entry, to_pairs};
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("a", 1);
/// map.insert("b", 2);
///
/// let result = to_pairs(&map);
/// let expected = vec![
///     Entry { key: "a", value: 1 },
///     Entry { key: "b", value: 2 },
/// ];
///
/// let mut sorted_result = result.clone();
/// sorted_result.sort_by(|a, b| a.key.cmp(&b.key));
///
/// let mut sorted_expected = expected.clone();
/// sorted_expected.sort_by(|a, b| a.key.cmp(&b.key));
///
/// assert_eq!(sorted_result, sorted_expected);
/// ```
pub fn to_pairs<K, V>(map: &HashMap<K, V>) -> Vec<Entry<K, V>>
where
    K: Clone + std::cmp::Eq + std::hash::Hash,
    V: Clone,
{
    entries::entries(map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entries::Entry;
    use std::collections::HashMap;

    #[test]
    fn test_to_pairs_single_entry() {
        let mut map = HashMap::new();
        map.insert("a", 1);

        let result = to_pairs(&map);
        let expected = vec![Entry { key: "a", value: 1 }];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_pairs_multiple_entries() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);

        let result = to_pairs(&map);
        let expected = vec![
            Entry { key: "a", value: 1 },
            Entry { key: "b", value: 2 },
            Entry { key: "c", value: 3 },
        ];

        // Since HashMap does not guarantee order, sort both vectors before comparison
        let mut sorted_result = result.clone();
        sorted_result.sort_by(|a, b| a.key.cmp(&b.key));

        let mut sorted_expected = expected.clone();
        sorted_expected.sort_by(|a, b| a.key.cmp(&b.key));

        assert_eq!(sorted_result, sorted_expected);
    }

    #[test]
    fn test_to_pairs_empty_map() {
        let map: HashMap<&str, i32> = HashMap::new();

        let result = to_pairs(&map);
        let expected: Vec<Entry<&str, i32>> = vec![];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_pairs_with_integers() {
        let mut map = HashMap::new();
        map.insert(1, "one");
        map.insert(2, "two");

        let result = to_pairs(&map);
        let expected = vec![
            Entry {
                key: 1,
                value: "one",
            },
            Entry {
                key: 2,
                value: "two",
            },
        ];

        // Sort for comparison
        let mut sorted_result = result.clone();
        sorted_result.sort_by(|a, b| a.key.cmp(&b.key));

        let mut sorted_expected = expected.clone();
        sorted_expected.sort_by(|a, b| a.key.cmp(&b.key));

        assert_eq!(sorted_result, sorted_expected);
    }

    #[test]
    fn test_to_pairs_with_complex_values() {
        let mut map = HashMap::new();
        map.insert("a", vec![1, 2, 3]);
        map.insert("b", vec![4, 5]);
        map.insert("c", vec![6]);

        let result = to_pairs(&map);
        let expected = vec![
            Entry {
                key: "a",
                value: vec![1, 2, 3],
            },
            Entry {
                key: "b",
                value: vec![4, 5],
            },
            Entry {
                key: "c",
                value: vec![6],
            },
        ];

        // Since HashMap does not guarantee order, sort before comparison
        let mut sorted_result = result.clone();
        sorted_result.sort_by(|a, b| a.key.cmp(&b.key));

        let mut sorted_expected = expected.clone();
        sorted_expected.sort_by(|a, b| a.key.cmp(&b.key));

        assert_eq!(sorted_result, sorted_expected);
    }
}
