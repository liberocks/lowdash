/// Apply a function to each item in a collection, filtering and transforming items based on a callback.
///
/// This function iterates over a collection and applies the provided `callback` function
/// to each item along with its index. If the callback returns `(R, true)`, the transformed
/// value `R` is included in the resulting vector.
///
/// # Arguments
/// * `collection` - A slice of items.
/// * `callback` - A function that takes a reference to an item and its index, returning a tuple `(R, bool)`
///                where `R` is the transformed value and `bool` indicates whether to include it.
///
/// # Returns
/// * `Vec<R>` - A vector containing the transformed items that passed the callback's predicate.
///
/// # Examples
/// ```rust
/// use lowdash::filter_map;
/// let numbers = vec![1, 2, 3, 4, 5];
/// // Double even numbers
/// let result = filter_map(&numbers, |x, _| {
///     if *x % 2 == 0 {
///         (x * 2, true)
///     } else {
///         (0, false)
///     }
/// });
/// assert_eq!(result, vec![4, 8]);
/// ```
///
/// ```rust
/// use lowdash::filter_map;
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
/// // Extract names of people older than 28
/// let names: Vec<String> = filter_map(&people, |p, _| {
///     if p.age > 28 {
///         (p.name.clone(), true)
///     } else {
///         (String::new(), false)
///     }
/// });
/// assert_eq!(names, vec!["Bob".to_string(), "Carol".to_string()]);
/// ```
pub fn filter_map<T, R, F>(collection: &[T], callback: F) -> Vec<R>
where
    F: Fn(&T, usize) -> (R, bool),
{
    let mut result = Vec::new();
    for (index, item) in collection.iter().enumerate() {
        let (mapped, include) = callback(item, index);
        if include {
            result.push(mapped);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_map_double_evens() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = filter_map(&numbers, |x, _| {
            if *x % 2 == 0 {
                (*x * 2, true)
            } else {
                (0, false)
            }
        });
        assert_eq!(result, vec![4, 8]);
    }

    #[test]
    fn test_filter_map_with_index() {
        let numbers = vec![10, 20, 30, 40];
        let result = filter_map(&numbers, |x, index| {
            if index % 2 == 0 {
                (*x + index as i32 * 10, true)
            } else {
                (0, false)
            }
        });
        let expected = vec![10, 50]; // Corrected expected values
        assert_eq!(result, expected);
    }

    #[test]
    fn test_filter_map_strings_to_lengths() {
        let strings = vec!["apple", "banana", "cherry", "date"];
        let lengths = filter_map(&strings, |s, _| {
            if s.len() > 5 {
                (s.len(), true)
            } else {
                (0, false)
            }
        });
        assert_eq!(lengths, vec![6, 6]);
    }

    #[test]
    fn test_filter_map_structs() {
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

        let ages: Vec<u32> = filter_map(&people, |p, _| {
            if p.age > 28 {
                (p.age, true)
            } else {
                (0, false)
            }
        });
        assert_eq!(ages, vec![30, 35]);
    }

    #[test]
    fn test_filter_map_empty_collection() {
        let empty: Vec<i32> = vec![];
        let result = filter_map(&empty, |x, _| (*x * 2, true));
        let expected: Vec<i32> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_filter_map_with_floats() {
        let float_collection = vec![1.1, 2.2, 3.3, 4.4];
        let result = filter_map(&float_collection, |x, _| {
            if *x > 2.2 {
                (*x * 1.5, true)
            } else {
                (0.0, false)
            }
        });
        let expected: Vec<f64> = vec![4.95, 6.6];
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
    fn test_filter_map_with_negative_numbers() {
        let numbers: Vec<i32> = vec![-1, -2, -3];
        let result: Vec<i32> =
            filter_map(
                &numbers,
                |x, _| {
                    if *x < 0 {
                        (x.abs(), true)
                    } else {
                        (0, false)
                    }
                },
            );
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_filter_map_with_complex_logic() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = filter_map(&numbers, |x, index| {
            if *x % 2 != 0 {
                (x * (index + 1) as i32, true)
            } else {
                (0, false)
            }
        });
        assert_eq!(result, vec![1, 9, 25]);
    }

    #[test]
    fn test_filter_map_all_included() {
        let numbers = vec![1, 2, 3];
        let result = filter_map(&numbers, |x, _| (*x, true));
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_filter_map_all_excluded() {
        let numbers = vec![1, 2, 3];
        let result = filter_map(&numbers, |_, _| (0, false));
        let expected: Vec<i32> = vec![];
        assert_eq!(result, expected);
    }
}
