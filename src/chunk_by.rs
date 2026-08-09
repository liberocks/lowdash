/// Groups adjacent items when their generated keys are equal.
///
/// A key appearing again after a different key starts a new group. The key
/// extractor is called once for each item.
///
/// **Time Complexity:** O(n), excluding the cost of key extraction and key
/// comparison, with O(n) output space.
///
/// # Examples
/// ```rust
/// use lowdash::chunk_by;
///
/// let numbers = [1, 1, 2, 2, 1];
/// assert_eq!(chunk_by(&numbers, |number| *number), vec![
///     vec![1, 1],
///     vec![2, 2],
///     vec![1],
/// ]);
/// ```
pub fn chunk_by<T, K, F>(collection: &[T], mut iteratee: F) -> Vec<Vec<T>>
where
    T: Clone,
    K: PartialEq,
    F: FnMut(&T) -> K,
{
    let first = match collection.first() {
        Some(first) => first,
        None => return Vec::new(),
    };

    let mut result = Vec::new();
    let mut current_key = iteratee(first);
    let mut current_group = vec![first.clone()];

    for item in &collection[1..] {
        let key = iteratee(item);
        if key == current_key {
            current_group.push(item.clone());
        } else {
            result.push(current_group);
            current_group = vec![item.clone()];
            current_key = key;
        }
    }

    result.push(current_group);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn groups_adjacent_equal_keys_only() {
        let numbers = [1, 1, 2, 2, 1, 1];

        assert_eq!(
            chunk_by(&numbers, |number| *number),
            vec![vec![1, 1], vec![2, 2], vec![1, 1]]
        );
    }

    #[test]
    fn does_not_merge_separated_runs() {
        let numbers = [1, 2, 1];

        assert_eq!(
            chunk_by(&numbers, |number| *number),
            vec![vec![1], vec![2], vec![1]]
        );
    }

    #[test]
    fn calls_the_extractor_once_per_item() {
        let calls = Cell::new(0);

        let groups = chunk_by(&[1, 2, 2], |number| {
            calls.set(calls.get() + 1);
            *number
        });

        assert_eq!(groups, vec![vec![1], vec![2, 2]]);
        assert_eq!(calls.get(), 3);
    }

    #[test]
    fn handles_empty_and_single_item_collections() {
        let empty: [i32; 0] = [];

        assert_eq!(chunk_by(&empty, |number| *number), Vec::<Vec<i32>>::new());
        assert_eq!(chunk_by(&[7], |number| *number), vec![vec![7]]);
    }

    #[test]
    fn supports_custom_non_clone_keys_and_items() {
        #[derive(Debug, PartialEq)]
        struct Kind(u8);

        #[derive(Debug, PartialEq, Clone)]
        struct Item {
            kind: u8,
            name: &'static str,
        }

        let items = [
            Item { kind: 1, name: "a" },
            Item { kind: 1, name: "b" },
            Item { kind: 2, name: "c" },
        ];

        assert_eq!(
            chunk_by(&items, |item| Kind(item.kind)),
            vec![
                vec![Item { kind: 1, name: "a" }, Item { kind: 1, name: "b" },],
                vec![Item { kind: 2, name: "c" }],
            ]
        );
    }
}
