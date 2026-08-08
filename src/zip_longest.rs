/// Pairs items at matching indices until both collections are exhausted.
///
/// The missing side of an uneven pair is represented by `None`.
///
/// **Time Complexity:** O(n), where `n` is the longer collection length, with
/// O(n) output space.
///
/// # Examples
/// ```rust
/// use lowdash::zip_longest;
///
/// assert_eq!(
///     zip_longest(&[1, 2, 3], &['a']),
///     vec![(Some(1), Some('a')), (Some(2), None), (Some(3), None)]
/// );
/// ```
pub fn zip_longest<T, U>(left: &[T], right: &[U]) -> Vec<(Option<T>, Option<U>)>
where
    T: Clone,
    U: Clone,
{
    let length = left.len().max(right.len());

    if left.len() == right.len() {
        let mut result = Vec::with_capacity(length);
        for (left, right) in left.iter().zip(right) {
            result.push((Some(left.clone()), Some(right.clone())));
        }
        return result;
    }

    let mut result = Vec::with_capacity(length);

    for index in 0..length {
        result.push((left.get(index).cloned(), right.get(index).cloned()));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pairs_equal_length_inputs() {
        assert_eq!(
            zip_longest(&[1, 2], &['a', 'b']),
            vec![(Some(1), Some('a')), (Some(2), Some('b'))]
        );
    }

    #[test]
    fn fills_missing_right_items_with_none() {
        assert_eq!(
            zip_longest(&[1, 2, 3], &['a']),
            vec![(Some(1), Some('a')), (Some(2), None), (Some(3), None)]
        );
    }

    #[test]
    fn fills_missing_left_items_with_none() {
        assert_eq!(
            zip_longest(&[1], &['a', 'b', 'c']),
            vec![(Some(1), Some('a')), (None, Some('b')), (None, Some('c'))]
        );
    }

    #[test]
    fn handles_empty_inputs() {
        let empty: [i32; 0] = [];

        assert_eq!(zip_longest(&empty, &['a']), vec![(None, Some('a'))]);
        assert_eq!(zip_longest(&[1], &empty), vec![(Some(1), None)]);
        assert_eq!(
            zip_longest(&empty, &empty),
            Vec::<(Option<i32>, Option<i32>)>::new()
        );
    }

    #[test]
    fn works_with_custom_types_and_does_not_change_inputs() {
        #[derive(Debug, PartialEq, Clone)]
        struct Person {
            name: &'static str,
        }

        let people = [Person { name: "Alice" }, Person { name: "Bob" }];
        let ids = [10_u64];

        assert_eq!(
            zip_longest(&people, &ids),
            vec![
                (Some(Person { name: "Alice" }), Some(10)),
                (Some(Person { name: "Bob" }), None),
            ]
        );
        assert_eq!(people, [Person { name: "Alice" }, Person { name: "Bob" }]);
        assert_eq!(ids, [10]);
    }
}
