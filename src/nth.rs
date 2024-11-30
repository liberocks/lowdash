use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct NthError {
    index: i64,
}

impl fmt::Display for NthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "nth: {} out of slice bounds", self.index)
    }
}

impl Error for NthError {}

/// Returns the nth element from the collection.
/// Supports both positive and negative indices.
/// Negative indices count from the end of the collection.
///
/// # Arguments
/// * `collection` - A slice of items
/// * `nth` - Index of the desired element (can be negative)
///
/// # Returns
/// * `Ok(&T)` - The element at the specified index
/// * `Err(NthError)` - If the index is out of bounds
///
/// # Examples
/// ```
/// use lowdash::nth;
///
/// let numbers = vec![1, 2, 3, 4, 5];
/// let result = nth(&numbers, 2);
/// assert_eq!(result.unwrap(), &3);
///
/// let result = nth(&numbers, -2);
/// assert_eq!(result.unwrap(), &4);
///
/// let result = nth(&numbers, 10);
/// assert!(result.is_err());
/// ```
pub fn nth<T>(collection: &[T], nth: i64) -> Result<&T, NthError> {
    let len = collection.len() as i64;

    if nth >= len || -nth > len {
        return Err(NthError { index: nth });
    }

    let index = if nth >= 0 { nth } else { len + nth } as usize;

    Ok(&collection[index])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_positive_index() {
        let collection = vec![1, 2, 3, 4, 5];
        let result = nth(&collection, 2);
        assert_eq!(result.unwrap(), &3);
    }

    #[test]
    fn test_nth_negative_index() {
        let collection = vec![1, 2, 3, 4, 5];
        let result = nth(&collection, -2);
        assert_eq!(result.unwrap(), &4);
    }

    #[test]
    fn test_nth_out_of_bounds_positive() {
        let collection = vec![1, 2, 3];
        let result = nth(&collection, 5);
        assert_eq!(result.unwrap_err(), NthError { index: 5 });
    }

    #[test]
    fn test_nth_out_of_bounds_negative() {
        let collection = vec![1, 2, 3];
        let result = nth(&collection, -5);
        assert_eq!(result.unwrap_err(), NthError { index: -5 });
    }

    #[test]
    fn test_nth_empty_collection() {
        let collection: Vec<i32> = vec![];
        let result = nth(&collection, 0);
        assert_eq!(result.unwrap_err(), NthError { index: 0 });
    }

    #[test]
    fn test_nth_with_structs() {
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

        let result = nth(&people, 1);
        assert_eq!(
            result.unwrap(),
            &Person {
                name: "Bob".to_string(),
                age: 30,
            }
        );

        let result = nth(&people, -1);
        assert_eq!(
            result.unwrap(),
            &Person {
                name: "Carol".to_string(),
                age: 35,
            }
        );
    }

    #[test]
    fn test_nth_first_and_last() {
        let collection = vec![1, 2, 3, 4, 5];
        assert_eq!(nth(&collection, 0).unwrap(), &1);
        assert_eq!(nth(&collection, -1).unwrap(), &5);
    }

    #[test]
    fn test_nth_with_single_element() {
        let collection = vec![42];
        assert_eq!(nth(&collection, 0).unwrap(), &42);
        assert_eq!(nth(&collection, -1).unwrap(), &42);
        assert!(nth(&collection, 1).is_err());
        assert!(nth(&collection, -2).is_err());
    }

    #[test]
    fn test_nth_error_display() {
        let error = NthError { index: 5 };
        assert_eq!(error.to_string(), "nth: 5 out of slice bounds");

        let error = NthError { index: -3 };
        assert_eq!(error.to_string(), "nth: -3 out of slice bounds");
    }
}
