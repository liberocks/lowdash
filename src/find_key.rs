/// Find the key in a map that corresponds to a given value.
/// If no key corresponds to the value, return None.
///
/// # Arguments
/// * `object` - A map of key-value pairs.
/// * `value` - The value to search for.
///
/// # Returns
/// * `Option<K>` - The key that corresponds to the value, or None if no key corresponds to the value.
///
/// # Examples
/// ```
/// use lowdash::find_key;
/// let mut map = std::collections::HashMap::new();
/// map.insert("a", 1);
/// map.insert("b", 2);
/// map.insert("c", 3);
/// let result = find_key(&map, 2);
/// assert_eq!(result, Some(&"b"));
/// ```
///
/// ```
/// use lowdash::find_key;
/// let mut map = std::collections::HashMap::new();
/// map.insert("a", 1);
/// map.insert("b", 2);
/// map.insert("c", 3);
/// let result = find_key(&map, 4);
/// assert_eq!(result, None);
/// ```
///
/// ```
/// use lowdash::find_key;
///
/// #[derive(Debug, PartialEq, Eq, Hash)]
/// struct Person {
///    name: String,
///    age: u32,
/// }
///
/// let mut map = std::collections::HashMap::new();
/// map.insert(Person { name: "Alice".to_string(), age: 25 }, "Engineer");
/// map.insert(Person { name: "Bob".to_string(), age: 30 }, "Manager");
/// map.insert(Person { name: "Carol".to_string(), age: 35 }, "Director");
///
/// let result = find_key(&map, "Manager");
/// assert_eq!(result, Some(&Person { name: "Bob".to_string(), age: 30 }));
/// ```
pub fn find_key<K, V>(object: &std::collections::HashMap<K, V>, value: V) -> Option<&K>
where
    K: std::cmp::Eq + std::hash::Hash,
    V: std::cmp::PartialEq,
{
    for (k, v) in object {
        if *v == value {
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
    fn test_find_key() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);
        let result = find_key(&map, 2);
        assert_eq!(result, Some(&"b"));
    }

    #[test]
    fn test_find_key_not_found() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);
        let result = find_key(&map, 4);
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_key_empty_map() {
        let map: HashMap<&str, i32> = HashMap::new();
        let result = find_key(&map, 1);
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_key_with_struct() {
        #[derive(Debug, PartialEq, Eq, Hash)]
        struct Person {
            name: String,
            age: u32,
        }

        let mut map = HashMap::new();
        map.insert(
            Person {
                name: "Alice".to_string(),
                age: 25,
            },
            "Engineer",
        );
        map.insert(
            Person {
                name: "Bob".to_string(),
                age: 30,
            },
            "Manager",
        );
        map.insert(
            Person {
                name: "Carol".to_string(),
                age: 35,
            },
            "Director",
        );

        let result = find_key(&map, "Manager");
        assert_eq!(
            result,
            Some(&Person {
                name: "Bob".to_string(),
                age: 30
            })
        );

        let not_found = find_key(&map, "Intern");
        assert_eq!(not_found, None);
    }
}
