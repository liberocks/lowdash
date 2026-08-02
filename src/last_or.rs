use crate::last::last;

/// Returns the last item, or `fallback` when the collection is empty.
///
/// # Arguments
/// * `collection` - Items to inspect.
/// * `fallback` - Empty-input result.
///
/// # Returns
/// * `T` - The last item or `fallback`.
///
/// # Examples
/// ```rust
/// use lowdash::last_or;
///
/// let numbers = vec![1, 2, 3];
/// let last_num = last_or(&numbers, 10);
/// assert_eq!(last_num, 3);
///
/// let empty: Vec<i32> = vec![];
/// let last_num = last_or(&empty, 10);
/// assert_eq!(last_num, 10);
/// ```
pub fn last_or<T>(collection: &[T], fallback: T) -> T
where
    T: Clone + Default,
{
    let (item, exists) = last(collection);
    if exists {
        item
    } else {
        fallback
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
    fn test_last_or_with_non_empty_collection() {
        let collection = vec![1, 2, 3];
        let last = last_or(&collection, 10);
        assert_eq!(last, 3);
    }

    #[test]
    fn test_last_or_with_empty_collection() {
        let collection: Vec<i32> = vec![];
        let last = last_or(&collection, 10);
        assert_eq!(last, 10);
    }

    #[test]
    fn test_last_or_with_structs() {
        let item1 = Item {
            name: "Item1".to_string(),
            value: 10,
        };
        let item2 = Item {
            name: "Item2".to_string(),
            value: 20,
        };
        let fallback = Item {
            name: "Fallback".to_string(),
            value: 0,
        };
        let collection = vec![item1.clone(), item2.clone()];
        let last = last_or(&collection, fallback.clone());
        assert_eq!(last, item2);
    }

    #[test]
    fn test_last_or_with_single_item() {
        let collection = vec![42];
        let last = last_or(&collection, 100);
        assert_eq!(last, 42);
    }

    #[test]
    fn test_last_or_with_empty_struct() {
        let collection: Vec<Item> = vec![];
        let fallback = Item {
            name: "Fallback".to_string(),
            value: -1,
        };
        let last = last_or(&collection, fallback.clone());
        assert_eq!(last, fallback);
    }
}
