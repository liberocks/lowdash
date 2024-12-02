/// Removes elements from the beginning of a collection as long as a predicate returns `true`,
/// and returns the remaining elements. As soon as the predicate returns `false`, the function stops dropping elements.
///
/// **Time Complexity:** O(n), where n is the number of elements until the predicate returns `false`.
///
/// # Arguments
///
/// * `collection` - A slice of items from which elements will be dropped.
/// * `predicate` - A function that takes an item and returns `true` or `false`.
///
/// # Type Parameters
///
/// * `T` - The type of elements in the collection. Must implement `Clone`.
/// * `F` - The type of the predicate function. Must implement `Fn(&T) -> bool`.
///
/// # Returns
///
/// * `Vec<T>` - A vector containing the elements after dropping from the beginning while the predicate holds true.
///
/// # Examples
///
/// ```rust
/// use lowdash::drop_while;
///
/// let numbers = vec![1, 2, 3, 4, 5];
/// let result = drop_while(&numbers, |&x| x < 3);
/// assert_eq!(result, vec![3, 4, 5]);
/// ```
///
/// ```rust
/// use lowdash::drop_while;
///
/// let letters = vec!['a', 'b', 'c', 'd'];
/// let result = drop_while(&letters, |&c| c < 'c');
/// assert_eq!(result, vec!['c', 'd']);
/// ```
pub fn drop_while<T, F>(collection: &[T], predicate: F) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    let mut i = 0;
    while i < collection.len() && predicate(&collection[i]) {
        i += 1;
    }
    collection[i..].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_while_basic() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = drop_while(&numbers, |&x| x < 3);
        assert_eq!(result, vec![3, 4, 5]);
    }

    #[test]
    fn test_drop_while_all_true() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = drop_while(&numbers, |_| true);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_drop_while_all_false() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = drop_while(&numbers, |_| false);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_drop_while_empty_collection() {
        let empty: Vec<i32> = vec![];
        let result = drop_while(&empty, |_| true);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_drop_while_with_structs() {
        #[derive(Clone, Debug, PartialEq)]
        struct Person {
            name: String,
            age: u32,
        }

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
                name: "Charlie".to_string(),
                age: 35,
            },
        ];

        let result = drop_while(&people, |person| person.age < 30);
        let expected = vec![
            Person {
                name: "Bob".to_string(),
                age: 30,
            },
            Person {
                name: "Charlie".to_string(),
                age: 35,
            },
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_drop_while_with_strings() {
        let words = vec!["apple", "banana", "cherry", "date"];
        let result = drop_while(&words, |word| word.len() < 6);
        assert_eq!(result, vec!["banana", "cherry", "date"]);
    }

    #[test]
    fn test_drop_while_no_elements_dropped() {
        let numbers = vec![1, 2, 3];
        let result = drop_while(&numbers, |&x| x > 3);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_drop_while_all_elements_dropped() {
        let numbers = vec![1, 2, 3];
        let result = drop_while(&numbers, |&x| x < 5);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_drop_while_with_chars() {
        let chars = vec!['a', 'b', 'c', 'd', 'e'];
        let result = drop_while(&chars, |&c| c < 'c');
        assert_eq!(result, vec!['c', 'd', 'e']);
    }
}
