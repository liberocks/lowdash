/// Execute a function on each item in a collection.
///
/// This function iterates over a collection, applying the provided `iteratee` function
/// to each item along with its index.
///
/// # Arguments
/// * `collection` - A slice of items.
/// * `iteratee` - A function that takes a reference to an item and its index.
///
/// # Examples
/// ```rust
/// use lowdash::foreach;
/// let numbers = vec![1, 2, 3, 4, 5];
/// let mut sum = 0;
/// foreach(&numbers, |x, _| sum += x);
/// assert_eq!(sum, 15);
/// ```
///
/// ```rust
/// use lowdash::foreach;
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
/// let mut names = Vec::new();
/// foreach(&people, |p, _| names.push(p.name.clone()));
/// assert_eq!(names, vec!["Alice", "Bob", "Carol"]);
/// ```
pub fn foreach<T, F>(collection: &[T], mut iteratee: F)
where
    F: FnMut(&T, usize),
{
    for (index, item) in collection.iter().enumerate() {
        iteratee(item, index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foreach_sum() {
        let numbers = vec![1, 2, 3, 4, 5];
        let mut sum = 0;
        foreach(&numbers, |x, _| sum += x);
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_foreach_with_index() {
        let numbers = vec![10, 20, 30];
        let mut result = Vec::new();
        foreach(&numbers, |x, index| result.push(*x + index as i32));
        assert_eq!(result, vec![10, 21, 32]);
    }

    #[test]
    fn test_foreach_struct() {
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

        let mut names = Vec::new();
        foreach(&people, |p, _| names.push(p.name.clone()));
        assert_eq!(names, vec!["Alice", "Bob", "Carol"]);
    }

    #[test]
    fn test_foreach_empty_collection() {
        let collection: Vec<i32> = vec![];
        let mut called = false;
        foreach(&collection, |_, _| called = true);
        assert!(!called);
    }

    #[test]
    fn test_foreach_multiple_types() {
        let chars = vec!['a', 'b', 'c'];
        let mut collected = String::new();
        foreach(&chars, |c, _| collected.push(*c));
        assert_eq!(collected, "abc");
    }

    #[test]
    fn test_foreach_with_floats() {
        let float_collection = vec![1.1, 2.2, 3.3];
        let mut product: f64 = 1.0;
        foreach(&float_collection, |x, _| product *= x);
        assert!((product - 7.986).abs() < 1e-10); // 1.1 * 2.2 * 3.3 = 7.986
    }

    #[test]
    fn test_foreach_with_optionals() {
        let numbers = vec![Some(1), None, Some(3), Some(4)];
        let mut sum = 0;
        foreach(&numbers, |x, _| {
            if let Some(n) = x {
                sum += n;
            }
        });
        assert_eq!(sum, 8);
    }

    #[test]
    fn test_foreach_with_index_and_condition() {
        let numbers = vec![1, 2, 3, 4, 5];
        let mut sum = 0;
        foreach(&numbers, |x, index| {
            if *x % 2 != 0 {
                sum += *x * (index as i32 + 1);
            }
        });
        // Calculation:
        // index 0: 1 * (0 + 1) = 1
        // index 2: 3 * (2 + 1) = 9
        // index 4: 5 * (4 + 1) = 25
        // Total sum = 1 + 9 + 25 = 35
        assert_eq!(sum, 35);
    }

    #[test]
    fn test_foreach_with_strings() {
        let strings = vec!["Hello", " ", "World", "!"];
        let mut concatenated = String::new();
        foreach(&strings, |s, _| concatenated.push_str(s));
        assert_eq!(concatenated, "Hello World!");
    }

    #[test]
    fn test_foreach_with_structs_complex_logic() {
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

        let mut total_age = 0;
        foreach(&people, |person, _| {
            if person.age > 20 {
                total_age += person.age;
            }
        });
        assert_eq!(total_age, 90);
    }
}
