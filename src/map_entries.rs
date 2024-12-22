use std::collections::{BTreeMap, HashMap};

/// Transforms the entries of a map using a provided function.
///
/// # Arguments
/// * `map` - The input map whose entries are to be transformed.
/// * `iteratee` - A function that takes a reference to a key and its value, returning a new key and value.
///
/// # Returns
/// * `BTreeMap<K2, V2>` - A new map with transformed entries.
///
/// # Examples
/// ```rust
/// use lowdash::map_entries;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("a", 1);
/// map.insert("b", 2);
///
/// let transformed = map_entries(&map, |k, v| (k.to_uppercase(), v * 10));
/// assert_eq!(transformed.get("A"), Some(&10));
/// assert_eq!(transformed.get("B"), Some(&20));
/// ```
pub fn map_entries<K1, V1, K2, V2, F>(map: &HashMap<K1, V1>, iteratee: F) -> BTreeMap<K2, V2>
where
    K1: Eq + std::hash::Hash + Clone + Ord,
    K2: Eq + std::hash::Hash + Ord,
    V1: Clone,
    V2: Clone,
    F: Fn(&K1, &V1) -> (K2, V2),
{
    let mut result = BTreeMap::new();

    // Collect and sort the keys in ascending order for deterministic behavior
    let mut keys: Vec<&K1> = map.keys().collect();
    keys.sort(); // Sort in ascending order. Use keys.sort_by(|a, b| b.cmp(a)) for descending.

    for k1 in keys {
        let v1 = &map[k1];
        let (k2, v2) = iteratee(k1, v1);
        result.insert(k2, v2);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeMap, HashMap};

    #[test]
    fn test_map_entries_basic() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        let transformed = map_entries(&map, |k, v| (k.to_uppercase(), v * 10));
        assert_eq!(transformed.get("A"), Some(&10));
        assert_eq!(transformed.get("B"), Some(&20));
    }

    #[test]
    fn test_map_entries_empty() {
        let map: HashMap<&str, i32> = HashMap::new();
        let transformed = map_entries(&map, |k, v| (k.to_uppercase(), v * 10));
        assert!(transformed.is_empty());
    }

    #[test]
    fn test_map_entries_overwrite() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("A", 2); // Both "a" and "A" will transform to "A"
        let transformed = map_entries(&map, |k, v| (k.to_uppercase(), v.clone()));
        // Since keys are sorted in ascending order, "A" comes before "a"
        // After transformation, "A" is inserted first with value 2, then "A" is inserted again with value 1
        // The final value for "A" should be 1
        assert_eq!(transformed.get("A"), Some(&1));
    }

    #[test]
    fn test_map_entries_transform_keys_and_values() {
        let mut map = HashMap::new();
        map.insert(1, "one");
        map.insert(2, "two");
        let transformed = map_entries(&map, |k, v| (k * 10, v.len()));
        assert_eq!(transformed.get(&10), Some(&3));
        assert_eq!(transformed.get(&20), Some(&3));
    }
}
