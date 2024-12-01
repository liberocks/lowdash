/// Apply a function to each item in a collection, producing a new collection of results.
///
/// This function iterates over a collection and applies the provided `iteratee` function
/// to each item along with its index, collecting the results into a new vector.
///
/// # Arguments
/// * `collection` - A slice of items.
/// * `iteratee` - A function that takes a reference to an item and its index, returning a new value.
///
/// # Returns
/// * `Vec<R>` - A vector containing the results of applying `iteratee` to each item.
///
/// # Examples
/// ```rust
/// use lowdash::map;
/// let numbers = vec![1, 2, 3, 4, 5];
/// let result = map(&numbers, |x, _| x * 2);
/// assert_eq!(result, vec![2, 4, 6, 8, 10]);
/// ```
///
/// ```rust
/// use lowdash::map;
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
/// let names: Vec<String> = map(&people, |p, _| p.name.clone());
/// assert_eq!(names, vec!["Alice", "Bob", "Carol"]);
/// ```
pub fn map<T, R, F>(collection: &[T], iteratee: F) -> Vec<R>
where
    F: Fn(&T, usize) -> R,
{
    collection
        .iter()
        .enumerate()
        .map(|(index, item)| iteratee(item, index))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_double_numbers() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = map(&numbers, |x, _| x * 2);
        assert_eq!(result, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_map_with_index() {
        let numbers = vec![10, 20, 30];
        let result = map(&numbers, |x, index| x + index as i32 * 10);
        assert_eq!(result, vec![10, 30, 50]);
    }

    #[test]
    fn test_map_strings_to_lengths() {
        let strings = vec!["apple", "banana", "cherry"];
        let lengths = map(&strings, |s, _| s.len());
        assert_eq!(lengths, vec![5, 6, 6]);
    }

    #[test]
    fn test_map_structs() {
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

        let ages: Vec<u32> = map(&people, |p, _| p.age);
        assert_eq!(ages, vec![25, 30, 35]);
    }

    #[test]
    fn test_map_empty_collection() {
        let empty: Vec<i32> = vec![];
        let result = map(&empty, |x, _| x * 2);
        let expected: Vec<i32> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_map_with_floats() {
        let float_collection = vec![1.1, 2.2, 3.3, 4.4];
        let result = map(&float_collection, |x, _| x * 1.5);
        let expected: Vec<f64> = vec![1.65, 3.3, 4.95, 6.6];
        let epsilon: f64 = 1e-10;

        // Ensure both vectors have the same length
        assert_eq!(
            result.len(),
            expected.len(),
            "Result and expected vectors have different lengths"
        );

        // Iterate through both vectors and compare each pair of elements within the tolerance
        for (res, exp) in result.iter().zip(expected.iter()) {
            assert!(
                (*res - *exp).abs() < epsilon,
                "Expected {}, but got {}",
                exp,
                res
            );
        }
    }

    #[test]
    fn test_map_with_negative_numbers() {
        let numbers: Vec<i32> = vec![-1, -2, -3];
        let result = map(&numbers, |x, _| (*x).abs());
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_map_with_complex_logic() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = map(&numbers, |x, index| x * (index + 1) as i32);
        assert_eq!(result, vec![1, 4, 9, 16, 25]);
    }
}
