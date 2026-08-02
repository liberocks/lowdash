/// Calls `predicate(index)` for each index in `0..count` and collects the results.
///
/// **Time Complexity:**  
/// O(n), where `n` is `count`.
///
/// # Arguments
///
/// * `count` - Number of calls.
/// * `predicate` - Function generating an item from its index.
///
/// # Type Parameters
///
/// * `T` - Item type.
/// * `F` - Generator type.
///
/// # Returns
///
/// * `Vec<T>` - The generated values.
///
/// # Examples
///
/// ```rust
/// use lowdash::repeat_by;
///
/// let filled = repeat_by(5, |i| i * 2);
/// assert_eq!(filled, vec![0, 2, 4, 6, 8]);
/// ```
///
/// ```rust
/// use lowdash::repeat_by;
///
/// #[derive(Debug, PartialEq, Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let filled_people = repeat_by(3, |i| Person {
///     name: format!("Person {}", i + 1),
///     age: 20 + i as u32 * 5,
/// });
/// assert_eq!(
///     filled_people,
///     vec![
///         Person { name: "Person 1".to_string(), age: 20 },
///         Person { name: "Person 2".to_string(), age: 25 },
///         Person { name: "Person 3".to_string(), age: 30 },
///     ]
/// );
/// ```
pub fn repeat_by<T, F>(count: usize, predicate: F) -> Vec<T>
where
    F: Fn(usize) -> T,
{
    let mut result = Vec::with_capacity(count);

    for i in 0..count {
        result.push(predicate(i));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    struct Person {
        name: String,
        age: u32,
    }

    #[test]
    fn test_repeat_by_integers() {
        let filled = repeat_by(5, |i| i * 2);
        assert_eq!(filled, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn test_repeat_by_strings() {
        let filled = repeat_by(3, |i| format!("Item {}", i + 1));
        assert_eq!(
            filled,
            vec![
                "Item 1".to_string(),
                "Item 2".to_string(),
                "Item 3".to_string(),
            ]
        );
    }

    #[test]
    fn test_repeat_by_with_structs() {
        let filled_people = repeat_by(3, |i| Person {
            name: format!("Person {}", i + 1),
            age: 20 + i as u32 * 5,
        });
        assert_eq!(
            filled_people,
            vec![
                Person {
                    name: "Person 1".to_string(),
                    age: 20
                },
                Person {
                    name: "Person 2".to_string(),
                    age: 25
                },
                Person {
                    name: "Person 3".to_string(),
                    age: 30
                },
            ]
        );
    }

    #[test]
    fn test_repeat_by_zero_times() {
        let filled: Vec<i32> = repeat_by(0, |i| i as i32);
        assert_eq!(filled, Vec::<i32>::new());
    }

    #[test]
    fn test_repeat_by_single_time() {
        let filled = repeat_by(1, |i| format!("Only {}", i));
        assert_eq!(filled, vec!["Only 0".to_string()]);
    }

    #[test]
    fn test_repeat_by_preserves_length() {
        let count = 10;
        let filled = repeat_by(count, |i| i * i);
        assert_eq!(filled.len(), count);
        assert_eq!(filled, (0..count).map(|i| i * i).collect::<Vec<_>>());
    }

    #[test]
    fn test_repeat_by_with_duplicates() {
        let filled = repeat_by(4, |_| 2);
        assert_eq!(filled, vec![2, 2, 2, 2]);
    }

    #[test]
    fn test_repeat_by_with_optionals() {
        let filled = repeat_by(3, |i| if i % 2 == 0 { Some(i) } else { None });
        assert_eq!(filled, vec![Some(0), None, Some(2)]);
    }

    #[test]
    fn test_repeat_by_with_floats() {
        let filled = repeat_by(3, |i| i as f64 * 1.5);
        assert_eq!(filled, vec![0.0, 1.5, 3.0]);
    }

    #[test]
    fn test_repeat_by_with_nan_floats() {
        let nan = std::f64::NAN;
        let filled = repeat_by(2, |_| nan);
        assert_eq!(filled.len(), 2);
        for value in filled {
            assert!(value.is_nan());
        }
    }
}
