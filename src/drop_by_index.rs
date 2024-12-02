/// Removes elements from a collection at the specified indices.
/// Supports negative indices which count from the end of the collection.
/// Indices that are out of bounds are ignored.
///
/// **Time Complexity:** O(n log n) for sorting the indices, O(m) for cloning, where m is the length of the resulting collection.
///
/// # Arguments
///
/// * `collection` - A slice of items from which elements will be removed.
/// * `indexes` - A slice of indices at which elements should be removed.
///
/// # Type Parameters
///
/// * `T` - The type of elements in the collection. Must implement `Clone`.
///
/// # Returns
///
/// * `Vec<T>` - A vector containing the elements after removing the specified indices.
///
/// # Examples
///
/// ```rust
/// use lowdash::drop_by_index;
///
/// let numbers = vec![1, 2, 3, 4, 5];
/// let result = drop_by_index(&numbers, &[1, 3]);
/// assert_eq!(result, vec![1, 3, 5]);
/// ```
///
/// ```rust
/// use lowdash::drop_by_index;
///
/// let letters = vec!['a', 'b', 'c', 'd', 'e'];
/// let result = drop_by_index(&letters, &[0, -1]);
/// assert_eq!(result, vec!['b', 'c', 'd']);
/// ```
pub fn drop_by_index<T>(collection: &[T], indexes: &[isize]) -> Vec<T>
where
    T: Clone,
{
    let initial_size = collection.len() as isize;
    if initial_size == 0 {
        return Vec::new();
    }

    // Adjust negative indices and filter out-of-bounds indices
    let mut adjusted_indexes: Vec<usize> = indexes
        .iter()
        .filter_map(|&idx| {
            let adjusted_idx = if idx < 0 { initial_size + idx } else { idx };

            if adjusted_idx >= 0 && adjusted_idx < initial_size {
                Some(adjusted_idx as usize)
            } else {
                None
            }
        })
        .collect();

    // Remove duplicates and sort indices
    adjusted_indexes.sort_unstable();
    adjusted_indexes.dedup();

    let mut result = Vec::with_capacity(collection.len() - adjusted_indexes.len());
    let mut skip_iter = adjusted_indexes.into_iter();
    let mut skip_idx = skip_iter.next();

    for (i, item) in collection.iter().enumerate() {
        if Some(i) == skip_idx {
            skip_idx = skip_iter.next();
        } else {
            result.push(item.clone());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_by_index_normal_case() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = drop_by_index(&numbers, &[1, 3]);
        assert_eq!(result, vec![1, 3, 5]);
    }

    #[test]
    fn test_drop_by_index_with_negative_indices() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = drop_by_index(&numbers, &[0, -1]);
        assert_eq!(result, vec![2, 3, 4]);
    }

    #[test]
    fn test_drop_by_index_with_out_of_bounds_indices() {
        let numbers = vec![1, 2, 3];
        let result = drop_by_index(&numbers, &[5, -5]);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_drop_by_index_with_duplicates() {
        let letters = vec!['a', 'b', 'c', 'd', 'e'];
        let result = drop_by_index(&letters, &[1, 2, 2, 1]);
        assert_eq!(result, vec!['a', 'd', 'e']);
    }

    #[test]
    fn test_drop_by_index_empty_collection() {
        let empty: Vec<i32> = vec![];
        let result = drop_by_index(&empty, &[0, 1, -1]);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_drop_by_index_no_indices() {
        let numbers = vec![1, 2, 3];
        let result = drop_by_index(&numbers, &[]);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_drop_by_index_all_indices() {
        let numbers = vec![1, 2, 3];
        let result = drop_by_index(&numbers, &[0, 1, 2]);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_drop_by_index_with_structs() {
        #[derive(Clone, Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        let points = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 2 },
            Point { x: 3, y: 3 },
        ];

        let result = drop_by_index(&points, &[1, -1]);
        let expected = vec![Point { x: 0, y: 0 }, Point { x: 2, y: 2 }];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_drop_by_index_with_invalid_negative_indices() {
        let numbers = vec![1, 2, 3];
        let result = drop_by_index(&numbers, &[-5, -4, -3]);
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn test_drop_by_index_preserves_order() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let result = drop_by_index(&numbers, &[4, 0, 2]);
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    fn test_drop_by_index_with_duplicate_elements() {
        let letters = vec!['a', 'b', 'c', 'a', 'b', 'c'];
        let result = drop_by_index(&letters, &[1, 3, 5]);
        assert_eq!(result, vec!['a', 'c', 'b']);
    }
}
