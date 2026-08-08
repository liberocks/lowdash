/// Splits pairs into two vectors while preserving pair order.
///
/// Each component is cloned into its corresponding output vector.
///
/// **Time Complexity:** O(n), where `n` is the number of pairs, with O(n)
/// output space.
///
/// # Examples
/// ```rust
/// use lowdash::unzip;
///
/// let pairs = [("Alice", 30), ("Bob", 25)];
/// assert_eq!(unzip(&pairs), (vec!["Alice", "Bob"], vec![30, 25]));
/// ```
pub fn unzip<T, U>(pairs: &[(T, U)]) -> (Vec<T>, Vec<U>)
where
    T: Clone,
    U: Clone,
{
    let mut left = Vec::with_capacity(pairs.len());
    let mut right = Vec::with_capacity(pairs.len());

    for (first, second) in pairs {
        left.push(first.clone());
        right.push(second.clone());
    }

    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn separates_pairs_in_order() {
        let pairs = [(1, 'a'), (2, 'b'), (3, 'c')];

        assert_eq!(unzip(&pairs), (vec![1, 2, 3], vec!['a', 'b', 'c']));
    }

    #[test]
    fn preserves_duplicate_components() {
        let pairs = [(1, 1), (1, 2), (2, 2)];

        assert_eq!(unzip(&pairs), (vec![1, 1, 2], vec![1, 2, 2]));
    }

    #[test]
    fn handles_empty_input() {
        let pairs: [(i32, char); 0] = [];

        assert_eq!(unzip(&pairs), (Vec::<i32>::new(), Vec::<char>::new()));
    }

    #[test]
    fn works_with_custom_types_and_different_component_types() {
        #[derive(Debug, PartialEq, Clone)]
        struct Person {
            name: &'static str,
        }

        let pairs = [
            (Person { name: "Alice" }, 10_u64),
            (Person { name: "Bob" }, 20),
        ];

        assert_eq!(
            unzip(&pairs),
            (
                vec![Person { name: "Alice" }, Person { name: "Bob" }],
                vec![10, 20]
            )
        );
        assert_eq!(pairs[0].0.name, "Alice");
    }
}
