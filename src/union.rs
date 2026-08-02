/// Returns the unique values from all collections.
///
/// Values are returned in the order of their first appearance across the collections.
///
/// **Time Complexity:** O(n²), where `n` is the total number of input values.
///
/// # Arguments
///
/// * `collections` - The collections to combine.
///
/// # Returns
///
/// * `Vec<T>` - A new vector containing each value once.
///
/// # Examples
///
/// ```rust
/// use lowdash::union;
///
/// let collections = vec![vec![1, 2, 2], vec![2, 3], vec![3, 4]];
/// let result = union(&collections);
///
/// assert_eq!(result, vec![1, 2, 3, 4]);
/// ```
pub fn union<T, Slice>(collections: &[Slice]) -> Vec<T>
where
    T: PartialEq + Clone,
    Slice: AsRef<[T]>,
{
    let capacity = collections
        .iter()
        .map(|collection| collection.as_ref().len())
        .sum();
    let mut result = Vec::with_capacity(capacity);

    for collection in collections {
        for item in collection.as_ref() {
            if !result.contains(item) {
                result.push(item.clone());
            }
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
    fn combines_collections_without_duplicates() {
        let collections = vec![vec![1, 2, 2], vec![2, 3], vec![3, 4]];

        assert_eq!(union(&collections), vec![1, 2, 3, 4]);
    }

    #[test]
    fn preserves_first_appearance_order() {
        let collections = vec![vec![3, 1, 3], vec![2, 1], vec![4, 2]];

        assert_eq!(union(&collections), vec![3, 1, 2, 4]);
    }

    #[test]
    fn handles_empty_collections() {
        let numbers = vec![1, 2, 3];
        let empty: Vec<i32> = Vec::new();

        assert_eq!(union::<i32, Vec<i32>>(&[]), Vec::new());
        assert_eq!(union(&[empty.clone(), numbers.clone()]), numbers);
        assert_eq!(union(&[numbers, empty]), vec![1, 2, 3]);
    }

    #[test]
    fn removes_duplicates_from_one_collection() {
        let collections = vec![vec![3, 1, 3, 2, 1]];

        assert_eq!(union(&collections), vec![3, 1, 2]);
    }

    #[test]
    fn works_with_custom_types() {
        let collections = vec![
            vec![Item { id: 1, name: "one" }, Item { id: 2, name: "two" }],
            vec![
                Item { id: 2, name: "two" },
                Item {
                    id: 3,
                    name: "three",
                },
            ],
        ];

        assert_eq!(
            union(&collections),
            vec![
                Item { id: 1, name: "one" },
                Item { id: 2, name: "two" },
                Item {
                    id: 3,
                    name: "three",
                },
            ]
        );
    }

    #[test]
    fn supports_floating_point_values() {
        let collections = vec![vec![1.5, 2.5], vec![2.5, 3.5]];

        assert_eq!(union(&collections), vec![1.5, 2.5, 3.5]);
    }
}
