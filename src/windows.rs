/// Returns all overlapping windows of `size` items.
///
/// A zero size panics with the same message as [`crate::chunk`]. Empty inputs
/// and sizes larger than the collection return an empty vector.
///
/// **Time Complexity:** O((n - size + 1) * size) to clone the windows, with
/// O((n - size + 1) * size) output space.
///
/// # Panics
/// Panics if `size` is zero.
///
/// # Examples
/// ```rust
/// use lowdash::windows;
///
/// assert_eq!(windows(&[1, 2, 3, 4], 3), vec![
///     vec![1, 2, 3],
///     vec![2, 3, 4],
/// ]);
/// ```
pub fn windows<T>(collection: &[T], size: usize) -> Vec<Vec<T>>
where
    T: Clone,
{
    if size == 0 {
        panic!("Chunk size must be greater than 0");
    }

    if size > collection.len() {
        return Vec::new();
    }

    let count = collection.len() - size + 1;
    let mut result = Vec::with_capacity(count);

    for start in 0..count {
        result.push(collection[start..start + size].to_vec());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_overlapping_windows() {
        assert_eq!(
            windows(&[1, 2, 3, 4], 3),
            vec![vec![1, 2, 3], vec![2, 3, 4]]
        );
    }

    #[test]
    fn returns_one_window_when_size_matches_length() {
        assert_eq!(windows(&[1, 2, 3], 3), vec![vec![1, 2, 3]]);
    }

    #[test]
    fn returns_empty_for_empty_or_oversized_inputs() {
        let empty: [i32; 0] = [];

        assert_eq!(windows(&empty, 2), Vec::<Vec<i32>>::new());
        assert_eq!(windows(&[1, 2], 3), Vec::<Vec<i32>>::new());
    }

    #[test]
    #[should_panic(expected = "Chunk size must be greater than 0")]
    fn panics_for_zero_size_like_chunk() {
        let _ = windows(&[1, 2, 3], 0);
    }

    #[test]
    fn works_with_custom_types() {
        #[derive(Debug, PartialEq, Clone)]
        struct Point {
            x: i32,
        }

        let points = [Point { x: 1 }, Point { x: 2 }, Point { x: 3 }];

        assert_eq!(
            windows(&points, 2),
            vec![
                vec![Point { x: 1 }, Point { x: 2 }],
                vec![Point { x: 2 }, Point { x: 3 }],
            ]
        );
    }
}
