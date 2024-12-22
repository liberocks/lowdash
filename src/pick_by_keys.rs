/// Filters a map by selecting only the specified keys.
///
/// Iterates over the provided keys and includes the key-value pair in the result map
/// only if the key exists in the input map.
///
/// # Arguments
/// * `map` - The input map to filter.
/// * `keys` - A slice of keys to select from the map.
///
/// # Returns
/// * `HashMap<K, V>` - A new map containing only the specified key-value pairs.
///
/// # Examples
/// ```rust
/// use lowdash::pick_by_keys;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("a", 1);
/// map.insert("b", 2);
/// map.insert("c", 3);
///
/// let keys = vec!["a", "c", "d"];
/// let result = pick_by_keys(&map, &keys);
/// assert_eq!(result.len(), 2);
/// assert!(result.contains_key("a"));
/// assert!(result.contains_key("c"));
/// ```
pub fn pick_by_keys<K, V>(
    map: &std::collections::HashMap<K, V>,
    keys: &[K],
) -> std::collections::HashMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Clone,
    V: Clone,
{
    let mut result = std::collections::HashMap::new();
    for key in keys {
        if let Some(value) = map.get(key) {
            result.insert(key.clone(), value.clone());
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_pick_by_keys_existing_keys() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);

        let keys = vec!["a", "c"];
        let result = pick_by_keys(&map, &keys);
        assert_eq!(result.len(), 2);
        assert!(result.contains_key("a"));
        assert!(result.contains_key("c"));
    }

    #[test]
    fn test_pick_by_keys_some_missing_keys() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        let keys = vec!["a", "c"];
        let result = pick_by_keys(&map, &keys);
        assert_eq!(result.len(), 1);
        assert!(result.contains_key("a"));
        assert!(!result.contains_key("c"));
    }

    #[test]
    fn test_pick_by_keys_all_missing_keys() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        let keys = vec!["c", "d"];
        let result = pick_by_keys(&map, &keys);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_pick_by_keys_empty_keys() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        let keys: Vec<&str> = vec![];
        let result = pick_by_keys(&map, &keys);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_pick_by_keys_empty_map() {
        let map: HashMap<&str, i32> = HashMap::new();
        let keys = vec!["a", "b"];
        let result = pick_by_keys(&map, &keys);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_pick_by_keys_with_integers() {
        let mut map = HashMap::new();
        map.insert(1, "one");
        map.insert(2, "two");
        map.insert(3, "three");

        let keys = vec![1, 3, 4];
        let result = pick_by_keys(&map, &keys);
        assert_eq!(result.len(), 2);
        assert!(result.contains_key(&1));
        assert!(result.contains_key(&3));
        assert!(!result.contains_key(&4));
    }

    #[test]
    fn test_pick_by_keys_with_mixed_types() {
        let mut map = HashMap::new();
        map.insert("a", "one");
        map.insert("b", "two");
        map.insert("c", "three");

        let keys = vec!["a", "b"];
        let result = pick_by_keys(&map, &keys);
        assert_eq!(result.len(), 2);
        assert!(result.contains_key("a"));
        assert!(result.contains_key("b"));
    }
}
