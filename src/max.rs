#![allow(clippy::eq_op)]
use crate::common;

/// Find the maximum element in a collection.
/// If the collection is empty, returns `None`.
///
/// # Arguments
/// * `collection` - A slice of items.
///
/// # Returns
/// * `Option<T>` - The maximum item in the collection, or `None` if the collection is empty.
///
/// # Examples
/// ```rust
/// use lowdash::max;
/// let numbers = vec![5, 3, 8, 1, 4];
/// let result = max(&numbers);
/// assert_eq!(result, Some(8));
/// ```
///
/// ```rust
/// use lowdash::max;
/// let strings = vec!["apple", "banana", "cherry"];
/// let result = max(&strings);
/// assert_eq!(result, Some("cherry"));
/// ```
///
/// ```rust
/// use lowdash::max;
///
/// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let people = vec![
///     Person { name: "Alice".to_string(), age: 25 },
///     Person { name: "Bob".to_string(), age: 30 },
///     Person { name: "Cia".to_string(), age: 20 },
/// ];
///
/// let result = max(&people);
/// assert_eq!(
///     result,
///     Some(Person { name: "Cia".to_string(), age: 20 })
/// );
/// ```
///
/// ```rust
/// use lowdash::max;
/// let collection = vec![3.14, 2.71, -1.0, 0.0];
/// let result = max(&collection);
/// assert_eq!(result, Some(3.14));
/// ```
pub fn max<T>(collection: &[T]) -> Option<T>
where
    T: PartialOrd + Clone + 'static,
{
    if collection.is_empty() {
        None
    } else if common::is_collection_float(
        &collection
            .iter()
            .map(|item| Box::new(item.clone()) as Box<dyn std::any::Any>)
            .collect::<Vec<_>>(),
    ) {
        let mut max = collection[0].clone();
        for item in &collection[1..] {
            if item > &max || max != max {
                // note: NaN != NaN is true because NaN is not equal to itself
                max = item.clone();
            }
        }
        Some(max)
    } else {
        let mut max = collection[0].clone();
        for item in &collection[1..] {
            if item > &max {
                max = item.clone();
            }
        }
        Some(max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_numbers() {
        let collection = vec![5, 3, 8, 1, 4];
        let result = max(&collection);
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_max_strings() {
        let collection = vec!["apple", "banana", "cherry"];
        let result = max(&collection);
        assert_eq!(result, Some("cherry"));
    }

    #[test]
    fn test_max_empty_collection() {
        let collection: Vec<i32> = vec![];
        let result = max(&collection);
        assert_eq!(result, None); // No maximum in an empty collection
    }

    #[test]
    fn test_max_single_element() {
        let collection = vec![42];
        let result = max(&collection);
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_max_with_struct() {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
        struct Person {
            age: u32,
            name: String,
        }

        let people = vec![
            Person {
                age: 25,
                name: "Alice".to_string(),
            },
            Person {
                age: 30,
                name: "Bob".to_string(),
            },
            Person {
                age: 20,
                name: "Cia".to_string(),
            },
        ];

        let result = max(&people);
        assert_eq!(
            result,
            Some(Person {
                age: 30,
                name: "Bob".to_string()
            })
        );
    }

    #[test]
    fn test_max_with_floating_points() {
        let collection = vec![3.14, 2.71, -1.0, 0.0];
        let result = max(&collection);
        assert_eq!(result, Some(3.14));
    }

    #[test]
    fn test_max_with_f32() {
        let collection = vec![3.14f32, 2.71, -1.0, 0.0];
        let result = max(&collection);
        assert_eq!(result, Some(3.14f32));
    }

    #[test]
    fn test_max_with_f64() {
        let collection = vec![3.14f64, 2.71, -1.0, 0.0];
        let result = max(&collection);
        assert_eq!(result, Some(3.14f64));
    }

    #[test]
    fn test_max_with_characters() {
        let collection = vec!['z', 'a', 'm', 'b'];
        let result = max(&collection);
        assert_eq!(result, Some('z'));
    }

    #[test]
    fn test_max_collection_with_nan() {
        let collection = vec![std::f64::NAN, 2.0, 3.0];
        let result = max(&collection);
        // Since NaN comparisons are always false, the maximum should be 3.0
        assert_eq!(result, Some(3.0));
    }

    #[test]
    fn test_max_all_nan() {
        let collection = vec![std::f64::NAN, std::f64::NAN];
        let result = max(&collection);
        assert!(result.unwrap().is_nan());
    }
}
