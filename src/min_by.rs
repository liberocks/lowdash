/// Find the minimum element in a collection based on a custom comparison function.
/// If the collection is empty, returns `None`.
///
/// # Arguments
/// * `collection` - A slice of items.
/// * `comparison` - A function that takes two items and returns `true` if the first item is considered less than the second.
///
/// # Returns
/// * `Option<T>` - The minimum item in the collection based on the comparison function, or `None` if the collection is empty.
///
/// # Examples
/// ```rust
/// use lowdash::min_by;
///
/// let numbers = vec![5, 3, 8, 1, 4];
/// let min = min_by(&numbers, |a, b| a < b);
/// assert_eq!(min, Some(1));
///
/// let strings = vec!["apple", "banana", "cherry"];
/// let min = min_by(&strings, |a, b| a.len() < b.len());
/// assert_eq!(min, Some("apple"));
///
/// let empty: Vec<i32> = vec![];
/// let min = min_by(&empty, |a, b| a < b);
/// assert_eq!(min, None);
/// ```
pub fn min_by<T, F>(collection: &[T], comparison: F) -> Option<T>
where
    T: Clone,
    F: Fn(&T, &T) -> bool,
{
    if collection.is_empty() {
        return None;
    }

    let mut min = collection[0].clone();

    for item in &collection[1..] {
        if comparison(item, &min) {
            min = item.clone();
        }
    }

    Some(min)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_by_integers() {
        let numbers = vec![5, 3, 8, 1, 4];
        let min = min_by(&numbers, |a, b| a < b);
        assert_eq!(min, Some(1));
    }

    #[test]
    fn test_min_by_strings() {
        let strings = vec!["apple", "banana", "cherry"];
        let min = min_by(&strings, |a, b| a.len() < b.len());
        assert_eq!(min, Some("apple"));
    }

    #[test]
    fn test_min_by_empty_collection() {
        let empty: Vec<i32> = vec![];
        let min = min_by(&empty, |a, b| a < b);
        assert_eq!(min, None);
    }

    #[test]
    fn test_min_by_single_element() {
        let single = vec![42];
        let min = min_by(&single, |a, b| a < b);
        assert_eq!(min, Some(42));
    }

    #[test]
    fn test_min_by_custom_struct() {
        #[derive(Debug, Clone, PartialEq)]
        struct Person {
            name: String,
            age: u32,
        }

        let people = vec![
            Person {
                name: "Alice".to_string(),
                age: 30,
            },
            Person {
                name: "Bob".to_string(),
                age: 25,
            },
            Person {
                name: "Carol".to_string(),
                age: 35,
            },
        ];

        let min = min_by(&people, |a, b| a.age < b.age);
        assert_eq!(
            min,
            Some(Person {
                name: "Bob".to_string(),
                age: 25
            })
        );
    }

    #[test]
    fn test_min_by_floats() {
        let float_collection = vec![3.5, 2.2, 4.8, 1.1, 3.3];
        let min = min_by(&float_collection, |a, b| a < b);
        assert_eq!(min, Some(1.1));
    }

    #[test]
    fn test_min_by_custom_comparison() {
        let numbers = vec![10, 20, 30, 40];
        // Define a comparison that prefers larger numbers
        let min = min_by(&numbers, |a, b| a > b);
        assert_eq!(min, Some(40));
    }
}
