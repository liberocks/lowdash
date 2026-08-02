use std::collections::HashMap;
use std::hash::Hash;

/// Counts occurrences of each mapped value.
///
/// **Time Complexity:** O(n), where `n` is the collection length.
///
/// # Arguments
///
/// * `collection` - Items to count.
/// * `mapper` - Function mapping an item to a key.
///
/// # Type Parameters
///
/// * `T` - Item type.
/// * `U` - Hashable, comparable key type.
///
/// # Returns
///
/// * `HashMap<U, usize>` - A map from each mapped value to its count.
///
/// # Examples
///
/// ```rust
/// use lowdash::count_values_by;
/// use std::collections::HashMap;
///
/// let chars = vec!['a', 'b', 'a', 'c', 'b', 'd'];
/// let result = count_values_by(&chars, |x| x.clone());
/// let mut expected = HashMap::new();
/// expected.insert('a', 2);
/// expected.insert('b', 2);
/// expected.insert('c', 1);
/// expected.insert('d', 1);
/// assert_eq!(result, expected);
/// ```
///
/// ```rust
/// use lowdash::count_values_by;
/// use std::collections::HashMap;
///
///  let numbers = vec![1, 2, 2, 3, 4, 3, 5];
/// let result = count_values_by(&numbers, |x| *x);
/// let mut expected = HashMap::new();
/// expected.insert(1, 1);
/// expected.insert(2, 2);
/// expected.insert(3, 2);
/// expected.insert(4, 1);
/// expected.insert(5, 1);
/// assert_eq!(result, expected);
/// ```
pub fn count_values_by<T, U, F>(collection: &[T], mapper: F) -> HashMap<U, usize>
where
    U: Hash + Eq + Clone,
    F: Fn(&T) -> U,
{
    let mut result = HashMap::new();
    for item in collection {
        let key = mapper(item);
        *result.entry(key).or_insert(0) += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Float;
    use std::collections::HashMap;

    #[test]
    fn test_count_values_by_integers() {
        let numbers = vec![1, 2, 2, 3, 4, 3, 5];
        let result = count_values_by(&numbers, |x| *x);
        let mut expected = HashMap::new();
        expected.insert(1, 1);
        expected.insert(2, 2);
        expected.insert(3, 2);
        expected.insert(4, 1);
        expected.insert(5, 1);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_values_by_strings() {
        let strings = vec!["apple", "banana", "apple", "cherry", "banana"];
        let result = count_values_by(&strings, |x| x.to_string());
        let mut expected = HashMap::new();
        expected.insert("apple".to_string(), 2);
        expected.insert("banana".to_string(), 2);
        expected.insert("cherry".to_string(), 1);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_values_by_structs() {
        #[derive(Debug, PartialEq, Eq, Hash, Clone)]
        struct Person {
            name: String,
            age: u32,
        }

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
                name: "Alice".to_string(),
                age: 25,
            },
            Person {
                name: "Carol".to_string(),
                age: 35,
            },
        ];

        let result = count_values_by(&people, |p| p.clone());
        let mut expected = HashMap::new();
        expected.insert(
            Person {
                name: "Alice".to_string(),
                age: 25,
            },
            2,
        );
        expected.insert(
            Person {
                name: "Bob".to_string(),
                age: 30,
            },
            1,
        );
        expected.insert(
            Person {
                name: "Carol".to_string(),
                age: 35,
            },
            1,
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_values_by_with_floats() {
        let float_collection = vec![
            Float(1.1),
            Float(2.2),
            Float(2.2),
            Float(3.3),
            Float(4.4),
            Float(3.3),
            Float(5.5),
        ];
        let result = count_values_by(&float_collection, |f| f.clone());
        let mut expected = HashMap::new();
        expected.insert(Float(1.1), 1);
        expected.insert(Float(2.2), 2);
        expected.insert(Float(3.3), 2);
        expected.insert(Float(4.4), 1);
        expected.insert(Float(5.5), 1);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_values_by_with_optionals() {
        let collection = vec![Some(1), None, Some(2), Some(1), None, Some(3), Some(2)];
        let result = count_values_by(&collection, |x| x.clone());
        let mut expected = HashMap::new();
        expected.insert(Some(1), 2);
        expected.insert(None, 2);
        expected.insert(Some(2), 2);
        expected.insert(Some(3), 1);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_values_by_with_identity_mapper() {
        let chars = vec!['a', 'b', 'a', 'c', 'b', 'd'];
        let result = count_values_by(&chars, |x| x.clone());
        let mut expected = HashMap::new();
        expected.insert('a', 2);
        expected.insert('b', 2);
        expected.insert('c', 1);
        expected.insert('d', 1);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_values_by_empty_collection() {
        let empty: Vec<i32> = vec![];
        let result: HashMap<i32, usize> = count_values_by(&empty, |x| *x);
        let expected: HashMap<i32, usize> = HashMap::new();
        assert_eq!(result, expected);
    }
}
