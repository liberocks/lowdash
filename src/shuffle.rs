use crate::common::random_usize;

/// Shuffle a collection, returning a new vector with the elements in random order.
///
/// This function takes a slice of items and returns a new `Vec<T>` containing all the elements
/// from the input collection rearranged in a random order. It utilizes the Fisher-Yates algorithm
/// in conjunction with random number generation functions from `common.rs`.
///
/// **Note:** This implementation relies on the random functions provided in `common.rs` and is not
/// suitable for cryptographic purposes. For more robust randomness, consider using external crates
/// like `rand`.
///
/// **Time Complexity:**  
/// O(n), where n is the number of elements in the collection.
///
/// # Arguments
///
/// * `collection` - A slice of items to be shuffled.
///
/// # Type Parameters
///
/// * `T` - The type of elements in the collection. Must implement `Clone`.
///
/// # Returns
///
/// * `Vec<T>` - A new vector containing all elements from the input collection in shuffled order.
///
/// # Examples
///
/// ```rust
/// use lowdash::shuffle;
///
/// let numbers = vec![1, 2, 3, 4, 5];
/// let shuffled = shuffle(&numbers);
/// assert_eq!(shuffled.len(), numbers.len());
/// assert!(shuffled.contains(&1));
/// assert!(shuffled.contains(&2));
/// assert!(shuffled.contains(&3));
/// assert!(shuffled.contains(&4));
/// assert!(shuffled.contains(&5));
/// ```
pub fn shuffle<T>(collection: &[T]) -> Vec<T>
where
    T: Clone,
{
    let mut shuffled = collection.to_vec();
    let len = shuffled.len();

    if len <= 1 {
        return shuffled;
    }

    for i in (1..len).rev() {
        let j = random_usize(i + 1);
        shuffled.swap(i, j);
    }

    shuffled
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
    fn test_shuffle_integers() {
        let numbers = vec![1, 2, 3, 4, 5];
        let shuffled = shuffle(&numbers);
        assert_eq!(shuffled.len(), numbers.len());
        for num in &numbers {
            assert!(shuffled.contains(num));
        }
    }

    #[test]
    fn test_shuffle_strings() {
        let strings = vec![
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
            "date".to_string(),
        ];
        let shuffled = shuffle(&strings);
        assert_eq!(shuffled.len(), strings.len());
        for s in &strings {
            assert!(shuffled.contains(s));
        }
    }

    #[test]
    fn test_shuffle_with_structs() {
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
            Person {
                name: "Dave".to_string(),
                age: 40,
            },
        ];

        let shuffled = shuffle(&people);
        assert_eq!(shuffled.len(), people.len());
        for person in &people {
            assert!(shuffled.contains(person));
        }
    }

    #[test]
    fn test_shuffle_empty_collection() {
        let empty: Vec<i32> = vec![];
        let shuffled = shuffle(&empty);
        assert_eq!(shuffled, Vec::<i32>::new());
    }

    #[test]
    fn test_shuffle_single_element() {
        let single = vec![42];
        let shuffled = shuffle(&single);
        assert_eq!(shuffled, single);
    }

    #[test]
    fn test_shuffle_preserves_elements() {
        let elements = vec![10, 20, 30, 40, 50];
        let shuffled = shuffle(&elements);
        assert_eq!(shuffled.len(), elements.len());
        let sorted_original = {
            let mut sorted = elements.clone();
            sorted.sort();
            sorted
        };
        let mut sorted_shuffled = shuffled.clone();
        sorted_shuffled.sort();
        assert_eq!(sorted_shuffled, sorted_original);
    }

    #[test]
    fn test_shuffle_with_duplicates() {
        let numbers = vec![1, 2, 2, 3, 3, 3];
        let shuffled = shuffle(&numbers);
        assert_eq!(shuffled.len(), numbers.len());
        for num in &numbers {
            assert!(shuffled.contains(num));
        }
    }

    #[test]
    fn test_shuffle_with_optionals() {
        let collection = vec![Some(1), None, Some(2), Some(3), None];
        let shuffled = shuffle(&collection);
        assert_eq!(shuffled.len(), collection.len());
        for item in &collection {
            assert!(shuffled.contains(item));
        }
    }

    #[test]
    fn test_shuffle_with_floats() {
        let float_collection = vec![1.1, 2.2, 3.3, 4.4, 5.5];
        let shuffled = shuffle(&float_collection);
        assert_eq!(shuffled.len(), float_collection.len());
        for num in &float_collection {
            assert!(shuffled.contains(num));
        }
    }

    #[test]
    fn test_shuffle_with_nan_floats() {
        let float_collection = vec![std::f64::NAN, 2.2, std::f64::NAN, 4.4];
        let shuffled = shuffle(&float_collection);
        assert_eq!(shuffled.len(), float_collection.len());
        // Since NaN != NaN, we check the count of NaNs
        let original_nan_count = float_collection.iter().filter(|x| x.is_nan()).count();
        let shuffled_nan_count = shuffled.iter().filter(|x| x.is_nan()).count();
        assert_eq!(original_nan_count, shuffled_nan_count);
        // Check non-NaN elements are present
        assert!(shuffled.contains(&2.2));
        assert!(shuffled.contains(&4.4));
    }
}
