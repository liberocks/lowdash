/// Returns the first item from the collection.
/// If the collection is empty, returns the provided fallback value.
///
/// # Arguments
/// * `collection` - A slice of items.
/// * `fallback` - The value to return if the collection is empty.
///
/// # Returns
/// * `T` - The first item in the collection.
///         If the collection is empty, returns the `fallback` value.
///
/// # Examples
/// ```
/// use lowdash::first_or;
///
/// let numbers = vec![1, 2, 3];
/// let first_num = first_or(&numbers, 10);
/// assert_eq!(first_num, 1);
///
/// let empty: Vec<i32> = vec![];
/// let first_num = first_or(&empty, 10);
/// assert_eq!(first_num, 10);
/// ```
///
/// ```
/// use lowdash::first_or;
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
/// ];
///
/// let first_person = first_or(&people, Person { name: "Default".to_string(), age: 0 });
/// assert_eq!(first_person, Person { name: "Alice".to_string(), age: 25 });
///
/// let empty_people: Vec<Person> = vec![];
/// let first_person = first_or(&empty_people, Person { name: "Default".to_string(), age: 0 });
/// assert_eq!(first_person, Person { name: "Default".to_string(), age: 0 });
/// ```
pub fn first_or<T>(collection: &[T], fallback: T) -> T
where
    T: Clone,
{
    collection.first().cloned().unwrap_or(fallback)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    struct Item {
        name: String,
        value: i32,
    }

    #[test]
    fn test_first_or_with_non_empty_collection() {
        let collection = vec![1, 2, 3];
        let first = first_or(&collection, 10);
        assert_eq!(first, 1);
    }

    #[test]
    fn test_first_or_with_empty_collection() {
        let collection: Vec<i32> = vec![];
        let first = first_or(&collection, 10);
        assert_eq!(first, 10);
    }

    #[test]
    fn test_first_or_with_structs() {
        let item1 = Item {
            name: "Item1".to_string(),
            value: 10,
        };
        let item2 = Item {
            name: "Item2".to_string(),
            value: 20,
        };
        let collection = vec![item1.clone(), item2.clone()];
        let first = first_or(
            &collection,
            Item {
                name: "Fallback".to_string(),
                value: 0,
            },
        );
        assert_eq!(first, item1);
    }

    #[test]
    fn test_first_or_with_single_item() {
        let collection = vec![42];
        let first = first_or(&collection, 100);
        assert_eq!(first, 42);
    }

    #[test]
    fn test_first_or_with_custom_fallback() {
        let collection: Vec<Item> = vec![];
        let fallback = Item {
            name: "Fallback".to_string(),
            value: -1,
        };
        let first = first_or(&collection, fallback.clone());
        assert_eq!(first, fallback);
    }
}
