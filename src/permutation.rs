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
    fn backtrack<T: Clone>(
        items: &[T],
        k: usize,
        buffer: &mut Vec<T>,
        used: &mut Vec<bool>,
        result: &mut Vec<Vec<T>>,
    ) {
        if buffer.len() == k {
            result.push(buffer.clone());
            return;
        }
        for i in 0..items.len() {
            if used[i] {
                continue;
            }
            used[i] = true;
            buffer.push(items[i].clone());
            backtrack(items, k, buffer, used, result);
            buffer.pop();
            used[i] = false;
        }
    }

    if k == 0 {
        return vec![vec![]];
    }
    if k > items.len() {
        return vec![];
    }

    let n = items.len();
    let count = (n - k + 1..=n).product::<usize>();
    let mut result = Vec::with_capacity(count);
    let mut buffer = Vec::with_capacity(k);
    let mut used = vec![false; n];
    backtrack(items, k, &mut buffer, &mut used, &mut result);
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
