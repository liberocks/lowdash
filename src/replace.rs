/// Replaces occurrences of a specified value in a collection with a new value, up to a maximum number of replacements.
///
/// This function iterates over a slice of items, replacing each occurrence of `old` with `new` until
/// `n` replacements have been made. If `n` is zero, no replacements are performed. The function
/// preserves the order of elements and does not modify the original collection.
///
/// **Time Complexity:** O(n), where n is the number of elements in the collection.
///
/// # Arguments
///
/// * `collection` - A slice of items in which to perform replacements.
/// * `old` - The value to be replaced.
/// * `new` - The value to replace with.
/// * `n` - The maximum number of replacements to perform. If `n` is zero, no replacements are done.
///
/// # Type Parameters
///
/// * `T` - The type of elements in the collection. Must implement `PartialEq` and `Clone`.
///
/// # Returns
///
/// * `Vec<T>` - A new vector with the specified replacements made.
///
/// # Examples
///
/// ```rust
/// use lowdash::replace;
///
/// let numbers = vec![1, 2, 2, 3, 4, 2, 5];
/// let result = replace(&numbers, 2, 9, 2);
/// assert_eq!(result, vec![1, 9, 9, 3, 4, 2, 5]);
/// ```
///
/// ```rust
/// use lowdash::replace;
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
/// let new_person = Person { name: "Dave".to_string(), age: 40 };
/// let result = replace(&people, people[0].clone(), new_person.clone(), 1);
/// assert_eq!(result, vec![new_person, people[1].clone(), people[2].clone(), people[3].clone()]);
/// ```
pub fn replace<T>(collection: &[T], old: T, new: T, mut n: usize) -> Vec<T>
where
    T: PartialEq + Clone,
{
    let mut result = collection.to_vec();
    for item in &mut result {
        if *item == old && n > 0 {
            *item = new.clone();
            n -= 1;
            if n == 0 {
                break;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_basic() {
        let numbers = vec![1, 2, 2, 3, 4, 2, 5];
        let result = replace(&numbers, 2, 9, 2);
        assert_eq!(result, vec![1, 9, 9, 3, 4, 2, 5]);
    }

    #[test]
    fn test_replace_no_replacements() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = replace(&numbers, 2, 9, 0);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_replace_all_occurrences() {
        let numbers = vec![1, 2, 2, 3, 2, 4, 2, 5];
        let result = replace(&numbers, 2, 9, 10);
        assert_eq!(result, vec![1, 9, 9, 3, 9, 4, 9, 5]);
    }

    #[test]
    fn test_replace_more_than_occurrences() {
        let numbers = vec![1, 2, 3, 2, 4];
        let result = replace(&numbers, 2, 9, 5);
        assert_eq!(result, vec![1, 9, 3, 9, 4]);
    }

    #[test]
    fn test_replace_no_matching_elements() {
        let numbers = vec![1, 3, 4, 5];
        let result = replace(&numbers, 2, 9, 2);
        assert_eq!(result, vec![1, 3, 4, 5]);
    }

    #[test]
    fn test_replace_with_structs() {
        #[derive(Debug, PartialEq, Clone)]
        struct Person {
            name: String,
            age: u32,
        }

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
        let result = replace(&people, alice.clone(), dave.clone(), 1);
        assert_eq!(
            result,
            vec![dave.clone(), bob.clone(), alice.clone(), carol.clone()]
        );
    }

    #[test]
    fn test_replace_with_strings() {
        let fruits = vec!["apple", "banana", "apple", "cherry", "banana"];
        let result = replace(&fruits, "banana", "orange", 1);
        assert_eq!(result, vec!["apple", "orange", "apple", "cherry", "banana"]);
    }

    #[test]
    fn test_replace_with_floats() {
        let floats = vec![1.1, 2.2, 2.2, 3.3, 2.2, 4.4];
        let result = replace(&floats, 2.2, 9.9, 2);
        assert_eq!(result, vec![1.1, 9.9, 9.9, 3.3, 2.2, 4.4]);
    }

    #[test]
    fn test_replace_in_empty_collection() {
        let empty: Vec<i32> = vec![];
        let result = replace(&empty, 1, 9, 1);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_replace_with_characters() {
        let chars = vec!['a', 'b', 'a', 'c', 'b', 'd'];
        let result = replace(&chars, 'a', 'z', 1);
        assert_eq!(result, vec!['z', 'b', 'a', 'c', 'b', 'd']);
    }

    #[test]
    fn test_replace_all_same_elements() {
        let numbers = vec![2, 2, 2, 2];
        let result = replace(&numbers, 2, 9, 4);
        assert_eq!(result, vec![9, 9, 9, 9]);
    }

    #[test]
    fn test_replace_partial_structs() {
        #[derive(Debug, PartialEq, Clone)]
        struct Item {
            id: u32,
            value: String,
        }

        let item1 = Item {
            id: 1,
            value: "one".to_string(),
        };
        let item2 = Item {
            id: 2,
            value: "two".to_string(),
        };
        let _ = Item {
            id: 1,
            value: "one".to_string(),
        };
        let item4 = Item {
            id: 3,
            value: "three".to_string(),
        };

        let items = vec![item1.clone(), item2.clone(), item1.clone(), item4.clone()];
        let result = replace(&items, item1.clone(), item4.clone(), 2);
        assert_eq!(
            result,
            vec![item4.clone(), item2.clone(), item4.clone(), item4.clone()]
        );
    }
}
