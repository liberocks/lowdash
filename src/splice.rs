/// Inserts elements into a collection at a specified index, handling negative indices and overflow.
/// Returns a new `Vec<T>` with the elements inserted.
///
/// **Time Complexity:** O(n), where n is the number of elements in the collection.
///
/// # Arguments
///
/// * `collection` - A slice of items in which to perform the insertion.
/// * `i` - The index at which to insert the elements. Can be negative to indicate an offset from the end.
/// * `elements` - A slice of elements to insert.
///
/// # Type Parameters
///
/// * `T` - The type of elements in the collection. Must implement `Clone`.
///
/// # Returns
///
/// * `Vec<T>` - A new vector with the specified elements inserted.
///
/// # Examples
///
/// ```rust
/// use lowdash::splice;
///
/// let numbers = vec![1, 2, 3, 4, 5];
/// let elements = vec![99, 100];
/// let result = splice(&numbers, 2, &elements);
/// assert_eq!(result, vec![1, 2, 99, 100, 3, 4, 5]);
/// ```
///
/// ```rust
/// use lowdash::splice;
///
/// let numbers = vec![1, 2, 3, 4, 5];
/// let elements = vec![99, 100];
/// // Insert at the end
/// let result = splice(&numbers, 10, &elements);
/// assert_eq!(result, vec![1, 2, 3, 4, 5, 99, 100]);
/// ```
///
/// ```rust
/// use lowdash::splice;
///
/// let numbers = vec![1, 2, 3, 4, 5];
/// let elements = vec![99];
/// // Insert at negative index (-2 means len - 2 = 3)
/// let result = splice(&numbers, -2, &elements);
/// assert_eq!(result, vec![1, 2, 3, 99, 4, 5]);
/// ```
///
/// ```rust
/// use lowdash::splice;
///
/// let numbers = vec![1, 2, 3, 4, 5];
/// let elements = vec![99];
/// // Negative index beyond the start, insert at beginning
/// let result = splice(&numbers, -10, &elements);
/// assert_eq!(result, vec![99, 1, 2, 3, 4, 5]);
/// ```
pub fn splice<T>(collection: &[T], i: isize, elements: &[T]) -> Vec<T>
where
    T: Clone,
{
    let size_collection = collection.len() as isize;
    let size_elements = elements.len();

    let mut output = Vec::with_capacity(collection.len() + size_elements);

    if size_elements == 0 {
        output.extend_from_slice(collection);
        return output;
    }

    let mut index = i;

    if index > size_collection {
        index = size_collection;
    } else if index < -size_collection {
        index = -size_collection;
    }

    if index < 0 {
        index += size_collection;
        if index < 0 {
            index = 0;
        }
    }

    let usize_index = index as usize;

    output.extend_from_slice(&collection[..usize_index]);
    output.extend_from_slice(elements);
    output.extend_from_slice(&collection[usize_index..]);

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    struct Person {
        name: String,
        age: u32,
    }

    #[test]
    fn test_splice_basic_insert() {
        let numbers = vec![1, 2, 3, 4, 5];
        let elements = vec![99, 100];
        let result = splice(&numbers, 2, &elements);
        assert_eq!(result, vec![1, 2, 99, 100, 3, 4, 5]);
    }

    #[test]
    fn test_splice_insert_at_end() {
        let numbers = vec![1, 2, 3, 4, 5];
        let elements = vec![99, 100];
        let result = splice(&numbers, 10, &elements);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 99, 100]);
    }

    #[test]
    fn test_splice_insert_negative_index() {
        let numbers = vec![1, 2, 3, 4, 5];
        let elements = vec![99];
        let result = splice(&numbers, -2, &elements);
        assert_eq!(result, vec![1, 2, 3, 99, 4, 5]);
    }

    #[test]
    fn test_splice_negative_index_beyond_start() {
        let numbers = vec![1, 2, 3, 4, 5];
        let elements = vec![99];
        let result = splice(&numbers, -10, &elements);
        assert_eq!(result, vec![99, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_splice_no_elements() {
        let numbers = vec![1, 2, 3, 4, 5];
        let elements: Vec<i32> = vec![];
        let result = splice(&numbers, 2, &elements);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_splice_insert_at_start() {
        let numbers = vec![2, 3, 4];
        let elements = vec![0, 1];
        let result = splice(&numbers, 0, &elements);
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_splice_insert_all_elements() {
        let numbers = vec![1, 2, 3];
        let elements = vec![4, 5, 6];
        let result = splice(&numbers, 1, &elements);
        assert_eq!(result, vec![1, 4, 5, 6, 2, 3]);
    }

    #[test]
    fn test_splice_insert_at_exemplary_middle() {
        let numbers = vec![1, 2, 3, 4, 5];
        let elements = vec![99];
        let result = splice(&numbers, 3, &elements);
        assert_eq!(result, vec![1, 2, 3, 99, 4, 5]);
    }

    #[test]
    fn test_splice_with_structs() {
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

        let people = vec![alice.clone(), bob.clone(), carol.clone()];
        let elements = vec![dave.clone()];
        let result = splice(&people, 1, &elements);
        assert_eq!(
            result,
            vec![alice.clone(), dave.clone(), bob.clone(), carol.clone()]
        );
    }

    #[test]
    fn test_splice_with_strings() {
        let fruits = vec!["apple", "banana", "cherry"];
        let elements = vec!["date", "elderberry"];
        let result = splice(&fruits, 2, &elements);
        assert_eq!(
            result,
            vec!["apple", "banana", "date", "elderberry", "cherry"]
        );
    }

    #[test]
    fn test_splice_with_floats() {
        let numbers = vec![1.1, 2.2, 3.3];
        let elements = vec![4.4, 5.5];
        let result = splice(&numbers, -1, &elements);
        assert_eq!(result, vec![1.1, 2.2, 4.4, 5.5, 3.3]);
    }

    #[test]
    fn test_splice_insert_zero_elements() {
        let numbers = vec![1, 2, 3];
        let elements: Vec<i32> = vec![];
        let result = splice(&numbers, 1, &elements);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_splice_insert_at_negative_index_consider_length() {
        let numbers = vec![1, 2, 3, 4];
        let elements = vec![99];
        let result = splice(&numbers, -3, &elements); // len=4, -3 => 1
        assert_eq!(result, vec![1, 99, 2, 3, 4]);
    }
}
