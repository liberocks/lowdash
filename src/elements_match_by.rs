/// Returns `true` when two collections produce the same keys with the same
/// multiplicities, regardless of item order.
///
/// The iteratee is called once per item in each collection. Key equality uses
/// `PartialEq`; duplicate keys are matched one at a time.
///
/// **Time Complexity:** O(n²), where `n` is the collection length.
/// **Space Complexity:** O(n) for keys and match tracking.
///
/// # Examples
/// ```rust
/// use lowdash::elements_match_by;
///
/// let left = [(1, "a"), (2, "b"), (1, "c")];
/// let right = [(1, "x"), (1, "y"), (2, "z")];
///
/// assert!(elements_match_by(&left, &right, |item| item.0));
/// ```
pub fn elements_match_by<T, K, F>(left: &[T], right: &[T], iteratee: F) -> bool
where
    K: PartialEq,
    F: Fn(&T) -> K,
{
    if left.len() != right.len() {
        return false;
    }

    let left_keys: Vec<K> = left.iter().map(&iteratee).collect();
    let right_keys: Vec<K> = right.iter().map(&iteratee).collect();
    let mut matched = vec![false; right_keys.len()];

    for left_key in &left_keys {
        let mut match_index = None;

        for (index, right_key) in right_keys.iter().enumerate() {
            if !matched[index] && left_key == right_key {
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
    use std::cell::Cell;

    #[test]
    fn compares_key_multiplicities_without_item_order() {
        let left = [(1, "a"), (2, "b"), (1, "c")];
        let right = [(1, "x"), (1, "y"), (2, "z")];
        let different = [(1, "x"), (2, "y"), (2, "z")];

        assert!(elements_match_by(&left, &right, |item| item.0));
        assert!(!elements_match_by(&left, &different, |item| item.0));
    }

    #[test]
    fn calls_iteratee_once_per_item_in_each_collection() {
        let calls = Cell::new(0);

        assert!(elements_match_by(&[1, 2], &[2, 1], |value| {
            calls.set(calls.get() + 1);
            *value
        }));
        assert_eq!(calls.get(), 4);
    }

    #[test]
    fn accepts_non_clone_keys() {
        struct Key(u8);

        impl PartialEq for Key {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        let left = [1, 2, 1];
        let right = [1, 1, 2];

        assert!(elements_match_by(&left, &right, |value| Key(*value)));
    }

    #[test]
    fn handles_empty_and_different_length_inputs() {
        assert!(elements_match_by::<i32, i32, _>(&[], &[], |value| *value));
        assert!(!elements_match_by(&[1], &[], |value| *value));
    }

    #[test]
    fn follows_partial_eq_for_nan_keys() {
        assert!(!elements_match_by(&[1], &[1], |_| f64::NAN));
    }
}
