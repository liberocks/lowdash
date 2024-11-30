use crate::common;

/// Returns a pseudo-random element from the collection.
/// If the collection is empty, returns the default value of T.
///
/// # Arguments
/// * `collection` - A slice of items
///
/// # Returns
/// * `T` - A pseudo-randomly selected item from the collection or the default value if empty
///
/// # Examples
/// ```
/// use lowdash::sample;
///
/// let numbers = vec![1, 2, 3, 4, 5];
/// let result = sample(&numbers);
/// assert!(numbers.contains(&result));
///
/// let empty: Vec<i32> = vec![];
/// let result = sample(&empty);
/// assert_eq!(result, 0); // i32::default()
/// ```
pub fn sample<T>(collection: &[T]) -> T
where
    T: Clone + Default,
{
    let size = collection.len();
    if size == 0 {
        return T::default();
    }

    let index = common::random_index(size);

    collection[index].clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_sample_with_numbers() {
        let collection = vec![1, 2, 3, 4, 5];
        let result = sample(&collection);
        assert!(collection.contains(&result));
    }

    #[test]
    fn test_sample_empty_collection() {
        let collection: Vec<i32> = vec![];
        let result = sample(&collection);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_sample_with_structs() {
        #[derive(Debug, PartialEq, Clone, Default)]
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

        let result = sample(&people);
        assert!(people.contains(&result));

        let empty_people: Vec<Person> = vec![];
        let result = sample(&empty_people);
        assert_eq!(result, Person::default());
    }

    #[test]
    fn test_sample_with_single_element() {
        let collection = vec![42];
        let result = sample(&collection);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_sample_distribution() {
        let collection = vec![1, 2, 3];
        let mut results = HashSet::new();

        // Run multiple samples to verify we get different values
        for _ in 0..100 {
            let result = sample(&collection);
            results.insert(result);
            // Small delay to ensure different system times
            std::thread::sleep(std::time::Duration::from_nanos(1));
        }

        // Verify that we got at least 2 different values
        // (we might not get all 3 due to the simple random implementation)
        assert!(results.len() > 1);
    }

    #[test]
    fn test_sample_with_strings() {
        let collection = vec!["apple", "banana", "cherry"];
        let result = sample(&collection);
        assert!(collection.contains(&result));
    }

    #[test]
    fn test_sample_with_option() {
        let collection = vec![Some(1), Some(2), Some(3), None];
        let result = sample(&collection);
        assert!(collection.contains(&result));
    }
}
