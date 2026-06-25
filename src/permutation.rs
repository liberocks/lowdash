/// Finds all permutation of a collection.
///
/// # Arguments
/// * `items` - A slice of items to permute
///
/// # Returns
/// * `Vec<Vec<T>>` - A vector containing all permutation of the input items
///
/// # Examples
/// ```rust
/// use lowdash::permutation;
///
/// let items = vec![1, 2, 3];
/// let result = permutation(&items);
/// assert_eq!(result.len(), 6);
/// // Possible permutation: [2, 1, 3]
/// assert!(result.contains(&vec![2, 1, 3]));
/// ```
pub fn permutation<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    fn backtrack<T: Clone>(
        items: &[T],
        buffer: &mut Vec<T>,
        used: &mut Vec<bool>,
        result: &mut Vec<Vec<T>>,
    ) {
        if buffer.len() == items.len() {
            result.push(buffer.clone());
            return;
        }
        for i in 0..items.len() {
            if used[i] {
                continue;
            }
            used[i] = true;
            buffer.push(items[i].clone());
            backtrack(items, buffer, used, result);
            buffer.pop();
            used[i] = false;
        }
    }

    if items.is_empty() {
        return vec![vec![]];
    }

    let n = items.len();
    let mut result = Vec::with_capacity((1..=n).product());
    let mut buffer = Vec::with_capacity(n);
    let mut used = vec![false; n];
    backtrack(items, &mut buffer, &mut used, &mut result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutation_empty() {
        let items: Vec<i32> = vec![];
        let result = permutation(&items);
        assert_eq!(result, vec![vec![]]);
    }

    #[test]
    fn test_permutation_single() {
        let items = vec![42];
        let result = permutation(&items);
        assert_eq!(result, vec![vec![42]]);
    }

    #[test]
    fn test_permutation_multiple() {
        let items = vec![1, 2, 3];
        let result = permutation(&items);
        assert_eq!(result.len(), 6);
        // Check that a known permutation is present.
        assert!(result.contains(&vec![2, 1, 3]));
    }
}
