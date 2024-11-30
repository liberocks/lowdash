/// Find the key in a map that satisfies a predicate based on both key and value.
/// If no key satisfies the predicate, return None.
///
/// # Arguments
/// * `object` - A map of key-value pairs.
/// * `predicate` - A function that takes a key and value and returns a boolean.
///
/// # Returns
/// * `Option<&K>` - The key that satisfies the predicate, or None if no key satisfies the predicate.
///
/// # Examples
/// ```
/// use lowdash::find_key_by;
/// let mut map = std::collections::HashMap::new();
/// map.insert("a", 1);
/// map.insert("b", 2);
/// map.insert("c", 3);
/// let result = find_key_by(&map, |k, v| *k == "b" && *v == 2);
/// assert_eq!(result, Some(&"b"));
/// ```
///
/// ```
/// use lowdash::find_key_by;
/// let mut map = std::collections::HashMap::new();
/// map.insert("a", 1);
/// map.insert("b", 2);
/// map.insert("c", 3);
/// let result = find_key_by(&map, |_, v| *v > 2);
/// assert_eq!(result, Some(&"c"));
/// ```
pub fn find_key_by<K, V, F>(object: &std::collections::HashMap<K, V>, predicate: F) -> Option<&K>
where
    K: std::cmp::Eq + std::hash::Hash,
    F: Fn(&K, &V) -> bool,
{
    for (k, v) in object {
        if predicate(k, v) {
            return Some(k);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_find_key_by() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);
        let result = find_key_by(&map, |k, v| *k == "b" && *v == 2);
        assert_eq!(result, Some(&"b"));
    }

    #[test]
    fn test_find_key_by_value_only() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);
        let result = find_key_by(&map, |_, v| *v > 2);
        assert_eq!(result, Some(&"c"));
    }

    #[test]
    fn test_find_key_by_not_found() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);
        let result = find_key_by(&map, |_, v| *v > 3);
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_key_by_empty_map() {
        let map: HashMap<&str, i32> = HashMap::new();
        let result = find_key_by(&map, |_, v| *v == 1);
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_key_by_with_struct() {
        #[derive(Debug, PartialEq)]
        struct Person {
            name: String,
            age: u32,
        }

        let mut map = HashMap::new();
        map.insert(
            "dev1",
            Person {
                name: "Alice".to_string(),
                age: 25,
            },
        );
        map.insert(
            "dev2",
            Person {
                name: "Bob".to_string(),
                age: 30,
            },
        );
        map.insert(
            "dev3",
            Person {
                name: "Carol".to_string(),
                age: 35,
            },
        );

        let result = find_key_by(&map, |_, p| p.age > 30);
        assert_eq!(result, Some(&"dev3"));

        let name_result = find_key_by(&map, |k, p| k.starts_with("dev2") && p.name == "Bob");
        assert_eq!(name_result, Some(&"dev2"));

        let not_found = find_key_by(&map, |_, p| p.age > 40);
        assert_eq!(not_found, None);
    }
}
