/// Returns the first item for each key present in every collection.
///
/// Keys are compared with `PartialEq`. Representatives follow the first
/// collection's order, and repeated keys in that collection produce one item.
/// A single collection therefore behaves like key-based deduplication.
///
/// **Time Complexity:** O(n * m), where `n` is the first collection length and
/// `m` is the total length of the other collections. Extra space is O(n).
///
/// # Examples
/// ```rust
/// use lowdash::intersection_by;
///
/// let collections = vec![vec![11, 12, 13], vec![2, 4], vec![22, 24]];
/// assert_eq!(intersection_by(&collections, |number| *number % 10), vec![12]);
/// ```
pub fn intersection_by<T, K, F, Slice>(collections: &[Slice], iteratee: F) -> Vec<T>
where
    T: Clone,
    K: PartialEq,
    F: Fn(&T) -> K,
    Slice: AsRef<[T]>,
{
    let Some(first) = collections.first().map(AsRef::as_ref) else {
        return Vec::new();
    };

    if first.is_empty() {
        return Vec::new();
    }

    let mut seen_keys = Vec::new();
    let mut result = Vec::new();

    for item in first {
        let key = iteratee(item);
        if seen_keys
            .iter()
            .any(|seen_key| PartialEq::eq(seen_key, &key))
        {
            continue;
        }

        let present_in_all = collections[1..].iter().all(|collection| {
            collection.as_ref().iter().any(|candidate| {
                let candidate_key = iteratee(candidate);
                PartialEq::eq(&candidate_key, &key)
            })
        });

        seen_keys.push(key);
        if present_in_all {
            result.push(item.clone());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_unique_matching_keys_in_first_collection_order() {
        let collections = vec![vec![1, 2, 2, 3], vec![2, 3, 4], vec![0, 2, 3]];

        assert_eq!(intersection_by(&collections, |number| *number), vec![2, 3]);
    }

    #[test]
    fn uses_the_first_item_as_the_representative() {
        #[derive(Debug, Clone, PartialEq)]
        struct Record {
            id: u8,
            label: &'static str,
        }

        let first = vec![
            Record {
                id: 1,
                label: "first",
            },
            Record {
                id: 1,
                label: "later",
            },
            Record {
                id: 2,
                label: "two",
            },
        ];
        let second = vec![Record {
            id: 1,
            label: "other",
        }];

        let result = intersection_by(&[first, second], |record| record.id);
        assert_eq!(
            result,
            vec![Record {
                id: 1,
                label: "first",
            }]
        );
    }

    #[test]
    fn handles_one_collection_and_nonmatching_keys() {
        let numbers = vec![3, 1, 3, 2, 1];

        assert_eq!(intersection_by(&[numbers], |number| *number), vec![3, 1, 2]);
        assert_eq!(
            intersection_by(&[vec![1, 2], vec![3, 4]], |number| *number),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn handles_empty_collection_lists_and_inputs() {
        let empty: Vec<i32> = Vec::new();
        let numbers = vec![1, 2, 3];

        assert_eq!(
            intersection_by::<i32, i32, _, Vec<i32>>(&[], |number| *number),
            Vec::new()
        );
        assert_eq!(
            intersection_by(&[empty.clone(), numbers.clone()], |number| *number),
            Vec::new()
        );
        assert_eq!(
            intersection_by(&[numbers, empty], |number| *number),
            Vec::new()
        );
    }

    #[test]
    fn supports_custom_non_clone_keys() {
        #[derive(Debug, PartialEq)]
        struct Key(u8);

        #[derive(Debug, Clone, PartialEq)]
        struct Item {
            key: u8,
        }

        let first = vec![Item { key: 1 }, Item { key: 2 }];
        let second = vec![Item { key: 2 }, Item { key: 3 }];

        assert_eq!(
            intersection_by(&[first, second], |item| Key(item.key)),
            vec![Item { key: 2 }]
        );
    }
}
