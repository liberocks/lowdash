/// Find all duplicate elements in a collection (elements that appear more than once).
/// Each duplicate element appears exactly once in the result.
///
/// # Arguments
/// * `collection` - A slice of items.
///
/// # Returns
/// * `Vec<T>` - A vector containing one instance of each duplicate element.
///
/// # Examples
/// ```rust
/// use lowdash::find_duplicates;
/// let numbers = vec![1, 2, 2, 3, 3, 4];
/// let result = find_duplicates(&numbers);
/// assert_eq!(result, vec![2, 3]);
/// ```
///
/// ```rust
/// use lowdash::find_duplicates;
/// let words = vec!["apple", "banana", "apple", "cherry", "banana"];
/// let result = find_duplicates(&words);
/// assert_eq!(result, vec!["apple", "banana"]);
/// ```
///
/// ```rust
/// use lowdash::find_duplicates;
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
/// let result = find_duplicates(&people);
/// assert_eq!(result, vec![Person { name: "Alice".to_string(), age: 25 }]);
/// ```
pub fn find_duplicates<T>(collection: &[T]) -> Vec<T>
where
    T: Clone + Eq + std::hash::Hash,
{
    use std::collections::HashMap;

    let mut seen: HashMap<&T, bool> = HashMap::new();
    let mut result = Vec::new();

    // Track items and their duplicate status
    for item in collection {
        match seen.get(item) {
            Some(&already_added) => {
                if !already_added {
                    result.push(item.clone());
                    seen.insert(item, true);
                }
            }
            None => {
                seen.insert(item, false);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicates_numbers() {
        let collection = vec![1, 2, 2, 3, 3, 4];
        let result = find_duplicates(&collection);
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn test_find_duplicates_strings() {
        let collection = vec!["apple", "banana", "apple", "cherry", "banana"];
        let result = find_duplicates(&collection);
        assert_eq!(result, vec!["apple", "banana"]);
    }

    #[test]
    fn test_find_duplicates_empty() {
        let collection: Vec<i32> = vec![];
        let result = find_duplicates(&collection);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_find_duplicates_no_duplicates() {
        let collection = vec![1, 2, 3, 4, 5];
        let result = find_duplicates(&collection);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_find_duplicates_all_duplicates() {
        let collection = vec![1, 1, 2, 2, 3, 3];
        let result = find_duplicates(&collection);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_find_duplicates_multiple_occurrences() {
        let collection = vec![1, 1, 1, 2, 2, 2];
        let result = find_duplicates(&collection);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_find_duplicates_with_struct() {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
                name: "Alice".to_string(),
                age: 25,
            },
            Person {
                name: "Bob".to_string(),
                age: 30,
            },
        ];

        let result = find_duplicates(&people);
        assert_eq!(
            result,
            vec![
                Person {
                    name: "Alice".to_string(),
                    age: 25,
                },
                Person {
                    name: "Bob".to_string(),
                    age: 30,
                },
            ]
        );
    }
}
