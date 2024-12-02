/// Applies a callback function to each item in a collection along with its index and collects the results
/// where the callback returns `false`.
///
/// This function iterates over a slice of items, applies the provided callback to each item and its index,
/// and collects the results (`R`) for which the callback returns `false`.
///
/// **Time Complexity:** O(n), where n is the number of elements in the collection.
///
/// # Arguments
///
/// * `collection` - A slice of items to iterate over.
/// * `callback` - A mutable function that takes a reference to an item and its index, returning a tuple `(R, bool)`.
///                If the second element of the tuple is `false`, the first element (`R`) is collected.
///
/// # Type Parameters
///
/// * `T` - The type of elements in the input collection.
/// * `R` - The type of elements in the resulting collection.
/// * `F` - The type of the callback function.
///
/// # Returns
///
/// * `Vec<R>` - A vector containing the results from the callback where the predicate is `false`.
///
/// # Examples
///
/// ```rust
/// use lowdash::reject_map;
///
/// let numbers = vec![1, 2, 3, 4, 5];
/// // Collect squares of odd numbers
/// let result = reject_map(&numbers, |&x, _| (x * x, x % 2 == 0));
/// assert_eq!(result, vec![1, 9, 25]);
/// ```
///
/// ```rust
/// use lowdash::reject_map;
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
/// ];
///
/// // Collect names of people who are not above 30
/// let result = reject_map(&people, |person, _| (person.name.clone(), person.age > 30));
/// assert_eq!(result, vec!["Alice".to_string(), "Bob".to_string()]);
/// ```
pub fn reject_map<T, R, F>(collection: &[T], mut callback: F) -> Vec<R>
where
    F: FnMut(&T, usize) -> (R, bool),
{
    let mut result = Vec::with_capacity(collection.len());

    for (index, item) in collection.iter().enumerate() {
        let (r, ok) = callback(item, index);
        if !ok {
            result.push(r);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reject_map_basic() {
        let numbers = vec![1, 2, 3, 4, 5];
        // Collect squares of odd numbers
        let result = reject_map(&numbers, |&x, _| (x * x, x % 2 == 0));
        assert_eq!(result, vec![1, 9, 25]);
    }

    #[test]
    fn test_reject_map_with_indices() {
        let data = vec!["a", "b", "c", "d", "e"];
        // Collect items at even indices (0-based indexing)
        let result = reject_map(&data, |&item, index| (item.to_uppercase(), index % 2 != 0));
        assert_eq!(
            result,
            vec!["A".to_uppercase(), "C".to_uppercase(), "E".to_uppercase()]
        );
    }

    #[test]
    fn test_reject_map_empty_collection() {
        let empty: Vec<i32> = vec![];
        let result = reject_map(&empty, |&x, _| (x * 2, x % 2 == 0));
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_reject_map_all_true() {
        let numbers = vec![2, 4, 6];
        // All callbacks return true, so result should be empty
        let result = reject_map(&numbers, |&x, _| (x / 2, x % 2 == 0));
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_reject_map_all_false() {
        let numbers = vec![1, 3, 5];
        // All callbacks return false, so all results are collected
        let result = reject_map(&numbers, |&x, _| (x * 10, false));
        assert_eq!(result, vec![10, 30, 50]);
    }

    #[test]
    fn test_reject_map_with_structs() {
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
                id: 3,
                value: "three".to_string(),
            },
            Item {
                id: 4,
                value: "four".to_string(),
            },
        ];

        // Collect values of items with even ids
        let result = reject_map(&items, |item, _| (item.value.clone(), item.id % 2 != 0));
        assert_eq!(result, vec!["two".to_string(), "four".to_string()]);
    }

    #[test]
    fn test_reject_map_with_mixed_conditions() {
        let numbers = vec![10, 15, 20, 25, 30];
        // Collect numbers divided by 5 where the number is not divisible by 10
        let result = reject_map(&numbers, |&x, _| (x / 5, x % 10 == 0));
        assert_eq!(result, vec![3, 5]); // [3, 5]
    }

    #[test]
    fn test_reject_map_preserves_order_all_collected() {
        let data = vec![true, false, true, false, true];
        // Collect all elements because the predicate is always false
        let result = reject_map(&data, |&x, _| (x, false));
        assert_eq!(result, vec![true, false, true, false, true]);
    }

    #[test]
    fn test_reject_map_preserves_order_none_collected() {
        let data = vec![true, false, true, false, true];
        // Collect no elements because the predicate is always true
        let result = reject_map(&data, |&x, _| (x, true));
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_reject_map_with_optionals() {
        let collection = vec![Some(1), None, Some(2), Some(3), None];
        // Collect values where the item is Some
        let result = reject_map(&collection, |item, _| {
            (item.clone().unwrap_or(0), item.is_none())
        });
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_reject_map_callback_side_effects() {
        let numbers = vec![1, 2, 3];
        let mut counter = 0;
        let result = reject_map(&numbers, |&x, _| {
            counter += x;
            (x * 2, x % 2 == 0)
        });
        assert_eq!(result, vec![2, 6]);
        assert_eq!(counter, 6); // 1 + 2 + 3
    }
}
