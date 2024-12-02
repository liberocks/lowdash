use std::cmp::Ordering;

/// Determines if a collection is sorted in ascending order based on a specified key.
///
/// This function iterates through the provided collection, applying the `iteratee` function
/// to each element to extract a key. It then checks if the sequence of keys is in non-decreasing
/// order. If all consecutive keys satisfy the ordering condition, the function returns `true`,
/// indicating that the collection is sorted based on the specified key. Otherwise, it returns `false`.
///
/// **Time Complexity:** O(n), where n is the number of elements in the collection.
///
/// # Arguments
///
/// * `collection` - A slice of items to be checked for sorted order based on the key.
/// * `iteratee` - A function that extracts the key from each item for comparison.
///
/// # Type Parameters
///
/// * `T` - The type of elements in the collection.
/// * `K` - The type of the key extracted from each element. Must implement `PartialOrd`.
/// * `F` - The type of the iteratee function.
///
/// # Returns
///
/// * `true` if the collection is sorted in ascending order based on the keys.
/// * `false` otherwise.
///
/// # Examples
///
/// ```rust
/// use lowdash::is_sorted_by_key;
///
/// let numbers = vec![
///     (1, "a"),
///     (2, "b"),
///     (3, "c"),
///     (4, "d"),
/// ];
/// let result = is_sorted_by_key(&numbers, |item| item.0);
/// assert_eq!(result, true);
/// ```
///
/// ```rust
/// use lowdash::is_sorted_by_key;
///
/// let numbers = vec![
///     (1, "a"),
///     (3, "b"),
///     (2, "c"),
///     (4, "d"),
/// ];
/// let result = is_sorted_by_key(&numbers, |item| item.0);
/// assert_eq!(result, false);
/// ```
///
/// ```rust
/// use lowdash::is_sorted_by_key;
///
/// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let people = vec![
///     Person { name: "Alice".to_string(), age: 25 },
///     Person { name: "Bob".to_string(), age: 30 },
///     Person { name: "Carol".to_string(), age: 35 },
/// ];
/// let result = is_sorted_by_key(&people, |p| p.age);
/// assert_eq!(result, true);
/// ```
///
/// ```rust
/// use lowdash::is_sorted_by_key;
///
/// let floats = vec![
///     (1.1, "apple"),
///     (2.2, "banana"),
///     (3.3, "cherry"),
///     (4.4, "date"),
/// ];
/// let result = is_sorted_by_key(&floats, |item| item.0);
/// assert_eq!(result, true);
/// ```
pub fn is_sorted_by_key<T, K, F>(collection: &[T], iteratee: F) -> bool
where
    F: Fn(&T) -> K,
    K: PartialOrd,
{
    if collection.len() < 2 {
        return true;
    }

    for i in 0..collection.len() - 1 {
        let current_key = iteratee(&collection[i]);
        let next_key = iteratee(&collection[i + 1]);
        match current_key.partial_cmp(&next_key) {
            Some(Ordering::Greater) | None => return false,
            Some(Ordering::Less) | Some(Ordering::Equal) => continue,
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
    struct Person {
        name: String,
        age: u32,
    }

    #[test]
    fn test_is_sorted_by_key_empty_collection() {
        let empty: Vec<i32> = vec![];
        let result = is_sorted_by_key(&empty, |&x| x);
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_sorted_by_key_single_element() {
        let single = vec![1];
        let result = is_sorted_by_key(&single, |&x| x);
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_sorted_by_key_already_sorted() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = is_sorted_by_key(&numbers, |&x| x);
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_sorted_by_key_not_sorted() {
        let numbers = vec![1, 3, 2, 4, 5];
        let result = is_sorted_by_key(&numbers, |&x| x);
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_sorted_by_key_with_duplicates_sorted() {
        let numbers = vec![1, 2, 2, 3, 4, 4, 5];
        let result = is_sorted_by_key(&numbers, |&x| x);
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_sorted_by_key_with_duplicates_not_sorted() {
        let numbers = vec![1, 2, 2, 1, 4, 5];
        let result = is_sorted_by_key(&numbers, |&x| x);
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_sorted_by_key_structs_sorted() {
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
        ];
        let result = is_sorted_by_key(&people, |p| p.age);
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_sorted_by_key_structs_not_sorted() {
        let people = vec![
            Person {
                name: "Carol".to_string(),
                age: 35,
            },
            Person {
                name: "Bob".to_string(),
                age: 30,
            },
            Person {
                name: "Alice".to_string(),
                age: 25,
            },
        ];
        let result = is_sorted_by_key(&people, |p| p.age);
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_sorted_by_key_complex_structs_sorted() {
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
                age: 40,
            },
        ];
        let result = is_sorted_by_key(&people, |p| (p.age, p.name.clone()));
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_sorted_by_key_complex_structs_not_sorted() {
        let people = vec![
            Person {
                name: "Alice".to_string(),
                age: 25,
            },
            Person {
                name: "Carol".to_string(),
                age: 25,
            },
            Person {
                name: "Bob".to_string(),
                age: 30,
            },
        ];
        let result = is_sorted_by_key(&people, |p| (p.age, p.name.clone()));
        assert_eq!(result, true); // This is correct since (25, "Alice") <= (25, "Carol") <= (30, "Bob")
    }

    #[test]
    fn test_is_sorted_by_key_floats_sorted() {
        let floats = vec![1.1, 2.2, 3.3, 4.4, 5.5];
        let result = is_sorted_by_key(&floats, |&x| x);
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_sorted_by_key_floats_not_sorted() {
        let floats = vec![1.1, 3.3, 2.2, 4.4];
        let result = is_sorted_by_key(&floats, |&x| x);
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_sorted_by_key_with_nan_floats_sorted() {
        let floats = vec![std::f64::NAN, std::f64::NAN, std::f64::NAN];
        // Comparisons involving NaN always return false, so the function should return false
        let result = is_sorted_by_key(&floats, |&x| x);
        assert_eq!(result, false); // Ensure the function returns false
    }

    #[test]
    fn test_is_sorted_by_key_with_partial_order() {
        #[derive(Debug, PartialEq, PartialOrd, Clone)]
        struct BoxedNumber {
            value: Option<i32>,
        }

        let collection = vec![
            BoxedNumber { value: Some(1) },
            BoxedNumber { value: Some(2) },
            BoxedNumber { value: Some(3) },
        ];
        let result = is_sorted_by_key(&collection, |bn| bn.value.clone().unwrap());
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_sorted_by_key_with_optionals_sorted() {
        let collection = vec![Some(1), Some(2), Some(3), Some(4)];
        let result = is_sorted_by_key(&collection, |&x| x.unwrap());
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_sorted_by_key_with_optionals_not_sorted() {
        let collection = vec![Some(1), Some(3), Some(2), Some(4)];
        let result = is_sorted_by_key(&collection, |&x| x.unwrap());
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_sorted_by_key_with_custom_iteratee() {
        let words = vec!["apple", "banana", "cherry", "date"];
        // Sort by the length of the string
        let result = is_sorted_by_key(&words, |word| word.len());
        assert_eq!(result, false); // Updated expectation to false
    }

    #[test]
    fn test_is_sorted_by_key_with_custom_iteratee_sorted() {
        let words = vec!["fig", "date", "apple", "banana", "cherry"];
        // Sort by the length of the string
        let result = is_sorted_by_key(&words, |word| word.len());
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_sorted_by_key_with_custom_iteratee_not_sorted() {
        let words = vec!["apple", "banana", "fig", "date"];
        // Sort by the length of the string
        let result = is_sorted_by_key(&words, |word| word.len());
        assert_eq!(result, false);
    }
}
