/// Returns the first representative for each generated key across collections.
///
/// Collections and items are visited in order. Repeated keys, including keys
/// repeated within one collection, contribute only their first item.
///
/// **Time Complexity:** O(n²) in the total number of items because keys use
/// `PartialEq` comparisons. The key extractor is called once per item.
///
/// # Examples
/// ```rust
/// use lowdash::union_by;
///
/// let collections = vec![vec![11, 12], vec![2, 13], vec![22]];
/// assert_eq!(union_by(&collections, |number| *number % 10), vec![11, 12]);
/// ```
pub fn union_by<T, K, F, Slice>(collections: &[Slice], iteratee: F) -> Vec<T>
where
    T: Clone,
    K: PartialEq,
    F: Fn(&T) -> K,
    Slice: AsRef<[T]>,
{
    let capacity = collections.iter().fold(0usize, |total, collection| {
        total.saturating_add(collection.as_ref().len())
    });
    let mut seen_keys = Vec::new();
    let mut result = Vec::with_capacity(capacity);

    for collection in collections {
        for item in collection.as_ref() {
            let key = iteratee(item);
            if seen_keys
                .iter()
                .any(|seen_key| PartialEq::eq(seen_key, &key))
            {
                continue;
            }

            seen_keys.push(key);
            result.push(item.clone());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_first_representatives_across_collections() {
        let collections = vec![vec![1, 2, 2], vec![2, 3], vec![3, 4]];

        assert_eq!(union_by(&collections, |number| *number), vec![1, 2, 3, 4]);
    }

    #[test]
    fn preserves_collection_and_item_order() {
        let collections = vec![vec![3, 1, 3], vec![2, 1], vec![4, 2]];

        assert_eq!(union_by(&collections, |number| *number), vec![3, 1, 2, 4]);
    }

    #[test]
    fn returns_the_first_item_for_each_key() {
        #[derive(Debug, Clone, PartialEq)]
        struct Record {
            id: u8,
            label: &'static str,
        }

        let collections = vec![
            vec![Record {
                id: 1,
                label: "first",
            }],
            vec![
                Record {
                    id: 1,
                    label: "later",
                },
                Record {
                    id: 2,
                    label: "two",
                },
            ],
        ];

        assert_eq!(
            union_by(&collections, |record| record.id),
            vec![
                Record {
                    id: 1,
                    label: "first",
                },
                Record {
                    id: 2,
                    label: "two",
                },
            ]
        );
    }

    #[test]
    fn handles_empty_collections() {
        let empty: Vec<i32> = Vec::new();
        let numbers = vec![1, 2, 3];

        assert_eq!(
            union_by::<i32, i32, _, Vec<i32>>(&[], |number| *number),
            Vec::new()
        );
        assert_eq!(
            union_by(&[empty.clone(), numbers.clone()], |number| *number),
            numbers
        );
        assert_eq!(union_by(&[numbers, empty], |number| *number), vec![1, 2, 3]);
    }

    #[test]
    fn supports_custom_non_clone_keys() {
        #[derive(Debug, PartialEq)]
        struct Key(u8);

        #[derive(Debug, Clone, PartialEq)]
        struct Item {
            key: u8,
        }

        let collections = vec![
            vec![Item { key: 1 }],
            vec![Item { key: 1 }, Item { key: 2 }],
        ];

        assert_eq!(
            union_by(&collections, |item| Key(item.key)),
            vec![Item { key: 1 }, Item { key: 2 }]
        );
    }
}
