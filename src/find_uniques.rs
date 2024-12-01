/// Find all unique elements in a collection (elements that appear exactly once).
///
/// # Arguments
/// * `collection` - A slice of items.
///
/// # Returns
/// * `Vec<T>` - A vector containing only the elements that appear exactly once in the input.
///
/// # Examples
/// ```rust
/// use lowdash::find_uniques;
/// let numbers = vec![1, 2, 2, 3, 3, 4];
/// let result = find_uniques(&numbers);
/// assert_eq!(result, vec![1, 4]);
/// ```
///
/// ```rust
/// use lowdash::find_uniques;
/// let words = vec!["apple", "banana", "apple", "cherry"];
/// let result = find_uniques(&words);
/// assert_eq!(result, vec!["banana", "cherry"]);
/// ```
///
/// ```rust
/// use lowdash::find_uniques;
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
/// let result = find_uniques(&people);
/// assert_eq!(result, vec![Person { name: "Bob".to_string(), age: 30 }]);
/// ```
pub fn find_uniques<T>(collection: &[T]) -> Vec<T>
where
    T: Clone + Eq + std::hash::Hash,
{
    use std::collections::HashMap;

    let mut counts = HashMap::new();

    // Count occurrences
    for item in collection {
        *counts.entry(item).or_insert(0) += 1;
    }

    // Collect items that appear exactly once
    collection
        .iter()
        .filter(|item| counts.get(item) == Some(&1))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_uniques_numbers() {
        let collection = vec![1, 2, 2, 3, 3, 4];
        let result = find_uniques(&collection);
        assert_eq!(result, vec![1, 4]);
    }

    #[test]
    fn test_find_uniques_strings() {
        let collection = vec!["apple", "banana", "apple", "cherry"];
        let result = find_uniques(&collection);
        assert_eq!(result, vec!["banana", "cherry"]);
    }

    #[test]
    fn test_find_uniques_empty() {
        let collection: Vec<i32> = vec![];
        let result = find_uniques(&collection);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_find_uniques_all_duplicates() {
        let collection = vec![1, 1, 2, 2, 3, 3];
        let result = find_uniques(&collection);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_find_uniques_all_unique() {
        let collection = vec![1, 2, 3, 4, 5];
        let result = find_uniques(&collection);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_find_uniques_with_struct() {
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
        ];

        let result = find_uniques(&people);
        assert_eq!(
            result,
            vec![Person {
                name: "Bob".to_string(),
                age: 30,
            }]
        );
    }
}
