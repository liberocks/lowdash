use crate::first::first;

/// Returns the first item from the collection.
/// If the collection is empty, returns the default value of `T`.
///
/// # Arguments
/// * `collection` - A slice of items.
///
/// # Returns
/// * `T` - The first item in the collection.
///         If the collection is empty, returns `T::default()`.
///
/// # Examples
/// ```rust
/// use lowdash::first_or_empty;
///
/// let numbers = vec![1, 2, 3];
/// let first_num = first_or_empty(&numbers);
/// assert_eq!(first_num, 1);
///
/// let empty: Vec<i32> = vec![];
/// let first_num = first_or_empty(&empty);
/// assert_eq!(first_num, 0); // i32::default() is 0
/// ```
///
/// ```rust
/// use lowdash::first_or_empty;
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
/// let first_person = first_or_empty(&people);
/// assert_eq!(first_person, Person { name: "Alice".to_string(), age: 25 });
///
/// let empty_people: Vec<Person> = vec![];
/// let first_person = first_or_empty(&empty_people);
/// assert_eq!(first_person, Person::default());
/// ```
pub fn first_or_empty<T>(collection: &[T]) -> T
where
    T: Clone + Default,
{
    let (item, _) = first(collection);
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
    fn test_first_or_empty_with_non_empty_collection() {
        let collection = vec![1, 2, 3];
        let first = first_or_empty(&collection);
        assert_eq!(first, 1);
    }

    #[test]
    fn test_first_or_empty_with_empty_collection() {
        let collection: Vec<i32> = vec![];
        let first = first_or_empty(&collection);
        assert_eq!(first, 0); // i32::default() is 0
    }

    #[test]
    fn test_first_or_empty_with_structs() {
        let item1 = Item {
            name: "Item1".to_string(),
            value: 10,
        };
        let item2 = Item {
            name: "Item2".to_string(),
            value: 20,
        };
        let collection = vec![item1.clone(), item2.clone()];
        let first = first_or_empty(&collection);
        assert_eq!(first, item1);
    }

    #[test]
    fn test_first_or_empty_with_single_item() {
        let collection = vec![42];
        let first = first_or_empty(&collection);
        assert_eq!(first, 42);
    }

    #[test]
    fn test_first_or_empty_with_custom_default() {
        let collection: Vec<Item> = vec![];
        let first = first_or_empty(&collection);
        assert_eq!(first, Item::default());
    }

    #[test]
    fn test_first_or_empty_with_partial_empty_struct() {
        let item = Item {
            name: "Partial".to_string(),
            value: 0,
        };
        let collection = vec![item.clone()];
        let first = first_or_empty(&collection);
        assert_eq!(first, item);
    }
}
