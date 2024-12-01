/// Find the first item in a collection that satisfies a predicate.
/// If no item satisfies the predicate, return the fallback value.
///
/// # Arguments
/// * `collection` - A collection of items.
/// * `fallback` - Value to return if no item satisfies the predicate.
/// * `predicate` - A function that takes an item from the collection and returns a boolean.
///
/// # Returns
/// * `&T` - Either the first item that satisfies the predicate or the fallback value.
///
/// # Examples
/// ```rust
/// use lowdash::find_or_else;
/// let numbers = vec![1, 2, 3, 4, 5];
/// let predicate = |x: &i32| *x == 3;
/// let result = find_or_else(&numbers, &0, predicate);
/// assert_eq!(result, &3);
/// ```
///
/// ```rust
/// use lowdash::find_or_else;
/// let numbers = vec![10, 20, 30, 40];
/// let result = find_or_else(&numbers, &0, |x| *x > 50);
/// assert_eq!(result, &0);
/// ```
///
/// ```rust
/// use lowdash::find_or_else;
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
/// let fallback = Person { name: "Unknown".to_string(), age: 0 };
/// let result = find_or_else(&people, &fallback, |p| p.age > 30);
/// assert_eq!(result, &Person { name: "Carol".to_string(), age: 35 });
/// ```
pub fn find_or_else<'a, T, F>(collection: &'a [T], fallback: &'a T, predicate: F) -> &'a T
where
    F: Fn(&T) -> bool,
{
    for item in collection {
        if predicate(item) {
            return item;
        }
    }
    fallback
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_or_else() {
        let collection = vec![1, 2, 3, 4, 5];
        let predicate = |x: &i32| *x == 3;
        let result = find_or_else(&collection, &0, predicate);
        assert_eq!(result, &3);
    }

    #[test]
    fn test_find_or_else_with_inline_predicate() {
        let numbers = vec![10, 20, 30, 40];
        let result = find_or_else(&numbers, &0, |x| *x > 25);
        assert_eq!(result, &30);
    }

    #[test]
    fn test_find_or_else_not_found() {
        let collection = vec![1, 2, 3, 4, 5];
        let predicate = |x: &i32| *x == 6;
        let result = find_or_else(&collection, &0, predicate);
        assert_eq!(result, &0);
    }

    #[test]
    fn test_find_or_else_empty_collection() {
        let collection: Vec<i32> = vec![];
        let predicate = |x: &i32| *x == 3;
        let result = find_or_else(&collection, &0, predicate);
        assert_eq!(result, &0);
    }

    #[test]
    fn test_find_or_else_with_struct() {
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

        let fallback = Person {
            name: "Unknown".to_string(),
            age: 0,
        };

        let result = find_or_else(&people, &fallback, |p| p.age > 30);
        assert_eq!(
            result,
            &Person {
                name: "Carol".to_string(),
                age: 35
            }
        );

        let name_result = find_or_else(&people, &fallback, |p| p.name.starts_with("B"));
        assert_eq!(
            name_result,
            &Person {
                name: "Bob".to_string(),
                age: 30
            }
        );

        let not_found = find_or_else(&people, &fallback, |p| p.age > 40);
        assert_eq!(
            not_found,
            &Person {
                name: "Unknown".to_string(),
                age: 0
            }
        );
    }

    #[test]
    fn test_find_or_else_with_floats() {
        let floats = vec![1.1, 2.2, 3.3, 4.4, 5.5];
        let fallback = 0.0;
        let result = find_or_else(&floats, &fallback, |x| *x > 3.0);
        assert_eq!(result, &3.3);

        let not_found = find_or_else(&floats, &fallback, |x| *x > 6.0);
        assert_eq!(not_found, &0.0);
    }
}
