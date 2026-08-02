use std::cmp::Ordering;

/// Returns a stable sort of a collection using a comparison function.
///
/// The input collection is not changed. Return `Ordering::Less` when the first item belongs first.
///
/// **Time Complexity:** O(n log n), where `n` is the collection length.
///
/// # Arguments
///
/// * `collection` - The items to sort.
/// * `compare` - A function that compares two items.
///
/// # Returns
///
/// A new vector containing the sorted items.
///
/// # Examples
///
/// ```rust
/// use lowdash::sort_by;
///
/// let numbers = vec![3, 1, 2];
/// let sorted = sort_by(&numbers, |left, right| left.cmp(right));
///
/// assert_eq!(sorted, vec![1, 2, 3]);
/// ```
pub fn sort_by<T, F>(collection: &[T], compare: F) -> Vec<T>
where
    T: Clone,
    F: FnMut(&T, &T) -> Ordering,
{
    let mut result = collection.to_vec();
    result.sort_by(compare);
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
    fn sorts_with_a_custom_comparator() {
        let numbers = vec![5, 1, 4, 2, 3];

        assert_eq!(
            sort_by(&numbers, |left, right| left.cmp(right)),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn supports_descending_order() {
        let numbers = vec![1, 3, 2];

        assert_eq!(
            sort_by(&numbers, |left, right| right.cmp(left)),
            vec![3, 2, 1]
        );
    }

    #[test]
    fn preserves_order_for_equal_items() {
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
            sort_by(&people, |left, right| left.age.cmp(&right.age)),
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

        let _ = sort_by(&numbers, |left, right| left.cmp(right));

        assert_eq!(numbers, vec![3, 1, 2]);
    }

    #[test]
    fn handles_empty_and_single_item_collections() {
        let empty: Vec<i32> = Vec::new();
        assert_eq!(
            sort_by(&empty, |left, right| left.cmp(right)),
            Vec::<i32>::new()
        );

        let single = vec![7];
        assert_eq!(sort_by(&single, |left, right| left.cmp(right)), vec![7]);
    }

    #[test]
    fn accepts_stateful_comparators() {
        let numbers = vec![3, 1, 2];
        let mut calls = 0;

        let sorted = sort_by(&numbers, |left, right| {
            calls += 1;
            left.cmp(right)
        });

        assert_eq!(sorted, vec![1, 2, 3]);
        assert!(calls > 0);
    }
}
