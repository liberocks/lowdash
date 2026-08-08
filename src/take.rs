/// Returns the first `n` elements, preserving their order.
///
/// If `n` is greater than the collection length, returns the whole collection.
///
/// **Time Complexity:** O(m) to clone the `m` returned elements.
///
/// # Examples
/// ```rust
/// use lowdash::take;
///
/// let numbers = vec![1, 2, 3, 4, 5];
/// assert_eq!(take(&numbers, 3), vec![1, 2, 3]);
/// ```
pub fn take<T>(collection: &[T], n: usize) -> Vec<T>
where
    T: Clone,
{
    collection[..n.min(collection.len())].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_the_first_items() {
        assert_eq!(take(&[1, 2, 3, 4, 5], 3), vec![1, 2, 3]);
    }

    #[test]
    fn returns_empty_when_n_is_zero() {
        assert_eq!(take(&[1, 2, 3], 0), Vec::<i32>::new());
    }

    #[test]
    fn returns_the_whole_collection_when_n_matches_length() {
        assert_eq!(take(&[1, 2, 3], 3), vec![1, 2, 3]);
    }

    #[test]
    fn returns_the_whole_collection_when_n_exceeds_length() {
        assert_eq!(take(&[1, 2, 3], 10), vec![1, 2, 3]);
    }

    #[test]
    fn handles_empty_collections() {
        let empty: Vec<i32> = Vec::new();

        assert_eq!(take(&empty, 2), Vec::<i32>::new());
    }

    #[test]
    fn works_with_custom_types() {
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

        assert_eq!(
            take(&points, 2),
            vec![Point { x: 0, y: 0 }, Point { x: 1, y: 1 }]
        );
    }
}
