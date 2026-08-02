/// Drops the last `n` elements, preserving the rest in order.
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
/// use lowdash::drop_right;
///
/// let numbers = vec![1, 2, 3, 4, 5];
/// let result = drop_right(&numbers, 2);
/// assert_eq!(result, vec![1, 2, 3]);
/// ```
///
/// ```rust
/// use lowdash::drop_right;
///
/// let letters = vec!['a', 'b', 'c', 'd'];
/// let result = drop_right(&letters, 10);
/// assert_eq!(result, vec![]);
/// ```
pub fn drop_right<T>(collection: &[T], n: usize) -> Vec<T>
where
    T: Clone,
{
    if n >= collection.len() {
        Vec::new()
    } else {
        collection[..collection.len() - n].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_right_normal_case() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = drop_right(&numbers, 2);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_drop_right_zero_elements() {
        let numbers = vec![1, 2, 3];
        let result = drop_right(&numbers, 0);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_drop_right_n_equal_length() {
        let numbers = vec![1, 2, 3];
        let result = drop_right(&numbers, 3);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_drop_right_n_greater_than_length() {
        let numbers = vec![1, 2, 3];
        let result = drop_right(&numbers, 5);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_drop_right_empty_collection() {
        let empty: Vec<i32> = vec![];
        let result = drop_right(&empty, 2);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_drop_right_with_strings() {
        let words = vec!["hello", "world", "rust"];
        let result = drop_right(&words, 1);
        assert_eq!(result, vec!["hello", "world"]);
    }

    #[test]
    fn test_drop_right_with_structs() {
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
        let result = drop_right(&points, 2);
        let expected = vec![Point { x: 0, y: 0 }];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_drop_right_n_is_zero() {
        let letters = vec!['a', 'b', 'c'];
        let result = drop_right(&letters, 0);
        assert_eq!(result, vec!['a', 'b', 'c']);
    }

    #[test]
    fn test_drop_right_full_drop() {
        let letters = vec!['a', 'b', 'c'];
        let result = drop_right(&letters, letters.len());
        assert_eq!(result, Vec::<char>::new());
    }
}
