use std::cmp::Ordering;

/// Determines if a collection is sorted in ascending order.
///
/// This function checks whether the elements in the provided slice are in non-decreasing order.
/// It iterates through the slice and compares each element with its predecessor.
///
/// **Time Complexity:** O(n), where n is the number of elements in the collection.
///
/// # Arguments
///
/// * `collection` - A slice of items to be checked for sorted order.
///
/// # Type Parameters
///
/// * `T` - The type of elements in the collection. Must implement `PartialOrd`.
///
/// # Returns
///
/// * `true` if the collection is sorted in ascending order.
/// * `false` otherwise.
///
/// # Examples
///
/// ```rust
/// use lowdash::is_sorted;
///
/// let numbers = vec![1, 2, 3, 4, 5];
/// assert_eq!(is_sorted(&numbers), true);
/// ```
///
/// ```rust
/// use lowdash::is_sorted;
///
/// let numbers = vec![5, 4, 3, 2, 1];
/// assert_eq!(is_sorted(&numbers), false);
/// ```
///
/// ```rust
/// use lowdash::is_sorted;
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
/// assert_eq!(is_sorted(&people), true);
/// ```
///
/// ```rust
/// use lowdash::is_sorted;
///
/// let floats = vec![1.1, 2.2, 3.3, 4.4];
/// assert_eq!(is_sorted(&floats), true);
/// ```
///
/// ```rust
/// use lowdash::is_sorted;
///
/// let floats = vec![1.1, std::f64::NAN, 3.3];
/// // Note: Any comparison with NaN returns false, so the slice is not considered sorted.
/// assert_eq!(is_sorted(&floats), false);
/// ```
pub fn is_sorted<T>(collection: &[T]) -> bool
where
    T: PartialOrd,
{
    for i in 1..collection.len() {
        match collection[i - 1].partial_cmp(&collection[i]) {
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
    fn test_is_sorted_empty() {
        let empty: Vec<i32> = vec![];
        assert_eq!(is_sorted(&empty), true);
    }

    #[test]
    fn test_is_sorted_single_element() {
        let single = vec![1];
        assert_eq!(is_sorted(&single), true);
    }

    #[test]
    fn test_is_sorted_already_sorted() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(is_sorted(&numbers), true);
    }

    #[test]
    fn test_is_sorted_not_sorted() {
        let numbers = vec![1, 3, 2, 4, 5];
        assert_eq!(is_sorted(&numbers), false);
    }

    #[test]
    fn test_is_sorted_with_duplicates_sorted() {
        let numbers = vec![1, 2, 2, 3, 4, 4, 5];
        assert_eq!(is_sorted(&numbers), true);
    }

    #[test]
    fn test_is_sorted_with_duplicates_not_sorted() {
        let numbers = vec![1, 2, 2, 1, 4, 5];
        assert_eq!(is_sorted(&numbers), false);
    }

    #[test]
    fn test_is_sorted_structs_sorted() {
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
        assert_eq!(is_sorted(&people), true);
    }

    #[test]
    fn test_is_sorted_structs_not_sorted() {
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
        assert_eq!(is_sorted(&people), false);
    }

    #[test]
    fn test_is_sorted_floats_sorted() {
        let floats = vec![1.1, 2.2, 3.3, 4.4, 5.5];
        assert_eq!(is_sorted(&floats), true);
    }

    #[test]
    fn test_is_sorted_floats_not_sorted() {
        let floats = vec![1.1, 3.3, 2.2, 4.4];
        assert_eq!(is_sorted(&floats), false);
    }

    #[test]
    fn test_is_sorted_floats_with_nan() {
        let floats = vec![1.1, std::f64::NAN, 3.3];
        // Any comparison with NaN returns false, so the slice is not considered sorted.
        assert_eq!(is_sorted(&floats), false);
    }

    #[test]
    fn test_is_sorted_floats_with_infinity() {
        let floats = vec![1.1, 2.2, std::f64::INFINITY, 4.4];
        // Since Infinity > 4.4, the slice is not sorted.
        assert_eq!(is_sorted(&floats), false);
    }

    #[test]
    fn test_is_sorted_characters_sorted() {
        let chars = vec!['a', 'b', 'c', 'd', 'e'];
        assert_eq!(is_sorted(&chars), true);
    }

    #[test]
    fn test_is_sorted_characters_not_sorted() {
        let chars = vec!['a', 'c', 'b', 'd'];
        assert_eq!(is_sorted(&chars), false);
    }

    #[test]
    fn test_is_sorted_with_optionals_sorted() {
        let collection = vec![Some(1), Some(2), Some(3)];
        assert_eq!(is_sorted(&collection), true);
    }

    #[test]
    fn test_is_sorted_with_optionals_not_sorted() {
        let collection = vec![Some(1), Some(3), Some(2)];
        assert_eq!(is_sorted(&collection), false);
    }
}
