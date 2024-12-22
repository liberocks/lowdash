use std::collections::HashMap;

/// Transforms the entries of a map into a slice using a provided function.
///
/// # Arguments
/// * `map` - The input map whose entries are to be transformed.
/// * `iteratee` - A function that takes a reference to a key and its value, returning a new value.
///
/// # Returns
/// * `Vec<R>` - A vector containing the transformed values.
///
/// # Examples
/// ```rust
/// use lowdash::map_to_slice;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("a", 1);
/// map.insert("b", 2);
///
/// let transformed = map_to_slice(&map, |k, v| format!("{}:{}", k, v));
/// assert!(transformed.contains(&"a:1".to_string()));
/// assert!(transformed.contains(&"b:2".to_string()));
/// ```
pub fn map_to_slice<K, V, R, F>(map: &HashMap<K, V>, iteratee: F) -> Vec<R>
where
    K: Eq + std::hash::Hash,
    F: Fn(&K, &V) -> R,
{
    let mut result = Vec::with_capacity(map.len());
    for (k, v) in map {
        result.push(iteratee(k, v));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_map_to_slice_basic() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        let transformed = map_to_slice(&map, |k, v| format!("{}:{}", k, v));
        assert!(transformed.contains(&"a:1".to_string()));
        assert!(transformed.contains(&"b:2".to_string()));
    }

    #[test]
    fn test_map_to_slice_empty() {
        let map: HashMap<&str, i32> = HashMap::new();
        let transformed: Vec<String> = map_to_slice(&map, |k, v| format!("{}:{}", k, v));
        assert!(transformed.is_empty());
    }

    #[test]
    fn test_map_to_slice_transform_values() {
        let mut map = HashMap::new();
        map.insert(1, "one");
        map.insert(2, "two");
        let transformed = map_to_slice(&map, |k, v| v.len());
        assert!(transformed.contains(&3));
        assert!(transformed.contains(&3));
    }

    #[test]
    fn test_map_to_slice_with_numbers() {
        let mut map = HashMap::new();
        map.insert(10, 100);
        map.insert(20, 200);
        let transformed: Vec<i32> = map_to_slice(&map, |k, v| k + v);
        assert!(transformed.contains(&110));
        assert!(transformed.contains(&220));
    }
}
