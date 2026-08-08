/// Returns every intermediate accumulator produced by a left-to-right scan.
///
/// The initial value is used for the first step but is not included in the
/// output. The callback receives the current accumulator, item, and index.
///
/// **Time Complexity:** O(n), plus the cost of the callback and accumulator
/// cloning. The output uses O(n) space.
///
/// # Examples
/// ```rust
/// use lowdash::scan;
///
/// let numbers = [1, 2, 3];
/// assert_eq!(scan(&numbers, |sum, number, _| sum + number, 0), vec![1, 3, 6]);
/// ```
pub fn scan<T, R, F>(collection: &[T], mut accumulator: F, initial: R) -> Vec<R>
where
    R: Clone,
    F: FnMut(R, &T, usize) -> R,
{
    let mut current = initial;
    let mut result = Vec::with_capacity(collection.len());

    for (index, item) in collection.iter().enumerate() {
        current = accumulator(current, item, index);
        result.push(current.clone());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_each_intermediate_accumulator() {
        let numbers = [1, 2, 3, 4];

        assert_eq!(
            scan(&numbers, |sum, number, _| sum + number, 0),
            vec![1, 3, 6, 10]
        );
    }

    #[test]
    fn does_not_include_the_initial_value() {
        assert_eq!(scan(&[5], |sum, number, _| sum + number, 10), vec![15]);
    }

    #[test]
    fn passes_zero_based_indices_to_the_callback() {
        let numbers = [10, 20, 30];

        assert_eq!(
            scan(
                &numbers,
                |sum, number, index| { sum + number + index as i32 },
                0
            ),
            vec![10, 31, 63]
        );
    }

    #[test]
    fn returns_empty_for_empty_input() {
        let empty: [i32; 0] = [];

        assert_eq!(scan(&empty, |sum, number, _| sum + number, 42), Vec::new());
    }

    #[test]
    fn works_with_custom_accumulators_and_items() {
        #[derive(Debug, PartialEq, Clone)]
        struct Total {
            count: usize,
            sum: i32,
        }

        #[derive(Debug)]
        struct Item {
            value: i32,
        }

        let items = [Item { value: 2 }, Item { value: 5 }];
        let initial = Total { count: 0, sum: 0 };
        let result = scan(
            &items,
            |mut total, item, _| {
                total.count += 1;
                total.sum += item.value;
                total
            },
            initial,
        );

        assert_eq!(
            result,
            vec![Total { count: 1, sum: 2 }, Total { count: 2, sum: 7 },]
        );
    }
}
