use std::collections::HashMap;
use std::hash::Hash;

/// Groups items by key while preserving item order and first-seen partition order.
///
/// **Time Complexity:**  
/// O(n), where `n` is the collection length.
///
/// # Arguments
///
/// * `collection` - Items to partition.
/// * `iteratee` - Function returning an item's partition key.
///
/// # Type Parameters
///
/// * `T` - Item type.
/// * `K` - Hashable partition-key type.
/// * `F` - Iteratee type.
///
/// # Returns
///
/// * `Vec<Vec<T>>` - Partitions in first-key-seen order.
///
/// # Examples
///
/// ```rust
/// use lowdash::partition_by;
///
/// let numbers = vec![1, 2, 2, 3, 4, 3, 5];
/// let partitions = partition_by(&numbers, |x| *x);
/// assert_eq!(
///     partitions,
///     vec![vec![1], vec![2, 2], vec![3, 3], vec![4], vec![5]]
/// );
/// ```
///
/// ```rust
/// use lowdash::partition_by;
///
/// #[derive(Debug, PartialEq, Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let people = vec![
///     Person { name: "Alice".to_string(), age: 25 },
///     Person { name: "Bob".to_string(), age: 30 },
///     Person { name: "Alice".to_string(), age: 25 },
///     Person { name: "Carol".to_string(), age: 35 },
/// ];
///
/// let partitions = partition_by(&people, |p| p.age);
/// assert_eq!(
///     partitions,
///     vec![
///         vec![
///             Person { name: "Alice".to_string(), age: 25 },
///             Person { name: "Alice".to_string(), age: 25 },
///         ],
///         vec![
///             Person { name: "Bob".to_string(), age: 30 },
///         ],
///         vec![
///             Person { name: "Carol".to_string(), age: 35 },
///         ],
///     ]
/// );
/// ```
pub fn partition_by<T, K, F>(collection: &[T], iteratee: F) -> Vec<Vec<T>>
where
    T: Clone,
    K: Eq + Hash,
    F: Fn(&T) -> K,
{
    let mut seen: HashMap<K, usize> = HashMap::new();
    let mut result: Vec<Vec<T>> = Vec::new();

    for item in collection {
        let key = iteratee(item);
        if let Some(&index) = seen.get(&key) {
            result[index].push(item.clone());
        } else {
            let index = result.len();
            seen.insert(key, index);
            result.push(vec![item.clone()]);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    struct Person {
        name: String,
        age: u32,
    }

    #[derive(Debug, PartialEq, Clone)]
    struct Item {
        id: u32,
        value: String,
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct NonCloneKey(u32);

    #[test]
    fn test_partition_by_integers() {
        let numbers = vec![1, 2, 2, 3, 4, 3, 5];
        let partitions = partition_by(&numbers, |x| *x);
        assert_eq!(
            partitions,
            vec![vec![1], vec![2, 2], vec![3, 3], vec![4], vec![5]]
        );
    }

    #[test]
    fn test_partition_by_accepts_non_clone_keys() {
        let numbers = vec![1, 2, 1];
        let partitions = partition_by(&numbers, |x| NonCloneKey(*x as u32));

        assert_eq!(partitions, vec![vec![1, 1], vec![2]]);
    }

    #[test]
    fn test_partition_by_strings() {
        let strings = vec!["apple", "banana", "apple", "cherry", "banana"];
        let partitions = partition_by(&strings, |s| s.to_string());
        assert_eq!(
            partitions,
            vec![
                vec!["apple", "apple"],
                vec!["banana", "banana"],
                vec!["cherry"]
            ]
        );
    }

    #[test]
    fn test_partition_by_with_structs() {
        let people = vec![
            Person {
                name: "Alice".to_string(),
                age: 25,
            },
            Person {
                name: "Bob".to_string(),
                age: 30,
            },
            Person {
                name: "Alice".to_string(),
                age: 25,
            },
            Person {
                name: "Carol".to_string(),
                age: 35,
            },
        ];

        let partitions = partition_by(&people, |p| p.age);
        assert_eq!(
            partitions,
            vec![
                vec![
                    Person {
                        name: "Alice".to_string(),
                        age: 25
                    },
                    Person {
                        name: "Alice".to_string(),
                        age: 25
                    },
                ],
                vec![Person {
                    name: "Bob".to_string(),
                    age: 30
                },],
                vec![Person {
                    name: "Carol".to_string(),
                    age: 35
                },],
            ]
        );
    }

    #[test]
    fn test_partition_by_with_empty_collection() {
        let empty: Vec<i32> = vec![];
        let partitions = partition_by(&empty, |x| *x);
        assert_eq!(partitions, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_partition_by_with_no_duplicates() {
        let collection = vec![1, 2, 3, 4, 5];
        let partitions = partition_by(&collection, |x| *x);
        assert_eq!(
            partitions,
            vec![vec![1], vec![2], vec![3], vec![4], vec![5]]
        );
    }

    #[test]
    fn test_partition_by_with_all_duplicates() {
        let collection = vec![1, 1, 1, 1, 1];
        let partitions = partition_by(&collection, |x| *x);
        assert_eq!(partitions, vec![vec![1, 1, 1, 1, 1]]);
    }

    #[test]
    fn test_partition_by_with_floats() {
        let float_collection: Vec<f64> = vec![1.1, 2.2, 2.2, 3.3, 4.4, 3.3, 5.5];
        let partitions = partition_by(&float_collection, |x| x.to_bits());
        assert_eq!(
            partitions,
            vec![
                vec![1.1],
                vec![2.2, 2.2],
                vec![3.3, 3.3],
                vec![4.4],
                vec![5.5]
            ]
        );
    }

    #[test]
    fn test_partition_by_with_characters() {
        let chars = vec!['a', 'b', 'a', 'c', 'b', 'd'];
        let partitions = partition_by(&chars, |c| *c);
        assert_eq!(
            partitions,
            vec![vec!['a', 'a'], vec!['b', 'b'], vec!['c'], vec!['d']]
        );
    }

    #[test]
    fn test_partition_by_preserves_order() {
        let numbers = vec![3, 1, 2, 3, 2, 4, 1, 5];
        let partitions = partition_by(&numbers, |x| *x);
        assert_eq!(
            partitions,
            vec![vec![3, 3], vec![1, 1], vec![2, 2], vec![4], vec![5]]
        );
    }

    #[test]
    fn test_partition_by_with_mixed_types() {
        let items = vec![
            Item {
                id: 1,
                value: "one".to_string(),
            },
            Item {
                id: 2,
                value: "two".to_string(),
            },
            Item {
                id: 1,
                value: "one".to_string(),
            },
            Item {
                id: 3,
                value: "three".to_string(),
            },
        ];

        let partitions = partition_by(&items, |item| item.id);
        assert_eq!(
            partitions,
            vec![
                vec![
                    Item {
                        id: 1,
                        value: "one".to_string()
                    },
                    Item {
                        id: 1,
                        value: "one".to_string()
                    },
                ],
                vec![Item {
                    id: 2,
                    value: "two".to_string()
                },],
                vec![Item {
                    id: 3,
                    value: "three".to_string()
                },],
            ]
        );
    }

    #[test]
    fn test_partition_by_with_optionals() {
        let collection = vec![Some(1), None, Some(2), Some(1), None, Some(3), Some(2)];
        let partitions = partition_by(&collection, |x| x.clone());
        assert_eq!(
            partitions,
            vec![
                vec![Some(1), Some(1)],
                vec![None, None],
                vec![Some(2), Some(2)],
                vec![Some(3)],
            ]
        );
    }

    #[test]
    fn test_partition_by_with_nan_floats() {
        let float_collection = vec![std::f64::NAN, 2.2, std::f64::NAN, 4.4, std::f64::NAN];
        let partitions = partition_by(&float_collection, |x| x.is_nan());

        // Keep NaN and non-NaN values in separate groups.
        assert_eq!(partitions.len(), 2);

        // Check each group.
        let mut nan_partition = false;
        let mut non_nan_partition = false;

        for partition in partitions {
            if partition.iter().all(|x| x.is_nan()) {
                assert!(!nan_partition, "NaN group already exists");
                nan_partition = true;
                assert_eq!(partition.len(), 3);
            } else {
                assert!(!non_nan_partition, "Non-NaN group already exists");
                non_nan_partition = true;
                assert!(partition.iter().all(|x| !x.is_nan()));
                assert_eq!(partition, vec![2.2, 4.4]);
            }
        }

        assert!(nan_partition, "NaN group was not found");
        assert!(non_nan_partition, "Non-NaN group was not found");
    }
}
