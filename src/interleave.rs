/// Interleave multiple collections into a single vector, preserving the order of elements.
///
/// This function takes multiple slices and interleaves their elements into a single `Vec<T>`. It
/// iterates over the collections in a round-robin fashion, taking one element from each collection
/// per iteration. If a collection is exhausted, it is skipped in subsequent iterations.
///
/// **Time Complexity:**  
/// O(n), where n is the total number of elements across all collections.
///
/// # Arguments
///
/// * `collections` - A slice of slices to be interleaved.
///
/// # Type Parameters
///
/// * `T` - The type of elements in the collections. Must implement `Clone`.
/// * `Slice` - The type of the inner slices. Must implement `AsRef<[T]>`.
///
/// # Returns
///
/// * `Vec<T>` - A vector containing the interleaved elements from the input collections.
///
/// # Examples
///
/// ```rust
/// use lowdash::interleave;
///
/// let a = vec![1, 2, 3];
/// let b = vec![4, 5, 6, 7];
/// let c = vec![8, 9];
///
/// let result = interleave(&[&a[..], &b[..], &c[..]]);
/// assert_eq!(result, vec![1, 4, 8, 2, 5, 9, 3, 6, 7]);
/// ```
///
/// ```rust
/// use lowdash::interleave;
///
/// #[derive(Debug, PartialEq, Clone)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let group1 = vec![
///     Person { name: "Alice".to_string(), age: 25 },
///     Person { name: "Bob".to_string(), age: 30 },
/// ];
///
/// let group2 = vec![
///     Person { name: "Carol".to_string(), age: 35 },
/// ];
///
/// let group3 = vec![
///     Person { name: "Dave".to_string(), age: 40 },
///     Person { name: "Eve".to_string(), age: 45 },
///     Person { name: "Frank".to_string(), age: 50 },
/// ];
///
/// let interleaved = interleave(&[&group1[..], &group2[..], &group3[..]]);
/// assert_eq!(
///     interleaved,
///     vec![
///         Person { name: "Alice".to_string(), age: 25 },
///         Person { name: "Carol".to_string(), age: 35 },
///         Person { name: "Dave".to_string(), age: 40 },
///         Person { name: "Bob".to_string(), age: 30 },
///         Person { name: "Eve".to_string(), age: 45 },
///         Person { name: "Frank".to_string(), age: 50 },
///     ]
/// );
/// ```
pub fn interleave<T, Slice>(collections: &[Slice]) -> Vec<T>
where
    Slice: AsRef<[T]>,
    T: Clone,
{
    let max_size = collections
        .iter()
        .map(|c| c.as_ref().len())
        .max()
        .unwrap_or(0);
    let total_size: usize = collections.iter().map(|c| c.as_ref().len()).sum();

    if max_size == 0 {
        return Vec::new();
    }

    let mut result = Vec::with_capacity(total_size);

    for i in 0..max_size {
        for collection in collections {
            let slice = collection.as_ref();
            if i < slice.len() {
                result.push(slice[i].clone());
            }
        }
    }

    result
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
    fn test_interleave_integers() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6, 7];
        let c = vec![8, 9];

        let result = interleave(&[&a[..], &b[..], &c[..]]);
        assert_eq!(result, vec![1, 4, 8, 2, 5, 9, 3, 6, 7]);
    }

    #[test]
    fn test_interleave_strings() {
        let a = vec!["a".to_string(), "b".to_string()];
        let b = vec!["c".to_string()];
        let c = vec!["d".to_string(), "e".to_string(), "f".to_string()];

        let result = interleave(&[&a[..], &b[..], &c[..]]);
        assert_eq!(result, vec!["a", "c", "d", "b", "e", "f"]);
    }

    #[test]
    fn test_interleave_with_structs() {
        let group1 = vec![
            Person {
                name: "Alice".to_string(),
                age: 25,
            },
            Person {
                name: "Bob".to_string(),
                age: 30,
            },
        ];

        let group2 = vec![Person {
            name: "Carol".to_string(),
            age: 35,
        }];

        let group3 = vec![
            Person {
                name: "Dave".to_string(),
                age: 40,
            },
            Person {
                name: "Eve".to_string(),
                age: 45,
            },
            Person {
                name: "Frank".to_string(),
                age: 50,
            },
        ];

        let interleaved = interleave(&[&group1[..], &group2[..], &group3[..]]);
        assert_eq!(
            interleaved,
            vec![
                Person {
                    name: "Alice".to_string(),
                    age: 25
                },
                Person {
                    name: "Carol".to_string(),
                    age: 35
                },
                Person {
                    name: "Dave".to_string(),
                    age: 40
                },
                Person {
                    name: "Bob".to_string(),
                    age: 30
                },
                Person {
                    name: "Eve".to_string(),
                    age: 45
                },
                Person {
                    name: "Frank".to_string(),
                    age: 50
                },
            ]
        );
    }

    #[test]
    fn test_interleave_with_empty_collections() {
        let a: Vec<i32> = vec![];
        let b = vec![1, 2, 3];
        let c: Vec<i32> = vec![];

        let result = interleave(&[&a[..], &b[..], &c[..]]);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_interleave_all_empty() {
        let a: Vec<i32> = vec![];
        let b: Vec<i32> = vec![];
        let c: Vec<i32> = vec![];

        let result = interleave(&[&a[..], &b[..], &c[..]]);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_interleave_single_collection() {
        let a = vec![1, 2, 3, 4, 5];

        let result = interleave(&[&a[..]]);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_interleave_multiple_collections_same_length() {
        let a = vec![1, 2];
        let b = vec![3, 4];
        let c = vec![5, 6];

        let result = interleave(&[&a[..], &b[..], &c[..]]);
        assert_eq!(result, vec![1, 3, 5, 2, 4, 6]);
    }

    #[test]
    fn test_interleave_with_optionals() {
        let a = vec![Some(1), Some(2)];
        let b = vec![None, Some(3), None];
        let c = vec![Some(4)];

        let result = interleave(&[&a[..], &b[..], &c[..]]);
        assert_eq!(result, vec![Some(1), None, Some(4), Some(2), Some(3), None]);
    }

    #[test]
    fn test_interleave_with_floats() {
        let a = vec![1.1, 2.2];
        let b = vec![3.3, 4.4, 5.5];
        let c = vec![6.6];

        let result = interleave(&[&a[..], &b[..], &c[..]]);
        assert_eq!(result, vec![1.1, 3.3, 6.6, 2.2, 4.4, 5.5]);
    }

    #[test]
    fn test_interleave_with_nan_floats() {
        let a = vec![std::f64::NAN, 2.2];
        let b = vec![std::f64::INFINITY, std::f64::NAN];
        let c = vec![1.1];

        let interleaved = interleave(&[&a[..], &b[..], &c[..]]);
        assert_eq!(interleaved.len(), 5);
        assert!(interleaved[0].is_nan());
        assert!(interleaved[1].is_infinite());
        assert!((interleaved[2] - 1.1).abs() < std::f64::EPSILON);
        assert!((interleaved[3] - 2.2).abs() < std::f64::EPSILON);
        assert!(interleaved[4].is_nan());
    }
}
