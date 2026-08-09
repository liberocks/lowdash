/// Returns full-size windows separated by `step` elements.
///
/// A step of `1` produces overlapping windows, a step equal to `size`
/// produces non-overlapping windows, and larger steps skip elements. Trailing
/// partial windows are omitted.
///
/// **Time Complexity:** O(k * size), where `k` is the number of windows.
/// **Space Complexity:** O(k * size) for the cloned windows.
///
/// # Panics
/// Panics if `size` or `step` is zero.
///
/// # Examples
/// ```rust
/// use lowdash::sliding;
///
/// assert_eq!(
///     sliding(&[1, 2, 3, 4, 5], 3, 2),
///     vec![vec![1, 2, 3], vec![3, 4, 5]],
/// );
/// ```
pub fn sliding<T>(collection: &[T], size: usize, step: usize) -> Vec<Vec<T>>
where
    T: Clone,
{
    if size == 0 {
        panic!("Sliding size must be greater than 0");
    }
    if step == 0 {
        panic!("Sliding step must be greater than 0");
    }
    if size > collection.len() {
        return Vec::new();
    }

    let last_start = collection.len() - size;
    (0..=last_start)
        .step_by(step)
        .map(|start| collection[start..start + size].to_vec())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_overlapping_windows() {
        assert_eq!(
            sliding(&[1, 2, 3, 4, 5], 3, 1),
            vec![vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]
        );
    }

    #[test]
    fn creates_non_overlapping_windows_when_step_matches_size() {
        assert_eq!(
            sliding(&[1, 2, 3, 4, 5, 6], 2, 2),
            vec![vec![1, 2], vec![3, 4], vec![5, 6]]
        );
    }

    #[test]
    fn skips_elements_when_step_is_larger_than_size() {
        assert_eq!(
            sliding(&[1, 2, 3, 4, 5, 6, 7, 8], 2, 3),
            vec![vec![1, 2], vec![4, 5], vec![7, 8]]
        );
    }

    #[test]
    fn omits_incomplete_windows() {
        assert_eq!(sliding(&[1, 2, 3, 4, 5], 3, 3), vec![vec![1, 2, 3]]);
        assert!(sliding(&[1, 2], 3, 1).is_empty());
        assert!(sliding::<i32>(&[], 3, 1).is_empty());
    }

    #[test]
    #[should_panic(expected = "Sliding size must be greater than 0")]
    fn rejects_zero_size() {
        let _ = sliding(&[1, 2], 0, 1);
    }

    #[test]
    #[should_panic(expected = "Sliding step must be greater than 0")]
    fn rejects_zero_step() {
        let _ = sliding(&[1, 2], 1, 0);
    }
}
