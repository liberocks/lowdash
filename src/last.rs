/// Returns the last item from the collection.
/// If the collection is empty, returns the default value of `T` and `false`.
///
/// # Arguments
/// * `collection` - A slice of items.
///
/// # Returns
/// * `(T, bool)` - A tuple containing the last item and `true`.
///                If the collection is empty, returns `(T::default(), false)`.
///
/// # Examples
/// ```rust
/// use lowdash::last;
///
/// let numbers = vec![1, 2, 3];
/// let (last_num, exists) = last(&numbers);
/// assert_eq!(last_num, 3);
/// assert!(exists);
///
/// let empty: Vec<i32> = vec![];
/// let (last_num, exists) = last(&empty);
/// assert_eq!(last_num, 0); // i32::default() is 0
/// assert!(!exists);
/// ```
///
/// ```rust
/// use lowdash::last;
///
/// #[derive(Debug, PartialEq, Clone, Default)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let people = vec![
///     Person { name: "Alice".to_string(), age: 25 },
///     Person { name: "Bob".to_string(), age: 30 },
/// ];
///
/// let (last_person, exists) = last(&people);
/// assert_eq!(last_person, Person { name: "Bob".to_string(), age: 30 });
/// assert!(exists);
///
/// let empty_people: Vec<Person> = vec![];
/// let (last_person, exists) = last(&empty_people);
/// assert_eq!(last_person, Person::default());
/// assert!(!exists);
/// ```
pub fn last<T>(collection: &[T]) -> (T, bool)
where
    T: Clone + Default,
{
    if let Some(item) = collection.last() {
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
    fn test_last_with_non_empty_collection() {
        let collection = vec![1, 2, 3];
        let (last, exists) = last(&collection);
        assert_eq!(last, 3);
        assert!(exists);
    }

    #[test]
    fn test_last_with_empty_collection() {
        let collection: Vec<i32> = vec![];
        let (last, exists) = last(&collection);
        assert_eq!(last, 0);
        assert!(!exists);
    }

    #[test]
    fn test_last_with_structs() {
        let item1 = Item {
            name: "Item1".to_string(),
            value: 10,
        };
        let item2 = Item {
            name: "Item2".to_string(),
            value: 20,
        };
        let collection = vec![item1.clone(), item2.clone()];
        let (last_item, exists) = last(&collection);
        assert_eq!(last_item, item2);
        assert!(exists);
    }

    #[test]
    fn test_last_with_single_item() {
        let collection = vec![42];
        let (last, exists) = last(&collection);
        assert_eq!(last, 42);
        assert!(exists);
    }

    #[test]
    fn test_last_with_custom_default() {
        let collection: Vec<Item> = vec![];
        let (last, exists) = last(&collection);
        assert_eq!(last, Item::default());
        assert!(!exists);
    }

    #[test]
    fn test_last_with_partial_empty_struct() {
        let item = Item {
            name: "Partial".to_string(),
            value: 0,
        };
        let collection = vec![item.clone()];
        let (last_item, exists) = last(&collection);
        assert_eq!(last_item, item);
        assert!(exists);
    }
}
