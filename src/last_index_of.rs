/// Finds the position of the last occurrence of an element in a collection.
/// Returns -1 if the element is not found.
///
/// # Arguments
///
/// * `collection` - A slice of items.
/// * `element` - The element to search for.
///
/// # Returns
///
/// * `isize` - The index of the last occurrence of the element, or -1 if not found.
///
/// # Examples
/// ```rust
/// use lowdash::last_index_of;
/// let collection = vec![1, 2, 3, 2, 1];
/// let index = last_index_of(&collection, 2);
/// assert_eq!(index, 3);
/// ```
///
/// ```rust
/// use lowdash::last_index_of;
/// let collection = vec!["apple", "banana", "cherry", "banana"];
/// let index = last_index_of(&collection, "banana");
/// assert_eq!(index, 3);
/// ```
///
/// ```rust
/// use lowdash::last_index_of;
/// let collection = vec![1, 2, 3, 4, 5];
/// let index = last_index_of(&collection, 6);
/// assert_eq!(index, -1);
/// ```
pub fn last_index_of<T: PartialEq>(collection: &[T], element: T) -> isize {
    for (i, item) in collection.iter().enumerate().rev() {
        if *item == element {
            return i as isize;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_last_index_of_found() {
        let collection = vec![1, 2, 3, 2, 1];
        assert_eq!(last_index_of(&collection, 2), 3);
    }

    #[test]
    fn test_last_index_of_not_found() {
        let collection = vec![1, 2, 3, 4, 5];
        assert_eq!(last_index_of(&collection, 6), -1);
    }

    #[test]
    fn test_last_index_of_empty_collection() {
        let collection: Vec<i32> = vec![];
        assert_eq!(last_index_of(&collection, 1), -1);
    }

    #[test]
    fn test_last_index_of_strings() {
        let collection = vec!["apple", "banana", "cherry", "banana"];
        assert_eq!(last_index_of(&collection, "banana"), 3);
        assert_eq!(last_index_of(&collection, "orange"), -1);
    }

    #[test]
    fn test_last_index_of_floats() {
        let floats = vec![1.1, 2.2, 3.3, 4.4, 5.5, 4.4];
        assert_eq!(last_index_of(&floats, 4.4), 5);
        assert_eq!(last_index_of(&floats, 3.3), 2);
        assert_eq!(last_index_of(&floats, 6.6), -1);
    }
}
