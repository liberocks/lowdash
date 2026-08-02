/// Returns `true` when at least one item satisfies a predicate.
///
/// The scan stops at the first item for which the predicate returns `true`.
/// An empty collection returns `false`.
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
/// * `bool` - `true` if at least one item matches the predicate; otherwise `false`.
///
/// # Examples
///
/// ```rust
/// use lowdash::some;
///
/// let numbers = vec![1, 3, 4, 7];
/// assert!(some(&numbers, |number| *number % 2 == 0));
/// ```
pub fn some<T, F>(collection: &[T], mut predicate: F) -> bool
where
    F: FnMut(&T) -> bool,
{
    for item in collection {
        if predicate(item) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Person {
        age: u32,
    }

    #[test]
    fn returns_true_when_an_item_matches() {
        let numbers = vec![1, 3, 4, 7];

        assert!(some(&numbers, |number| *number % 2 == 0));
    }

    #[test]
    fn returns_false_when_no_item_matches() {
        let numbers = vec![1, 3, 5, 7];

        assert!(!some(&numbers, |number| *number % 2 == 0));
    }

    #[test]
    fn returns_false_for_empty_collections() {
        let empty: Vec<i32> = Vec::new();

        assert!(!some(&empty, |_| true));
    }

    #[test]
    fn stops_after_the_first_match() {
        let numbers = vec![1, 3, 4, 6];
        let mut calls = 0;

        let result = some(&numbers, |number| {
            calls += 1;
            *number % 2 == 0
        });

        assert!(result);
        assert_eq!(calls, 3);
    }

    #[test]
    fn accepts_stateful_predicates() {
        let numbers = vec![1, 2, 3];
        let mut calls = 0;

        let result = some(&numbers, |_| {
            calls += 1;
            calls == 3
        });

        assert!(result);
        assert_eq!(calls, 3);
    }

    #[test]
    fn works_with_custom_types() {
        let people = vec![Person { age: 16 }, Person { age: 21 }];

        assert!(some(&people, |person| person.age >= 18));
        assert!(!some(&people, |person| person.age < 10));
    }
}
