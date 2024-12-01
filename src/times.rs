/// Generates a collection by invoking the provided function `iteratee` a specified number of times.
///
/// This function calls the `iteratee` function `count` times, passing in the current index each time,
/// and collects the results into a `Vec<T>`.
///
/// # Arguments
/// * `count` - The number of times to invoke `iteratee`.
/// * `iteratee` - A function that takes the current index and returns a value of type `T`.
///
/// # Returns
/// * `Vec<T>` - A vector containing the results of each invocation of `iteratee`.
///
/// # Examples
/// ```rust
/// use lowdash::times;
/// let result = times(5, |i| i * 2);
/// assert_eq!(result, vec![0, 2, 4, 6, 8]);
/// ```
///
/// ```rust
/// use lowdash::times;
/// let result = times(3, |i| format!("Item {}", i));
/// assert_eq!(result, vec!["Item 0", "Item 1", "Item 2"]);
/// ```
pub fn times<T, F>(count: usize, mut iteratee: F) -> Vec<T>
where
    F: FnMut(usize) -> T,
{
    let mut result = Vec::with_capacity(count);
    for i in 0..count {
        result.push(iteratee(i));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_times_integers() {
        let result = times(5, |i| i * 2);
        assert_eq!(result, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn test_times_strings() {
        let result = times(3, |i| format!("Item {}", i));
        assert_eq!(result, vec!["Item 0", "Item 1", "Item 2"]);
    }

    #[test]
    fn test_times_with_struct() {
        #[derive(Debug, PartialEq)]
        struct Person {
            id: usize,
            name: String,
        }

        let names = vec!["Alice", "Bob", "Carol"];
        let people = times(names.len(), |i| Person {
            id: i,
            name: names[i].to_string(),
        });

        assert_eq!(
            people,
            vec![
                Person {
                    id: 0,
                    name: "Alice".to_string()
                },
                Person {
                    id: 1,
                    name: "Bob".to_string()
                },
                Person {
                    id: 2,
                    name: "Carol".to_string()
                },
            ]
        );
    }

    #[test]
    fn test_times_zero_count() {
        let result: Vec<i32> = times(0, |_| 42);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_times_large_count() {
        let count = 1000;
        let result = times(count, |i| i);
        assert_eq!(result.len(), count);
        assert_eq!(result[0], 0);
        assert_eq!(result[count - 1], count - 1);
    }

    #[test]
    fn test_times_with_option() {
        let result = times(5, |i| if i % 2 == 0 { Some(i) } else { None });
        assert_eq!(result, vec![Some(0), None, Some(2), None, Some(4)]);
    }

    #[test]
    fn test_times_with_floats() {
        let result = times(4, |i| (i as f64) * 1.5);
        assert_eq!(result, vec![0.0, 1.5, 3.0, 4.5]);
    }

    #[test]
    fn test_times_mutating_external_state() {
        let mut external = 0;
        let result = times(4, |i| {
            external += i;
            external
        });
        assert_eq!(result, vec![0, 1, 3, 6]);
        assert_eq!(external, 6);
    }

    #[test]
    fn test_times_with_characters() {
        let result = times(5, |i| (b'a' + i as u8) as char);
        assert_eq!(result, vec!['a', 'b', 'c', 'd', 'e']);
    }

    #[test]
    fn test_times_with_boolean() {
        let result = times(3, |i| i % 2 == 0);
        assert_eq!(result, vec![true, false, true]);
    }

    #[test]
    fn test_times_nested() {
        let result = times(3, |i| times(2, move |j| i * j));
        assert_eq!(result, vec![vec![0, 0], vec![0, 1], vec![0, 2]]);
    }

    #[test]
    fn test_times_with_empty_function() {
        let result = times(3, |_| {});
        assert_eq!(result, vec![(), (), ()]);
    }
}
