/// Remove duplicate elements from a collection, preserving the order of their first occurrence.
///
/// This function takes a slice of items and returns a new `Vec<T>` containing only the unique elements,
/// preserving the order in which they first appear in the input collection.
///
/// **Note:** Unlike the previous implementation, this version does not require `T` to implement `Hash` and `Eq`.
/// This allows the function to work with types like floating-point numbers (`f32`, `f64`), which do not implement `Eq`
/// due to the presence of `NaN` (Not a Number) values.
///
/// However, this approach has a time complexity of O(n²) because it performs a linear search for each element to check for duplicates.
/// Use it with caution on large collections.
///
/// # Arguments
/// * `collection` - A slice of items from which to extract unique elements.
///
/// # Type Parameters
/// * `T` - The type of elements in the collection. Must implement `PartialEq` and `Clone`.
///
/// # Returns
/// * `Vec<T>` - A vector containing the unique elements from the input collection, in the order they first appear.
///
/// # Examples
/// ```rust
/// use lowdash::uniq;
/// let numbers = vec![1, 2, 2, 3, 4, 3, 5];
/// let unique_numbers = uniq(&numbers);
/// assert_eq!(unique_numbers, vec![1, 2, 3, 4, 5]);
/// ```
///
/// ```rust
/// use lowdash::uniq;
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
///     Person { name: "Alice".to_string(), age: 25 },
///     Person { name: "Carol".to_string(), age: 35 },
/// ];
///
/// let unique_people = uniq(&people);
/// assert_eq!(unique_people, vec![
///     Person { name: "Alice".to_string(), age: 25 },
///     Person { name: "Bob".to_string(), age: 30 },
///     Person { name: "Carol".to_string(), age: 35 },
/// ]);
/// ```
pub fn uniq<T>(collection: &[T]) -> Vec<T>
where
    T: PartialEq + Clone,
{
    let mut seen = Vec::with_capacity(collection.len());
    let mut result = Vec::with_capacity(collection.len());

    for item in collection {
        if !seen.contains(item) {
            seen.push(item.clone());
            result.push(item.clone());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    struct Person {
        name: String,
        age: u32,
    }

    #[test]
    fn test_uniq_integers() {
        let numbers = vec![1, 2, 2, 3, 4, 3, 5];
        let unique_numbers = uniq(&numbers);
        assert_eq!(unique_numbers, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_uniq_strings() {
        let strings = vec!["apple", "banana", "apple", "cherry", "banana"];
        let unique_strings = uniq(&strings);
        assert_eq!(unique_strings, vec!["apple", "banana", "cherry"]);
    }

    #[test]
    fn test_uniq_with_structs() {
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

        let unique_people = uniq(&people);
        assert_eq!(
            unique_people,
            vec![
                Person {
                    name: "Alice".to_string(),
                    age: 25
                },
                Person {
                    name: "Bob".to_string(),
                    age: 30
                },
                Person {
                    name: "Carol".to_string(),
                    age: 35
                },
            ]
        );
    }

    #[test]
    fn test_uniq_with_empty_collection() {
        let empty: Vec<i32> = vec![];
        let unique = uniq(&empty);
        assert_eq!(unique, Vec::<i32>::new());
    }

    #[test]
    fn test_uniq_with_no_duplicates() {
        let collection = vec![1, 2, 3, 4, 5];
        let unique = uniq(&collection);
        assert_eq!(unique, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_uniq_with_all_duplicates() {
        let collection = vec![1, 1, 1, 1, 1];
        let unique = uniq(&collection);
        assert_eq!(unique, vec![1]);
    }

    #[test]
    fn test_uniq_with_floats() {
        let float_collection = vec![1.1, 2.2, 2.2, 3.3, 4.4, 3.3, 5.5];
        let unique_floats = uniq(&float_collection);
        assert_eq!(unique_floats, vec![1.1, 2.2, 3.3, 4.4, 5.5]);
    }

    #[test]
    fn test_uniq_with_characters() {
        let chars = vec!['a', 'b', 'a', 'c', 'b', 'd'];
        let unique_chars = uniq(&chars);
        assert_eq!(unique_chars, vec!['a', 'b', 'c', 'd']);
    }

    #[test]
    fn test_uniq_preserves_order() {
        let numbers = vec![3, 1, 2, 3, 2, 4, 1, 5];
        let unique_numbers = uniq(&numbers);
        assert_eq!(unique_numbers, vec![3, 1, 2, 4, 5]);
    }

    #[test]
    fn test_uniq_with_mixed_types() {
        #[derive(Debug, PartialEq, Clone)]
        struct Item {
            id: u32,
            value: String,
        }

        let items = vec![
            Item {
                id: 1,
                value: "one".to_string(),
            },
            Item {
                id: 2,
                value: "two".to_string(),
            },
            Item {
                id: 1,
                value: "one".to_string(),
            },
            Item {
                id: 3,
                value: "three".to_string(),
            },
        ];

        let unique_items = uniq(&items);
        assert_eq!(
            unique_items,
            vec![
                Item {
                    id: 1,
                    value: "one".to_string()
                },
                Item {
                    id: 2,
                    value: "two".to_string()
                },
                Item {
                    id: 3,
                    value: "three".to_string()
                },
            ]
        );
    }

    #[test]
    fn test_uniq_with_optionals() {
        let collection = vec![Some(1), None, Some(2), Some(1), None, Some(3), Some(2)];
        let unique = uniq(&collection);
        assert_eq!(unique, vec![Some(1), None, Some(2), Some(3),]);
    }

    #[test]
    fn test_uniq_with_nan_floats() {
        let float_collection = vec![std::f64::NAN, std::f64::INFINITY, std::f64::NAN, 1.0];
        let unique_floats = uniq(&float_collection);
        // Note: NaN != NaN, so each NaN is considered unique
        assert_eq!(unique_floats.len(), 4);
        assert!(unique_floats[0].is_nan());
        assert!(unique_floats[1].is_infinite());
        assert!(unique_floats[2].is_nan());
        assert_eq!(unique_floats[3], 1.0);
    }
}
