/// Returns `true` when every item satisfies a predicate.
///
/// The scan stops at the first item for which the predicate returns `false`.
/// An empty collection returns `true`.
///
/// **Time Complexity:** O(n) in the worst case, where `n` is the collection length.
///
/// # Arguments
///
/// * `collection` - The items to check.
/// * `predicate` - A function that checks each item.
///
/// # Returns
///
/// * `bool` - `true` if every item matches the predicate; otherwise `false`.
///
/// # Examples
///
/// ```rust
/// use lowdash::every;
///
/// let numbers = vec![2, 4, 6];
/// assert!(every(&numbers, |number| *number % 2 == 0));
/// ```
pub fn every<T, F>(collection: &[T], mut predicate: F) -> bool
where
    F: FnMut(&T) -> bool,
{
    for item in collection {
        if !predicate(item) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Person {
        age: u32,
    }

    #[test]
    fn returns_true_when_all_items_match() {
        let numbers = vec![2, 4, 6, 8];

        assert!(every(&numbers, |number| *number % 2 == 0));
    }

    #[test]
    fn returns_false_when_an_item_fails() {
        let numbers = vec![2, 4, 5, 8];

        assert!(!every(&numbers, |number| *number % 2 == 0));
    }

    #[test]
    fn returns_true_for_empty_collections() {
        let empty: Vec<i32> = Vec::new();

        assert!(every(&empty, |_| false));
    }

    #[test]
    fn stops_after_the_first_failure() {
        let numbers = vec![2, 4, 5, 6];
        let mut calls = 0;

        let result = every(&numbers, |number| {
            calls += 1;
            *number % 2 == 0
        });

        assert!(!result);
        assert_eq!(calls, 3);
    }

    #[test]
    fn accepts_stateful_predicates() {
        let numbers = vec![1, 2, 3];
        let mut next = 0;

        let result = every(&numbers, |number| {
            next += 1;
            *number == next
        });

        assert!(result);
    }

    #[test]
    fn works_with_custom_types() {
        let people = vec![Person { age: 30 }, Person { age: 42 }];

        assert!(every(&people, |person| person.age >= 18));
        assert!(!every(&people, |person| person.age < 40));
    }
}
