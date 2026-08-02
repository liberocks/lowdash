use std::collections::HashMap;

/// Swaps keys and values in a map.
/// If values repeat, later HashMap iteration overwrites earlier keys; iteration
/// order is unspecified.
///
/// # Arguments
/// * `input` - Map to invert.
///
/// # Returns
/// * `HashMap<V, K>` - The inverted map.
///
/// # Examples
/// ```rust
/// use lowdash::{invert, Entry};
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("a", 1);
/// map.insert("b", 2);
/// map.insert("c", 3);
///
/// let result = invert(&map);
/// let mut expected = HashMap::new();
/// expected.insert(1, "a");
/// expected.insert(2, "b");
/// expected.insert(3, "c");
///
/// assert_eq!(result.len(), expected.len());
/// for (key, value) in &expected {
///    assert_eq!(result.get(key), Some(value));
/// }
/// ```
pub fn invert<K, V>(input: &HashMap<K, V>) -> HashMap<V, K>
where
    K: Clone + std::cmp::Eq + std::hash::Hash,
    V: Clone + std::cmp::Eq + std::hash::Hash,
{
    let mut inverted = HashMap::with_capacity(input.len());
    for (k, v) in input {
        inverted.insert(v.clone(), k.clone());
    }
    inverted
}

///
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invert_single_entry() {
        let mut map = HashMap::new();
        map.insert("a", 1);

        let result = invert(&map);
        let mut expected = HashMap::new();
        expected.insert(1, "a");

        assert_eq!(result.len(), expected.len());
        for (key, value) in &expected {
            assert_eq!(result.get(key), Some(value));
        }
    }

    #[test]
    fn test_invert_multiple_entries() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);

        let result = invert(&map);
        let mut expected = HashMap::new();
        expected.insert(1, "a");
        expected.insert(2, "b");
        expected.insert(3, "c");

        assert_eq!(result.len(), expected.len());
        for (key, value) in &expected {
            assert_eq!(result.get(key), Some(value));
        }
    }

    #[test]
    fn test_invert_empty_map() {
        let map: HashMap<&str, i32> = HashMap::new();

        let result = invert(&map);
        let expected: HashMap<i32, &str> = HashMap::new();

        assert_eq!(result.len(), expected.len());
        for (key, value) in &expected {
            assert_eq!(result.get(key), Some(value));
        }
    }

    #[test]
    fn test_invert_with_integers() {
        let mut map = HashMap::new();
        map.insert(1, "one");
        map.insert(2, "two");

        let result = invert(&map);
        let mut expected = HashMap::new();
        expected.insert("one", 1);
        expected.insert("two", 2);

        assert_eq!(result.len(), expected.len());
        for (key, value) in &expected {
            assert_eq!(result.get(key), Some(value));
        }
    }

    #[test]
    fn test_invert_with_complex_values() {
        let mut map = HashMap::new();
        map.insert("a", vec![1, 2, 3]);
        map.insert("b", vec![4, 5]);
        map.insert("c", vec![6]);

        let result = invert(&map);
        let mut expected: HashMap<Vec<i32>, &str> = HashMap::new();
        expected.insert(vec![1, 2, 3], "a");
        expected.insert(vec![4, 5], "b");
        expected.insert(vec![6], "c");

        assert_eq!(result.len(), expected.len());
        for (key, value) in &expected {
            assert_eq!(result.get(key), Some(value));
        }
    }
}
