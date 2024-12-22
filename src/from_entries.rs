use crate::entries::Entry;
use std::collections::HashMap;

/// Constructs a `HashMap` from a slice of `Entry` structs.
///
/// Iterates over each `Entry` in the provided slice and inserts them into a new `HashMap`.
/// If duplicate keys are present, the value from the last `Entry` with that key will be used.
///
/// # Arguments
/// * `entries` - A slice of `Entry` structs to convert into a `HashMap`.
///
/// # Returns
/// * `HashMap<K, V>` - A new `HashMap` containing all key-value pairs from the `entries` slice.
///
/// # Examples
/// ```rust
/// use lowdash::{Entry, from_entries};
/// use std::collections::HashMap;
///
/// let entries = vec![
///     Entry { key: "a", value: 1 },
///     Entry { key: "b", value: 2 },
/// ];
///
/// let result = from_entries(&entries);
/// let mut expected = HashMap::new();
/// expected.insert("a", 1);
/// expected.insert("b", 2);
///
/// assert_eq!(result.len(), expected.len());
/// for (key, value) in &expected {
///    assert_eq!(result.get(key), Some(value));
/// }
/// ```
pub fn from_entries<K, V>(entries: &[Entry<K, V>]) -> HashMap<K, V>
where
    K: Clone + std::cmp::Eq + std::hash::Hash,
    V: Clone,
{
    let mut map = HashMap::with_capacity(entries.len());
    for entry in entries {
        map.insert(entry.key.clone(), entry.value.clone());
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entries::Entry;

    #[test]
    fn test_from_entries_single_entry() {
        let entries = vec![Entry { key: "a", value: 1 }];
        let result = from_entries(&entries);
        let mut expected = HashMap::new();
        expected.insert("a", 1);
        assert_eq!(result.len(), expected.len());
        for (key, value) in &expected {
            assert_eq!(result.get(key), Some(value));
        }

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_entries_multiple_entries() {
        let entries = vec![
            Entry { key: "a", value: 1 },
            Entry { key: "b", value: 2 },
            Entry { key: "c", value: 3 },
        ];
        let result = from_entries(&entries);
        let mut expected = HashMap::new();
        expected.insert("a", 1);
        expected.insert("b", 2);
        expected.insert("c", 3);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_entries_empty_entries() {
        let entries: Vec<Entry<&str, i32>> = vec![];
        let result = from_entries(&entries);
        let expected: HashMap<&str, i32> = HashMap::new();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_entries_with_integers() {
        let entries = vec![
            Entry {
                key: 1,
                value: "one",
            },
            Entry {
                key: 2,
                value: "two",
            },
        ];
        let result = from_entries(&entries);
        let mut expected = HashMap::new();
        expected.insert(1, "one");
        expected.insert(2, "two");

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_entries_with_complex_values() {
        let entries = vec![
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
        let result = from_entries(&entries);
        let mut expected: HashMap<&str, Vec<i32>> = HashMap::new();
        expected.insert("a", vec![1, 2, 3]);
        expected.insert("b", vec![4, 5]);
        expected.insert("c", vec![6]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_entries_with_duplicate_keys() {
        let entries = vec![
            Entry { key: "a", value: 1 },
            Entry { key: "b", value: 2 },
            Entry { key: "a", value: 3 }, // Duplicate key "a"
        ];
        let result = from_entries(&entries);
        let mut expected = HashMap::new();
        expected.insert("a", 3); // Expect the last value for key "a"
        expected.insert("b", 2);

        assert_eq!(result, expected);
    }
}
