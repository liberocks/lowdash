/// Filters a map by omitting specified keys.
///
/// Iterates over the provided keys and removes them from a cloned version of the input map.
/// Only key-value pairs not present in the keys slice are included in the result map.
///
/// # Arguments
/// * `map` - The input map to filter.
/// * `keys` - A slice of keys to omit from the result map.
///
/// # Returns
/// * `HashMap<K, V>` - A new map containing only the key-value pairs that are not in the keys slice.
///
/// # Examples
/// ```rust
/// use lowdash::omit_by_keys;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("a", 1);
/// map.insert("b", 2);
/// map.insert("c", 3);
///
/// let keys = vec!["b", "d"];
/// let result = omit_by_keys(&map, &keys);
/// assert_eq!(result.len(), 2);
/// assert!(result.contains_key("a"));
/// assert!(result.contains_key("c"));
/// assert!(!result.contains_key("b"));
/// ```
pub fn omit_by_keys<K, V>(
    map: &std::collections::HashMap<K, V>,
    keys: &[K],
) -> std::collections::HashMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Clone,
    V: Clone,
{
    let mut result = map.clone();
    for key in keys {
        result.remove(key);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_omit_by_keys_existing_keys() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);

        let keys = vec!["b", "d"];
        let result = omit_by_keys(&map, &keys);
        assert_eq!(result.len(), 2);
        assert!(result.contains_key("a"));
        assert!(result.contains_key("c"));
        assert!(!result.contains_key("b"));
    }

    #[test]
    fn test_omit_by_keys_some_missing_keys() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        let keys = vec!["a", "c"];
        let result = omit_by_keys(&map, &keys);
        assert_eq!(result.len(), 1);
        assert!(!result.contains_key("a"));
        assert!(result.contains_key("b"));
        assert!(!result.contains_key("c"));
    }

    #[test]
    fn test_omit_by_keys_all_missing_keys() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        let keys = vec!["c", "d"];
        let result = omit_by_keys(&map, &keys);
        assert_eq!(result.len(), 2);
        assert!(result.contains_key("a"));
        assert!(result.contains_key("b"));
        assert!(!result.contains_key("c"));
        assert!(!result.contains_key("d"));
    }

    #[test]
    fn test_omit_by_keys_empty_keys() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        let keys: Vec<&str> = vec![];
        let result = omit_by_keys(&map, &keys);
        assert_eq!(result.len(), 2);
        assert!(result.contains_key("a"));
        assert!(result.contains_key("b"));
    }

    #[test]
    fn test_omit_by_keys_empty_map() {
        let map: HashMap<&str, i32> = HashMap::new();
        let keys = vec!["a", "b"];
        let result = omit_by_keys(&map, &keys);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_omit_by_keys_with_integers() {
        let mut map = HashMap::new();
        map.insert(1, "one");
        map.insert(2, "two");
        map.insert(3, "three");

        let keys = vec![2, 4];
        let result = omit_by_keys(&map, &keys);
        assert_eq!(result.len(), 2);
        assert!(result.contains_key(&1));
        assert!(result.contains_key(&3));
        assert!(!result.contains_key(&2));
    }
}
