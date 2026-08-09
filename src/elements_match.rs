/// Returns `true` when two collections contain the same values with the same
/// multiplicities, regardless of order.
///
/// Equality uses `PartialEq`; duplicate values are matched one at a time.
///
/// **Time Complexity:** O(n²), where `n` is the collection length.
/// **Space Complexity:** O(n) for match tracking.
///
/// # Examples
/// ```rust
/// use lowdash::elements_match;
///
/// assert!(elements_match(&[1, 2, 1], &[1, 1, 2]));
/// assert!(!elements_match(&[1, 2], &[1, 1]));
/// ```
pub fn elements_match<T>(left: &[T], right: &[T]) -> bool
where
    T: PartialEq,
{
    if left.len() != right.len() {
        return false;
    }

    let mut matched = vec![false; right.len()];

    for item in left {
        let mut match_index = None;

        for (index, candidate) in right.iter().enumerate() {
            if !matched[index] && item == candidate {
                match_index = Some(index);
                break;
            }
        }

        let Some(index) = match_index else {
            return false;
        };
        matched[index] = true;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignores_order_but_counts_duplicates() {
        assert!(elements_match(&[1, 2, 1], &[1, 1, 2]));
        assert!(!elements_match(&[1, 2], &[1, 1]));
    }

    #[test]
    fn handles_empty_and_different_length_inputs() {
        assert!(elements_match::<i32>(&[], &[]));
        assert!(!elements_match(&[1], &[]));
    }

    #[test]
    fn accepts_non_clone_values() {
        #[derive(Debug, PartialEq)]
        struct Item(u8);

        let left = [Item(1), Item(2), Item(1)];
        let right = [Item(1), Item(1), Item(2)];

        assert!(elements_match(&left, &right));
    }

    #[test]
    fn follows_partial_eq_for_nan() {
        assert!(!elements_match(&[f64::NAN], &[f64::NAN]));
    }
}
