/// Returns `true` when every item produces a distinct key.
///
/// The iteratee is called once per item visited. Key equality uses
/// `PartialEq`, so incomparable keys such as `NaN` follow normal `PartialEq`
/// behavior.
///
/// **Time Complexity:** O(n²), where `n` is the collection length.
/// **Space Complexity:** O(n) for the collected keys.
///
/// # Examples
/// ```rust
/// use lowdash::is_uniq_by;
///
/// let records = [(1, "first"), (2, "second"), (1, "duplicate")];
/// assert!(!is_uniq_by(&records, |record| record.0));
/// ```
pub fn is_uniq_by<T, U, F>(collection: &[T], iteratee: F) -> bool
where
    U: PartialEq,
    F: Fn(&T) -> U,
{
    let mut seen = Vec::with_capacity(collection.len());

    for item in collection {
        let key = iteratee(item);
        if seen.contains(&key) {
            return false;
        }
        seen.push(key);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn returns_true_for_empty_and_unique_keys() {
        assert!(is_uniq_by::<i32, _, _>(&[], |value| *value));
        assert!(is_uniq_by(&["a", "bb", "ccc"], |value| value.len()));
    }

    #[test]
    fn detects_duplicate_keys() {
        let values = [(1, "first"), (2, "second"), (1, "duplicate")];

        assert!(!is_uniq_by(&values, |value| value.0));
    }

    #[test]
    fn calls_iteratee_once_per_visited_item() {
        let calls = Cell::new(0);
        let values = [1, 2, 1, 3];

        assert!(!is_uniq_by(&values, |value| {
            calls.set(calls.get() + 1);
            *value
        }));
        assert_eq!(calls.get(), 3);
    }

    #[test]
    fn accepts_non_clone_items_and_keys() {
        struct Item(u8);
        struct Key(u8);

        impl PartialEq for Key {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        let values = [Item(1), Item(2), Item(1)];

        assert!(!is_uniq_by(&values, |item| Key(item.0)));
    }

    #[test]
    fn follows_partial_eq_for_nan_keys() {
        assert!(is_uniq_by(&[f64::NAN, f64::NAN], |value| *value));
    }
}
