/// Finds the position of the first occurrence of an element in a collection.
/// Returns -1 if the element is not found.
///
/// # Arguments
/// * `collection` - A slice of items.
/// * `element` - The element to search for.
///
/// # Returns
/// * `isize` - The index of the first occurrence of the element, or -1 if not found.
///
/// # Examples
/// ```rust
/// use lowdash::index_of;
/// let collection = vec![1, 2, 3, 4, 5];
/// let index = index_of(&collection, 3);
/// assert_eq!(index, 2);
/// ```
///
/// ```rust
/// use lowdash::index_of;
/// let collection = vec!["apple", "banana", "cherry"];
/// let index = index_of(&collection, "banana");
/// assert_eq!(index, 1);
/// ```
///
/// ```rust
/// use lowdash::index_of;
/// let collection = vec![1, 2, 3, 4, 5];
/// let index = index_of(&collection, 6);
/// assert_eq!(index, -1);
/// ```
pub fn index_of<T: PartialEq>(collection: &[T], element: T) -> isize {
    for (i, item) in collection.iter().enumerate() {
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
    fn test_index_of_found() {
        let collection = vec![1, 2, 3, 4, 5];
        assert_eq!(index_of(&collection, 3), 2);
    }

    #[test]
    fn test_index_of_not_found() {
        let collection = vec![1, 2, 3, 4, 5];
        assert_eq!(index_of(&collection, 6), -1);
    }

    #[test]
    fn test_index_of_empty_collection() {
        let collection: Vec<i32> = vec![];
        assert_eq!(index_of(&collection, 1), -1);
    }

    #[test]
    fn test_index_of_strings() {
        let collection = vec!["apple", "banana", "cherry"];
        assert_eq!(index_of(&collection, "banana"), 1);
        assert_eq!(index_of(&collection, "orange"), -1);
    }

    #[test]
    fn test_index_of_floats() {
        let collection = vec![1.1, 2.2, 3.3, 4.4, 5.5];
        assert_eq!(index_of(&collection, 3.3), 2);
        assert_eq!(index_of(&collection, 6.6), -1);
    }
}
