/// Find the last item in a collection that satisfies a predicate and return its index.
/// If no item satisfies the predicate, return None.
///
/// # Arguments
/// * `collection` - A collection of items.
/// * `predicate` - A function that takes an item from the collection and returns a boolean.
///
/// # Returns
/// * `Option<(&T, usize)>` - A tuple containing the last item in the collection that satisfies the predicate and its index, or None if no item satisfies the predicate.
///
/// # Examples
/// ```
/// use lowdash::find_last_index_of;
/// let numbers = vec![1, 2, 3, 4, 5, 3];
/// let predicate = |x: &i32| *x == 3;
/// let result = find_last_index_of(&numbers, predicate);
/// assert_eq!(result, Some((&3, 5)));
/// ```
///
/// ```
/// use lowdash::find_last_index_of;
/// let numbers = vec![10, 20, 30, 40, 30];
/// let result = find_last_index_of(&numbers, |x| *x > 25);
/// assert_eq!(result, Some((&30, 4)));
/// ```
///
/// ```
/// use lowdash::find_last_index_of;
///
/// #[derive(Debug, PartialEq)]
/// struct Person {
///    name: String,
///    age: u32,
/// }
///
/// let people = vec![
///    Person { name: "Alice".to_string(), age: 25 },
///    Person { name: "Bob".to_string(), age: 30 },
///    Person { name: "Carol".to_string(), age: 35 },
///    Person { name: "Dave".to_string(), age: 35 },
/// ];
///
/// let result = find_last_index_of(&people, |p| p.age > 30);
/// assert_eq!(result, Some((&Person { name: "Dave".to_string(), age: 35 }, 3)));
/// ```
pub fn find_last_index_of<T, F>(collection: &[T], predicate: F) -> Option<(&T, usize)>
where
    F: Fn(&T) -> bool,
{
    for (i, item) in collection.iter().enumerate().rev() {
        if predicate(item) {
            return Some((item, i));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_last_index_of() {
        let collection = vec![1, 2, 3, 4, 5, 3];
        let predicate = |x: &i32| *x == 3;
        let result = find_last_index_of(&collection, predicate);
        assert_eq!(result, Some((&3, 5)));
    }

    #[test]
    fn test_find_last_index_of_with_inline_predicate() {
        let numbers = vec![10, 20, 30, 40, 30];
        let result = find_last_index_of(&numbers, |x| *x > 25);
        assert_eq!(result, Some((&30, 4)));
    }

    #[test]
    fn test_find_last_index_of_not_found() {
        let collection = vec![1, 2, 3, 4, 5];
        let predicate = |x: &i32| *x == 6;
        let result = find_last_index_of(&collection, predicate);
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_last_index_of_not_found_with_inline_predicate() {
        let numbers = vec![10, 20, 30, 40];
        let result = find_last_index_of(&numbers, |x| *x > 50);
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_last_index_of_empty_collection() {
        let collection: Vec<i32> = vec![];
        let predicate = |x: &i32| *x == 3;
        let result = find_last_index_of(&collection, predicate);
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_last_index_of_with_struct() {
        #[derive(Debug, PartialEq)]
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
                name: "Carol".to_string(),
                age: 35,
            },
            Person {
                name: "Dave".to_string(),
                age: 35,
            },
        ];

        let result = find_last_index_of(&people, |p| p.age > 30);
        assert_eq!(
            result,
            Some((
                &Person {
                    name: "Dave".to_string(),
                    age: 35
                },
                3
            ))
        );

        let name_result = find_last_index_of(&people, |p| p.name.starts_with("B"));
        assert_eq!(
            name_result,
            Some((
                &Person {
                    name: "Bob".to_string(),
                    age: 30
                },
                1
            ))
        );

        let not_found = find_last_index_of(&people, |p| p.age > 40);
        assert_eq!(not_found, None);
    }
}
