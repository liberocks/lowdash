/// Counts the number of occurrences of a specific value in a collection.
///
/// This function iterates over a slice of items and returns the number of times
/// the specified `value` appears in the collection.
///
/// **Time Complexity:** O(n), where n is the number of elements in the collection.
///
/// # Arguments
///
/// * `collection` - A slice of items in which to count occurrences of `value`.
/// * `value` - The value to count within the collection.
///
/// # Type Parameters
///
/// * `T` - The type of elements in the collection. Must implement `PartialEq`.
///
/// # Returns
///
/// * `usize` - The number of times `value` appears in `collection`.
///
/// # Examples
///
/// ```rust
/// use lowdash::count;
///
/// let numbers = vec![1, 2, 2, 3, 4, 2];
/// let result = count(&numbers, 2);
/// assert_eq!(result, 3);
/// ```
///
/// ```rust
/// use lowdash::count;
///
/// #[derive(Debug, PartialEq)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let people = vec![
///     Person { name: "Alice".to_string(), age: 25 },
///     Person { name: "Bob".to_string(), age: 30 },
///     Person { name: "Alice".to_string(), age: 25 },
/// ];
///
/// let count_alice = count(&people, Person { name: "Alice".to_string(), age: 25 });
/// assert_eq!(count_alice, 2);
/// ```
pub fn count<T>(collection: &[T], value: T) -> usize
where
    T: PartialEq,
{
    let mut count = 0;
    for item in collection {
        if *item == value {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Person {
        name: String,
        age: u32,
    }

    #[test]
    fn test_count_integers() {
        let numbers = vec![1, 2, 2, 3, 4, 2];
        let result = count(&numbers, 2);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_count_strings() {
        let strings = vec!["apple", "banana", "apple", "cherry", "banana"];
        let result = count(&strings, "apple");
        assert_eq!(result, 2);
    }

    #[test]
    fn test_count_structs() {
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

        let count_alice = count(
            &people,
            Person {
                name: "Alice".to_string(),
                age: 25,
            },
        );
        assert_eq!(count_alice, 2);

        let count_bob = count(
            &people,
            Person {
                name: "Bob".to_string(),
                age: 30,
            },
        );
        assert_eq!(count_bob, 1);

        let count_dave = count(
            &people,
            Person {
                name: "Dave".to_string(),
                age: 40,
            },
        );
        assert_eq!(count_dave, 0);
    }

    #[test]
    fn test_count_with_empty_collection() {
        let empty: Vec<i32> = vec![];
        let result = count(&empty, 1);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_count_with_no_matches() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = count(&numbers, 6);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_count_with_all_matches() {
        let numbers = vec![2, 2, 2, 2];
        let result = count(&numbers, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_count_with_optionals() {
        let collection = vec![Some(1), None, Some(2), Some(1), None, Some(3), Some(2)];
        let result_some_1 = count(&collection, Some(1));
        assert_eq!(result_some_1, 2);

        let result_none = count(&collection, None);
        assert_eq!(result_none, 2);
    }

    #[test]
    fn test_count_with_floats() {
        let float_collection = vec![1.1, 2.2, 2.2, 3.3, 4.4, 2.2];
        let result = count(&float_collection, 2.2);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_count_with_negatives() {
        let numbers = vec![-1, -2, -2, -3, -4, -2];
        let result = count(&numbers, -2);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_count_preserves_order() {
        let numbers = vec![3, 1, 2, 3, 2, 4, 1, 5];
        let result = count(&numbers, 2);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_count_with_custom_types() {
        #[derive(Debug, PartialEq)]
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

        let count_item1 = count(
            &items,
            Item {
                id: 1,
                value: "one".to_string(),
            },
        );
        assert_eq!(count_item1, 2);

        let count_item2 = count(
            &items,
            Item {
                id: 2,
                value: "two".to_string(),
            },
        );
        assert_eq!(count_item2, 1);

        let count_item4 = count(
            &items,
            Item {
                id: 4,
                value: "four".to_string(),
            },
        );
        assert_eq!(count_item4, 0);
    }

    #[test]
    fn test_count_with_characters() {
        let chars = vec!['a', 'b', 'a', 'c', 'b', 'd'];
        let result = count(&chars, 'a');
        assert_eq!(result, 2);
    }

    #[test]
    fn test_count_with_nan_floats() {
        let float_collection = vec![std::f64::NAN, 2.2, std::f64::NAN, 4.4];
        let result_nan = count(&float_collection, std::f64::NAN);
        // Note: In Rust, NaN != NaN, so each comparison with NaN returns false
        assert_eq!(result_nan, 0);

        let result_2_2 = count(&float_collection, 2.2);
        assert_eq!(result_2_2, 1);
    }

    #[test]
    fn test_count_with_string_slices() {
        let words = vec!["hello", "world", "hello", "rust"];
        let result = count(&words, "hello");
        assert_eq!(result, 2);
    }
}
