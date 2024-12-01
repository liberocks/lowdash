/// Fill a collection with a specified value, returning a new vector with all elements set to the initial value.
///
/// This function takes a count and an initial value, then returns a new `Vec<T>` containing the initial value
/// repeated `count` times. The original value remains unmodified.
///
/// **Time Complexity:**  
/// O(n), where n is the number of times to repeat the initial value.
///
/// # Arguments
///
/// * `count` - The number of times to repeat the initial value.
/// * `initial` - The value to fill the new vector with.
///
/// # Type Parameters
///
/// * `T` - The type of elements to be repeated. Must implement the `Clone` trait to allow duplication.
///
/// # Returns
///
/// * `Vec<T>` - A new vector containing the initial value repeated `count` times.
///
/// # Examples
///
/// ```rust
/// use lowdash::repeat;
///
/// let filled = repeat(5, 0);
/// assert_eq!(filled, vec![0, 0, 0, 0, 0]);
/// ```
///
/// ```rust
/// use lowdash::repeat;
///
/// #[derive(Debug, PartialEq, Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let filled_people = repeat(3, Person { name: "Dave".to_string(), age: 40 });
/// assert_eq!(
///     filled_people,
///     vec![
///         Person { name: "Dave".to_string(), age: 40 },
///         Person { name: "Dave".to_string(), age: 40 },
///         Person { name: "Dave".to_string(), age: 40 },
///     ]
/// );
/// ```
pub fn repeat<T>(count: usize, initial: T) -> Vec<T>
where
    T: Clone,
{
    let mut result = Vec::with_capacity(count);

    for _ in 0..count {
        result.push(initial.clone());
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
    fn test_repeat_integers() {
        let filled = repeat(5, 0);
        assert_eq!(filled, vec![0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_repeat_strings() {
        let filled = repeat(3, "hello".to_string());
        assert_eq!(
            filled,
            vec![
                "hello".to_string(),
                "hello".to_string(),
                "hello".to_string()
            ]
        );
    }

    #[test]
    fn test_repeat_with_structs() {
        let initial_person = Person {
            name: "Dave".to_string(),
            age: 40,
        };
        let filled_people = repeat(3, initial_person.clone());
        assert_eq!(
            filled_people,
            vec![
                initial_person.clone(),
                initial_person.clone(),
                initial_person.clone(),
            ]
        );
    }

    #[test]
    fn test_repeat_zero_times() {
        let filled: Vec<i32> = repeat(0, 42);
        assert_eq!(filled, Vec::<i32>::new());
    }

    #[test]
    fn test_repeat_single_time() {
        let filled = repeat(1, "single".to_string());
        assert_eq!(filled, vec!["single".to_string()]);
    }

    #[test]
    fn test_repeat_preserves_length() {
        let count = 10;
        let filled = repeat(count, true);
        assert_eq!(filled.len(), count);
        assert_eq!(filled, vec![true; count]);
    }

    #[test]
    fn test_repeat_with_duplicates() {
        let filled = repeat(4, 2);
        assert_eq!(filled, vec![2, 2, 2, 2]);
    }

    #[test]
    fn test_repeat_with_optionals() {
        let filled = repeat(3, Some(1));
        assert_eq!(filled, vec![Some(1), Some(1), Some(1)]);
    }

    #[test]
    fn test_repeat_with_floats() {
        let filled = repeat(3, 3.14);
        assert_eq!(filled, vec![3.14, 3.14, 3.14]);
    }

    #[test]
    fn test_repeat_with_nan_floats() {
        let nan = std::f64::NAN;
        let filled = repeat(2, nan);
        assert_eq!(filled.len(), 2);
        for value in filled {
            assert!(value.is_nan());
        }
    }
}
