/// Fill a collection with a specified value, returning a new vector with all elements set to the initial value.
///
/// This function takes a slice of items and an initial value, then returns a new `Vec<T>` containing the initial value
/// repeated for the length of the input collection. The original collection remains unmodified.
///
/// **Time Complexity:**  
/// O(n), where n is the number of elements in the collection.
///
/// # Arguments
///
/// * `collection` - A slice of items indicating the size of the resulting vector.
/// * `initial` - The value to fill the new vector with.
///
/// # Type Parameters
///
/// * `T` - The type of elements in the collection. Must implement the `Clone` trait to allow duplication.
///
/// # Returns
///
/// * `Vec<T>` - A new vector containing the initial value repeated for the length of the input collection.
///
/// # Examples
///
/// ```rust
/// use lowdash::fill;
///
/// let numbers = vec![1, 2, 3, 4, 5];
/// let filled = fill(&numbers, 0);
/// assert_eq!(filled, vec![0, 0, 0, 0, 0]);
/// ```
///
/// ```rust
/// use lowdash::fill;
///
/// #[derive(Debug, PartialEq, Clone)]
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
/// let filled_people = fill(&people, Person { name: "Dave".to_string(), age: 40 });
/// assert_eq!(
///     filled_people,
///     vec![
///         Person { name: "Dave".to_string(), age: 40 },
///         Person { name: "Dave".to_string(), age: 40 },
///         Person { name: "Dave".to_string(), age: 40 },
///     ]
/// );
/// ```
pub fn fill<T>(collection: &[T], initial: T) -> Vec<T>
where
    T: Clone,
{
    let mut result = Vec::with_capacity(collection.len());

    for _ in collection {
        result.push(initial.clone());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    struct Person {
        name: String,
        age: u32,
    }

    #[test]
    fn test_fill_integers() {
        let numbers = vec![1, 2, 3, 4, 5];
        let filled = fill(&numbers, 0);
        assert_eq!(filled, vec![0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_fill_strings() {
        let strings = vec!["apple", "banana", "cherry"];
        let filled = fill(&strings, "fruit");
        assert_eq!(
            filled,
            vec![
                "fruit".to_string(),
                "fruit".to_string(),
                "fruit".to_string()
            ]
        );
    }

    #[test]
    fn test_fill_with_structs() {
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

        let filled_people = fill(
            &people,
            Person {
                name: "Dave".to_string(),
                age: 40,
            },
        );

        assert_eq!(
            filled_people,
            vec![
                Person {
                    name: "Dave".to_string(),
                    age: 40,
                },
                Person {
                    name: "Dave".to_string(),
                    age: 40,
                },
                Person {
                    name: "Dave".to_string(),
                    age: 40,
                },
            ]
        );
    }

    #[test]
    fn test_fill_empty_collection() {
        let empty: Vec<i32> = vec![];
        let filled = fill(&empty, 100);
        assert_eq!(filled, Vec::<i32>::new());
    }

    #[test]
    fn test_fill_single_element() {
        let single = vec![42];
        let filled = fill(&single, 99);
        assert_eq!(filled, vec![99]);
    }

    #[test]
    fn test_fill_preserves_length() {
        let original = vec![true, false, true, false];
        let filled = fill(&original, true);
        assert_eq!(filled.len(), original.len());
        assert_eq!(filled, vec![true, true, true, true]);
    }

    #[test]
    fn test_fill_with_duplicates() {
        let numbers = vec![1, 1, 2, 2, 3, 3];
        let filled = fill(&numbers, 0);
        assert_eq!(filled, vec![0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_fill_with_optionals() {
        let collection = vec![Some(1), None, Some(2), None];
        let filled = fill(&collection, None);
        assert_eq!(filled, vec![None, None, None, None]);
    }

    #[test]
    fn test_fill_with_floats() {
        let float_collection = vec![1.1, 2.2, 3.3];
        let filled = fill(&float_collection, 0.0);
        assert_eq!(filled, vec![0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_fill_with_nan_floats() {
        let float_collection = vec![std::f64::NAN, 2.2, std::f64::INFINITY];
        let filled = fill(&float_collection, std::f64::NAN);
        assert_eq!(filled.len(), float_collection.len());
        for &value in &filled {
            assert!(value.is_nan());
        }
    }
}
