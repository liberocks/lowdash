/// Returns the first item from the collection.
/// If the collection is empty, returns the default value of `T` and `false`.
///
/// # Arguments
/// * `collection` - A slice of items.
///
/// # Returns
/// * `(T, bool)` - A tuple containing the first item and `true`.
///                If the collection is empty, returns `(T::default(), false)`.
///
/// # Examples
/// ```rust
/// use lowdash::first;
///
/// let numbers = vec![1, 2, 3];
/// let (first_num, exists) = first(&numbers);
/// assert_eq!(first_num, 1);
/// assert!(exists);
///
/// let empty: Vec<i32> = vec![];
/// let (first_num, exists) = first(&empty);
/// assert_eq!(first_num, 0); // i32::default() is 0
/// assert!(!exists);
/// ```
pub fn first<T>(collection: &[T]) -> (T, bool)
where
    T: Clone + Default,
{
    if let Some(item) = collection.first() {
        (item.clone(), true)
    } else {
        (T::default(), false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone, Default)]
    struct Item {
        name: String,
        value: i32,
    }

    #[test]
    fn test_first_with_non_empty_collection() {
        let collection = vec![1, 2, 3];
        let (first, exists) = first(&collection);
        assert_eq!(first, 1);
        assert!(exists);
    }

    #[test]
    fn test_first_with_empty_collection() {
        let collection: Vec<i32> = vec![];
        let (first, exists) = first(&collection);
        assert_eq!(first, 0);
        assert!(!exists);
    }

    #[test]
    fn test_first_with_structs() {
        let item1 = Item {
            name: "Item1".to_string(),
            value: 10,
        };
        let item2 = Item {
            name: "Item2".to_string(),
            value: 20,
        };
        let collection = vec![item1.clone(), item2.clone()];
        let (first, exists) = first(&collection);
        assert_eq!(first, item1);
        assert!(exists);
    }

    #[test]
    fn test_first_with_single_item() {
        let collection = vec![42];
        let (first, exists) = first(&collection);
        assert_eq!(first, 42);
        assert!(exists);
    }

    #[test]
    fn test_first_with_custom_default() {
        let collection: Vec<Item> = vec![];
        let (first, exists) = first(&collection);
        assert_eq!(first, Item::default());
        assert!(!exists);
    }
}
