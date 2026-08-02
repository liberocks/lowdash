/// Removes duplicates, preserving first-occurrence order.
/// Equality uses `PartialEq`, so `NaN` values follow normal `PartialEq` behavior.
/// **Time Complexity:** O(n²), where `n` is the collection length.
///
/// # Arguments
/// * `collection` - Items to deduplicate.
///
/// # Type Parameters
/// * `T` - Partially comparable, cloneable item type.
///
/// # Returns
/// * `Vec<T>` - The unique items in first-occurrence order.
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
