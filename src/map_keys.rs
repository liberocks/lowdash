use std::collections::{BTreeMap, HashMap};

/// Transforms the keys of a map using a provided function.
///
/// # Arguments
/// * `map` - The input map whose keys are to be transformed.
/// * `iteratee` - A function that takes a reference to a value and its key, returning a new key.
///
/// # Returns
/// * `BTreeMap<R, V>` - A new map with transformed keys.
///
/// # Examples
/// ```rust
/// use lowdash::map_keys;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("a", 1);
/// map.insert("b", 2);
/// let transformed = map_keys(&map, |&v, &k| format!("key_{}", k));
/// assert_eq!(transformed.get("key_a"), Some(&1));
/// assert_eq!(transformed.get("key_b"), Some(&2));
/// ```
pub fn map_keys<K, V, R, F>(map: &HashMap<K, V>, iteratee: F) -> BTreeMap<R, V>
where
    K: Eq + std::hash::Hash + Ord,
    V: Clone,
    R: Ord,
    F: Fn(&V, &K) -> R,
{
    let mut result = BTreeMap::new();
    let mut keys: Vec<&K> = map.keys().collect();

    // Sort keys in descending order to ensure later keys overwrite earlier ones
    keys.sort_by(|a, b| b.cmp(a));

    for k in keys {
        let v = &map[k];
        let new_key = iteratee(v, k);
        result.insert(new_key, v.clone());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_map_keys() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        let transformed = map_keys(&map, |&v, &k| format!("key_{}", k));
        assert_eq!(transformed.get("key_a"), Some(&1));
        assert_eq!(transformed.get("key_b"), Some(&2));
    }

    #[test]
    fn test_map_keys_empty() {
        let map: HashMap<&str, i32> = HashMap::new();
        let transformed = map_keys(&map, |&v, &k| k.len());
        assert!(transformed.is_empty());
    }

    #[test]
    fn test_map_keys_overwrite() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("A", 2); // "a" and "A" will both transform to "a"
        let transformed = map_keys(&map, |&v, k| k.to_lowercase());
        // Since keys are sorted in descending order, "a" is inserted first, then "A" overwrites it
        assert_eq!(transformed.get("a"), Some(&2));
    }
}
