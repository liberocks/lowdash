/// Apply a function to each item in a collection, accumulating a single result from right to left.
///
/// This function iterates over a collection from the end to the start and applies the provided
/// `accumulator` function to each item along with its index and the current accumulated value.
/// The accumulated value is updated with each iteration based on the result of the `accumulator`.
///
/// # Arguments
/// * `collection` - A slice of items.
/// * `accumulator` - A function that takes the current accumulated value, a reference to an item, and its index,
///                   then returns the new accumulated value.
/// * `initial` - The initial value for the accumulation.
///
/// # Returns
/// * `R` - The final accumulated value after processing all items in the collection from right to left.
///
/// # Examples
/// ```rust
/// use lowdash::reduce_right;
/// let numbers = vec![1, 2, 3, 4, 5];
/// // Sum of all numbers using reduce_right
/// let sum = reduce_right(&numbers, |acc, x, _| acc + x, 0);
/// assert_eq!(sum, 15);
/// ```
///
/// ```rust
/// use lowdash::reduce_right;
///
/// #[derive(Debug, PartialEq)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let people = vec![
///     Person { name: "Alice".to_string(), age: 25 },
///     Person { name: "Bob".to_string(), age: 30 },
///     Person { name: "Carol".to_string(), age: 35 },
/// ];
///
/// // Concatenate all names in reverse order
/// let all_names = reduce_right(&people, |acc, person, _| {
///     if acc.is_empty() {
///         person.name.clone()
///     } else {
///         format!("{} {}", acc, person.name)
///     }
/// }, String::new());
/// assert_eq!(all_names, "Carol Bob Alice");
/// ```
pub fn reduce_right<T, R, F>(collection: &[T], accumulator: F, initial: R) -> R
where
    F: Fn(R, &T, usize) -> R,
{
    let mut acc = initial;
    // Iterate over indices in reverse order
    for i in (0..collection.len()).rev() {
        let item = &collection[i];
        acc = accumulator(acc, item, i);
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce_right_sum() {
        let numbers = vec![1, 2, 3, 4, 5];
        let sum = reduce_right(&numbers, |acc, x, _| acc + x, 0);
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_reduce_right_product() {
        let numbers = vec![1, 2, 3, 4];
        let product = reduce_right(&numbers, |acc, x, _| acc * x, 1);
        assert_eq!(product, 24);
    }

    #[test]
    fn test_reduce_right_with_index() {
        let numbers = vec![10, 20, 30];
        let result = reduce_right(&numbers, |acc, x, index| acc + x + ((index as i32) * 5), 0);
        // Calculation from right to left:
        // index 2: 0 + 30 + (2 * 5) = 40
        // index 1: 40 + 20 + (1 * 5) = 65
        // index 0: 65 + 10 + (0 * 5) = 75
        assert_eq!(result, 75);
    }

    #[test]
    fn test_reduce_right_strings_concatenation() {
        let names = vec!["Alice", "Bob", "Carol"];
        let concatenated = reduce_right(
            &names,
            |acc, name, _| {
                if acc.is_empty() {
                    name.to_string()
                } else {
                    format!("{} {}", acc, name)
                }
            },
            String::new(),
        );
        assert_eq!(concatenated, "Carol Bob Alice");
    }

    #[test]
    fn test_reduce_right_structs() {
        #[derive(Debug, PartialEq)]
        struct Person {
            name: String,
            age: u32,
        }

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
                age: 35,
            },
        ];

        // Calculate total age using reduce_right
        let total_age = reduce_right(&people, |acc, person, _| acc + person.age, 0);
        assert_eq!(total_age, 90);
    }

    #[test]
    fn test_reduce_right_empty_collection() {
        let empty: Vec<i32> = vec![];
        let result = reduce_right(&empty, |acc, x, _| acc + x, 100);
        assert_eq!(result, 100);
    }

    #[test]
    fn test_reduce_right_with_floats() {
        let float_collection = vec![1.5, 2.5, 3.5];
        let sum: f64 = reduce_right(&float_collection, |acc, x, _| acc + x, 0.0);
        let expected: f64 = 7.5;
        let epsilon = 1e-10;
        assert!((sum - expected).abs() < epsilon);
    }

    #[test]
    fn test_reduce_right_with_optionals() {
        let numbers = vec![Some(1), None, Some(3), Some(4)];
        let sum = reduce_right(
            &numbers,
            |acc, x, _| {
                if let Some(n) = x {
                    acc + n
                } else {
                    acc
                }
            },
            0,
        );
        assert_eq!(sum, 8);
    }

    #[test]
    fn test_reduce_right_with_complex_logic() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = reduce_right(
            &numbers,
            |acc, x, index| {
                if *x % 2 != 0 {
                    acc + (*x * (index as i32 + 1))
                } else {
                    acc
                }
            },
            0,
        );
        // Calculation from right to left:
        // index 4: 0 + 5 * (4 + 1) = 25
        // index 3: 25 + 4 * (3 + 1) = 25 (since 4 is even)
        // index 2: 25 + 3 * (2 + 1) = 34
        // index 1: 34 + 2 * (1 + 1) = 34 (since 2 is even)
        // index 0: 34 + 1 * (0 + 1) = 35
        assert_eq!(result, 35);
    }

    #[test]
    fn test_reduce_right_all_included() {
        let numbers = vec![1, 2, 3];
        let sum = reduce_right(&numbers, |acc, x, _| acc + x, 0);
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_reduce_right_all_excluded() {
        let numbers = vec![1, 2, 3];
        let result = reduce_right(&numbers, |acc, _, _| acc, 100);
        assert_eq!(result, 100);
    }
}
