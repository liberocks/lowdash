/// Finds all combinations of k elements from a collection.
///
/// # Arguments
/// * `items` - Items to combine.
/// * `k` - Length of each combination.
///
/// # Returns
/// * `Vec<Vec<T>>` - All length-`k` combinations, or an empty vector when `k` exceeds the input length.
///
/// # Examples
/// ```rust
/// use lowdash::combination;
///
/// let items = vec![1, 2, 3, 4];
/// let result = combination(&items, 2);
/// assert_eq!(result.len(), 6);
/// // Example: [2, 3]
/// assert!(result.contains(&vec![2, 3]));
/// ```
pub fn combination<T: Clone>(items: &[T], k: usize) -> Vec<Vec<T>> {
    if k == 0 {
        return vec![vec![]];
    }
    if k > items.len() {
        return vec![];
    }

    fn backtrack<T: Clone>(
        items: &[T],
        start: usize,
        k: usize,
        current: &mut Vec<T>,
        result: &mut Vec<Vec<T>>,
    ) {
        if current.len() == k {
            result.push(current.clone());
            return;
        }

        let remaining = k - current.len();
        let end = items.len() - remaining;
        for index in start..=end {
            current.push(items[index].clone());
            backtrack(items, index + 1, k, current, result);
            current.pop();
        }
    }

    let choose = k.min(items.len() - k);
    let capacity = (1..=choose).try_fold(1usize, |count, index| {
        count
            .checked_mul(items.len() - choose + index)?
            .checked_div(index)
    });
    let mut result = capacity.map_or_else(Vec::new, Vec::with_capacity);
    let mut current = Vec::with_capacity(k);
    backtrack(items, 0, k, &mut current, &mut result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_k_zero() {
        let items = vec![1, 2, 3];
        let result = combination(&items, 0);
        assert_eq!(result, vec![vec![]]);
    }

    #[test]
    fn test_combination_k_greater_than_len() {
        let items = vec![1, 2];
        let result = combination(&items, 3);
        assert!(result.is_empty());
    }

    #[test]
    fn test_combination_single_element() {
        let items = vec![42];
        let result = combination(&items, 1);
        assert_eq!(result, vec![vec![42]]);
    }

    #[test]
    fn test_combination_multiple() {
        let items = vec![1, 2, 3, 4];
        let result = combination(&items, 2);
        // Expected combinations.
        assert_eq!(
            result,
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4]
            ]
        );
    }

    #[test]
    fn test_combination_all_elements() {
        let items = vec![1, 2, 3];
        let result = combination(&items, 3);
        assert_eq!(result, vec![vec![1, 2, 3]]);
    }
}
