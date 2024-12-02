/// Removes all zero-valued elements from a collection, preserving the order of non-zero elements.
///
/// This function iterates over a slice of items, removing each element that is equal to the zero value.
/// The zero value is determined by the `Default` trait implementation for the type `T`.
/// The function preserves the order of the remaining elements and does not modify the original collection.
///
/// **Time Complexity:** O(n), where n is the number of elements in the collection.
///
/// # Arguments
///
/// * `collection` - A slice of items from which to remove zero-valued elements.
///
/// # Type Parameters
///
/// * `T` - The type of elements in the collection. Must implement `PartialEq`, `Clone`, and `Default`.
///
/// # Returns
///
/// * `Vec<T>` - A new vector containing only the non-zero elements from the input collection.
///
/// # Examples
///
/// ```rust
/// use lowdash::compact;
///
/// let numbers = vec![0, 1, 0, 2, 3, 0, 4];
/// let compacted = compact(&numbers);
/// assert_eq!(compacted, vec![1, 2, 3, 4]);
/// ```
///
/// ```rust
/// use lowdash::compact;
///
/// #[derive(Debug, PartialEq, Clone, Default)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let people = vec![
///     Person { name: "".to_string(), age: 0 },
///     Person { name: "Bob".to_string(), age: 30 },
///     Person { name: "".to_string(), age: 0 },
///     Person { name: "Dave".to_string(), age: 40 },
/// ];
///
/// let compacted = compact(&people);
/// assert_eq!(
///     compacted,
///     vec![
///         Person { name: "Bob".to_string(), age: 30 },
///         Person { name: "Dave".to_string(), age: 40 },
///     ]
/// );
/// ```
///
/// ```rust
/// use lowdash::compact;
///
/// let floats = vec![0.0, 1.1, 0.0, 2.2, 3.3, 0.0, 4.4];
/// let compacted = compact(&floats);
/// assert_eq!(compacted, vec![1.1, 2.2, 3.3, 4.4]);
/// ```
pub fn compact<T>(collection: &[T]) -> Vec<T>
where
    T: PartialEq + Clone + Default,
{
    let zero = T::default();
    let mut result = Vec::with_capacity(collection.len());

    for item in collection {
        if *item != zero {
            result.push(item.clone());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone, Default)]
    struct Person {
        name: String,
        age: u32,
    }

    #[test]
    fn test_compact_integers() {
        let numbers = vec![0, 1, 0, 2, 3, 0, 4];
        let compacted = compact(&numbers);
        assert_eq!(compacted, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_compact_floats() {
        let floats = vec![0.0, 1.1, 0.0, 2.2, 3.3, 0.0, 4.4];
        let compacted = compact(&floats);
        assert_eq!(compacted, vec![1.1, 2.2, 3.3, 4.4]);
    }

    #[test]
    fn test_compact_structs() {
        let people = vec![
            Person {
                name: "".to_string(),
                age: 0,
            },
            Person {
                name: "Bob".to_string(),
                age: 30,
            },
            Person {
                name: "".to_string(),
                age: 0,
            },
            Person {
                name: "Dave".to_string(),
                age: 40,
            },
        ];
        let compacted = compact(&people);
        assert_eq!(
            compacted,
            vec![
                Person {
                    name: "Bob".to_string(),
                    age: 30
                },
                Person {
                    name: "Dave".to_string(),
                    age: 40
                },
            ]
        );
    }

    #[test]
    fn test_compact_no_zero_elements() {
        let numbers = vec![1, 2, 3, 4, 5];
        let compacted = compact(&numbers);
        assert_eq!(compacted, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_compact_all_zero_elements() {
        let numbers = vec![0, 0, 0, 0, 0];
        let compacted = compact(&numbers);
        assert_eq!(compacted, Vec::<i32>::new());
    }

    #[test]
    fn test_compact_empty_collection() {
        let empty: Vec<i32> = vec![];
        let compacted = compact(&empty);
        assert_eq!(compacted, Vec::<i32>::new());
    }

    #[test]
    fn test_compact_with_strings() {
        #[derive(Debug, PartialEq, Clone, Default)]
        struct StringWrapper(String);

        let strings = vec![
            StringWrapper(String::new()),
            StringWrapper("hello".to_string()),
            StringWrapper(String::new()),
            StringWrapper("world".to_string()),
        ];
        let compacted = compact(&strings);
        assert_eq!(
            compacted,
            vec![
                StringWrapper("hello".to_string()),
                StringWrapper("world".to_string()),
            ]
        );
    }

    #[test]
    fn test_compact_with_optionals() {
        #[derive(Debug, PartialEq, Clone, Default)]
        struct OptionalInt(Option<i32>);

        let collection = vec![
            OptionalInt(None),
            OptionalInt(Some(1)),
            OptionalInt(None),
            OptionalInt(Some(2)),
            OptionalInt(Some(3)),
            OptionalInt(None),
        ];
        let compacted = compact(&collection);
        assert_eq!(
            compacted,
            vec![
                OptionalInt(Some(1)),
                OptionalInt(Some(2)),
                OptionalInt(Some(3)),
            ]
        );
    }

    #[test]
    fn test_compact_with_characters() {
        let chars = vec!['\0', 'a', '\0', 'b', 'c', '\0', 'd'];
        let compacted = compact(&chars);
        assert_eq!(compacted, vec!['a', 'b', 'c', 'd']);
    }

    #[test]
    fn test_compact_with_custom_default() {
        #[derive(Debug, PartialEq, Clone)]
        struct CustomDefault {
            value: u32,
        }

        impl Default for CustomDefault {
            fn default() -> Self {
                CustomDefault { value: 0 }
            }
        }

        let collection = vec![
            CustomDefault { value: 0 },
            CustomDefault { value: 1 },
            CustomDefault { value: 0 },
            CustomDefault { value: 2 },
        ];
        let compacted = compact(&collection);
        assert_eq!(
            compacted,
            vec![CustomDefault { value: 1 }, CustomDefault { value: 2 },]
        );
    }
}
