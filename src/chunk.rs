/// Splits a collection into chunks of at most `size` elements, preserving order.
/// The last chunk may be shorter.
///
/// **Panics:**  
/// Panics if `size` is less than or equal to 0.
///
/// **Time Complexity:**  
/// O(n), where `n` is the collection length.
///
/// # Arguments
///
/// * `collection` - Items to split.
/// * `size` - Maximum chunk length.
///
/// # Type Parameters
///
/// * `T` - Item type.
///
/// # Returns
///
/// * `Vec<Vec<T>>` - The chunks.
///
/// # Examples
///
/// ```rust
/// use lowdash::chunk;
///
/// let numbers = vec![1, 2, 3, 4, 5, 6, 7];
/// let chunks = chunk(&numbers, 3);
/// assert_eq!(
///     chunks,
///     vec![vec![1, 2, 3], vec![4, 5, 6], vec![7]]
/// );
/// ```
///
/// ```rust
/// use lowdash::chunk;
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
///     Person { name: "Dave".to_string(), age: 40 },
/// ];
///
/// let chunks = chunk(&people, 2);
/// assert_eq!(
///     chunks,
///     vec![
///         vec![
///             Person { name: "Alice".to_string(), age: 25 },
///             Person { name: "Bob".to_string(), age: 30 },
///         ],
///         vec![
///             Person { name: "Carol".to_string(), age: 35 },
///             Person { name: "Dave".to_string(), age: 40 },
///         ],
///     ]
/// );
/// ```
pub fn chunk<T>(collection: &[T], size: usize) -> Vec<Vec<T>>
where
    T: Clone,
{
    if size == 0 {
        panic!("Chunk size must be greater than 0");
    }

    let chunk_count = collection.len() / size + usize::from(!collection.len().is_multiple_of(size));
    let mut result: Vec<Vec<T>> = Vec::with_capacity(chunk_count);
    let mut start = 0;

    while start < collection.len() {
        let end = if start + size > collection.len() {
            collection.len()
        } else {
            start + size
        };
        let chunk = collection[start..end].to_vec();
        result.push(chunk);
        start += size;
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

    #[derive(Debug, PartialEq, Clone)]
    struct Item {
        id: u32,
        value: String,
    }

    #[test]
    fn test_chunk_integers_exact_division() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let chunks = chunk(&numbers, 2);
        assert_eq!(chunks, vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
    }

    #[test]
    fn test_chunk_integers_non_exact_division() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7];
        let chunks = chunk(&numbers, 3);
        assert_eq!(chunks, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7]]);
    }

    #[test]
    fn test_chunk_strings_exact_division() {
        let strings = vec!["a", "b", "c", "d"];
        let chunks = chunk(&strings, 2);
        assert_eq!(chunks, vec![vec!["a", "b"], vec!["c", "d"]]);
    }

    #[test]
    fn test_chunk_strings_non_exact_division() {
        let strings = vec!["apple", "banana", "cherry"];
        let chunks = chunk(&strings, 2);
        assert_eq!(chunks, vec![vec!["apple", "banana"], vec!["cherry"]]);
    }

    #[test]
    fn test_chunk_with_structs() {
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
            Person {
                name: "Dave".to_string(),
                age: 40,
            },
        ];

        let chunks = chunk(&people, 3);
        assert_eq!(
            chunks,
            vec![
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
                ],
                vec![Person {
                    name: "Dave".to_string(),
                    age: 40
                },],
            ]
        );
    }

    #[test]
    fn test_chunk_with_empty_collection() {
        let empty: Vec<i32> = vec![];
        let chunks = chunk(&empty, 3);
        assert_eq!(chunks, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_chunk_with_size_larger_than_collection() {
        let numbers = vec![1, 2];
        let chunks = chunk(&numbers, 5);
        assert_eq!(chunks, vec![vec![1, 2]]);
    }

    #[test]
    #[should_panic(expected = "Chunk size must be greater than 0")]
    fn test_chunk_with_zero_size() {
        let numbers = vec![1, 2, 3];
        let _chunks = chunk(&numbers, 0);
    }

    #[test]
    fn test_chunk_with_size_one() {
        let numbers = vec![1, 2, 3];
        let chunks = chunk(&numbers, 1);
        assert_eq!(chunks, vec![vec![1], vec![2], vec![3]]);
    }

    #[test]
    fn test_chunk_with_mixed_types() {
        let items = vec![
            Item {
                id: 1,
                value: "one".to_string(),
            },
            Item {
                id: 2,
                value: "two".to_string(),
            },
            Item {
                id: 3,
                value: "three".to_string(),
            },
            Item {
                id: 4,
                value: "four".to_string(),
            },
            Item {
                id: 5,
                value: "five".to_string(),
            },
        ];

        let chunks = chunk(&items, 2);
        assert_eq!(
            chunks,
            vec![
                vec![
                    Item {
                        id: 1,
                        value: "one".to_string()
                    },
                    Item {
                        id: 2,
                        value: "two".to_string()
                    },
                ],
                vec![
                    Item {
                        id: 3,
                        value: "three".to_string()
                    },
                    Item {
                        id: 4,
                        value: "four".to_string()
                    },
                ],
                vec![Item {
                    id: 5,
                    value: "five".to_string()
                },],
            ]
        );
    }

    #[test]
    fn test_chunk_preserves_order() {
        let numbers = vec![5, 4, 3, 2, 1];
        let chunks = chunk(&numbers, 2);
        assert_eq!(chunks, vec![vec![5, 4], vec![3, 2], vec![1]]);
    }

    #[test]
    fn test_chunk_with_optionals() {
        let collection = vec![Some(1), None, Some(2), None, Some(3)];
        let chunks = chunk(&collection, 2);
        assert_eq!(
            chunks,
            vec![vec![Some(1), None], vec![Some(2), None], vec![Some(3)],]
        );
    }

    #[test]
    fn test_chunk_with_nan_floats() {
        let float_collection = vec![f64::NAN, 2.2, f64::NAN, 4.4, 5.5];
        let chunks = chunk(&float_collection, 2);
        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0].len(), 2);
        assert!(chunks[0][0].is_nan());
        assert_eq!(chunks[0][1], 2.2);
        assert_eq!(chunks[1].len(), 2);
        assert!(chunks[1][0].is_nan());
        assert_eq!(chunks[1][1], 4.4);
        assert_eq!(chunks[2], vec![5.5]);
    }
}
