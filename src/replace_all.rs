use crate::replace;

/// Replaces all occurrences of a specified value in a collection with a new value.
///
/// This function iterates over a slice of items, replacing each occurrence of `old` with `new`.
/// It preserves the order of elements and does not modify the original collection.
///
/// **Time Complexity:** O(n), where n is the number of elements in the collection.
///
/// # Arguments
///
/// * `collection` - A slice of items in which to perform replacements.
/// * `old` - The value to be replaced.
/// * `new` - The value to replace with.
///
/// # Type Parameters
///
/// * `T` - The type of elements in the collection. Must implement `PartialEq` and `Clone`.
///
/// # Returns
///
/// * `Vec<T>` - A new vector with all specified replacements made.
///
/// # Examples
///
/// ```rust
/// use lowdash::replace_all;
///
/// let numbers = vec![1, 2, 2, 3, 4, 2, 5];
/// let result = replace_all(&numbers, 2, 9);
/// assert_eq!(result, vec![1, 9, 9, 3, 4, 9, 5]);
/// ```
///
/// ```rust
/// use lowdash::replace_all;
///
/// #[derive(Debug, PartialEq, Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let people = vec![
///     Person { name: "Alice".to_string(), age: 25 },
///     Person { name: "Bob".to_string(), age: 30 },
///     Person { name: "Alice".to_string(), age: 25 },
///     Person { name: "Carol".to_string(), age: 35 },
/// ];
///
/// let dave = Person { name: "Dave".to_string(), age: 40 };
/// let result = replace_all(&people, people[0].clone(), dave.clone());
/// assert_eq!(
///     result,
///     vec![
///         dave.clone(),
///         people[1].clone(),
///         dave.clone(),
///         people[3].clone(),
///     ]
/// );
/// ```
pub fn replace_all<T>(collection: &[T], old: T, new: T) -> Vec<T>
where
    T: PartialEq + Clone,
{
    replace(collection, old, new, usize::MAX)
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
    fn test_replace_all_basic() {
        let numbers = vec![1, 2, 2, 3, 4, 2, 5];
        let result = replace_all(&numbers, 2, 9);
        assert_eq!(result, vec![1, 9, 9, 3, 4, 9, 5]);
    }

    #[test]
    fn test_replace_all_no_replacements() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = replace_all(&numbers, 6, 9);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_replace_all_with_structs() {
        let alice = Person {
            name: "Alice".to_string(),
            age: 25,
        };
        let bob = Person {
            name: "Bob".to_string(),
            age: 30,
        };
        let carol = Person {
            name: "Carol".to_string(),
            age: 35,
        };
        let dave = Person {
            name: "Dave".to_string(),
            age: 40,
        };

        let people = vec![alice.clone(), bob.clone(), alice.clone(), carol.clone()];
        let result = replace_all(&people, alice.clone(), dave.clone());
        assert_eq!(
            result,
            vec![dave.clone(), bob.clone(), dave.clone(), carol.clone()]
        );
    }

    #[test]
    fn test_replace_all_with_empty_collection() {
        let empty: Vec<i32> = vec![];
        let result = replace_all(&empty, 1, 9);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_replace_all_with_strings() {
        let fruits = vec!["apple", "banana", "apple", "cherry", "banana"];
        let result = replace_all(&fruits, "banana", "orange");
        assert_eq!(result, vec!["apple", "orange", "apple", "cherry", "orange"]);
    }

    #[test]
    fn test_replace_all_with_floats() {
        let floats = vec![1.1, 2.2, 2.2, 3.3, 2.2, 4.4];
        let result = replace_all(&floats, 2.2, 9.9);
        assert_eq!(result, vec![1.1, 9.9, 9.9, 3.3, 9.9, 4.4]);
    }

    #[test]
    fn test_replace_all_with_characters() {
        let chars = vec!['a', 'b', 'a', 'c', 'b', 'd'];
        let result = replace_all(&chars, 'a', 'z');
        assert_eq!(result, vec!['z', 'b', 'z', 'c', 'b', 'd']);
    }

    #[test]
    fn test_replace_all_preserves_order() {
        let numbers = vec![3, 1, 2, 3, 2, 4, 1, 5];
        let result = replace_all(&numbers, 1, 9);
        assert_eq!(result, vec![3, 9, 2, 3, 2, 4, 9, 5]);
    }

    #[test]
    fn test_replace_all_with_nan_floats() {
        let float_collection = vec![std::f64::NAN, 2.2, std::f64::NAN, 1.0];
        let new_nan = std::f64::NAN;
        let result = replace_all(&float_collection, std::f64::NAN, new_nan);
        // Since NaN != NaN, no replacements should occur
        assert_eq!(result.len(), 4);
        // Each NaN remains unchanged
        assert!(result[0].is_nan());
        assert_eq!(result[1], 2.2);
        assert!(result[2].is_nan());
        assert_eq!(result[3], 1.0);
    }
}
