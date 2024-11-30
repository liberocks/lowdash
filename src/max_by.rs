/// Find the maximum element in a collection based on a custom comparison function.
/// If the collection is empty, returns `None`.
///
/// # Arguments
///
/// * `collection` - A slice of items.
/// * `comparison` - A function that takes two items and returns `true` if the first item is considered greater than the second.
///
/// # Returns
///
/// * `Option<T>` - The maximum item in the collection based on the comparison function, or `None` if the collection is empty.
///
/// # Examples
/// ```rust
/// use lowdash::max_by;
///
/// #[derive(Debug, PartialEq, Eq, Clone)]
/// struct Person {
///     age: u32,
///     name: String,
/// }
///
/// let people = vec![
///     Person { age: 25, name: "Alice".to_string() },
///     Person { age: 30, name: "Bob".to_string() },
///     Person { age: 20, name: "Carol".to_string() },
/// ];
///
/// let result = max_by(&people, |a, b| a.age > b.age);
/// assert_eq!(
///     result,
///     Some(Person { age: 30, name: "Bob".to_string() })
/// );
/// ```
pub fn max_by<T, F>(collection: &[T], comparison: F) -> Option<T>
where
    T: Clone,
    F: Fn(&T, &T) -> bool,
{
    if collection.is_empty() {
        return None;
    }

    if collection.len() == 1 {
        return Some(collection[0].clone());
    }

    let mut max = collection[0].clone();

    for item in &collection[1..] {
        if comparison(item, &max) {
            max = item.clone();
        }
    }

    Some(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_by_age() {
        #[derive(Debug, PartialEq, Eq, Clone)]
        struct Person {
            age: u32,
            name: String,
        }

        let people = vec![
            Person {
                age: 25,
                name: "Alice".to_string(),
            },
            Person {
                age: 30,
                name: "Bob".to_string(),
            },
            Person {
                age: 20,
                name: "Carol".to_string(),
            },
        ];

        let result = max_by(&people, |a, b| a.age > b.age);
        assert_eq!(
            result,
            Some(Person {
                age: 30,
                name: "Bob".to_string()
            })
        );
    }

    #[test]
    fn test_max_by_name() {
        #[derive(Debug, PartialEq, Eq, Clone)]
        struct Person {
            age: u32,
            name: String,
        }

        let people = vec![
            Person {
                age: 25,
                name: "Alice".to_string(),
            },
            Person {
                age: 30,
                name: "Bob".to_string(),
            },
            Person {
                age: 20,
                name: "Carol".to_string(),
            },
        ];

        let result = max_by(&people, |a, b| a.name > b.name);
        assert_eq!(
            result,
            Some(Person {
                age: 20,
                name: "Carol".to_string()
            })
        );
    }

    #[test]
    fn test_max_by_empty_collection() {
        #[derive(Debug, PartialEq, Eq, Clone)]
        struct Person {
            age: u32,
            name: String,
        }

        let people: Vec<Person> = vec![];
        let result = max_by(&people, |a, b| a.age > b.age);
        assert_eq!(result, None);
    }

    #[test]
    fn test_max_by_single_element() {
        #[derive(Debug, PartialEq, Eq, Clone)]
        struct Person {
            age: u32,
            name: String,
        }

        let person = Person {
            age: 25,
            name: "Alice".to_string(),
        };
        let people = vec![person.clone()];
        let result = max_by(&people, |a, b| a.age > b.age);
        assert_eq!(result, Some(person));
    }

    #[test]
    fn test_max_by_with_struct_various_criteria() {
        #[derive(Debug, PartialEq, Eq, Clone)]
        struct Person {
            age: u32,
            name: String,
        }

        let people = vec![
            Person {
                age: 25,
                name: "Alice".to_string(),
            },
            Person {
                age: 30,
                name: "Bob".to_string(),
            },
            Person {
                age: 35,
                name: "Carol".to_string(),
            },
        ];

        // Max by age
        let result_age = max_by(&people, |a, b| a.age > b.age);
        assert_eq!(
            result_age,
            Some(Person {
                age: 35,
                name: "Carol".to_string()
            })
        );

        // Max by name lexicographically
        let result_name = max_by(&people, |a, b| a.name > b.name);
        assert_eq!(
            result_name,
            Some(Person {
                age: 35,
                name: "Carol".to_string()
            })
        );
    }

    #[test]
    fn test_max_by_floats() {
        let float_collection = vec![1.1, 2.2, 3.3, 4.4];
        let result = max_by(&float_collection, |a, b| a > b);
        assert_eq!(result, Some(4.4));

        let more_floats = vec![5.5, 3.3, 5.5, 2.2];
        let result_duplicate = max_by(&more_floats, |a, b| a > b);
        assert_eq!(result_duplicate, Some(5.5));

        let negative_floats = vec![-1.1, -2.2, -0.5, -3.3];
        let result_negatives = max_by(&negative_floats, |a, b| a > b);
        assert_eq!(result_negatives, Some(-0.5));

        let all_nan = vec![std::f64::NAN, std::f64::NAN];
        let result_all_nan = max_by(&all_nan, |a, b| a > b);
        // The first NaN will remain as max since NaN > NaN is false
        // Hence, the result should be the first NaN
        assert!(result_all_nan.unwrap().is_nan());
    }
}
