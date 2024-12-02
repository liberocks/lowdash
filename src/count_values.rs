use std::collections::HashMap;
use std::hash::Hash;

/// Counts the number of occurrences of each value in a collection.
///
/// This function iterates over a slice of items and returns a `HashMap` where each key is a unique
/// item from the collection, and the corresponding value is the number of times that item appears.
///
/// **Time Complexity:** O(n), where n is the number of elements in the collection.
///
/// # Arguments
///
/// * `collection` - A slice of items to be counted.
///
/// # Type Parameters
///
/// * `T` - The type of elements in the input collection. Must implement `Hash`, `Eq`, and `Clone`.
///
/// # Returns
///
/// * `HashMap<T, usize>` - A map where keys are unique items from the collection and values are their counts.
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
    let mut result = HashMap::new();
    for item in collection {
        *result.entry(item.clone()).or_insert(0) += 1;
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
            common::Float(std::f64::NAN),
            common::Float(std::f64::INFINITY),
            common::Float(std::f64::NAN),
            common::Float(1.0),
        ];
        let result = count_values(&float_collection);
        let mut expected = HashMap::new();
        expected.insert(common::Float(std::f64::NAN), 2);
        expected.insert(common::Float(std::f64::INFINITY), 1);
        expected.insert(common::Float(1.0), 1);
        // Note: HashMap treats different NaN representations as distinct keys
        assert_eq!(result.get(&common::Float(std::f64::NAN)), Some(&2));
        assert_eq!(result.get(&common::Float(std::f64::INFINITY)), Some(&1));
        assert_eq!(result.get(&common::Float(1.0)), Some(&1));
    }
}
