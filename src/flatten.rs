/// Flatten a collection of slices into a single vector, preserving the order of elements.
///
/// This function takes a slice of slices and concatenates all inner slices into a single `Vec<T>`.
/// The order of elements is preserved based on their original ordering in the input collection.
///
/// **Time Complexity:**  
/// O(n), where n is the total number of elements across all inner slices.
///
/// # Arguments
///
/// * `collection` - A slice of slices to be flattened.
///
/// # Type Parameters
///
/// * `T` - The type of elements in the slices. Must implement `Clone`.
/// * `Slice` - The type of the inner slices. Must implement `AsRef<[T]>`.
///
/// # Returns
///
/// * `Vec<T>` - A vector containing all elements from the input slices, flattened into a single collection.
///
/// # Examples
///
/// ```rust
/// use lowdash::flatten;
///
/// let nested = vec![vec![1, 2], vec![3, 4], vec![5]];
/// let flat = flatten(&nested);
/// assert_eq!(flat, vec![1, 2, 3, 4, 5]);
/// ```
///
/// ```rust
/// use lowdash::flatten;
///
/// #[derive(Debug, PartialEq, Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let people_groups = vec![
///     vec![
///         Person { name: "Alice".to_string(), age: 25 },
///         Person { name: "Bob".to_string(), age: 30 },
///     ],
///     vec![
///         Person { name: "Carol".to_string(), age: 35 },
///         Person { name: "Dave".to_string(), age: 40 },
///     ],
/// ];
///
/// let flat_people = flatten(&people_groups);
/// assert_eq!(
///     flat_people,
///     vec![
///         Person { name: "Alice".to_string(), age: 25 },
///         Person { name: "Bob".to_string(), age: 30 },
///         Person { name: "Carol".to_string(), age: 35 },
///         Person { name: "Dave".to_string(), age: 40 },
///     ]
/// );
/// ```
pub fn flatten<T, Slice>(collection: &[Slice]) -> Vec<T>
where
    Slice: AsRef<[T]>,
    T: Clone,
{
    let total_len = collection.iter().map(|slice| slice.as_ref().len()).sum();
    let mut result = Vec::with_capacity(total_len);

    for slice in collection {
        result.extend(slice.as_ref().iter().cloned());
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
    fn test_flatten_integers() {
        let nested = vec![vec![1, 2], vec![3, 4], vec![5]];
        let flat = flatten(&nested);
        assert_eq!(flat, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_flatten_strings() {
        let nested = vec![
            vec!["apple".to_string(), "banana".to_string()],
            vec!["cherry".to_string()],
            vec!["date".to_string(), "elderberry".to_string()],
        ];
        let flat = flatten(&nested);
        assert_eq!(
            flat,
            vec![
                "apple".to_string(),
                "banana".to_string(),
                "cherry".to_string(),
                "date".to_string(),
                "elderberry".to_string()
            ]
        );
    }

    #[test]
    fn test_flatten_with_structs() {
        let people_groups = vec![
            vec![
                Person {
                    name: "Alice".to_string(),
                    age: 25,
                },
                Person {
                    name: "Bob".to_string(),
                    age: 30,
                },
            ],
            vec![
                Person {
                    name: "Carol".to_string(),
                    age: 35,
                },
                Person {
                    name: "Dave".to_string(),
                    age: 40,
                },
            ],
        ];

        let flat_people = flatten(&people_groups);
        assert_eq!(
            flat_people,
            vec![
                Person {
                    name: "Alice".to_string(),
                    age: 25
                },
                Person {
                    name: "Bob".to_string(),
                    age: 30
                },
                Person {
                    name: "Carol".to_string(),
                    age: 35
                },
                Person {
                    name: "Dave".to_string(),
                    age: 40
                },
            ]
        );
    }

    #[test]
    fn test_flatten_with_empty_inner_slices() {
        let nested: Vec<Vec<i32>> = vec![vec![1, 2], vec![], vec![3], vec![]];
        let flat = flatten(&nested);
        assert_eq!(flat, vec![1, 2, 3]);
    }

    #[test]
    fn test_flatten_with_all_empty_slices() {
        let nested: Vec<Vec<i32>> = vec![vec![], vec![], vec![]];
        let flat = flatten(&nested);
        assert_eq!(flat, Vec::<i32>::new());
    }

    #[test]
    fn test_flatten_with_single_inner_slice() {
        let nested = vec![vec![1, 2, 3, 4, 5]];
        let flat = flatten(&nested);
        assert_eq!(flat, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_flatten_with_multiple_empty_and_non_empty_slices() {
        let nested = vec![
            vec![],
            vec![1],
            vec![],
            vec![2, 3],
            vec![],
            vec![4, 5, 6],
            vec![],
        ];
        let flat = flatten(&nested);
        assert_eq!(flat, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_flatten_with_nested_empty_vectors() {
        let nested: Vec<Vec<i32>> = vec![vec![]];
        let flat = flatten(&nested);
        assert_eq!(flat, Vec::<i32>::new());
    }

    #[test]
    fn test_flatten_with_optionals() {
        let nested = vec![vec![Some(1), None], vec![Some(2), Some(3)], vec![None]];
        let flat = flatten(&nested);
        assert_eq!(flat, vec![Some(1), None, Some(2), Some(3), None]);
    }

    #[test]
    fn test_flatten_with_floats() {
        let nested = vec![vec![1.1, 2.2], vec![3.3], vec![4.4, 5.5]];
        let flat = flatten(&nested);
        assert_eq!(flat, vec![1.1, 2.2, 3.3, 4.4, 5.5]);
    }
}
