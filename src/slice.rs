/// Returns a subset of the collection based on the provided start and end indices.
///
/// This function extracts a subset from the given collection starting at the specified `start` index
/// and ending before the `end` index. If the `start` or `end` indices are negative, they are offset
/// from the end of the collection. The function ensures that the resulting subset does not exceed
/// the bounds of the original collection.
///
/// **Time Complexity:** O(n), where n is the number of elements in the subset.
///
/// # Arguments
///
/// * `collection` - A slice of items from which to extract the subset.
/// * `start` - The starting index for the subset. Can be negative to indicate an offset from the end.
/// * `end` - The ending index for the subset. Can be negative to indicate an offset from the end.
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
/// use lowdash::slice;
///
/// let numbers = vec![1, 2, 3, 4, 5];
/// let result = slice(&numbers, 1, 3);
/// assert_eq!(result, vec![2, 3]);
/// ```
///
/// ```rust
/// use lowdash::slice;
///
/// let numbers = vec![1, 2, 3, 4, 5];
/// let result = slice(&numbers, -3, -1);
/// assert_eq!(result, vec![3, 4]);
/// ```
///
/// ```rust
/// use lowdash::slice;
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
/// let result = slice(&people, 1, 3);
/// assert_eq!(
///     result,
///     vec![
///         Person { name: "Bob".to_string(), age: 30 },
///         Person { name: "Carol".to_string(), age: 35 },
///     ]
/// );
/// ```
pub fn slice<T>(collection: &[T], start: isize, end: isize) -> Vec<T>
where
    T: Clone,
{
    let size = collection.len() as isize;

    // Adjust start index
    let mut adjusted_start = if start < 0 { size + start } else { start };
    if adjusted_start < 0 {
        adjusted_start = 0;
    }
    if adjusted_start > size {
        adjusted_start = size;
    }

    // Adjust end index
    let mut adjusted_end = if end < 0 { size + end } else { end };
    if adjusted_end < 0 {
        adjusted_end = 0;
    }
    if adjusted_end > size {
        adjusted_end = size;
    }

    // If start is greater than or equal to end, return empty vector
    if adjusted_start >= adjusted_end {
        return Vec::new();
    }

    // Convert to usize for slicing
    let start_usize = adjusted_start as usize;
    let end_usize = adjusted_end as usize;

    collection[start_usize..end_usize].to_vec()
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
    fn test_slice_positive_indices_within_bounds() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = slice(&numbers, 1, 3);
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn test_slice_positive_indices_exceed_bounds() {
        let numbers = vec![1, 2, 3];
        let result = slice(&numbers, 1, 5);
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn test_slice_negative_start_within_bounds() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = slice(&numbers, -3, -1);
        assert_eq!(result, vec![3, 4]);
    }

    #[test]
    fn test_slice_negative_start_exceeds_bounds() {
        let numbers = vec![1, 2, 3];
        let result = slice(&numbers, -5, 2);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_slice_negative_end_within_bounds() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = slice(&numbers, 1, -1);
        assert_eq!(result, vec![2, 3, 4]);
    }

    #[test]
    fn test_slice_negative_end_exceeds_bounds() {
        let numbers = vec![1, 2, 3];
        let result = slice(&numbers, 0, -5);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_slice_start_greater_than_end() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = slice(&numbers, 4, 2);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_slice_start_equals_end() {
        let numbers = vec![1, 2, 3];
        let result = slice(&numbers, 2, 2);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_slice_full_range() {
        let numbers = vec![1, 2, 3];
        let result = slice(&numbers, 0, 3);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_slice_empty_collection() {
        let empty: Vec<i32> = vec![];
        let result = slice(&empty, 0, 3);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_slice_with_structs() {
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

        let result = slice(&people, 1, 3);
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
    fn test_slice_with_floats() {
        let float_collection = vec![1.1, 2.2, 3.3, 4.4, 5.5];
        let result = slice(&float_collection, -4, -1);
        assert_eq!(result, vec![2.2, 3.3, 4.4]);
    }

    #[test]
    fn test_slice_with_characters() {
        let chars = vec!['a', 'b', 'c', 'd', 'e'];
        let result = slice(&chars, 2, 4);
        assert_eq!(result, vec!['c', 'd']);
    }

    #[test]
    fn test_slice_with_overlapping_negative_indices() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = slice(&numbers, -4, -1);
        assert_eq!(result, vec![2, 3, 4]);
    }

    #[test]
    fn test_slice_full_negative_indices() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = slice(&numbers, -5, -1);
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_slice_partial_negative_indices() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = slice(&numbers, 1, -2);
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn test_slice_start_negative_end_positive() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = slice(&numbers, -3, 4);
        assert_eq!(result, vec![3, 4]);
    }

    #[test]
    fn test_slice_end_negative_start_positive() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = slice(&numbers, 1, -1);
        assert_eq!(result, vec![2, 3, 4]);
    }
}
