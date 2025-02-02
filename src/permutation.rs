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
/// use lowdash::permutation::permutation;
///
/// let items = vec![1, 2, 3];
/// let result = permutation(&items);
/// assert_eq!(result.len(), 6);
/// // Possible permutation: [2, 1, 3]
/// assert!(result.contains(&vec![2, 1, 3]));
/// ```
///
/// This implementation uses recursion and the standard library only.
pub fn permutation<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    if items.is_empty() {
        return vec![vec![]];
    }
    let mut result = Vec::new();
    for (i, item) in items.iter().enumerate() {
        let mut rest = items.to_vec();
        rest.remove(i);
        for mut perm in permutation(&rest) {
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
