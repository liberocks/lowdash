/// Returns the Cartesian product of two collections in left-major order.
///
/// Every item from `right` is paired with each item from `left` before the
/// next left item is considered. Empty inputs produce an empty result.
///
/// **Time Complexity:** O(n * m), where `n` and `m` are the input lengths. The
/// output uses O(n * m) space; capacity reservation is guarded against integer
/// overflow.
///
/// # Examples
/// ```rust
/// use lowdash::cartesian_product;
///
/// assert_eq!(
///     cartesian_product(&[1, 2], &['a', 'b']),
///     vec![(1, 'a'), (1, 'b'), (2, 'a'), (2, 'b')]
/// );
/// ```
pub fn cartesian_product<T, U>(left: &[T], right: &[U]) -> Vec<(T, U)>
where
    T: Clone,
    U: Clone,
{
    if left.is_empty() || right.is_empty() {
        return Vec::new();
    }

    let mut result = match left.len().checked_mul(right.len()) {
        Some(capacity) => Vec::with_capacity(capacity),
        None => Vec::new(),
    };

    for left_item in left {
        for right_item in right {
            result.push((left_item.clone(), right_item.clone()));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_left_major_order() {
        assert_eq!(
            cartesian_product(&[1, 2], &['a', 'b', 'c']),
            vec![(1, 'a'), (1, 'b'), (1, 'c'), (2, 'a'), (2, 'b'), (2, 'c'),]
        );
    }

    #[test]
    fn handles_empty_inputs() {
        let empty: [i32; 0] = [];

        assert_eq!(cartesian_product(&empty, &[1, 2]), Vec::<(i32, i32)>::new());
        assert_eq!(cartesian_product(&[1, 2], &empty), Vec::<(i32, i32)>::new());
        assert_eq!(cartesian_product(&empty, &empty), Vec::<(i32, i32)>::new());
    }

    #[test]
    fn handles_single_items_and_duplicate_values() {
        assert_eq!(cartesian_product(&[1], &[2]), vec![(1, 2)]);
        assert_eq!(
            cartesian_product(&[1, 1], &[2, 2]),
            vec![(1, 2), (1, 2), (1, 2), (1, 2)]
        );
    }

    #[test]
    fn works_with_custom_types_and_does_not_change_inputs() {
        #[derive(Debug, PartialEq, Clone)]
        struct Point {
            x: i32,
        }

        let points = [Point { x: 1 }, Point { x: 2 }];
        let labels = ["a", "b"];

        assert_eq!(
            cartesian_product(&points, &labels),
            vec![
                (Point { x: 1 }, "a"),
                (Point { x: 1 }, "b"),
                (Point { x: 2 }, "a"),
                (Point { x: 2 }, "b"),
            ]
        );
        assert_eq!(points, [Point { x: 1 }, Point { x: 2 }]);
        assert_eq!(labels, ["a", "b"]);
    }
}
