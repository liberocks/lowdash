/// Drops the first `n` elements, preserving the rest in order.
/// Returns an empty vector when `n` is at least the collection length.
///
/// **Time Complexity:** O(m) to clone the `m` retained elements.
///
/// # Arguments
///
/// * `collection` - Items to trim.
/// * `n` - Number of items to drop.
///
/// # Type Parameters
///
/// * `T` - Item type.
///
/// # Returns
///
/// * `Vec<T>` - The retained elements.
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
