/// Remove duplicate elements from a collection based on a key extracted by a provided function,
/// preserving the order of their first occurrence.
///
/// This function takes a slice of items and returns a new `Vec<T>` containing only the unique elements,
/// determined by the key extracted using the provided `iteratee` function. The order of first occurrences
/// is preserved.
///
/// **Note:** This implementation requires `U` to implement `PartialEq` and `Clone`.
/// While it doesn't leverage hashing for efficiency, it ensures compatibility with all types,
/// including those like floating-point numbers (`f32`, `f64`) that do not implement `Eq`.
///
/// # Arguments
///
/// * `collection` - A slice of items from which to extract unique elements.
/// * `iteratee` - A function that takes a reference to an item and returns a key of type `U`.
///
/// # Type Parameters
///
/// * `T` - The type of elements in the collection. Must implement `Clone`.
/// * `U` - The type of the key extracted from each element used to determine uniqueness. Must implement `PartialEq` and `Clone`.
/// * `F` - The type of the iteratee function. Must implement `Fn(&T) -> U`.
///
/// # Returns
///
/// * `Vec<T>` - A vector containing the unique elements from the input collection, in the order they first appear.
///
/// # Examples
///
/// ```rust
/// use lowdash::uniq_by;
/// let numbers = vec![1, 2, 2, 3, 4, 3, 5];
/// let unique_numbers = uniq_by(&numbers, |x| *x);
/// assert_eq!(unique_numbers, vec![1, 2, 3, 4, 5]);
/// ```
///
/// ```rust
/// use lowdash::uniq_by;
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
/// let unique_people = uniq_by(&people, |p| p.name.clone());
/// assert_eq!(unique_people, vec![
///     Person { name: "Alice".to_string(), age: 25 },
///     Person { name: "Bob".to_string(), age: 30 },
///     Person { name: "Carol".to_string(), age: 35 },
/// ]);
/// ```
pub fn uniq_by<T, U, F>(collection: &[T], iteratee: F) -> Vec<T>
where
    T: Clone,
    U: PartialEq + Clone,
    F: Fn(&T) -> U,
{
    let mut seen = Vec::with_capacity(collection.len());
    let mut result = Vec::with_capacity(collection.len());

    for item in collection {
        let key = iteratee(item);
        if !seen.contains(&key) {
            seen.push(key.clone());
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

    #[derive(Debug, PartialEq, Clone)]
    struct Item {
        id: u32,
        value: String,
    }

    #[test]
    fn test_uniq_by_integers() {
        let numbers = vec![1, 2, 2, 3, 4, 3, 5];
        let unique_numbers = uniq_by(&numbers, |x| *x);
        assert_eq!(unique_numbers, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_uniq_by_strings() {
        let strings = vec!["apple", "banana", "apple", "cherry", "banana"];
        let unique_strings = uniq_by(&strings, |s| s.to_string());
        assert_eq!(unique_strings, vec!["apple", "banana", "cherry"]);
    }

    #[test]
    fn test_uniq_by_with_structs() {
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

        let unique_people = uniq_by(&people, |p| p.name.clone());
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
    fn test_uniq_by_with_structs_multiple_keys() {
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
                age: 30,
            },
            Person {
                name: "Carol".to_string(),
                age: 35,
            },
        ];

        // Uniq by name
        let unique_by_name = uniq_by(&people, |p| p.name.clone());
        assert_eq!(
            unique_by_name,
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

        // Uniq by age
        let unique_by_age = uniq_by(&people, |p| p.age);
        assert_eq!(
            unique_by_age,
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
    fn test_uniq_by_with_empty_collection() {
        let empty: Vec<i32> = vec![];
        let unique = uniq_by(&empty, |x| *x);
        assert_eq!(unique, Vec::<i32>::new());
    }

    #[test]
    fn test_uniq_by_with_no_duplicates() {
        let collection = vec![1, 2, 3, 4, 5];
        let unique = uniq_by(&collection, |x| *x);
        assert_eq!(unique, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_uniq_by_with_all_duplicates() {
        let collection = vec![1, 1, 1, 1, 1];
        let unique = uniq_by(&collection, |x| *x);
        assert_eq!(unique, vec![1]);
    }

    #[test]
    fn test_uniq_by_with_floats() {
        let float_collection = vec![1.1, 2.2, 2.2, 3.3, 4.4, 3.3, 5.5];
        let unique_floats = uniq_by(&float_collection, |x| *x);
        assert_eq!(unique_floats, vec![1.1, 2.2, 3.3, 4.4, 5.5]);
    }

    #[test]
    fn test_uniq_by_with_characters() {
        let chars = vec!['a', 'b', 'a', 'c', 'b', 'd'];
        let unique_chars = uniq_by(&chars, |c| *c);
        assert_eq!(unique_chars, vec!['a', 'b', 'c', 'd']);
    }

    #[test]
    fn test_uniq_by_preserves_order() {
        let numbers = vec![3, 1, 2, 3, 2, 4, 1, 5];
        let unique_numbers = uniq_by(&numbers, |x| *x);
        assert_eq!(unique_numbers, vec![3, 1, 2, 4, 5]);
    }

    #[test]
    fn test_uniq_by_with_mixed_types() {
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

        let unique_items = uniq_by(&items, |item| item.id);
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
    fn test_uniq_by_with_optionals() {
        let collection = vec![Some(1), None, Some(2), Some(1), None, Some(3), Some(2)];
        let unique = uniq_by(&collection, |x| x.clone());
        assert_eq!(unique, vec![Some(1), None, Some(2), Some(3)]);
    }

    #[test]
    fn test_uniq_by_with_nan_floats() {
        let float_collection: Vec<f64> =
            vec![std::f64::NAN, std::f64::INFINITY, std::f64::NAN, 1.0];
        let unique_floats = uniq_by(&float_collection, |x| *x);
        // Note: NaN != NaN, so each NaN is considered unique
        assert_eq!(unique_floats.len(), 4);
        assert!(unique_floats[0].is_nan());
        assert!(unique_floats[1].is_infinite());
        assert!(unique_floats[2].is_nan());
        assert_eq!(unique_floats[3], 1.0);
    }

    #[test]
    fn test_uniq_by_with_keys() {
        #[derive(Debug, PartialEq, Clone)]
        struct Record {
            id: u32,
            name: String,
        }

        let records = vec![
            Record {
                id: 1,
                name: "Record1".to_string(),
            },
            Record {
                id: 2,
                name: "Record2".to_string(),
            },
            Record {
                id: 1,
                name: "Record1_Duplicate".to_string(),
            },
            Record {
                id: 3,
                name: "Record3".to_string(),
            },
            Record {
                id: 2,
                name: "Record2_Duplicate".to_string(),
            },
        ];

        let unique_by_id = uniq_by(&records, |r| r.id);
        assert_eq!(
            unique_by_id,
            vec![
                Record {
                    id: 1,
                    name: "Record1".to_string()
                },
                Record {
                    id: 2,
                    name: "Record2".to_string()
                },
                Record {
                    id: 3,
                    name: "Record3".to_string()
                },
            ]
        );

        let unique_by_name = uniq_by(&records, |r| r.name.clone());
        assert_eq!(
            unique_by_name,
            vec![
                Record {
                    id: 1,
                    name: "Record1".to_string()
                },
                Record {
                    id: 2,
                    name: "Record2".to_string()
                },
                Record {
                    id: 1,
                    name: "Record1_Duplicate".to_string()
                },
                Record {
                    id: 3,
                    name: "Record3".to_string()
                },
                Record {
                    id: 2,
                    name: "Record2_Duplicate".to_string()
                },
            ]
        );
    }
}
