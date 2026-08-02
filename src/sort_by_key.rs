/// Returns a stable, ascending sort of a collection using a key function.
///
/// The input collection is not changed. Items with equal keys keep their original order.
///
/// **Time Complexity:** O(n log n), where `n` is the collection length.
///
/// # Arguments
///
/// * `collection` - The items to sort.
/// * `iteratee` - A function that returns a sort key for each item.
///
/// # Returns
///
/// A new vector containing the sorted items.
///
/// # Examples
///
/// ```rust
/// use lowdash::sort_by_key;
///
/// let records = vec![(3, "c"), (1, "a"), (2, "b")];
/// let sorted = sort_by_key(&records, |record| record.0);
///
/// assert_eq!(sorted, vec![(1, "a"), (2, "b"), (3, "c")]);
/// assert_eq!(records, vec![(3, "c"), (1, "a"), (2, "b")]);
/// ```
pub fn sort_by_key<T, K, F>(collection: &[T], iteratee: F) -> Vec<T>
where
    T: Clone,
    K: Ord,
    F: FnMut(&T) -> K,
{
    let mut result = collection.to_vec();
    result.sort_by_cached_key(iteratee);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone)]
    struct Person {
        name: &'static str,
        age: u32,
    }

    #[test]
    fn sorts_in_ascending_key_order() {
        let numbers = vec![5, 1, 4, 2, 3];

        assert_eq!(sort_by_key(&numbers, |number| *number), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn preserves_order_for_equal_keys() {
        let people = vec![
            Person {
                name: "Alice",
                age: 30,
            },
            Person {
                name: "Bob",
                age: 20,
            },
            Person {
                name: "Carol",
                age: 30,
            },
        ];

        assert_eq!(
            sort_by_key(&people, |person| person.age),
            vec![
                Person {
                    name: "Bob",
                    age: 20,
                },
                Person {
                    name: "Alice",
                    age: 30,
                },
                Person {
                    name: "Carol",
                    age: 30,
                },
            ]
        );
    }

    #[test]
    fn does_not_change_input() {
        let numbers = vec![3, 1, 2];

        let _ = sort_by_key(&numbers, |number| *number);

        assert_eq!(numbers, vec![3, 1, 2]);
    }

    #[test]
    fn handles_empty_and_single_item_collections() {
        let empty: Vec<i32> = Vec::new();
        assert_eq!(sort_by_key(&empty, |number| *number), Vec::<i32>::new());

        let single = vec![7];
        assert_eq!(sort_by_key(&single, |number| *number), vec![7]);
    }

    #[test]
    fn accepts_stateful_key_functions() {
        let numbers = vec![3, 1, 2];
        let mut calls = 0;

        let sorted = sort_by_key(&numbers, |number| {
            calls += 1;
            *number
        });

        assert_eq!(sorted, vec![1, 2, 3]);
        assert!(calls > 0);
    }
}
