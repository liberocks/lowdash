/// Collects all unique values from one or more maps into a single vector.
///
/// Iterates over each map and collects all unique values into a single vector.
///
/// # Arguments
/// * `maps` - A slice of references to maps to collect unique values from
///
/// # Returns
/// * `Vec<V>` - A vector containing all unique values from the input maps
///
/// # Examples
/// ```
/// use lowdash::uniq_values;
/// use std::collections::HashMap;
///
/// let mut map1 = HashMap::new();
/// map1.insert("a", 1);
/// map1.insert("b", 2);
///
/// let mut map2 = HashMap::new();
/// map2.insert("c", 2);
/// map2.insert("d", 3);
///
/// let result = uniq_values(&[&map1, &map2]);
/// assert_eq!(result.len(), 3);
/// assert!(result.contains(&1));
/// assert!(result.contains(&2));
/// assert!(result.contains(&3));
/// ```
pub fn uniq_values<K, V>(maps: &[&std::collections::HashMap<K, V>]) -> Vec<V>
where
    V: Clone + std::cmp::Eq + std::hash::Hash,
{
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();

    for map in maps {
        for value in map.values() {
            if seen.insert(value.clone()) {
                result.push(value.clone());
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
    fn test_uniq_values_single_map() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        let result = uniq_values(&[&map]);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&1));
        assert!(result.contains(&2));
    }

    #[test]
    fn test_uniq_values_multiple_maps() {
        let mut map1 = HashMap::new();
        map1.insert("a", 1);
        map1.insert("b", 2);

        let mut map2 = HashMap::new();
        map2.insert("c", 2);
        map2.insert("d", 3);

        let result = uniq_values(&[&map1, &map2]);
        assert_eq!(result.len(), 3);
        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert!(result.contains(&3));
    }

    #[test]
    fn test_uniq_values_empty_maps() {
        let map1: HashMap<&str, i32> = HashMap::new();
        let map2: HashMap<&str, i32> = HashMap::new();

        let result = uniq_values(&[&map1, &map2]);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_uniq_values_with_integers() {
        let mut map1 = HashMap::new();
        map1.insert(1, "a");
        map1.insert(2, "b");

        let mut map2 = HashMap::new();
        map2.insert(3, "b");
        map2.insert(4, "c");

        let result = uniq_values(&[&map1, &map2]);
        assert_eq!(result.len(), 3);
        assert!(result.contains(&"a"));
        assert!(result.contains(&"b"));
        assert!(result.contains(&"c"));
    }

    #[test]
    fn test_uniq_values_with_mixed_types() {
        let mut map1 = HashMap::new();
        map1.insert("a", 1);
        map1.insert("b", 2);

        let mut map2 = HashMap::new();
        map2.insert("c", 2);
        map2.insert("d", 3);

        let result = uniq_values(&[&map1, &map2]);
        assert_eq!(result.len(), 3);
        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert!(result.contains(&3));
    }
}
