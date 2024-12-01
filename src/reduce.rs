/// Apply a function to each item in a collection, accumulating a single result.
///
/// This function iterates over a collection and applies the provided `accumulator` function
/// to each item along with its index and the current accumulated value. The accumulated
/// value is updated with each iteration based on the result of the `accumulator`.
///
/// # Arguments
/// * `collection` - A slice of items.
/// * `accumulator` - A function that takes the current accumulated value, a reference to an item, and its index,
///                   then returns the new accumulated value.
/// * `initial` - The initial value for the accumulation.
///
/// # Returns
/// * `R` - The final accumulated value after processing all items in the collection.
///
/// # Examples
/// ```rust
/// use lowdash::reduce;
/// let numbers = vec![1, 2, 3, 4, 5];
/// // Sum of all numbers
/// let sum = reduce(&numbers, |acc, x, _| acc + x, 0);
/// assert_eq!(sum, 15);
/// ```
///
/// ```rust
/// use lowdash::reduce;
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
/// // Concatenate all names
/// let all_names = reduce(&people, |acc, person, _| format!("{} {}", acc, person.name), String::new());
/// assert_eq!(all_names.trim(), "Alice Bob Carol");
/// ```
pub fn reduce<T, R, F>(collection: &[T], accumulator: F, initial: R) -> R
where
    F: Fn(R, &T, usize) -> R,
{
    let mut acc = initial;
    for (index, item) in collection.iter().enumerate() {
        acc = accumulator(acc, item, index);
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce_sum() {
        let numbers = vec![1, 2, 3, 4, 5];
        let sum = reduce(&numbers, |acc, x, _| acc + x, 0);
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_reduce_product() {
        let numbers = vec![1, 2, 3, 4];
        let product = reduce(&numbers, |acc, x, _| acc * x, 1);
        assert_eq!(product, 24);
    }

    #[test]
    fn test_reduce_with_index() {
        let numbers = vec![10, 20, 30];
        let result = reduce(&numbers, |acc, x, index| acc + x + (index as i32 * 10), 0);
        // Calculation:
        // acc = 0 + 10 + (0 * 10) = 10
        // acc = 10 + 20 + (1 * 10) = 40
        // acc = 40 + 30 + (2 * 10) = 90
        assert_eq!(result, 90); // Updated expected value
    }

    #[test]
    fn test_reduce_strings_concatenation() {
        let strings = vec!["Hello", " ", "World", "!"];
        let concatenated = reduce(&strings, |acc, s, _| format!("{}{}", acc, s), String::new());
        assert_eq!(concatenated, "Hello World!");
    }

    #[test]
    fn test_reduce_structs() {
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

        // Calculate total age
        let total_age = reduce(&people, |acc, person, _| acc + person.age, 0);
        assert_eq!(total_age, 90);
    }

    #[test]
    fn test_reduce_empty_collection() {
        let empty: Vec<i32> = vec![];
        let result = reduce(&empty, |acc, x, _| acc + x, 100);
        assert_eq!(result, 100);
    }

    #[test]
    fn test_reduce_with_floats() {
        let float_collection = vec![1.5, 2.5, 3.5];
        let sum: f64 = reduce(&float_collection, |acc, x, _| acc + x, 0.0);
        let expected: f64 = 7.5;
        let epsilon = 1e-10;
        assert!((sum - expected).abs() < epsilon);
    }

    #[test]
    fn test_reduce_with_optionals() {
        let numbers = vec![Some(1), None, Some(3), Some(4)];
        let sum = reduce(
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
    fn test_reduce_with_complex_logic() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = reduce(
            &numbers,
            |acc, x, index| {
                if *x % 2 != 0 {
                    acc + (*x * (index + 1) as i32)
                } else {
                    acc
                }
            },
            0,
        );
        // Calculation:
        // index 0: 1 (odd) -> acc + 1*1 = 1
        // index 1: 2 (even) -> acc = 1
        // index 2: 3 (odd) -> acc + 3*3 = 10
        // index 3: 4 (even) -> acc = 10
        // index 4: 5 (odd) -> acc + 5*5 = 35
        assert_eq!(result, 35);
    }

    #[test]
    fn test_reduce_all_included() {
        let numbers = vec![1, 2, 3];
        let sum = reduce(&numbers, |acc, x, _| acc + x, 0);
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_reduce_all_excluded() {
        let numbers = vec![1, 2, 3];
        let result = reduce(&numbers, |acc, _, _| acc, 100);
        assert_eq!(result, 100);
    }
}
