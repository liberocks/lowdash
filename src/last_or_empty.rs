use crate::last::last;

/// Returns the last item from the collection.
/// If the collection is empty, returns the default value of `T`.
///
/// # Arguments
/// * `collection` - A slice of items.
///
/// # Returns
/// * `T` - The last item in the collection.
///         If the collection is empty, returns `T::default()`.
///
/// # Examples
/// ```rust
/// use lowdash::last_or_empty;
///
/// let numbers = vec![1, 2, 3];
/// let last_num = last_or_empty(&numbers);
/// assert_eq!(last_num, 3);
///
/// let empty: Vec<i32> = vec![];
/// let last_num = last_or_empty(&empty);
/// assert_eq!(last_num, 0); // i32::default() is 0
/// ```
///
/// ```rust
/// use lowdash::last_or_empty;
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
/// let last_person = last_or_empty(&people);
/// assert_eq!(last_person, Person { name: "Bob".to_string(), age: 30 });
///
/// let empty_people: Vec<Person> = vec![];
/// let last_person = last_or_empty(&empty_people);
/// assert_eq!(last_person, Person::default());
/// ```
pub fn last_or_empty<T>(collection: &[T]) -> T
where
    T: Clone + Default,
{
    let (item, _) = last(collection);
    item
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
    fn test_last_or_empty_with_non_empty_collection() {
        let collection = vec![1, 2, 3];
        let last = last_or_empty(&collection);
        assert_eq!(last, 3);
    }

    #[test]
    fn test_last_or_empty_with_empty_collection() {
        let collection: Vec<i32> = vec![];
        let last = last_or_empty(&collection);
        assert_eq!(last, 0); // i32::default() is 0
    }

    #[test]
    fn test_last_or_empty_with_structs() {
        let item1 = Item {
            name: "Item1".to_string(),
            value: 10,
        };
        let item2 = Item {
            name: "Item2".to_string(),
            value: 20,
        };
        let collection = vec![item1.clone(), item2.clone()];
        let last = last_or_empty(&collection);
        assert_eq!(last, item2);
    }

    #[test]
    fn test_last_or_empty_with_single_item() {
        let collection = vec![42];
        let last = last_or_empty(&collection);
        assert_eq!(last, 42);
    }

    #[test]
    fn test_last_or_empty_with_custom_default() {
        let collection: Vec<Item> = vec![];
        let last = last_or_empty(&collection);
        assert_eq!(last, Item::default());
    }

    #[test]
    fn test_last_or_empty_with_partial_empty_struct() {
        let item = Item {
            name: "Partial".to_string(),
            value: 0,
        };
        let collection = vec![item.clone()];
        let last = last_or_empty(&collection);
        assert_eq!(last, item);
    }
}
