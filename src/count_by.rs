/// Counts the number of elements in a collection that satisfy a given predicate.
///
/// This function iterates over a slice of items and applies the provided predicate to each item.
/// It returns the total count of items for which the predicate returns `true`.
///
/// **Time Complexity:** O(n), where n is the number of elements in the collection.
///
/// # Arguments
///
/// * `collection` - A slice of items to be evaluated.
/// * `predicate` - A function that takes a reference to an item and returns a boolean.
///                If the predicate returns `true`, the item is counted.
///
/// # Type Parameters
///
/// * `T` - The type of elements in the input collection.
/// * `F` - The type of the predicate function.
///
/// # Returns
///
/// * `usize` - The number of elements in the collection that satisfy the predicate.
///
/// # Examples
///
/// ```rust
/// use lowdash::count_by;
///
/// let numbers = vec![1, 2, 3, 4, 5];
/// // Count even numbers
/// let result = count_by(&numbers, |&x| x % 2 == 0);
/// assert_eq!(result, 2);
/// ```
///
/// ```rust
/// use lowdash::count_by;
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
///     Person { name: "Carol".to_string(), age: 35 },
///     Person { name: "Dave".to_string(), age: 30 },
/// ];
///
/// // Count people who are at least 30 years old
/// let count_adults = count_by(&people, |p| p.age >= 30);
/// assert_eq!(count_adults, 3);
/// ```
pub fn count_by<T, F>(collection: &[T], predicate: F) -> usize
where
    F: Fn(&T) -> bool,
{
    let mut count = 0;
    for item in collection {
        if predicate(item) {
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
    fn test_count_by_integers() {
        let numbers = vec![1, 2, 3, 4, 5];
        // Count even numbers
        let result = count_by(&numbers, |&x| x % 2 == 0);
        assert_eq!(result, 2);

        // Count numbers greater than 3
        let result_gt_3 = count_by(&numbers, |&x| x > 3);
        assert_eq!(result_gt_3, 2);
    }

    #[test]
    fn test_count_by_strings() {
        let strings = vec!["apple", "banana", "apple", "cherry", "banana"];
        // Count occurrences of "apple"
        let count_apple = count_by(&strings, |&s| s == "apple");
        assert_eq!(count_apple, 2);

        // Count strings longer than 5 characters
        let count_long = count_by(&strings, |&s| s.len() > 5);
        assert_eq!(count_long, 3); // Corrected from 2 to 3
    }

    #[test]
    fn test_count_by_structs() {
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
                name: "Carol".to_string(),
                age: 35,
            },
            Person {
                name: "Dave".to_string(),
                age: 30,
            },
        ];

        // Count people who are at least 30 years old
        let count_adults = count_by(&people, |p| p.age >= 30);
        assert_eq!(count_adults, 3);

        // Count people with names starting with 'A'
        let count_a_names = count_by(&people, |p| p.name.starts_with('A'));
        assert_eq!(count_a_names, 1);
    }

    #[test]
    fn test_count_by_empty_collection() {
        let empty: Vec<i32> = vec![];
        let result = count_by(&empty, |&x| x > 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_count_by_no_matches() {
        let numbers = vec![1, 2, 3, 4, 5];
        // Count numbers greater than 10
        let result = count_by(&numbers, |&x| x > 10);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_count_by_all_matches() {
        let numbers = vec![2, 4, 6, 8];
        // Count even numbers
        let result = count_by(&numbers, |&x| x % 2 == 0);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_count_by_with_optionals() {
        let collection = vec![Some(1), None, Some(2), Some(1), None, Some(3), Some(2)];
        // Count occurrences of Some(1)
        let count_some_1 = count_by(&collection, |item| *item == Some(1));
        assert_eq!(count_some_1, 2);

        // Count None values
        let count_none = count_by(&collection, |item| item.is_none());
        assert_eq!(count_none, 2);
    }

    #[test]
    fn test_count_by_with_floats() {
        let float_collection = vec![1.1, 2.2, 2.2, 3.3, 4.4, 3.3, 5.5];
        // Count occurrences of 2.2
        let count_2_2 = count_by(&float_collection, |&x| x == 2.2);
        assert_eq!(count_2_2, 2);

        // Count numbers greater than 3.0
        let count_gt_3 = count_by(&float_collection, |&x| x > 3.0);
        assert_eq!(count_gt_3, 4); // Corrected from 3 to 4
    }

    #[test]
    fn test_count_by_with_characters() {
        let chars = vec!['a', 'b', 'a', 'c', 'b', 'd'];
        // Count occurrences of 'a'
        let count_a = count_by(&chars, |&c| c == 'a');
        assert_eq!(count_a, 2);

        // Count vowels
        let count_vowels = count_by(&chars, |&c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'));
        assert_eq!(count_vowels, 2);
    }

    #[test]
    fn test_count_by_with_nan_floats() {
        let float_collection = vec![std::f64::NAN, 2.2, std::f64::NAN, 4.4];
        // Count NaN values
        let count_nan = count_by(&float_collection, |&x| x.is_nan());
        // Note: In Rust, NaN != NaN, so each is considered unique
        assert_eq!(count_nan, 2);

        // Count non-NaN values
        let count_non_nan = count_by(&float_collection, |&x| !x.is_nan());
        assert_eq!(count_non_nan, 2);
    }

    #[test]
    fn test_count_by_preserves_order() {
        let numbers = vec![3, 1, 2, 3, 2, 4, 1, 5];
        // Count occurrences of 2
        let result = count_by(&numbers, |&x| x == 2);
        assert_eq!(result, 2);

        // Count occurrences of 1
        let count_1 = count_by(&numbers, |&x| x == 1);
        assert_eq!(count_1, 2);
    }

    #[test]
    fn test_count_by_with_custom_types() {
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

        // Count items with id 1
        let count_id_1 = count_by(&items, |item| item.id == 1);
        assert_eq!(count_id_1, 2);

        // Count items with value "two"
        let count_two = count_by(&items, |item| item.value == "two");
        assert_eq!(count_two, 1);

        // Count items with id 4
        let count_id_4 = count_by(&items, |item| item.id == 4);
        assert_eq!(count_id_4, 0);
    }
}
