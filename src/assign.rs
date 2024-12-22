use std::collections::HashMap;

/// Merges multiple maps into a single map.
/// If the same key exists in multiple maps, the value from the last map is used.
///
/// # Arguments
/// * `maps` - A variadic number of maps to merge.
///
/// # Returns
/// * `HashMap<K, V>` - The merged map.
///
/// # Examples
/// ```
/// use lowdash::assign;
/// use std::collections::HashMap;
///
/// let mut map1 = HashMap::new();
/// map1.insert("a", 1);
/// let mut map2 = HashMap::new();
/// map2.insert("b", 2);
///
/// let merged = assign(&[map1, map2]);
/// assert_eq!(merged.get("a"), Some(&1));
/// assert_eq!(merged.get("b"), Some(&2));
/// ```
pub fn assign<K, V>(maps: &[HashMap<K, V>]) -> HashMap<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    let mut out = HashMap::new();
    for map in maps {
        for (k, v) in map {
            out.insert(k.clone(), v.clone());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assign() {
        let mut map1 = HashMap::new();
        map1.insert("a", 1);
        let mut map2 = HashMap::new();
        map2.insert("b", 2);
        // Updated function call to pass a slice of maps
        let merged = assign(&[map1, map2]);
        assert_eq!(merged.get("a"), Some(&1));
        assert_eq!(merged.get("b"), Some(&2));
    }

    #[test]
    fn test_assign_overlapping_keys() {
        let mut map1 = HashMap::new();
        map1.insert("a", 1);
        let mut map2 = HashMap::new();
        map2.insert("a", 2);
        // Updated function call to pass a slice of maps
        let merged = assign(&[map1, map2]);
        assert_eq!(merged.get("a"), Some(&2));
    }

    #[test]
    fn test_assign_empty() {
        // Updated function call to pass an empty slice
        let merged: HashMap<&str, i32> = assign(&[]);
        assert!(merged.is_empty());
    }
}
