/// Apply a function to each item in a collection, flattening the results based on a callback.
///
/// This function iterates over a collection and applies the provided `iteratee` function
/// to each item along with its index. The `iteratee` returns a vector of transformed items,
/// which are then concatenated into a single resulting vector.
///
/// # Arguments
/// * `collection` - A slice of items.
/// * `iteratee` - A function that takes a reference to an item and its index, returning a vector of transformed items.
///
/// # Returns
/// * `Vec<R>` - A vector containing all the transformed items from each iteration.
///
/// # Examples
/// ```rust
/// use lowdash::flat_map;
/// let numbers = vec![1, 2, 3];
/// // For each number, generate a vector containing the number and its double
/// let result = flat_map(&numbers, |x, _| vec![*x, *x * 2]);
/// assert_eq!(result, vec![1, 2, 2, 4, 3, 6]);
/// ```
///
/// ```rust
/// use lowdash::flat_map;
///
/// #[derive(Debug, PartialEq)]
/// struct Person {
///     name: String,
///     hobbies: Vec<String>,
/// }
///
/// let people = vec![
///     Person {
///         name: "Alice".to_string(),
///         hobbies: vec!["Reading".to_string(), "Cycling".to_string()],
///     },
///     Person {
///         name: "Bob".to_string(),
///         hobbies: vec!["Cooking".to_string()],
///     },
/// ];
///
/// // Extract all hobbies from the list of people
/// let all_hobbies: Vec<String> = flat_map(&people, |person, _| person.hobbies.clone());
/// assert_eq!(
///     all_hobbies,
///     vec![
///         "Reading".to_string(),
///         "Cycling".to_string(),
///         "Cooking".to_string()
///     ]
/// );
/// ```
pub fn flat_map<T, R, F>(collection: &[T], iteratee: F) -> Vec<R>
where
    F: Fn(&T, usize) -> Vec<R>,
{
    let mut result = Vec::new();
    for (index, item) in collection.iter().enumerate() {
        let mapped_items = iteratee(item, index);
        result.extend(mapped_items);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flat_map_double_and_triple() {
        let numbers = vec![1, 2, 3];
        let result = flat_map(&numbers, |x, _| vec![*x * 2, *x * 3]);
        assert_eq!(result, vec![2, 3, 4, 6, 6, 9]);
    }

    #[test]
    fn test_flat_map_with_index() {
        let numbers = vec![10, 20, 30];
        let result = flat_map(&numbers, |x, index| vec![*x + index as i32 * 10]);
        assert_eq!(result, vec![10, 30, 50]);
    }

    #[test]
    fn test_flat_map_strings_to_chars() {
        let strings = vec!["hi", "rust"];
        let chars: Vec<char> = flat_map(&strings, |s, _| s.chars().collect());
        assert_eq!(chars, vec!['h', 'i', 'r', 'u', 's', 't']);
    }

    #[test]
    fn test_flat_map_structs() {
        #[derive(Debug, PartialEq)]
        struct Person {
            name: String,
            hobbies: Vec<String>,
        }

        let people = vec![
            Person {
                name: "Alice".to_string(),
                hobbies: vec!["Reading".to_string(), "Cycling".to_string()],
            },
            Person {
                name: "Bob".to_string(),
                hobbies: vec!["Cooking".to_string()],
            },
        ];

        let all_hobbies: Vec<String> = flat_map(&people, |person, _| person.hobbies.clone());
        assert_eq!(
            all_hobbies,
            vec![
                "Reading".to_string(),
                "Cycling".to_string(),
                "Cooking".to_string()
            ]
        );
    }

    #[test]
    fn test_flat_map_empty_collection() {
        let empty: Vec<i32> = vec![];
        let result = flat_map(&empty, |x, _| vec![*x * 2]);
        let expected: Vec<i32> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_flat_map_with_floats() {
        let float_collection = vec![1.1, 2.2, 3.3];
        let result = flat_map(&float_collection, |x, _| vec![*x, *x * 2.0]);
        let expected: Vec<f64> = vec![1.1, 2.2, 2.2, 4.4, 3.3, 6.6];
        let epsilon = 1e-10;

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
    fn test_flat_map_with_nested_vectors() {
        let numbers = vec![1, 2, 3];
        let result = flat_map(&numbers, |x, _| vec![vec![*x], vec![*x * 10]]);
        let expected: Vec<Vec<i32>> = vec![vec![1], vec![10], vec![2], vec![20], vec![3], vec![30]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_flat_map_with_optionals() {
        let numbers = vec![Some(1), None, Some(3)];
        let result = flat_map(&numbers, |x, _| match x {
            Some(n) => vec![*n],
            None => vec![],
        });
        assert_eq!(result, vec![1, 3]);
    }

    #[test]
    fn test_flat_map_with_complex_logic() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = flat_map(&numbers, |x, index| {
            if *x % 2 != 0 {
                vec![*x, *x * (index + 1) as i32]
            } else {
                vec![]
            }
        });
        let expected = vec![1, 1, 3, 9, 5, 25]; // Corrected expected values
        assert_eq!(result, expected);
    }

    #[test]
    fn test_flat_map_all_included() {
        let numbers = vec![1, 2, 3];
        let result = flat_map(&numbers, |x, _| vec![*x, *x + 1]);
        assert_eq!(result, vec![1, 2, 2, 3, 3, 4]);
    }

    #[test]
    fn test_flat_map_all_excluded() {
        let numbers = vec![1, 2, 3];
        let result = flat_map(&numbers, |x, _| if *x % 2 == 0 { vec![*x] } else { vec![] });
        assert_eq!(result, vec![2]);
    }
}
