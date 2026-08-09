/// Returns the last item that satisfies `predicate`.
///
/// The collection is scanned from right to left, so the search stops at the first
/// match encountered from the end.
///
/// **Time Complexity:** O(n) in the worst case and O(1) extra space.
///
/// # Examples
/// ```rust
/// use lowdash::find_last;
///
/// let numbers = vec![1, 2, 3, 2, 4];
/// assert_eq!(find_last(&numbers, |number| *number % 2 == 0), Some(&4));
/// ```
pub fn find_last<T, F>(collection: &[T], predicate: F) -> Option<&T>
where
    F: Fn(&T) -> bool,
{
    collection.iter().rev().find(|item| predicate(item))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_the_last_matching_item() {
        let numbers = [1, 2, 3, 2, 4];

        assert_eq!(find_last(&numbers, |number| *number % 2 == 0), Some(&4));
    }

    #[test]
    fn scans_from_the_right() {
        let numbers = [1, 2, 3, 2, 4];

        assert_eq!(find_last(&numbers, |number| *number == 2), Some(&2));
    }

    #[test]
    fn returns_none_without_a_match_or_for_empty_input() {
        let numbers = [1, 3, 5];
        let empty: [i32; 0] = [];

        assert_eq!(find_last(&numbers, |number| *number % 2 == 0), None);
        assert_eq!(find_last(&empty, |_| true), None);
    }

    #[test]
    fn works_with_custom_types() {
        #[derive(Debug, PartialEq)]
        struct Person {
            name: &'static str,
            active: bool,
        }

        let people = [
            Person {
                name: "Alice",
                active: true,
            },
            Person {
                name: "Bob",
                active: false,
            },
            Person {
                name: "Carol",
                active: true,
            },
        ];

        assert_eq!(
            find_last(&people, |person| person.active),
            Some(&Person {
                name: "Carol",
                active: true,
            })
        );
    }
}
