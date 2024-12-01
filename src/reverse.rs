/// Reverse a collection, returning a new vector with the elements in reverse order.
///
/// This function takes a slice of items and returns a new `Vec<T>` containing all the elements
/// from the input collection in reverse order.
///
/// **Time Complexity:**  
/// O(n), where n is the number of elements in the collection.
///
/// # Arguments
///
/// * `collection` - A slice of items to be reversed.
///
/// # Type Parameters
///
/// * `T` - The type of elements in the collection. Must implement `Clone`.
///
/// # Returns
///
/// * `Vec<T>` - A new vector containing all elements from the input collection in reverse order.
///
/// # Examples
///
/// ```rust
/// use lowdash::reverse;
///
/// let numbers = vec![1, 2, 3, 4, 5];
/// let reversed = reverse(&numbers);
/// assert_eq!(reversed, vec![5, 4, 3, 2, 1]);
/// ```
///
/// ```rust
/// use lowdash::reverse;
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
/// let reversed_people = reverse(&people);
/// assert_eq!(reversed_people, vec![
///     Person { name: "Carol".to_string(), age: 35 },
///     Person { name: "Bob".to_string(), age: 30 },
///     Person { name: "Alice".to_string(), age: 25 },
/// ]);
/// ```
pub fn reverse<T>(collection: &[T]) -> Vec<T>
where
    T: Clone,
{
    collection.iter().rev().cloned().collect()
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
    fn test_reverse_integers() {
        let numbers = vec![1, 2, 3, 4, 5];
        let reversed = reverse(&numbers);
        assert_eq!(reversed, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_reverse_strings() {
        let strings = vec!["apple", "banana", "cherry", "date"];
        let reversed = reverse(&strings);
        assert_eq!(reversed, vec!["date", "cherry", "banana", "apple"]);
    }

    #[test]
    fn test_reverse_with_structs() {
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

        let reversed_people = reverse(&people);
        assert_eq!(
            reversed_people,
            vec![
                Person {
                    name: "Carol".to_string(),
                    age: 35
                },
                Person {
                    name: "Bob".to_string(),
                    age: 30
                },
                Person {
                    name: "Alice".to_string(),
                    age: 25
                },
            ]
        );
    }

    #[test]
    fn test_reverse_empty_collection() {
        let empty: Vec<i32> = vec![];
        let reversed = reverse(&empty);
        assert_eq!(reversed, Vec::<i32>::new());
    }

    #[test]
    fn test_reverse_single_element() {
        let single = vec![42];
        let reversed = reverse(&single);
        assert_eq!(reversed, single);
    }

    #[test]
    fn test_reverse_preserves_elements() {
        let elements = vec![10, 20, 30, 40, 50];
        let reversed = reverse(&elements);
        assert_eq!(reversed, vec![50, 40, 30, 20, 10]);
    }

    #[test]
    fn test_reverse_with_duplicates() {
        let numbers = vec![1, 2, 2, 3, 4, 3, 5];
        let reversed = reverse(&numbers);
        assert_eq!(reversed, vec![5, 3, 4, 3, 2, 2, 1]);
    }

    #[test]
    fn test_reverse_with_optionals() {
        let collection = vec![Some(1), None, Some(2), Some(3), None];
        let reversed = reverse(&collection);
        assert_eq!(reversed, vec![None, Some(3), Some(2), None, Some(1)]);
    }

    #[test]
    fn test_reverse_with_floats() {
        let float_collection = vec![1.1, 2.2, 3.3, 4.4, 5.5];
        let reversed = reverse(&float_collection);
        assert_eq!(reversed, vec![5.5, 4.4, 3.3, 2.2, 1.1]);
    }

    #[test]
    fn test_reverse_with_nan_floats() {
        let float_collection = vec![std::f64::NAN, 2.2, std::f64::NAN, 4.4];
        let reversed = reverse(&float_collection);
        assert_eq!(reversed.len(), float_collection.len());
        // Verify each element is reversed correctly
        for i in 0..float_collection.len() {
            if float_collection[i].is_nan() {
                assert!(reversed[float_collection.len() - 1 - i].is_nan());
            } else {
                assert_eq!(
                    reversed[float_collection.len() - 1 - i],
                    float_collection[i]
                );
            }
        }
    }
}
