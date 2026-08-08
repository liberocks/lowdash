/// Returns unique values that occur in exactly one of the two collections.
///
/// Values are emitted in stable first-appearance order while scanning the left
/// collection and then the right collection. Repeated values within one input
/// produce one output item; duplicates do not cancel one another. Membership
/// and uniqueness use `PartialEq` semantics.
///
/// **Time Complexity:** O((n + m)²), where `n` and `m` are the input lengths.
/// Extra space is O(n + m) for the result.
///
/// # Examples
/// ```rust
/// use lowdash::symmetric_difference;
///
/// assert_eq!(
///     symmetric_difference(&[1, 2, 2, 4], &[2, 3, 3, 4]),
///     vec![1, 3]
/// );
/// ```
pub fn symmetric_difference<T>(left: &[T], right: &[T]) -> Vec<T>
where
    T: PartialEq + Clone,
{
    let mut result = Vec::new();

    for item in left.iter().chain(right) {
        if result.contains(item) {
            continue;
        }

        let in_left = left.contains(item);
        let in_right = right.contains(item);
        if in_left != in_right {
            result.push(item.clone());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_values_present_in_exactly_one_input() {
        assert_eq!(
            symmetric_difference(&[1, 2, 2, 4], &[2, 3, 3, 4]),
            vec![1, 3]
        );
    }

    #[test]
    fn preserves_stable_first_appearance_order() {
        assert_eq!(
            symmetric_difference(&[3, 1, 2], &[2, 4, 3, 5]),
            vec![1, 4, 5]
        );
    }

    #[test]
    fn emits_each_exclusive_duplicate_once() {
        assert_eq!(symmetric_difference(&[1, 1, 1], &[2, 2]), vec![1, 2]);
        assert_eq!(symmetric_difference(&[1, 1], &[1]), Vec::<i32>::new());
    }

    #[test]
    fn handles_empty_inputs() {
        let empty: [i32; 0] = [];

        assert_eq!(symmetric_difference(&empty, &[1, 2]), vec![1, 2]);
        assert_eq!(symmetric_difference(&[1, 2], &empty), vec![1, 2]);
        assert_eq!(symmetric_difference(&empty, &empty), Vec::new());
    }

    #[test]
    fn works_with_custom_types() {
        #[derive(Debug, PartialEq, Clone)]
        struct Item {
            id: u8,
            label: &'static str,
        }

        let left = [
            Item {
                id: 1,
                label: "one",
            },
            Item {
                id: 2,
                label: "left",
            },
        ];
        let right = [
            Item {
                id: 2,
                label: "left",
            },
            Item {
                id: 3,
                label: "three",
            },
        ];

        assert_eq!(
            symmetric_difference(&left, &right),
            vec![
                Item {
                    id: 1,
                    label: "one"
                },
                Item {
                    id: 3,
                    label: "three"
                }
            ]
        );
    }
}
