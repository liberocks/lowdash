/// Returns the items from `collection` that are not in `excluded`.
///
/// The result keeps the input order and keeps repeated items that are not excluded.
///
/// **Time Complexity:** O(n * m), where `n` is the collection length and `m` is the excluded length.
///
/// # Arguments
///
/// * `collection` - The items to filter.
/// * `excluded` - The items to remove.
///
/// # Returns
///
/// * `Vec<T>` - A new vector containing items that are not present in `excluded`.
///
/// # Examples
///
/// ```rust
/// use lowdash::difference;
///
/// let numbers = vec![1, 2, 2, 3, 4];
/// let result = difference(&numbers, &[2, 4]);
///
/// assert_eq!(result, vec![1, 3]);
/// ```
pub fn difference<T>(collection: &[T], excluded: &[T]) -> Vec<T>
where
    T: PartialEq + Clone,
{
    if collection.is_empty() {
        return Vec::new();
    }

    if excluded.is_empty() {
        return collection.to_vec();
    }

    let mut result = Vec::with_capacity(collection.len());
    for item in collection {
        if !excluded.iter().any(|excluded_item| excluded_item == item) {
            result.push(item.clone());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    struct Item {
        id: u32,
        name: &'static str,
    }

    #[test]
    fn removes_excluded_items() {
        let numbers = vec![1, 2, 3, 4, 5];

        assert_eq!(difference(&numbers, &[2, 4]), vec![1, 3, 5]);
    }

    #[test]
    fn preserves_order_and_unexcluded_duplicates() {
        let numbers = vec![3, 1, 3, 2, 1, 4];

        assert_eq!(difference(&numbers, &[2]), vec![3, 1, 3, 1, 4]);
    }

    #[test]
    fn handles_duplicate_excluded_items() {
        let numbers = vec![1, 2, 3, 2];

        assert_eq!(difference(&numbers, &[2, 2]), vec![1, 3]);
    }

    #[test]
    fn works_with_custom_types() {
        let items = vec![
            Item { id: 1, name: "one" },
            Item { id: 2, name: "two" },
            Item {
                id: 3,
                name: "three",
            },
        ];

        let result = difference(&items, &[Item { id: 2, name: "two" }]);

        assert_eq!(
            result,
            vec![
                Item { id: 1, name: "one" },
                Item {
                    id: 3,
                    name: "three",
                },
            ]
        );
    }

    #[test]
    fn handles_empty_inputs() {
        let numbers = vec![1, 2, 3];
        let empty: Vec<i32> = Vec::new();

        assert_eq!(difference(&empty, &numbers), Vec::<i32>::new());
        assert_eq!(difference(&numbers, &empty), numbers);
        assert_eq!(difference(&empty, &empty), Vec::<i32>::new());
    }

    #[test]
    fn does_not_change_input() {
        let numbers = vec![1, 2, 3];

        let _ = difference(&numbers, &[2]);

        assert_eq!(numbers, vec![1, 2, 3]);
    }

    #[test]
    fn supports_floating_point_values() {
        let numbers = vec![1.5, 2.5, 3.5];

        assert_eq!(difference(&numbers, &[2.5]), vec![1.5, 3.5]);
    }
}
