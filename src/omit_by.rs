/// Filters a map by omitting key-value pairs that satisfy a predicate.
///
/// Iterates over each key-value pair in the input map and excludes it from the result map
/// if the predicate returns `true` for that pair.
///
/// # Arguments
/// * `map` - The input map to filter.
/// * `predicate` - A function that takes a key and value, and returns `true` if the pair should be omitted.
///
/// # Returns
/// * `HashMap<K, V>` - A new map containing only the key-value pairs that do not satisfy the predicate.
///
/// # Examples
/// ```rust
/// use lowdash::omit_by;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("a", 1);
/// map.insert("b", 2);
/// map.insert("c", 3);
///
/// // Omit entries where the value is greater than 1
/// let result = omit_by(&map, |_, v| *v > 1);
/// assert_eq!(result.len(), 1);
/// assert!(result.contains_key("a"));
/// assert!(!result.contains_key("b"));
/// assert!(!result.contains_key("c"));
/// ```
pub fn omit_by<K, V, F>(
    map: &std::collections::HashMap<K, V>,
    predicate: F,
) -> std::collections::HashMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Clone,
    V: Clone,
    F: Fn(&K, &V) -> bool,
{
    let mut result = std::collections::HashMap::new();
    for (k, v) in map.iter() {
        if !predicate(k, v) {
            result.insert(k.clone(), v.clone());
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_omit_by_some_predicates() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);

        // Omit entries where the value is greater than 1
        let result = omit_by(&map, |_, v| *v > 1);
        assert_eq!(result.len(), 1);
        assert!(result.contains_key("a"));
        assert!(!result.contains_key("b"));
        assert!(!result.contains_key("c"));
    }

    #[test]
    fn test_omit_by_all_predicates_true() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        // Omit all entries
        let result = omit_by(&map, |_, _| true);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_omit_by_all_predicates_false() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        // Omit no entries
        let result = omit_by(&map, |_, _| false);
        assert_eq!(result.len(), 2);
        assert!(result.contains_key("a"));
        assert!(result.contains_key("b"));
    }

    #[test]
    fn test_omit_by_empty_map() {
        let map: HashMap<&str, i32> = HashMap::new();
        let result = omit_by(&map, |_, _| true);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_omit_by_with_strings() {
        let mut map = HashMap::new();
        map.insert("apple", "red");
        map.insert("banana", "yellow");
        map.insert("grape", "purple");

        // Omit entries where the value starts with 'y'
        let result = omit_by(&map, |_, v| v.starts_with('y'));
        assert_eq!(result.len(), 2);
        assert!(result.contains_key("apple"));
        assert!(!result.contains_key("banana"));
        assert!(result.contains_key("grape"));
    }

    #[test]
    fn test_omit_by_with_mixed_types() {
        let mut map = HashMap::new();
        map.insert(1, "one");
        map.insert(2, "two");
        map.insert(3, "three");

        // Omit entries where the key is even
        let result = omit_by(&map, |k, _| *k % 2 == 0);
        assert_eq!(result.len(), 2);
        assert!(result.contains_key(&1));
        assert!(!result.contains_key(&2));
        assert!(result.contains_key(&3));
    }

    #[test]
    fn test_omit_by_with_complex_predicate() {
        let mut map = HashMap::new();
        map.insert("a", vec![1, 2, 3]);
        map.insert("b", vec![4, 5]);
        map.insert("c", vec![6]);

        // Omit entries where the vector length is less than 3
        let result = omit_by(&map, |_, v| v.len() < 3);
        assert_eq!(result.len(), 1);
        assert!(result.contains_key("a"));
        assert!(!result.contains_key("b"));
        assert!(!result.contains_key("c"));
    }
}
