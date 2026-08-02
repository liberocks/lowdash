/// Pairs items from two collections at matching indices.
///
/// The result stops when either collection ends. The input collections are not changed.
///
/// **Time Complexity:** O(n), where `n` is the shorter collection length.
///
/// # Arguments
///
/// * `left` - The first collection.
/// * `right` - The second collection.
///
/// # Returns
///
/// A new vector of pairs from matching positions.
///
/// # Examples
///
/// ```rust
/// use lowdash::zip;
///
/// let names = vec!["Alice", "Bob"];
/// let ages = vec![30, 25];
/// let result = zip(&names, &ages);
///
/// assert_eq!(result, vec![("Alice", 30), ("Bob", 25)]);
/// ```
pub fn zip<T, U>(left: &[T], right: &[U]) -> Vec<(T, U)>
where
    T: Clone,
    U: Clone,
{
    let length = left.len().min(right.len());
    let mut result = Vec::with_capacity(length);

    for index in 0..length {
        result.push((left[index].clone(), right[index].clone()));
    }

    result
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
    fn pairs_items_at_matching_indices() {
        let numbers = vec![1, 2, 3];
        let letters = vec!['a', 'b', 'c'];

        assert_eq!(zip(&numbers, &letters), vec![(1, 'a'), (2, 'b'), (3, 'c')]);
    }

    #[test]
    fn truncates_when_left_is_longer() {
        let numbers = vec![1, 2, 3];
        let letters = vec!['a', 'b'];

        assert_eq!(zip(&numbers, &letters), vec![(1, 'a'), (2, 'b')]);
    }

    #[test]
    fn truncates_when_right_is_longer() {
        let numbers = vec![1, 2];
        let letters = vec!['a', 'b', 'c'];

        assert_eq!(zip(&numbers, &letters), vec![(1, 'a'), (2, 'b')]);
    }

    #[test]
    fn handles_empty_inputs() {
        let numbers = vec![1, 2];
        let empty: Vec<char> = Vec::new();

        assert_eq!(zip(&numbers, &empty), Vec::<(i32, char)>::new());
        assert_eq!(zip(&empty, &numbers), Vec::<(char, i32)>::new());
        assert_eq!(zip(&empty, &empty), Vec::<(char, char)>::new());
    }

    #[test]
    fn works_with_custom_types_and_different_types() {
        let people = vec![
            Person {
                name: "Alice",
                age: 30,
            },
            Person {
                name: "Bob",
                age: 25,
            },
        ];
        let ids = vec![1_u64, 2_u64];

        assert_eq!(
            zip(&people, &ids),
            vec![
                (
                    Person {
                        name: "Alice",
                        age: 30,
                    },
                    1,
                ),
                (
                    Person {
                        name: "Bob",
                        age: 25,
                    },
                    2,
                ),
            ]
        );
    }

    #[test]
    fn preserves_input_order_and_does_not_change_inputs() {
        let left = vec![3, 1, 2];
        let right = vec![30, 10, 20];

        assert_eq!(zip(&left, &right), vec![(3, 30), (1, 10), (2, 20)]);
        assert_eq!(left, vec![3, 1, 2]);
        assert_eq!(right, vec![30, 10, 20]);
    }
}
