/// Filters a map by selecting only the specified values.
///
/// Iterates over each key-value pair in the input map and includes it in the result map
/// only if the value is present in the provided values slice.
///
/// # Arguments
/// * `map` - The input map to filter.
/// * `values` - A slice of values to select from the map.
///
/// # Returns
/// * `HashMap<K, V>` - A new map containing only the key-value pairs that have values present in the values slice.
///
/// # Examples
/// ```rust
/// use lowdash::pick_by_values;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("a", 1);
/// map.insert("b", 2);
/// map.insert("c", 3);
///
/// let values = vec![1, 3];
/// let result = pick_by_values(&map, &values);
/// assert_eq!(result.len(), 2);
/// assert!(result.contains_key("a"));
/// assert!(result.contains_key("c"));
/// ```
pub fn pick_by_values<K, V>(
    map: &std::collections::HashMap<K, V>,
    values: &[V],
) -> std::collections::HashMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Clone,
    V: std::cmp::Eq + std::hash::Hash + Clone,
{
    let value_set: std::collections::HashSet<V> = values.iter().cloned().collect();
    let mut result = std::collections::HashMap::new();
    for (k, v) in map.iter() {
        if value_set.contains(v) {
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
    fn test_pick_by_values_existing_values() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);

        let values = vec![1, 3];
        let result = pick_by_values(&map, &values);
        assert_eq!(result.len(), 2);
        assert!(result.contains_key("a"));
        assert!(result.contains_key("c"));
    }

    #[test]
    fn test_pick_by_values_some_missing_values() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        let values = vec![1, 3];
        let result = pick_by_values(&map, &values);
        assert_eq!(result.len(), 1);
        assert!(result.contains_key("a"));
        assert!(!result.contains_key("b"));
    }

    #[test]
    fn test_pick_by_values_all_missing_values() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        let values = vec![3, 4];
        let result = pick_by_values(&map, &values);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_pick_by_values_empty_values() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        let values: Vec<i32> = vec![];
        let result = pick_by_values(&map, &values);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_pick_by_values_empty_map() {
        let map: HashMap<&str, i32> = HashMap::new();
        let values = vec![1, 2];
        let result = pick_by_values(&map, &values);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_pick_by_values_with_strings() {
        let mut map = HashMap::new();
        map.insert("apple", "red");
        map.insert("banana", "yellow");
        map.insert("grape", "purple");

        let values = vec!["red", "purple"];
        let result = pick_by_values(&map, &values);
        assert_eq!(result.len(), 2);
        assert!(result.contains_key("apple"));
        assert!(result.contains_key("grape"));
    }

    #[test]
    fn test_pick_by_values_with_mixed_types() {
        let mut map = HashMap::new();
        map.insert(1, "one");
        map.insert(2, "two");
        map.insert(3, "three");

        let values = vec!["one", "three", "four"];
        let result = pick_by_values(&map, &values);
        assert_eq!(result.len(), 2);
        assert!(result.contains_key(&1));
        assert!(result.contains_key(&3));
        assert!(!result.contains_key(&2));
    }
}
