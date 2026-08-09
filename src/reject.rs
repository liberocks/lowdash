/// Returns references to items for which `predicate(item, index)` is `false`.
///
/// # Arguments
/// * `collection` - Items to filter.
/// * `predicate` - Function called as `(item, index)`.
///
/// # Returns
/// * `Vec<&T>` - References to rejected items, in input order.
///
/// # Examples
/// ```rust
/// use lowdash::reject;
/// let numbers = vec![1, 2, 3, 4, 5];
/// let result = reject(&numbers, |x, _| *x % 2 == 0);
/// assert_eq!(result, vec![&1, &3, &5]);
/// ```
pub fn reject<'a, T, F>(collection: &'a [T], predicate: F) -> Vec<&'a T>
where
    F: Fn(&'a T, usize) -> bool,
{
    let mut result = Vec::with_capacity(collection.len());
    for (index, item) in collection.iter().enumerate() {
        if !predicate(item, index) {
            result.push(item);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reject_even_numbers() {
        let collection = vec![1, 2, 3, 4, 5];
        let predicate = |x: &i32, _: usize| -> bool { *x % 2 == 0 };
        let result = reject(&collection, predicate);
        assert_eq!(result, vec![&1, &3, &5]);
    }

    #[test]
    fn test_reject_with_index() {
        let collection = vec!["a", "b", "c", "d"];
        // Reject items at even indices
        let predicate = |_: &&str, index: usize| index % 2 == 0;
        let result = reject(&collection, predicate);
        assert_eq!(result, vec![&"b", &"d"]);
    }

    #[test]
    fn test_reject_all() {
        let collection = vec![10, 20, 30];
        let predicate = |_, _| true;
        let result = reject(&collection, predicate);
        let expected: Vec<&i32> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_reject_none() {
        let collection = vec![10, 20, 30];
        let predicate = |_, _| false;
        let result = reject(&collection, predicate);
        assert_eq!(result, vec![&10, &20, &30]);
    }

    #[test]
    fn test_reject_empty_collection() {
        let collection: Vec<i32> = vec![];
        let predicate = |_, _| true;
        let result = reject(&collection, predicate);
        let expected: Vec<&i32> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_reject_with_structs() {
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

        let result = reject(&people, |p, _| p.age > 30);
        assert_eq!(result, vec![&people[0], &people[1]]);
    }
}
