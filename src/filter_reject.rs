/// Filters a collection into two separate vectors based on a predicate function.
///
/// This function iterates over a slice of items, applies the provided predicate to each item along
/// with its index, and separates the items into two vectors:
/// - `kept`: Contains items for which the predicate returned `true`.
/// - `rejected`: Contains items for which the predicate returned `false`.
///
/// **Time Complexity:** O(n), where n is the number of elements in the collection.
///
/// # Arguments
///
/// * `collection` - A slice of items to be filtered.
/// * `predicate` - A function that takes a reference to an item and its index, returning a boolean.
///                If the predicate returns `true`, the item is kept; otherwise, it is rejected.
///
/// # Type Parameters
///
/// * `T` - The type of elements in the input collection. Must implement `Clone`.
/// * `F` - The type of the predicate function.
///
/// # Returns
///
/// * `(Vec<T>, Vec<T>)` - A tuple containing two vectors:
///     - The first vector (`kept`) contains all items for which the predicate returned `true`.
///     - The second vector (`rejected`) contains all items for which the predicate returned `false`.
///
/// # Examples
///
/// ```rust
/// use lowdash::filter_reject;
///
/// let numbers = vec![1, 2, 3, 4, 5];
/// // Separate even and odd numbers
/// let (evens, odds) = filter_reject(&numbers, |&x, _| x % 2 == 0);
/// assert_eq!(evens, vec![2, 4]);
/// assert_eq!(odds, vec![1, 3, 5]);
/// ```
///
/// ```rust
/// use lowdash::filter_reject;
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
///     Person { name: "Carol".to_string(), age: 35 },
/// ];
///
/// // Separate people who are at least 30 years old
/// let (adults, juniors) = filter_reject(&people, |person, _| person.age >= 30);
/// assert_eq!(adults, vec![
///     Person { name: "Bob".to_string(), age: 30 },
///     Person { name: "Carol".to_string(), age: 35 },
/// ]);
/// assert_eq!(juniors, vec![
///     Person { name: "Alice".to_string(), age: 25 },
/// ]);
/// ```
pub fn filter_reject<T, F>(collection: &[T], predicate: F) -> (Vec<T>, Vec<T>)
where
    T: Clone,
    F: Fn(&T, usize) -> bool,
{
    let mut kept = Vec::with_capacity(collection.len());
    let mut rejected = Vec::with_capacity(collection.len());

    for (index, item) in collection.iter().enumerate() {
        if predicate(item, index) {
            kept.push(item.clone());
        } else {
            rejected.push(item.clone());
        }
    }

    (kept, rejected)
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
    fn test_filter_reject_basic() {
        let numbers = vec![1, 2, 3, 4, 5];
        // Separate even and odd numbers
        let (evens, odds) = filter_reject(&numbers, |&x, _| x % 2 == 0);
        assert_eq!(evens, vec![2, 4]);
        assert_eq!(odds, vec![1, 3, 5]);
    }

    #[test]
    fn test_filter_reject_with_indices() {
        let data = vec!["a", "b", "c", "d", "e"];
        // Separate elements at even and odd indices
        let (even_indices, odd_indices) = filter_reject(&data, |_, index| index % 2 == 0);
        assert_eq!(even_indices, vec!["a", "c", "e"]);
        assert_eq!(odd_indices, vec!["b", "d"]);
    }

    #[test]
    fn test_filter_reject_empty_collection() {
        let empty: Vec<i32> = vec![];
        let (kept, rejected) = filter_reject(&empty, |&x, _| x > 0);
        assert_eq!(kept, Vec::<i32>::new());
        assert_eq!(rejected, Vec::<i32>::new());
    }

    #[test]
    fn test_filter_reject_all_kept() {
        let numbers = vec![1, 2, 3];
        // All predicates return true
        let (kept, rejected) = filter_reject(&numbers, |&x, _| x > 0);
        assert_eq!(kept, vec![1, 2, 3]);
        assert_eq!(rejected, Vec::<i32>::new());
    }

    #[test]
    fn test_filter_reject_all_rejected() {
        let numbers = vec![1, 2, 3];
        // All predicates return false
        let (kept, rejected) = filter_reject(&numbers, |&x, _| x < 0);
        assert_eq!(kept, Vec::<i32>::new());
        assert_eq!(rejected, vec![1, 2, 3]);
    }

    #[test]
    fn test_filter_reject_with_structs() {
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
                age: 28,
            },
        ];

        // Separate people who are at least 30 years old
        let (adults, juniors) = filter_reject(&people, |person, _| person.age >= 30);
        assert_eq!(
            adults,
            vec![
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
        assert_eq!(
            juniors,
            vec![
                Person {
                    name: "Alice".to_string(),
                    age: 25
                },
                Person {
                    name: "Dave".to_string(),
                    age: 28
                },
            ]
        );
    }

    #[test]
    fn test_filter_reject_with_various_conditions() {
        let mixed = vec![Some(1), None, Some(2), Some(3), None];
        // Keep Some(_) and reject None
        let (somes, nones) = filter_reject(&mixed, |item, _| item.is_some());
        assert_eq!(somes, vec![Some(1), Some(2), Some(3)]);
        assert_eq!(nones, vec![None, None]);
    }

    #[test]
    fn test_filter_reject_preserves_order() {
        let numbers = vec![5, 3, 8, 1, 4, 7, 2];
        // Keep numbers greater than 4
        let (kept, rejected) = filter_reject(&numbers, |&x, _| x > 4);
        assert_eq!(kept, vec![5, 8, 7]);
        assert_eq!(rejected, vec![3, 1, 4, 2]);
    }

    #[test]
    fn test_filter_reject_with_strings() {
        let words = vec!["apple", "banana", "avocado", "cherry", "apricot"];
        // Keep words that start with 'a'
        let (a_words, other_words) = filter_reject(&words, |word, _| word.starts_with('a'));
        assert_eq!(a_words, vec!["apple", "avocado", "apricot"]);
        assert_eq!(other_words, vec!["banana", "cherry"]);
    }

    #[test]
    fn test_filter_reject_with_floats() {
        let floats = vec![1.5, 2.3, 3.7, 4.0, 5.2];
        // Keep floats greater than 3.0
        let (high_floats, low_floats) = filter_reject(&floats, |&x, _| x > 3.0);
        assert_eq!(high_floats, vec![3.7, 4.0, 5.2]);
        assert_eq!(low_floats, vec![1.5, 2.3]);
    }
}
