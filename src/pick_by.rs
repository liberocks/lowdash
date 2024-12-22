/// Filters a map by applying a predicate to its key-value pairs.
///
/// Iterates over each key-value pair in the input map and includes it in the result map
/// only if the predicate returns `true` for that pair.
///
/// # Arguments
/// * `map` - The input map to filter.
/// * `predicate` - A function that takes a key and value, and returns `true` if the pair should be included.
///
/// # Returns
/// * `HashMap<K, V>` - A new map containing all key-value pairs that satisfy the predicate.
///
/// # Examples
/// ```rust
/// use lowdash::pick_by;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("a", 1);
/// map.insert("b", 2);
/// map.insert("c", 3);
///
/// let result = pick_by(&map, |_, v| *v > 1);
/// assert_eq!(result.len(), 2);
/// assert!(result.contains_key("b"));
/// assert!(result.contains_key("c"));
/// ```
pub fn pick_by<K, V, F>(
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
        if predicate(k, v) {
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
    fn test_pick_by_single_condition() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);

        let result = pick_by(&map, |_, v| *v > 1);
        assert_eq!(result.len(), 2);
        assert!(result.contains_key("b"));
        assert!(result.contains_key("c"));
    }

    #[test]
    fn test_pick_by_no_match() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        let result = pick_by(&map, |_, v| *v > 3);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_pick_by_all_match() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);

        let result = pick_by(&map, |_, _| true);
        assert_eq!(result.len(), 2);
        assert!(result.contains_key("a"));
        assert!(result.contains_key("b"));
    }

    #[test]
    fn test_pick_by_empty_map() {
        let map: HashMap<&str, i32> = HashMap::new();
        let result = pick_by(&map, |_, _| true);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_pick_by_with_strings() {
        let mut map = HashMap::new();
        map.insert("apple", "red");
        map.insert("banana", "yellow");
        map.insert("grape", "purple");

        let result = pick_by(&map, |_, v| v.starts_with('y'));
        assert_eq!(result.len(), 1);
        assert!(result.contains_key("banana"));
    }

    #[test]
    fn test_pick_by_with_numbers() {
        let mut map = HashMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        map.insert(3, 30);

        let result = pick_by(&map, |_, v| *v >= 20);
        assert_eq!(result.len(), 2);
        assert!(result.contains_key(&2));
        assert!(result.contains_key(&3));
    }
}
