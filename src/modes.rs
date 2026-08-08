use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;

/// Returns the most frequent values in first-appearance order.
///
/// Every value tied for the highest frequency is returned once. An empty
/// collection returns an empty vector.
///
/// **Time Complexity:** O(n) expected for `n` items, with O(n) extra space for
/// counts and first-appearance tracking.
///
/// # Examples
/// ```rust
/// use lowdash::modes;
///
/// assert_eq!(modes(&[1, 2, 2, 3, 3, 4]), vec![2, 3]);
/// ```
pub fn modes<T>(collection: &[T]) -> Vec<T>
where
    T: Eq + Hash + Clone,
{
    let mut counts = HashMap::with_capacity(collection.len());
    let mut order = Vec::with_capacity(collection.len());

    for item in collection {
        match counts.entry(item.clone()) {
            Entry::Occupied(mut entry) => *entry.get_mut() += 1,
            Entry::Vacant(entry) => {
                entry.insert(1);
                order.push(item);
            }
        }
    }

    let max_count = counts.values().copied().max().unwrap_or(0);
    order
        .into_iter()
        .filter(|item| counts.get(*item).copied() == Some(max_count))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_all_most_frequent_values() {
        assert_eq!(modes(&[1, 2, 2, 3, 3, 4]), vec![2, 3]);
    }

    #[test]
    fn keeps_ties_in_first_appearance_order() {
        assert_eq!(modes(&[3, 1, 2, 3, 2, 1]), vec![3, 1, 2]);
    }

    #[test]
    fn handles_empty_and_single_value_inputs() {
        let empty: [i32; 0] = [];

        assert_eq!(modes(&empty), Vec::<i32>::new());
        assert_eq!(modes(&[7]), vec![7]);
        assert_eq!(modes(&[1, 2, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn returns_a_value_once_even_when_it_repeats() {
        assert_eq!(modes(&[4, 4, 4, 2, 2]), vec![4]);
    }

    #[test]
    fn works_with_custom_hashable_types() {
        #[derive(Debug, PartialEq, Eq, Hash, Clone)]
        struct Tag {
            name: &'static str,
        }

        let tags = [
            Tag { name: "red" },
            Tag { name: "blue" },
            Tag { name: "blue" },
            Tag { name: "red" },
            Tag { name: "green" },
        ];

        assert_eq!(
            modes(&tags),
            vec![Tag { name: "red" }, Tag { name: "blue" }]
        );
    }
}
