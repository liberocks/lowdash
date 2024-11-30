#![allow(clippy::eq_op)]

use crate::common;

/// Find the minimum element in a collection.
/// If the collection is empty, returns `None`.
///
/// # Arguments
/// * `collection` - A slice of items.
///
/// # Returns
/// * `Option<T>` - The minimum item in the collection, or `None` if the collection is empty.
///
/// # Examples
/// ```
/// use lowdash::min;
/// let numbers = vec![5, 3, 8, 1, 4];
/// let result = min(&numbers);
/// assert_eq!(result, Some(1));
/// ```
///
/// ```
/// use lowdash::min;
/// let strings = vec!["apple", "banana", "cherry"];
/// let result = min(&strings);
/// assert_eq!(result, Some("apple"));
/// ```
///
/// ```
/// use lowdash::min;
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
///     Person { name: "Carol".to_string(), age: 20 },
/// ];
///
/// let result = min(&people);
/// assert_eq!(
///     result,
///     Some(Person { name: "Alice".to_string(), age: 25 })
/// );
/// ```
///
/// ```
/// use lowdash::min;
/// let collection = vec![3.14, 2.71, -1.0, 0.0];
/// let result = min(&collection);
/// assert_eq!(result, Some(-1.0));
/// ```
pub fn min<T>(collection: &[T]) -> Option<T>
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
        let mut min = collection[0].clone();
        for item in &collection[1..] {
            if item < &min || min != min {
                // note: NaN != NaN is true because NaN is not equal to itself
                min = item.clone();
            }
        }
        return Some(min);
    } else {
        let mut min = collection[0].clone();
        for item in &collection[1..] {
            if item < &min {
                min = item.clone();
            }
        }
        Some(min)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_numbers() {
        let collection = vec![5, 3, 8, 1, 4];
        let result = min(&collection);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_min_strings() {
        let collection = vec!["apple", "banana", "cherry"];
        let result = min(&collection);
        assert_eq!(result, Some("apple"));
    }

    #[test]
    fn test_min_empty_collection() {
        let collection: Vec<i32> = vec![];
        let result = min(&collection);
        assert_eq!(result, None); // No minimum in an empty collection
    }

    #[test]
    fn test_min_single_element() {
        let collection = vec![42];
        let result = min(&collection);
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_min_with_struct() {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
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
                age: 20,
            },
        ];

        let result = min(&people);
        assert_eq!(
            result,
            Some(Person {
                name: "Alice".to_string(),
                age: 25
            })
        );
    }

    #[test]
    fn test_min_with_floating_points() {
        let collection = vec![3.14, 2.71, -1.0, 0.0];
        let result = min(&collection);
        assert_eq!(result, Some(-1.0));
    }

    #[test]
    fn test_min_with_f32() {
        let collection = vec![3.14f32, 2.71, -1.0, 0.0];
        let result = min(&collection);
        assert_eq!(result, Some(-1.0f32));
    }

    #[test]
    fn test_min_with_f64() {
        let collection = vec![3.14f64, 2.71, -1.0, 0.0];
        let result = min(&collection);
        assert_eq!(result, Some(-1.0f64));
    }

    #[test]
    fn test_min_with_characters() {
        let collection = vec!['z', 'a', 'm', 'b'];
        let result = min(&collection);
        assert_eq!(result, Some('a'));
    }

    #[test]
    fn test_min_collection_with_nan() {
        let collection = vec![std::f64::NAN, 2.0, 3.0];
        let result = min(&collection);
        // Since NaN comparisons are always false, the minimum should be 2.0
        assert_eq!(result, Some(2.0));
    }

    #[test]
    fn test_min_all_nan() {
        let collection = vec![std::f64::NAN, std::f64::NAN];
        let result = min(&collection);
        assert!(result.unwrap().is_nan());
    }
}
