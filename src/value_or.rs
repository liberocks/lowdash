/// Returns a value from a map for a given key, or a fallback value if the key doesn't exist.
///
/// # Arguments
/// * `map` - The map to get the value from
/// * `key` - The key to look up
/// * `fallback` - The value to return if the key doesn't exist
///
/// # Returns
/// * `V` - Either the value from the map or the fallback value
///
/// # Examples
/// ```
/// use lowdash::value_or;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("a", 1);
///
/// assert_eq!(value_or(&map, &"a", 42), 1);
/// assert_eq!(value_or(&map, &"b", 42), 42);
/// ```
pub fn value_or<K, V>(map: &std::collections::HashMap<K, V>, key: &K, fallback: V) -> V
where
    K: std::cmp::Eq + std::hash::Hash,
    V: Clone,
{
    map.get(key).cloned().unwrap_or(fallback)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_value_or_existing_key() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        assert_eq!(value_or(&map, &"a", 42), 1);
    }

    #[test]
    fn test_value_or_missing_key() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        assert_eq!(value_or(&map, &"b", 42), 42);
    }

    #[test]
    fn test_value_or_empty_map() {
        let map: HashMap<&str, i32> = HashMap::new();
        assert_eq!(value_or(&map, &"a", 42), 42);
    }

    #[test]
    fn test_value_or_with_strings() {
        let mut map = HashMap::new();
        map.insert(1, "hello");
        assert_eq!(value_or(&map, &1, "world"), "hello");
        assert_eq!(value_or(&map, &2, "world"), "world");
    }

    #[test]
    fn test_value_or_with_floats() {
        let mut map = HashMap::new();
        map.insert("pi", 3.14);
        assert_eq!(value_or(&map, &"pi", 0.0), 3.14);
        assert_eq!(value_or(&map, &"e", 0.0), 0.0);
    }
}
