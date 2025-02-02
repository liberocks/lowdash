/// Finds all combinations of k elements from a collection.
///
/// # Arguments
/// * `items` - A slice of items to combine
/// * `k` - The number of elements to select in each combination
///
/// # Returns
/// * `Vec<Vec<T>>` - A vector containing all combinations of k elements from the input
///
/// # Examples
/// ```rust
/// use lowdash::combination;
///
/// let items = vec![1, 2, 3, 4];
/// let result = combination(&items, 2);
/// assert_eq!(result.len(), 6);
/// // One possible combination: [2, 3]
/// assert!(result.contains(&vec![2, 3]));
/// ```
pub fn combination<T: Clone>(items: &[T], k: usize) -> Vec<Vec<T>> {
    if k == 0 {
        return vec![vec![]];
    }
    if k > items.len() {
        return vec![];
    }
    let mut result = Vec::new();
    for i in 0..=items.len() - k {
        let current = items[i].clone();
        let rest_combinations = combination(&items[i + 1..], k - 1);
        for mut comb in rest_combinations {
            let mut entry = vec![current.clone()];
            entry.append(&mut comb);
            result.push(entry);
        }
    }
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
        // combinations: [1,2], [1,3], [1,4], [2,3], [2,4], [3,4]
        assert_eq!(result.len(), 6);
        assert!(result.contains(&vec![1, 2]));
        assert!(result.contains(&vec![2, 4]));
    }

    #[test]
    fn test_combination_all_elements() {
        let items = vec![1, 2, 3];
        let result = combination(&items, 3);
        assert_eq!(result, vec![vec![1, 2, 3]]);
    }
}
