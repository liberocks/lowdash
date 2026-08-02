/// Returns the unique values present in every collection.
///
/// Values are returned in the order of their first appearance in the first collection.
/// An empty collection list or an empty input collection returns an empty vector.
///
/// **Time Complexity:** O(n * m), where `n` is the first collection length and `m` is the total
/// length of the other collections.
///
/// # Arguments
///
/// * `collections` - The collections to compare.
///
/// # Returns
///
/// * `Vec<T>` - A new vector containing unique values found in every collection.
///
/// # Examples
///
/// ```rust
/// use lowdash::intersection;
///
/// let collections = vec![vec![1, 2, 2, 3], vec![2, 3, 4], vec![0, 2, 3]];
/// let result = intersection(&collections);
///
/// assert_eq!(result, vec![2, 3]);
/// ```
pub fn intersection<T, Slice>(collections: &[Slice]) -> Vec<T>
where
    T: PartialEq + Clone,
    Slice: AsRef<[T]>,
{
    if collections.is_empty() {
        return Vec::new();
    }

    let first = collections[0].as_ref();
    if first.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();
    for item in first {
        if result.contains(item) {
            continue;
        }

        if collections[1..]
            .iter()
            .all(|collection| collection.as_ref().contains(item))
        {
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
    fn returns_values_present_in_every_collection() {
        let collections = vec![vec![1, 2, 2, 3], vec![2, 3, 4], vec![0, 2, 3]];

        assert_eq!(intersection(&collections), vec![2, 3]);
    }

    #[test]
    fn keeps_first_collection_order() {
        let collections = vec![vec![3, 1, 2, 1], vec![2, 3, 1], vec![1, 2, 3]];

        assert_eq!(intersection(&collections), vec![3, 1, 2]);
    }

    #[test]
    fn returns_empty_when_no_value_is_shared() {
        let collections = vec![vec![1, 2], vec![3, 4]];

        assert_eq!(intersection(&collections), Vec::<i32>::new());
    }

    #[test]
    fn handles_empty_collections() {
        let empty: Vec<i32> = Vec::new();
        let numbers = vec![1, 2, 3];

        assert_eq!(intersection::<i32, Vec<i32>>(&[]), Vec::new());
        assert_eq!(intersection(&[empty.clone(), numbers.clone()]), Vec::new());
        assert_eq!(intersection(&[numbers, empty]), Vec::new());
    }

    #[test]
    fn removes_duplicates_with_one_collection() {
        let collections = vec![vec![3, 1, 3, 2, 1]];

        assert_eq!(intersection(&collections), vec![3, 1, 2]);
    }

    #[test]
    fn works_with_custom_types() {
        let first = vec![Item { id: 1, name: "one" }, Item { id: 2, name: "two" }];
        let second = vec![
            Item { id: 2, name: "two" },
            Item {
                id: 3,
                name: "three",
            },
        ];

        assert_eq!(
            intersection(&[first, second]),
            vec![Item { id: 2, name: "two" }]
        );
    }

    #[test]
    fn supports_floating_point_values() {
        let collections = vec![vec![1.5, 2.5, 3.5], vec![2.5, 4.5]];

        assert_eq!(intersection(&collections), vec![2.5]);
    }
}
