/// Returns items whose generated keys do not occur in `excluded`.
///
/// Input order and duplicates are preserved. The key extractor is called once
/// per excluded item and once per input item; an empty excluded collection
/// returns a clone without invoking it.
///
/// **Time Complexity:** O(n + m), where `n` is the input length and `m` is the
/// excluded length. Extra space is O(m) for excluded keys and O(n) for output.
///
/// # Examples
/// ```rust
/// use lowdash::difference_by;
///
/// let numbers = [11, 12, 13, 12];
/// assert_eq!(difference_by(&numbers, &[2], |number| *number % 10), vec![11, 13]);
/// ```
pub fn difference_by<T, K, F>(collection: &[T], excluded: &[T], iteratee: F) -> Vec<T>
where
    T: Clone,
    K: Eq + std::hash::Hash,
    F: Fn(&T) -> K,
{
    if collection.is_empty() {
        return Vec::new();
    }

    if excluded.is_empty() {
        return collection.to_vec();
    }

    let excluded_keys: std::collections::HashSet<K> =
        excluded.iter().map(|item| iteratee(item)).collect();
    let mut result = Vec::with_capacity(collection.len());

    for item in collection {
        let key = iteratee(item);
        if !excluded_keys.contains(&key) {
            result.push(item.clone());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn removes_items_with_matching_keys() {
        let numbers = [11, 12, 13, 14];

        assert_eq!(
            difference_by(&numbers, &[2], |number| *number % 10),
            vec![11, 13, 14]
        );
    }

    #[test]
    fn preserves_input_order_and_duplicates() {
        let numbers = [31, 12, 31, 13, 12];

        assert_eq!(
            difference_by(&numbers, &[2], |number| *number % 10),
            vec![31, 31, 13]
        );
    }

    #[test]
    fn compares_generated_keys_instead_of_items() {
        #[derive(Debug, Clone, PartialEq)]
        struct Record {
            id: u8,
            label: &'static str,
        }

        let records = [
            Record { id: 1, label: "a" },
            Record { id: 2, label: "b" },
            Record { id: 1, label: "c" },
        ];
        let excluded = [Record {
            id: 2,
            label: "other",
        }];

        let result = difference_by(&records, &excluded, |record| record.id);
        assert_eq!(
            result,
            vec![Record { id: 1, label: "a" }, Record { id: 1, label: "c" },]
        );
    }

    #[test]
    fn does_not_call_the_extractor_for_an_empty_excluded_collection() {
        let calls = Cell::new(0);
        let numbers = [1, 2, 3];

        let result = difference_by(&numbers, &[], |number| {
            calls.set(calls.get() + 1);
            *number
        });

        assert_eq!(result, numbers);
        assert_eq!(calls.get(), 0);
    }

    #[test]
    fn calls_the_extractor_once_for_each_relevant_item() {
        let calls = Cell::new(0);

        let result = difference_by(&[1, 2, 3], &[2, 4], |number| {
            calls.set(calls.get() + 1);
            *number
        });

        assert_eq!(result, vec![1, 3]);
        assert_eq!(calls.get(), 5);
    }

    #[test]
    fn handles_empty_inputs_and_custom_non_clone_keys() {
        #[derive(Debug, PartialEq, Eq, Hash)]
        struct Key(u8);

        #[derive(Debug, Clone, PartialEq)]
        struct Item {
            key: u8,
        }

        let empty: [Item; 0] = [];
        let items = [Item { key: 1 }, Item { key: 2 }];

        assert_eq!(
            difference_by(&empty, &items, |item| Key(item.key)),
            Vec::<Item>::new()
        );
        assert_eq!(
            difference_by(&items, &[], |item| Key(item.key)),
            items.to_vec()
        );
    }
}
