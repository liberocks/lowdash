/// Filter items from a collection that satisfy a predicate.
///
/// This function iterates over a collection and returns a new vector containing
/// all items for which the predicate returns `true`.
///
/// # Arguments
/// * `collection` - A slice of items.
/// * `predicate` - A function that takes an item and its index, returning a boolean.
///
/// # Returns
/// * `Vec<&T>` - A vector of references to items that satisfy the predicate.
///
/// # Examples
/// ```rust
/// use lowdash::filter;
/// let numbers = vec![1, 2, 3, 4, 5];
/// let result = filter(&numbers, |x, _| *x % 2 == 0);
/// assert_eq!(result, vec![&2, &4]);
/// ```
///
/// ```rust
/// use lowdash::filter;
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
/// let result = filter(&people, |p, _| p.age > 30);
/// assert_eq!(result, vec![&people[2]]);
/// ```
pub fn filter<'a, T, F>(collection: &'a [T], predicate: F) -> Vec<&'a T>
where
    F: Fn(&'a T, usize) -> bool,
{
    collection
        .iter()
        .enumerate()
        .filter(|(index, item)| predicate(item, *index))
        .map(|(_, item)| item)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_even_numbers() {
        let collection = vec![1, 2, 3, 4, 5];
        let predicate = |x: &i32, _| *x % 2 == 0;
        let result = filter(&collection, predicate);
        assert_eq!(result, vec![&2, &4]);
    }

    #[test]
    fn test_filter_with_index() {
        let collection = vec!["a", "b", "c", "d", "e"];
        // Filter items at even indices
        let predicate = |_, index| index % 2 == 0;
        let result = filter(&collection, predicate);
        assert_eq!(result, vec![&"a", &"c", &"e"]);
    }

    #[test]
    fn test_filter_all() {
        let collection = vec![10, 20, 30];
        let predicate = |_, _| true;
        let result = filter(&collection, predicate);
        assert_eq!(result, vec![&10, &20, &30]);
    }

    #[test]
    fn test_filter_none() {
        let collection = vec![10, 20, 30];
        let predicate = |_, _| false;
        let result = filter(&collection, predicate);
        let expected: Vec<&i32> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_filter_empty_collection() {
        let collection: Vec<i32> = vec![];
        let predicate = |_, _| true;
        let result = filter(&collection, predicate);
        let expected: Vec<&i32> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_filter_with_structs() {
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

        let result = filter(&people, |p, _| p.age > 30);
        assert_eq!(result, vec![&people[2]]);
    }

    #[test]
    fn test_filter_with_floats() {
        let float_collection = vec![1.1, 2.2, 3.3, 4.4];
        let predicate = |x: &f64, _| *x > 2.0;
        let result = filter(&float_collection, predicate);
        // Include 2.2, 3.3, and 4.4 in the expected result
        assert_eq!(
            result,
            vec![
                &float_collection[1],
                &float_collection[2],
                &float_collection[3]
            ]
        );
    }

    #[test]
    fn test_filter_with_negative_floats() {
        let float_collection = vec![-3.5, -2.2, -1.1, 0.0, 1.1];
        let predicate = |x: &f64, _| *x < 0.0;
        let result = filter(&float_collection, predicate);
        assert_eq!(result, vec![&-3.5, &-2.2, &-1.1]);
    }

    #[test]
    fn test_filter_with_nan_floats() {
        let float_collection = vec![std::f64::NAN, 2.2, std::f64::NAN, 4.4];
        let predicate = |x: &f64, _| x.is_nan();
        let result = filter(&float_collection, predicate);
        assert_eq!(result.len(), 2);
        assert!(result[0].is_nan());
        assert!(result[1].is_nan());
    }

    #[test]
    fn test_filter_with_multiple_matching_floats() {
        let float_collection = vec![1.1, 2.2, 2.2, 3.3, 4.4];
        let predicate = |x: &f64, _| *x == 2.2;
        let result = filter(&float_collection, predicate);
        assert_eq!(result, vec![&float_collection[1], &float_collection[2]]);
    }
}
