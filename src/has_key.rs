/// Checks if a map contains a specific key.
///
/// # Arguments
/// * `map` - The map to check for the key
/// * `key` - The key to check for in the map
///
/// # Returns
/// * `bool` - `true` if the key is present in the map, `false` otherwise
///
/// # Examples
/// ```
/// use lowdash::has_key;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("a", 1);
/// map.insert("b", 2);
///
/// assert!(has_key(&map, &"a"));
/// assert!(!has_key(&map, &"c"));
/// ```
pub fn has_key<K, V>(map: &std::collections::HashMap<K, V>, key: &K) -> bool
where
    K: std::cmp::Eq + std::hash::Hash,
{
    map.contains_key(key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_has_key_present() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        assert!(has_key(&map, &"a"));
        assert!(has_key(&map, &"b"));
    }

    #[test]
    fn test_has_key_absent() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        assert!(!has_key(&map, &"c"));
    }

    #[test]
    fn test_has_key_empty_map() {
        let map: HashMap<&str, i32> = HashMap::new();

        assert!(!has_key(&map, &"a"));
    }

    #[test]
    fn test_has_key_with_integers() {
        let mut map = HashMap::new();
        map.insert(1, "a");
        map.insert(2, "b");

        assert!(has_key(&map, &1));
        assert!(has_key(&map, &2));
        assert!(!has_key(&map, &3));
    }

    #[test]
    fn test_has_key_with_mixed_types() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        assert!(has_key(&map, &"a"));
        assert!(has_key(&map, &"b"));
        assert!(!has_key(&map, &"c"));
    }
}
