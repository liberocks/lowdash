use std::collections::HashMap;
use std::hash::Hash;

/// Group elements of a collection based on a key extracted by a provided function,
/// preserving the order of their first occurrence.
///
/// This function takes a slice of items and returns a `HashMap<U, Vec<T>>` where each key
/// corresponds to a group of items that share the same key.
///
/// **Note:** This implementation requires `U` to implement `Hash`, `Eq`, and `Clone`, and `T` to implement `Clone`.
///
/// # Arguments
///
/// * `collection` - A slice of items to be grouped.
/// * `iteratee` - A function that takes a reference to an item and returns a key of type `U`.
///
/// # Type Parameters
///
/// * `T` - The type of elements in the collection. Must implement `Clone`.
/// * `U` - The type of the key extracted from each element used to group the elements. Must implement `Hash`, `Eq`, and `Clone`.
/// * `F` - The type of the iteratee function. Must implement `Fn(&T) -> U`.
///
/// # Returns
///
/// * `HashMap<U, Vec<T>>` - A map where each key is associated with a vector of items that share the same key.
///
/// # Examples
///
/// ```rust
/// use lowdash::group_by;
///
/// let numbers = vec![1, 2, 2, 3, 4, 3, 5];
/// let grouped = group_by(&numbers, |x| *x % 2 == 0);
///
/// assert_eq!(grouped.get(&false), Some(&vec![1, 3, 3, 5]));
/// assert_eq!(grouped.get(&true), Some(&vec![2, 2, 4]));
/// ```
///
/// ```rust
/// use lowdash::group_by;
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
///     Person { name: "Carol".to_string(), age: 25 },
/// ];
///
/// let grouped = group_by(&people, |p| p.age);
///
/// assert_eq!(grouped.get(&25), Some(&vec![
///     Person { name: "Alice".to_string(), age: 25 },
///     Person { name: "Carol".to_string(), age: 25 },
/// ]));
/// assert_eq!(grouped.get(&30), Some(&vec![
///     Person { name: "Bob".to_string(), age: 30 },
/// ]));
/// ```
pub fn group_by<T, U, F>(collection: &[T], iteratee: F) -> HashMap<U, Vec<T>>
where
    T: Clone,
    U: Eq + Hash + Clone,
    F: Fn(&T) -> U,
{
    let mut result: HashMap<U, Vec<T>> = HashMap::new();

    for item in collection {
        let key = iteratee(item);
        result.entry(key).or_default().push(item.clone());
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

    #[test]
    fn test_group_by_integers() {
        let numbers = vec![1, 2, 2, 3, 4, 3, 5];
        let grouped = group_by(&numbers, |x| *x % 2 == 0);
        assert_eq!(grouped.get(&false), Some(&vec![1, 3, 3, 5]));
        assert_eq!(grouped.get(&true), Some(&vec![2, 2, 4]));
    }

    #[test]
    fn test_group_by_strings() {
        let strings = vec!["apple", "banana", "apple", "cherry", "banana"];
        let grouped = group_by(&strings, |s| s.len());
        assert_eq!(grouped.get(&5), Some(&vec!["apple", "apple"]));
        assert_eq!(grouped.get(&6), Some(&vec!["banana", "cherry", "banana"]));
    }

    #[test]
    fn test_group_by_with_structs() {
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
                name: "Carol".to_string(),
                age: 25,
            },
            Person {
                name: "Dave".to_string(),
                age: 30,
            },
        ];

        let grouped = group_by(&people, |p| p.age);
        assert_eq!(
            grouped.get(&25),
            Some(&vec![
                Person {
                    name: "Alice".to_string(),
                    age: 25
                },
                Person {
                    name: "Carol".to_string(),
                    age: 25
                },
            ])
        );
        assert_eq!(
            grouped.get(&30),
            Some(&vec![
                Person {
                    name: "Bob".to_string(),
                    age: 30
                },
                Person {
                    name: "Dave".to_string(),
                    age: 30
                },
            ])
        );
    }

    #[test]
    fn test_group_by_with_empty_collection() {
        let empty: Vec<i32> = vec![];
        let grouped = group_by(&empty, |x| *x);
        assert!(grouped.is_empty());
    }

    #[test]
    fn test_group_by_no_duplicates() {
        let collection = vec![1, 2, 3, 4, 5];
        let grouped = group_by(&collection, |x| *x);
        for x in &collection {
            assert_eq!(grouped.get(x), Some(&vec![*x]));
        }
    }

    #[test]
    fn test_group_by_all_duplicates() {
        let collection = vec![1, 1, 1, 1, 1];
        let grouped = group_by(&collection, |x| *x);
        assert_eq!(grouped.len(), 1);
        assert_eq!(grouped.get(&1), Some(&vec![1, 1, 1, 1, 1]));
    }

    #[test]
    fn test_group_by_with_floats() {
        let float_collection = vec![1.1, 2.2, 2.2, 3.3, 4.4, 3.3, 5.5];
        let grouped = group_by(&float_collection, |x| *x > 3.0);
        assert_eq!(grouped.get(&false), Some(&vec![1.1, 2.2, 2.2]));
        assert_eq!(grouped.get(&true), Some(&vec![3.3, 4.4, 3.3, 5.5]));
    }

    #[test]
    fn test_group_by_with_characters() {
        let chars = vec!['a', 'b', 'a', 'c', 'b', 'd'];
        let grouped = group_by(&chars, |c| *c);
        assert_eq!(grouped.get(&'a'), Some(&vec!['a', 'a']));
        assert_eq!(grouped.get(&'b'), Some(&vec!['b', 'b']));
        assert_eq!(grouped.get(&'c'), Some(&vec!['c']));
        assert_eq!(grouped.get(&'d'), Some(&vec!['d']));
    }

    #[test]
    fn test_group_by_preserves_order() {
        let numbers = vec![3, 1, 2, 3, 2, 4, 1, 5];
        let grouped = group_by(&numbers, |x| *x);
        assert_eq!(grouped.get(&3), Some(&vec![3, 3]));
        assert_eq!(grouped.get(&1), Some(&vec![1, 1]));
        assert_eq!(grouped.get(&2), Some(&vec![2, 2]));
        assert_eq!(grouped.get(&4), Some(&vec![4]));
        assert_eq!(grouped.get(&5), Some(&vec![5]));
    }

    #[test]
    fn test_group_by_with_mixed_types() {
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

        let grouped = group_by(&items, |item| item.id);
        assert_eq!(
            grouped.get(&1),
            Some(&vec![
                Item {
                    id: 1,
                    value: "one".to_string()
                },
                Item {
                    id: 1,
                    value: "one".to_string()
                },
            ])
        );
        assert_eq!(
            grouped.get(&2),
            Some(&vec![Item {
                id: 2,
                value: "two".to_string()
            },])
        );
        assert_eq!(
            grouped.get(&3),
            Some(&vec![Item {
                id: 3,
                value: "three".to_string()
            },])
        );
    }

    #[test]
    fn test_group_by_with_optionals() {
        let collection = vec![Some(1), None, Some(2), Some(1), None, Some(3), Some(2)];
        let grouped = group_by(&collection, |x| x.clone());
        assert_eq!(grouped.get(&Some(1)), Some(&vec![Some(1), Some(1)]));
        assert_eq!(grouped.get(&None), Some(&vec![None, None]));
        assert_eq!(grouped.get(&Some(2)), Some(&vec![Some(2), Some(2)]));
        assert_eq!(grouped.get(&Some(3)), Some(&vec![Some(3)]));
    }

    #[test]
    fn test_group_by_with_nan_floats() {
        let float_collection = vec![std::f64::NAN, 2.2, std::f64::NAN, 4.4];
        let grouped = group_by(&float_collection, |x| x.is_nan());

        // Verify the NaN group
        if let Some(nans) = grouped.get(&true) {
            assert_eq!(nans.len(), 2);
            assert!(nans[0].is_nan());
            assert!(nans[1].is_nan());
        } else {
            panic!("Expected Some(&vec![NaN, NaN]) for key true");
        }

        // Verify the non-NaN group
        assert_eq!(grouped.get(&false), Some(&vec![2.2, 4.4]));
    }
}
