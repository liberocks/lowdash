use std::collections::HashMap;

/// Represents a key-value pair entry in a map.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Entry<K, V> {
    /// The key of the entry.
    pub key: K,
    /// The value of the entry.
    pub value: V,
}

/// Collects all entries from a map into a vector of `Entry` structs.
///
/// Iterates over each key-value pair in the input map and collects them into a vector.
///
/// # Arguments
/// * `map` - The input map from which to collect entries.
///
/// # Returns
/// * `Vec<Entry<K, V>>` - A vector containing all key-value pairs as `Entry` structs.
///
/// # Examples
/// ```rust
/// use lowdash::{Entry, entries};
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("a", 1);
/// map.insert("b", 2);
///
/// let result = entries(&map);
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
pub fn entries<K, V>(map: &HashMap<K, V>) -> Vec<Entry<K, V>>
where
    K: Clone + std::cmp::Eq + std::hash::Hash,
    V: Clone,
{
    map.iter()
        .map(|(k, v)| Entry {
            key: k.clone(),
            value: v.clone(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_entries_single_entry() {
        let mut map = HashMap::new();
        map.insert("a", 1);

        let result = entries(&map);
        let expected = vec![Entry { key: "a", value: 1 }];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_entries_multiple_entries() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);

        let result = entries(&map);
        let expected = vec![
            Entry { key: "a", value: 1 },
            Entry { key: "b", value: 2 },
            Entry { key: "c", value: 3 },
        ];

        // Since HashMap does not guarantee order, we need to sort both vectors before comparison
        let mut sorted_result = result.clone();
        sorted_result.sort_by(|a, b| a.key.cmp(&b.key));

        let mut sorted_expected = expected.clone();
        sorted_expected.sort_by(|a, b| a.key.cmp(&b.key));

        assert_eq!(sorted_result, sorted_expected);
    }

    #[test]
    fn test_entries_empty_map() {
        let map: HashMap<&str, i32> = HashMap::new();

        let result = entries(&map);
        let expected: Vec<Entry<&str, i32>> = vec![];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_entries_with_integers() {
        let mut map = HashMap::new();
        map.insert(1, "one");
        map.insert(2, "two");

        let result = entries(&map);
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
    fn test_entries_with_complex_values() {
        let mut map = HashMap::new();
        map.insert("a", vec![1, 2, 3]);
        map.insert("b", vec![4, 5]);
        map.insert("c", vec![6]);

        let result = entries(&map);
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
