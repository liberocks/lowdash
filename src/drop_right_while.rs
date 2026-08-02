/// Drops the matching suffix while `predicate` returns `true`.
///
/// **Time Complexity:** O(n), where `n` is the number of checked items.
///
/// # Arguments
///
/// * `collection` - Items to scan.
/// * `predicate` - Function deciding whether to drop an item.
///
/// # Type Parameters
///
/// * `T` - Item type.
/// * `F` - Predicate type.
///
/// # Returns
///
/// * `Vec<T>` - The remaining prefix.
///
/// # Examples
///
/// ```rust
/// use lowdash::drop_right_while;
///
/// let numbers = vec![1, 2, 3, 4, 5];
/// let result = drop_right_while(&numbers, |&x| x > 3);
/// assert_eq!(result, vec![1, 2, 3]);
/// ```
///
/// ```rust
/// use lowdash::drop_right_while;
///
/// let letters = vec!['a', 'b', 'c', 'd', 'e'];
/// let result = drop_right_while(&letters, |&c| c != 'c');
/// assert_eq!(result, vec!['a', 'b', 'c']);
/// ```
pub fn drop_right_while<T, F>(collection: &[T], predicate: F) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    let mut i = collection.len();
    while i > 0 && predicate(&collection[i - 1]) {
        i -= 1;
    }
    collection[..i].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_right_while_basic() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = drop_right_while(&numbers, |&x| x > 3);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_drop_right_while_all_true() {
        let numbers = vec![5, 4, 3, 2, 1];
        let result = drop_right_while(&numbers, |_| true);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_drop_right_while_all_false() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = drop_right_while(&numbers, |_| false);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_drop_right_while_empty_collection() {
        let empty: Vec<i32> = vec![];
        let result = drop_right_while(&empty, |_| true);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_drop_right_while_with_structs() {
        #[derive(Clone, Debug, PartialEq)]
        struct Person {
            name: String,
            age: u32,
        }

        let people = vec![
            Person {
                name: "Alice".to_string(),
                age: 20,
            },
            Person {
                name: "Bob".to_string(),
                age: 25,
            },
            Person {
                name: "Charlie".to_string(),
                age: 30,
            },
        ];

        let result = drop_right_while(&people, |person| person.age > 20);
        let expected = vec![Person {
            name: "Alice".to_string(),
            age: 20,
        }];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_drop_right_while_no_elements_dropped() {
        let numbers = vec![1, 2, 3];
        let result = drop_right_while(&numbers, |&x| x > 5);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_drop_right_while_all_elements_dropped() {
        let numbers = vec![1, 2, 3];
        let result = drop_right_while(&numbers, |&x| x > 0);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_drop_right_while_with_chars() {
        let chars = vec!['a', 'b', 'c', 'd', 'e'];
        let result = drop_right_while(&chars, |&c| c != 'c');
        assert_eq!(result, vec!['a', 'b', 'c']);
    }
}
