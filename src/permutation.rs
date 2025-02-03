/// Finds all permutations of k elements from a collection.
///
/// # Arguments
/// * `items` - A slice of items to permute.
/// * `k` - The number of elements in each permutation.
///
/// # Returns
/// * `Vec<Vec<T>>` - A vector containing all permutations of k elements from the input.
///
/// # Examples
/// ```rust
/// use lowdash::permutation;
///
/// let items = vec![1, 2, 3];
/// let result = permutation(&items, 2);
/// // Expected permutations: [ [1,2], [1,3], [2,1], [2,3], [3,1], [3,2] ]
/// assert_eq!(result.len(), 6);
/// assert!(result.contains(&vec![2, 1]));
/// ```
pub fn permutation<T: Clone>(items: &[T], k: usize) -> Vec<Vec<T>> {
    if k == 0 {
        return vec![vec![]];
    }
    if k > items.len() {
        return vec![];
    }
    let mut result = Vec::new();
    for (i, item) in items.iter().enumerate() {
        let mut remaining = items.to_vec();
        remaining.remove(i);
        for mut perm in permutation(&remaining, k - 1) {
            let mut current = vec![item.clone()];
            current.append(&mut perm);
            result.push(current);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutation_k_zero() {
        let items: Vec<i32> = vec![];
        assert_eq!(permutation(&items, 0), vec![vec![]]);
    }

    #[test]
    fn test_permutation_k_greater_than_len() {
        let items = vec![42];
        assert_eq!(permutation(&items, 2), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_permutation_single_element() {
        let items = vec![42];
        assert_eq!(permutation(&items, 1), vec![vec![42]]);
    }

    #[test]
    fn test_permutation_multiple() {
        let items = vec![1, 2, 3];
        let perms = permutation(&items, 2);
        assert_eq!(perms.len(), 6);
        assert!(perms.contains(&vec![2, 1]));
    }
}
