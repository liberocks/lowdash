use std::collections::HashMap;
use std::hash::Hash;

/// Counts occurrences of each value in a collection.
///
/// **Time Complexity:** O(n), where `n` is the collection length.
///
/// # Arguments
///
/// * `collection` - Items to count.
///
/// # Type Parameters
///
/// * `T` - Hashable, comparable item type.
///
/// # Returns
///
/// * `HashMap<T, usize>` - A map from each value to its count.
///
/// # Examples
///
/// ```rust
/// use lowdash::count_values;
/// use std::collections::HashMap;
///
/// let numbers = vec![1, 2, 2, 3, 4, 3, 5];
/// let result = count_values(&numbers);
/// let mut expected = HashMap::new();
/// expected.insert(1, 1);
/// expected.insert(2, 2);
/// expected.insert(3, 2);
/// expected.insert(4, 1);
/// expected.insert(5, 1);
/// assert_eq!(result, expected);
/// ```
///
/// ```rust
/// use lowdash::count_values;
/// use std::collections::HashMap;
///
/// #[derive(Debug, PartialEq, Eq, Hash, Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let people = vec![
///     Person { name: "Alice".to_string(), age: 25 },
///     Person { name: "Bob".to_string(), age: 30 },
///     Person { name: "Alice".to_string(), age: 25 },
///     Person { name: "Carol".to_string(), age: 35 },
/// ];
///
/// let result = count_values(&people);
/// let mut expected = HashMap::new();
/// expected.insert(
///     Person { name: "Alice".to_string(), age: 25 },
///     2
/// );
/// expected.insert(
///     Person { name: "Bob".to_string(), age: 30 },
///     1
/// );
/// expected.insert(
///     Person { name: "Carol".to_string(), age: 35 },
///     1
/// );
/// assert_eq!(result, expected);
/// ```
pub fn count_values<T>(collection: &[T]) -> HashMap<T, usize>
where
    T: Hash + Eq + Clone,
{
    let mut counts: HashMap<&T, usize> = HashMap::new();
    for item in collection {
        *counts.entry(item).or_insert(0) += 1;
    }

    let mut result = HashMap::with_capacity(counts.len());
    for (item, count) in counts {
        result.insert(item.clone(), count);
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::common;

    use super::*;
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    struct Person {
        name: String,
        age: u32,
    }

    #[test]
    fn test_count_values_integers() {
        let numbers = vec![1, 2, 2, 3, 4, 3, 5];
        let result = count_values(&numbers);
        let mut expected = HashMap::new();
        expected.insert(1, 1);
        expected.insert(2, 2);
        expected.insert(3, 2);
        expected.insert(4, 1);
        expected.insert(5, 1);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_values_strings() {
        let strings = vec!["apple", "banana", "apple", "cherry", "banana"];
        let result = count_values(&strings);
        let mut expected = HashMap::new();
        expected.insert("apple", 2);
        expected.insert("banana", 2);
        expected.insert("cherry", 1);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_values_with_structs() {
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

        let result = count_values(&people);
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
    fn test_count_values_empty_collection() {
        let empty: Vec<i32> = vec![];
        let result = count_values(&empty);
        let expected: HashMap<i32, usize> = HashMap::new();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_values_no_duplicates() {
        let collection = vec![1, 2, 3, 4, 5];
        let result = count_values(&collection);
        let mut expected = HashMap::new();
        expected.insert(1, 1);
        expected.insert(2, 1);
        expected.insert(3, 1);
        expected.insert(4, 1);
        expected.insert(5, 1);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_values_all_duplicates() {
        let collection = vec![2, 2, 2, 2];
        let result = count_values(&collection);
        let mut expected = HashMap::new();
        expected.insert(2, 4);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_values_with_optionals() {
        let collection = vec![Some(1), None, Some(2), Some(1), None, Some(3), Some(2)];
        let result = count_values(&collection);
        let mut expected = HashMap::new();
        expected.insert(Some(1), 2);
        expected.insert(None, 2);
        expected.insert(Some(2), 2);
        expected.insert(Some(3), 1);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_values_with_floats() {
        let float_collection = vec![
            common::Float(1.1),
            common::Float(2.2),
            common::Float(2.2),
            common::Float(3.3),
            common::Float(4.4),
            common::Float(3.3),
            common::Float(5.5),
        ];
        let result = count_values(&float_collection);
        let mut expected = HashMap::new();
        expected.insert(common::Float(1.1), 1);
        expected.insert(common::Float(2.2), 2);
        expected.insert(common::Float(3.3), 2);
        expected.insert(common::Float(4.4), 1);
        expected.insert(common::Float(5.5), 1);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_values_with_characters() {
        let chars = vec!['a', 'b', 'a', 'c', 'b', 'd'];
        let result = count_values(&chars);
        let mut expected = HashMap::new();
        expected.insert('a', 2);
        expected.insert('b', 2);
        expected.insert('c', 1);
        expected.insert('d', 1);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_values_with_nan_floats() {
        let float_collection = vec![
            common::Float(f64::NAN),
            common::Float(f64::INFINITY),
            common::Float(f64::NAN),
            common::Float(1.0),
        ];
        let result = count_values(&float_collection);
        let mut expected = HashMap::new();
        expected.insert(common::Float(f64::NAN), 2);
        expected.insert(common::Float(f64::INFINITY), 1);
        expected.insert(common::Float(1.0), 1);
        // Note: HashMap treats different NaN representations as distinct keys
        assert_eq!(result.get(&common::Float(f64::NAN)), Some(&2));
        assert_eq!(result.get(&common::Float(f64::INFINITY)), Some(&1));
        assert_eq!(result.get(&common::Float(1.0)), Some(&1));
    }
}
