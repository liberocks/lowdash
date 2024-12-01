/// Find the first item in a collection that satisfies a predicate.
/// If no item satisfies the predicate, return None.
///
/// # Arguments
/// * `collection` - A collection of items.
/// * `predicate` - A function that takes an item from the collection and returns a boolean.
///
/// # Returns
/// * `Some(&T)` - The first item in the collection that satisfies the predicate.
/// * `None` - If no item satisfies the predicate.
///
/// # Examples
/// ```rust
/// use lowdash::find;
/// let numbers = vec![1, 2, 3, 4, 5];
/// let predicate = |x: &i32| *x == 3;
/// let result = find(&numbers, predicate);
/// assert_eq!(result, Some(&3));
/// ```
///
/// ```rust
/// use lowdash::find;
/// let numbers = vec![10, 20, 30, 40];
/// let result = find(&numbers, |x| *x > 25);
/// assert_eq!(result, Some(&30));
/// ```
///
/// ```rust
/// use lowdash::find;
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
/// ];
///
/// let result = find(&people, |p| p.age > 30);
/// assert_eq!(result, Some(&Person { name: "Carol".to_string(), age: 35 }));
/// ```
pub fn find<T, F>(collection: &[T], predicate: F) -> Option<&T>
where
    F: Fn(&T) -> bool,
{
    collection.iter().find(|&item| predicate(item))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        let collection = vec![1, 2, 3, 4, 5];
        let predicate = |x: &i32| *x == 3;
        let result = find(&collection, predicate);
        assert_eq!(result, Some(&3));
    }

    #[test]
    fn test_find_with_inline_predicate() {
        let numbers = vec![10, 20, 30, 40];
        let result = find(&numbers, |x| *x > 25);
        assert_eq!(result, Some(&30));
    }

    #[test]
    fn test_find_not_found() {
        let collection = vec![1, 2, 3, 4, 5];
        let predicate = |x: &i32| *x == 6;
        let result = find(&collection, predicate);
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_not_found_with_inline_predicate() {
        let numbers = vec![10, 20, 30, 40];
        let result = find(&numbers, |x| *x > 50);
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_empty_collection() {
        let collection: Vec<i32> = vec![];
        let predicate = |x: &i32| *x == 3;
        let result = find(&collection, predicate);
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_with_struct() {
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
        ];

        let result = find(&people, |p| p.age > 30);
        assert_eq!(
            result,
            Some(&Person {
                name: "Carol".to_string(),
                age: 35
            })
        );

        let name_result = find(&people, |p| p.name.starts_with("B"));
        assert_eq!(
            name_result,
            Some(&Person {
                name: "Bob".to_string(),
                age: 30
            })
        );

        let not_found = find(&people, |p| p.age > 40);
        assert_eq!(not_found, None);
    }

    #[test]
    fn test_find_with_floats() {
        let float_collection = vec![1.1, 2.2, 3.3, 4.4];
        let predicate = |x: &f64| *x > 2.0;
        let result = find(&float_collection, predicate);
        assert_eq!(result, Some(&2.2));

        let not_found = find(&float_collection, |x| *x > 5.0);
        assert_eq!(not_found, None);
    }

    #[test]
    fn test_find_with_negative_floats() {
        let float_collection = vec![-3.5, -2.2, -1.1, 0.0, 1.1];
        let predicate = |x: &f64| *x < 0.0;
        let result = find(&float_collection, predicate);
        assert_eq!(result, Some(&-3.5));
    }

    #[test]
    fn test_find_with_nan_floats() {
        let float_collection = vec![std::f64::NAN, 2.2, std::f64::NAN, 4.4];
        let predicate = |x: &f64| x.is_nan();
        let result = find(&float_collection, predicate);
        assert!(result.unwrap().is_nan());
    }

    #[test]
    fn test_find_with_multiple_matching_floats() {
        let float_collection = vec![1.1, 2.2, 2.2, 3.3, 4.4];
        let predicate = |x: &f64| *x == 2.2;
        let result = find(&float_collection, predicate);
        assert_eq!(result, Some(&2.2));
    }
}
