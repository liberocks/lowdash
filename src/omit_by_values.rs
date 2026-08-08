/// Omits entries whose values occur in `values`.
///
/// # Arguments
/// * `map` - Map to filter.
/// * `values` - Values to omit.
///
/// # Returns
/// * `HashMap<K, V>` - The entries not omitted.
///
/// # Examples
/// ```rust
/// use lowdash::omit_by_values;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("a", 1);
/// map.insert("b", 2);
/// map.insert("c", 3);
///
/// let values = vec![2, 4];
/// let result = omit_by_values(&map, &values);
/// assert_eq!(result.len(), 2);
/// assert!(result.contains_key("a"));
/// assert!(result.contains_key("c"));
/// assert!(!result.contains_key("b"));
/// ```
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub fn omit_by_values<K, V>(map: &HashMap<K, V>, values: &[V]) -> HashMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    let value_set: HashSet<V> = values.iter().cloned().collect();
    let mut result = HashMap::with_capacity(map.len());
    for (k, v) in map.iter() {
        if !value_set.contains(v) {
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
    fn test_omit_by_values_existing_values() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);

        let values = vec![2, 4];
        let result = omit_by_values(&map, &values);
        assert_eq!(result.len(), 2);
        assert!(result.contains_key("a"));
        assert!(result.contains_key("c"));
        assert!(!result.contains_key("b"));
    }

    #[test]
    fn test_omit_by_values_some_missing_values() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        let values = vec![1, 3];
        let result = omit_by_values(&map, &values);
        assert_eq!(result.len(), 1);
        assert!(!result.contains_key("a"));
        assert!(result.contains_key("b"));
    }

    #[test]
    fn test_omit_by_values_all_missing_values() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        let values = vec![3, 4];
        let result = omit_by_values(&map, &values);
        assert_eq!(result.len(), 2);
        assert!(result.contains_key("a"));
        assert!(result.contains_key("b"));
    }

    #[test]
    fn test_omit_by_values_empty_values() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        let values: Vec<i32> = vec![];
        let result = omit_by_values(&map, &values);
        assert_eq!(result.len(), 2);
        assert!(result.contains_key("a"));
        assert!(result.contains_key("b"));
    }

    #[test]
    fn test_omit_by_values_empty_map() {
        let map: HashMap<&str, i32> = HashMap::new();
        let values = vec![1, 2];
        let result = omit_by_values(&map, &values);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_omit_by_values_with_strings() {
        let mut map = HashMap::new();
        map.insert("apple", "red");
        map.insert("banana", "yellow");
        map.insert("grape", "purple");

        let values = vec!["yellow", "blue"];
        let result = omit_by_values(&map, &values);
        assert_eq!(result.len(), 2);
        assert!(result.contains_key("apple"));
        assert!(result.contains_key("grape"));
        assert!(!result.contains_key("banana"));
    }

    #[test]
    fn test_omit_by_values_with_mixed_types() {
        let mut map = HashMap::new();
        map.insert(1, "one");
        map.insert(2, "two");
        map.insert(3, "three");

        let values = vec!["one", "three", "four"];
        let result = omit_by_values(&map, &values);
        assert_eq!(result.len(), 1);
        assert!(!result.contains_key(&1));
        assert!(result.contains_key(&2));
        assert!(!result.contains_key(&3));
    }
}
