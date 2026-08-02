/// Returns the longest suffix whose items satisfy a predicate.
///
/// The scan starts at the end and stops at the first item for which the predicate returns `false`.
/// The returned items keep their original order.
///
/// **Time Complexity:** O(n), where `n` is the number of checked items.
///
/// # Arguments
///
/// * `collection` - The items to scan.
/// * `predicate` - A function that decides whether an item belongs in the suffix.
///
/// # Returns
///
/// A new vector containing the matching suffix.
///
/// # Examples
///
/// ```rust
/// use lowdash::take_right_while;
///
/// let numbers = vec![1, 2, 3, 4, 5];
/// let result = take_right_while(&numbers, |number| *number > 2);
///
/// assert_eq!(result, vec![3, 4, 5]);
/// ```
pub fn take_right_while<T, F>(collection: &[T], mut predicate: F) -> Vec<T>
where
    T: Clone,
    F: FnMut(&T) -> bool,
{
    let mut start = collection.len();
    while start > 0 && predicate(&collection[start - 1]) {
        start -= 1;
    }

    collection[start..].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    struct Person {
        name: &'static str,
        age: u32,
    }

    #[test]
    fn returns_the_matching_suffix() {
        let numbers = vec![1, 2, 3, 4, 5];

        assert_eq!(
            take_right_while(&numbers, |number| *number > 2),
            vec![3, 4, 5]
        );
    }

    #[test]
    fn returns_all_items_when_the_predicate_always_matches() {
        let numbers = vec![1, 2, 3];

        assert_eq!(take_right_while(&numbers, |_| true), numbers);
    }

    #[test]
    fn returns_no_items_when_the_last_item_fails() {
        let numbers = vec![1, 2, 3];

        assert_eq!(take_right_while(&numbers, |_| false), Vec::<i32>::new());
    }

    #[test]
    fn handles_empty_collections() {
        let empty: Vec<i32> = Vec::new();

        assert_eq!(take_right_while(&empty, |_| true), Vec::<i32>::new());
    }

    #[test]
    fn stops_after_the_first_failure_from_the_right() {
        let numbers = vec![1, 2, 3, 4];
        let mut calls = 0;

        let result = take_right_while(&numbers, |number| {
            calls += 1;
            *number > 2
        });

        assert_eq!(result, vec![3, 4]);
        assert_eq!(calls, 3);
    }

    #[test]
    fn works_with_custom_types() {
        let people = vec![
            Person {
                name: "Alice",
                age: 20,
            },
            Person {
                name: "Bob",
                age: 25,
            },
            Person {
                name: "Carol",
                age: 30,
            },
        ];

        assert_eq!(
            take_right_while(&people, |person| person.age >= 25),
            vec![
                Person {
                    name: "Bob",
                    age: 25,
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
        let numbers = vec![1, 2, 3];

        let _ = take_right_while(&numbers, |number| *number > 2);

        assert_eq!(numbers, vec![1, 2, 3]);
    }
}
