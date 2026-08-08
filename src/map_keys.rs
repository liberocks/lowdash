use std::collections::{BTreeMap, HashMap};

/// Transforms map keys into a sorted `BTreeMap`.
/// The callback receives `(value, key)`. Input keys are processed in
/// descending order, so later inserts win on transformed-key collisions.
///
/// # Arguments
/// * `map` - Map to transform.
/// * `iteratee` - Function returning a new key for `(value, key)`.
///
/// # Returns
/// * `BTreeMap<R, V>` - The transformed map.
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
    K: Ord,
    V: Clone,
    R: Ord,
    F: Fn(&V, &K) -> R,
{
    let mut result = BTreeMap::new();
    let mut entries: Vec<(&K, &V)> = map.iter().collect();

    // Sort descending so later inserts win.
    entries.sort_by(|(a, _), (b, _)| b.cmp(a));

    for (k, v) in entries {
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
        let transformed = map_keys(&map, |&_, &k| format!("key_{}", k));
        assert_eq!(transformed.get("key_a"), Some(&1));
        assert_eq!(transformed.get("key_b"), Some(&2));
    }

    #[test]
    fn test_map_keys_empty() {
        let map: HashMap<&str, i32> = HashMap::new();
        let transformed = map_keys(&map, |&_, &k| k.len());
        assert!(transformed.is_empty());
    }

    #[test]
    fn test_map_keys_overwrite() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("A", 2); // Both keys become "a".
        let transformed = map_keys(&map, |&_, k| k.to_lowercase());
        // Descending order makes "A" overwrite "a".
        assert_eq!(transformed.get("a"), Some(&2));
    }
}
