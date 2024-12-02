/// Removes the first `n` elements from a collection and returns the remaining elements.
/// If `n` is greater than or equal to the length of the collection, returns an empty `Vec`.
///
/// **Time Complexity:** O(1) for slices, as it creates a slice from an existing slice.  
/// O(m) for cloning elements, where m is the number of elements after dropping.
///
/// # Arguments
///
/// * `collection` - A slice of items from which elements will be dropped.
/// * `n` - The number of elements to drop from the beginning of the collection.
///
/// # Type Parameters
///
/// * `T` - The type of elements in the collection. Must implement `Clone`.
///
/// # Returns
///
/// * `Vec<T>` - A vector containing the elements after dropping the first `n` elements.
///
/// # Examples
///
/// ```rust
/// use lowdash::drop;
///
/// let numbers = vec![1, 2, 3, 4, 5];
/// let result = drop(&numbers, 2);
/// assert_eq!(result, vec![3, 4, 5]);
/// ```
///
/// ```rust
/// use lowdash::drop;
///
/// let letters = vec!['a', 'b', 'c', 'd'];
/// let result = drop(&letters, 10);
/// assert_eq!(result, vec![]);
/// ```
pub fn drop<T>(collection: &[T], n: usize) -> Vec<T>
where
    T: Clone,
{
    if n >= collection.len() {
        Vec::new()
    } else {
        collection[n..].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_normal_case() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = drop(&numbers, 2);
        assert_eq!(result, vec![3, 4, 5]);
    }

    #[test]
    fn test_drop_zero_elements() {
        let numbers = vec![1, 2, 3];
        let result = drop(&numbers, 0);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_drop_n_equal_length() {
        let numbers = vec![1, 2, 3];
        let result = drop(&numbers, 3);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_drop_n_greater_than_length() {
        let numbers = vec![1, 2, 3];
        let result = drop(&numbers, 5);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_drop_empty_collection() {
        let empty: Vec<i32> = vec![];
        let result = drop(&empty, 2);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_drop_with_strings() {
        let words = vec!["hello", "world", "rust"];
        let result = drop(&words, 1);
        assert_eq!(result, vec!["world", "rust"]);
    }

    #[test]
    fn test_drop_with_structs() {
        #[derive(Debug, PartialEq, Clone)]
        struct Point {
            x: i32,
            y: i32,
        }

        let points = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 2 },
        ];
        let result = drop(&points, 2);
        let expected = vec![Point { x: 2, y: 2 }];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_drop_n_is_zero() {
        let letters = vec!['a', 'b', 'c'];
        let result = drop(&letters, 0);
        assert_eq!(result, vec!['a', 'b', 'c']);
    }

    #[test]
    fn test_drop_full_drop() {
        let letters = vec!['a', 'b', 'c'];
        let result = drop(&letters, letters.len());
        assert_eq!(result, Vec::<char>::new());
    }
}
