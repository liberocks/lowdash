/// Collects all unique keys from one or more maps into a single vector.
///
/// Iterates over each map and collects all unique keys into a single vector.
///
/// # Arguments
/// * `maps` - A slice of references to maps to collect unique keys from
///
/// # Returns
/// * `Vec<K>` - A vector containing all unique keys from the input maps
///
/// # Examples
/// ```
/// use lowdash::uniq_keys;
/// use std::collections::HashMap;
///
/// let mut map1 = HashMap::new();
/// map1.insert("a", 1);
/// map1.insert("b", 2);
///
/// let mut map2 = HashMap::new();
/// map2.insert("b", 3);
/// map2.insert("c", 4);
///
/// let result = uniq_keys(&[&map1, &map2]);
/// assert_eq!(result.len(), 3);
/// assert!(result.contains(&"a"));
/// assert!(result.contains(&"b"));
/// assert!(result.contains(&"c"));
/// ```
pub fn uniq_keys<K, V>(maps: &[&std::collections::HashMap<K, V>]) -> Vec<K>
where
    K: Clone + std::cmp::Eq + std::hash::Hash,
{
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();

    for map in maps {
        for key in map.keys() {
            if seen.insert(key.clone()) {
                result.push(key.clone());
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_uniq_keys_single_map() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        let result = uniq_keys(&[&map]);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"a"));
        assert!(result.contains(&"b"));
    }

    #[test]
    fn test_uniq_keys_multiple_maps() {
        let mut map1 = HashMap::new();
        map1.insert("a", 1);
        map1.insert("b", 2);

        let mut map2 = HashMap::new();
        map2.insert("b", 3);
        map2.insert("c", 4);

        let result = uniq_keys(&[&map1, &map2]);
        assert_eq!(result.len(), 3);
        assert!(result.contains(&"a"));
        assert!(result.contains(&"b"));
        assert!(result.contains(&"c"));
    }

    #[test]
    fn test_uniq_keys_empty_maps() {
        let map1: HashMap<&str, i32> = HashMap::new();
        let map2: HashMap<&str, i32> = HashMap::new();

        let result = uniq_keys(&[&map1, &map2]);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_uniq_keys_with_integers() {
        let mut map1 = HashMap::new();
        map1.insert(1, "a");
        map1.insert(2, "b");

        let mut map2 = HashMap::new();
        map2.insert(2, "c");
        map2.insert(3, "d");

        let result = uniq_keys(&[&map1, &map2]);
        assert_eq!(result.len(), 3);
        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert!(result.contains(&3));
    }

    #[test]
    fn test_uniq_keys_with_mixed_types() {
        let mut map1 = HashMap::new();
        map1.insert("a", 1);
        map1.insert("b", 2);

        let mut map2 = HashMap::new();
        map2.insert("b", 3);
        map2.insert("c", 4);

        let result = uniq_keys(&[&map1, &map2]);
        assert_eq!(result.len(), 3);
        assert!(result.contains(&"a"));
        assert!(result.contains(&"b"));
        assert!(result.contains(&"c"));
    }
}
