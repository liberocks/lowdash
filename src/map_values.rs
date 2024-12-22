use std::collections::HashMap;

/// Transforms the values of a map using a provided function.
///
/// # Arguments
/// * `map` - The input map whose values are to be transformed.
/// * `iteratee` - A function that takes a reference to a value and its key, returning a new value.
///
/// # Returns
/// * `HashMap<K, R>` - A new map with transformed values.
///
/// # Examples
/// ```
/// use lowdash::map_values;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("a", 1);
/// map.insert("b", 2);
///
/// let transformed = map_values(&map, |&v, _k| v * 10);
/// assert_eq!(transformed.get("a"), Some(&10));
/// assert_eq!(transformed.get("b"), Some(&20));
/// ```
pub fn map_values<K, V, R, F>(map: &HashMap<K, V>, iteratee: F) -> HashMap<K, R>
where
    K: Eq + std::hash::Hash + Clone,
    F: Fn(&V, &K) -> R,
{
    let mut result = HashMap::with_capacity(map.len());
    for (k, v) in map {
        let new_value = iteratee(v, k);
        result.insert(k.clone(), new_value);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_map_values() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        let transformed = map_values(&map, |&v, _k| v * 10);
        assert_eq!(transformed.get("a"), Some(&10));
        assert_eq!(transformed.get("b"), Some(&20));
    }

    #[test]
    fn test_map_values_empty() {
        let map: HashMap<&str, i32> = HashMap::new();
        let transformed = map_values(&map, |&v, _k| v * 10);
        assert!(transformed.is_empty());
    }

    #[test]
    fn test_map_values_with_keys() {
        let mut map = HashMap::new();
        map.insert("x", 5);
        map.insert("y", 10);
        let transformed = map_values(&map, |&v, k| if k == &"x" { v + 1 } else { v * 2 });
        assert_eq!(transformed.get("x"), Some(&6));
        assert_eq!(transformed.get("y"), Some(&20));
    }
}
