use std::collections::HashMap;
use std::hash::Hash;

/// Creates a `HashMap` by mapping each element in a collection to a key using an iteratee function.
/// If multiple elements map to the same key, the last occurrence will overwrite previous ones.
///
/// **Note:** The key type `K` must implement both `Eq` and `Hash` traits to be used in a `HashMap`.
///
/// **Time Complexity:**  
/// O(n), where n is the number of elements in the collection.
///
/// # Arguments
///
/// * `collection` - A slice of items from which to create the `HashMap`.
/// * `iteratee` - A function that takes an item from the collection and returns a key of type `K`.
///
/// # Type Parameters
///
/// * `K` - The type of keys in the resulting `HashMap`. Must implement `Eq` and `Hash`.
/// * `V` - The type of values in the collection. Must implement `Clone`.
/// * `F` - The type of the iteratee function. Must implement `Fn(&V) -> K`.
///
/// # Returns
///
/// * `HashMap<K, V>` - A `HashMap` where each key is the result of applying the iteratee to an element in the collection,
///   and each value is the corresponding element from the collection.
///
/// # Examples
///
/// ```rust
/// use lowdash::key_by;
/// use std::collections::HashMap;
///
/// let numbers = vec![1, 2, 3, 4, 5];
/// let map = key_by(&numbers, |&x| x % 2);
/// let mut expected = HashMap::new();
/// expected.insert(1, 5); // Last odd number
/// expected.insert(0, 4); // Last even number
/// assert_eq!(map, expected);
/// ```
///
/// ```rust
/// use lowdash::key_by;
/// use std::collections::HashMap;
///
/// #[derive(Debug, PartialEq, Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let people = vec![
///     Person { name: "Alice".to_string(), age: 25 },
///     Person { name: "Bob".to_string(), age: 30 },
///     Person { name: "Charlie".to_string(), age: 35 },
/// ];
///
/// let map = key_by(&people, |person| person.name.clone());
/// let mut expected = HashMap::new();
/// expected.insert("Alice".to_string(), people[0].clone());
/// expected.insert("Bob".to_string(), people[1].clone());
/// expected.insert("Charlie".to_string(), people[2].clone());
/// assert_eq!(map, expected);
/// ```
///
/// ```rust
/// use lowdash::key_by;
/// use std::collections::HashMap;
///
/// let strings = vec!["apple", "banana", "apricot", "blueberry"];
/// let map = key_by(&strings, |s| s.chars().next().unwrap());
/// let mut expected = HashMap::new();
/// expected.insert('a', "apricot");
/// expected.insert('b', "blueberry");
/// assert_eq!(map, expected);
/// ```
pub fn key_by<K, V, F>(collection: &[V], iteratee: F) -> HashMap<K, V>
where
    K: Eq + Hash,
    V: Clone,
    F: Fn(&V) -> K,
{
    let mut result = HashMap::with_capacity(collection.len());

    for item in collection {
        let key = iteratee(item);
        result.insert(key, item.clone());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Clone)]
    struct Person {
        name: String,
        age: u32,
    }

    #[test]
    fn test_key_by_integers_modulo() {
        let numbers = vec![1, 2, 3, 4, 5];
        let map = key_by(&numbers, |&x| x % 2);
        let mut expected = HashMap::new();
        expected.insert(1, 5); // Last odd number
        expected.insert(0, 4); // Last even number
        assert_eq!(map, expected);
    }

    #[test]
    fn test_key_by_strings_first_char() {
        let strings = vec!["apple", "banana", "apricot", "blueberry"];
        let map = key_by(&strings, |s| s.chars().next().unwrap());
        let mut expected = HashMap::new();
        expected.insert('a', "apricot");
        expected.insert('b', "blueberry");
        assert_eq!(map, expected);
    }

    #[test]
    fn test_key_by_with_structs() {
        let people = vec![
            Person {
                name: "Alice".to_string(),
                age: 25,
            },
            Person {
                name: "Bob".to_string(),
                age: 30,
            },
            Person {
                name: "Charlie".to_string(),
                age: 35,
            },
        ];

        let map = key_by(&people, |person| person.name.clone());
        let mut expected = HashMap::new();
        expected.insert("Alice".to_string(), people[0].clone());
        expected.insert("Bob".to_string(), people[1].clone());
        expected.insert("Charlie".to_string(), people[2].clone());
        assert_eq!(map, expected);
    }

    #[test]
    fn test_key_by_with_duplicate_keys() {
        let numbers = vec![1, 2, 3, 2, 4, 3, 5];
        let map = key_by(&numbers, |&x| x);
        let mut expected = HashMap::new();
        expected.insert(1, 1);
        expected.insert(2, 2); // Last occurrence
        expected.insert(3, 3); // Last occurrence
        expected.insert(4, 4);
        expected.insert(5, 5);
        assert_eq!(map, expected);
    }

    #[test]
    fn test_key_by_with_empty_collection() {
        let empty: Vec<i32> = vec![];
        let map: HashMap<i32, i32> = key_by(&empty, |&x| x);
        let expected: HashMap<i32, i32> = HashMap::new();
        assert_eq!(map, expected);
    }

    #[test]
    fn test_key_by_with_single_element() {
        let single = vec![42];
        let map = key_by(&single, |&x| x);
        let mut expected = HashMap::new();
        expected.insert(42, 42);
        assert_eq!(map, expected);
    }

    #[test]
    fn test_key_by_preserves_latest_value() {
        let numbers = vec![10, 20, 30, 20, 10];
        let map = key_by(&numbers, |&x| x);
        let mut expected = HashMap::new();
        expected.insert(10, 10); // Last occurrence
        expected.insert(20, 20); // Last occurrence
        expected.insert(30, 30);
        assert_eq!(map, expected);
    }

    #[test]
    fn test_key_by_with_optionals() {
        let collection = vec![Some(1), None, Some(2), Some(1), None, Some(3), Some(2)];
        let map = key_by(&collection, |&x| x.clone());
        let mut expected = HashMap::new();
        expected.insert(Some(1), Some(1)); // Last occurrence
        expected.insert(None, None); // Last occurrence
        expected.insert(Some(2), Some(2)); // Last occurrence
        expected.insert(Some(3), Some(3));
        assert_eq!(map, expected);
    }

    #[test]
    fn test_key_by_with_custom_key() {
        let people = vec![
            Person {
                name: "Alice".to_string(),
                age: 25,
            },
            Person {
                name: "Bob".to_string(),
                age: 30,
            },
            Person {
                name: "Charlie".to_string(),
                age: 35,
            },
            Person {
                name: "Alice".to_string(),
                age: 28,
            },
        ];

        let map = key_by(&people, |person| person.name.clone());
        let mut expected = HashMap::new();
        expected.insert("Alice".to_string(), people[3].clone()); // Last Alice
        expected.insert("Bob".to_string(), people[1].clone());
        expected.insert("Charlie".to_string(), people[2].clone());
        assert_eq!(map, expected);
    }

    #[test]
    fn test_key_by_with_hashmap_keys() {
        let collection = vec![("a", 1), ("b", 2), ("a", 3), ("c", 4)];
        let map = key_by(&collection, |&(k, _)| k.to_string());
        let mut expected = HashMap::new();
        expected.insert("a".to_string(), ("a", 3));
        expected.insert("b".to_string(), ("b", 2));
        expected.insert("c".to_string(), ("c", 4));
        assert_eq!(map, expected);
    }

    #[test]
    fn test_key_by_with_vectors_as_values() {
        let collection = vec![vec![1], vec![2, 2], vec![1], vec![3, 3, 3]];
        let map = key_by(&collection, |v| v.len());
        let mut expected = HashMap::new();
        expected.insert(1, vec![1]); // Last vector with length 1
        expected.insert(2, vec![2, 2]); // Last vector with length 2
        expected.insert(3, vec![3, 3, 3]); // Last vector with length 3
        assert_eq!(map, expected);
    }
}
