/// Returns a subset of the collection based on the provided offset and length.
///
/// This function extracts a subset from the given collection starting at the specified `offset` and spanning
/// up to `length` elements. If the `offset` is negative, it counts from the end of the collection. The function
/// ensures that the resulting subset does not exceed the bounds of the original collection.
///
/// **Time Complexity:** O(n), where n is the number of elements in the subset.
///
/// # Arguments
///
/// * `collection` - A slice of items from which to extract the subset.
/// * `offset` - The starting position for the subset. Can be negative to indicate an offset from the end.
/// * `length` - The number of elements to include in the subset.
///
/// # Type Parameters
///
/// * `T` - The type of elements in the collection. Must implement `Clone`.
///
/// # Returns
///
/// * `Vec<T>` - A vector containing the subset of elements.
///
/// # Examples
///
/// ```rust
/// use lowdash::subset;
///
/// let numbers = vec![1, 2, 3, 4, 5];
/// let result = subset(&numbers, 1, 3);
/// assert_eq!(result, vec![2, 3, 4]);
/// ```
pub fn subset<T>(collection: &[T], offset: isize, length: usize) -> Vec<T>
where
    T: Clone,
{
    let size = collection.len() as isize;

    let mut start = if offset < 0 { size + offset } else { offset };

    if start < 0 {
        start = 0;
    }

    if start as usize > collection.len() {
        return Vec::new();
    }

    let end = ((start + length as isize) as usize).min(collection.len());

    collection[start as usize..end].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subset_positive_offset_within_bounds() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = subset(&numbers, 1, 3);
        assert_eq!(result, vec![2, 3, 4]);
    }

    #[test]
    fn test_subset_positive_offset_exceeds_bounds() {
        let numbers = vec![1, 2, 3];
        let result = subset(&numbers, 5, 2);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_subset_negative_offset_within_bounds() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = subset(&numbers, -3, 2);
        assert_eq!(result, vec![3, 4]);
    }

    #[test]
    fn test_subset_negative_offset_exceeds_bounds() {
        let numbers = vec![1, 2, 3];
        let result = subset(&numbers, -5, 2);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_subset_length_exceeds_bounds() {
        let numbers = vec![1, 2, 3, 4];
        let result = subset(&numbers, 2, 5);
        assert_eq!(result, vec![3, 4]);
    }

    #[test]
    fn test_subset_zero_length() {
        let numbers = vec![1, 2, 3];
        let result = subset(&numbers, 1, 0);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_subset_empty_collection() {
        let empty: Vec<i32> = vec![];
        let result = subset(&empty, 0, 3);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_subset_with_structs() {
        #[derive(Debug, PartialEq, Clone)]
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
            Person {
                name: "Dave".to_string(),
                age: 40,
            },
        ];

        let result = subset(&people, 1, 2);
        assert_eq!(
            result,
            vec![
                Person {
                    name: "Bob".to_string(),
                    age: 30
                },
                Person {
                    name: "Carol".to_string(),
                    age: 35
                },
            ]
        );
    }

    #[test]
    fn test_subset_with_floats() {
        let float_collection = vec![1.1, 2.2, 3.3, 4.4, 5.5];
        let result = subset(&float_collection, -3, 2);
        assert_eq!(result, vec![3.3, 4.4]);
    }

    #[test]
    fn test_subset_with_characters() {
        let chars = vec!['a', 'b', 'c', 'd', 'e'];
        let result = subset(&chars, 2, 2);
        assert_eq!(result, vec!['c', 'd']);
    }

    #[test]
    fn test_subset_entire_collection() {
        let numbers = vec![1, 2, 3];
        let result = subset(&numbers, 0, 3);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_subset_full_negative_offset() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = subset(&numbers, -5, 5);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_subset_partial_negative_offset() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = subset(&numbers, -2, 2);
        assert_eq!(result, vec![4, 5]);
    }
}
