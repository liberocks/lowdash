/// Returns the first item with the smallest key produced by `iteratee`.
///
/// Equal keys keep the first item encountered. The key is computed once per
/// item and does not need to be cloned or stored after comparison.
///
/// **Time Complexity:** O(n), plus key extraction, with O(1) extra space apart
/// from the returned clone.
///
/// # Examples
/// ```rust
/// use lowdash::min_by_key;
///
/// let words = ["rust", "is", "fun"];
/// assert_eq!(min_by_key(&words, |word| word.len()), Some("is"));
/// ```
pub fn min_by_key<T, K, F>(collection: &[T], iteratee: F) -> Option<T>
where
    T: Clone,
    K: Ord,
    F: Fn(&T) -> K,
{
    let mut items = collection.iter();
    let first = items.next()?;
    let mut min_item = first.clone();
    let mut min_key = iteratee(first);

    for item in items {
        let key = iteratee(item);
        if key < min_key {
            min_key = key;
            min_item = item.clone();
        }
    }

    Some(min_item)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_the_item_with_the_smallest_key() {
        let numbers = [5, 3, 8, 1, 4];

        assert_eq!(min_by_key(&numbers, |number| *number), Some(1));
    }

    #[test]
    fn keeps_the_first_item_on_a_tie() {
        #[derive(Debug, PartialEq, Clone)]
        struct Record {
            name: &'static str,
            score: u8,
        }

        let records = [
            Record {
                name: "first",
                score: 10,
            },
            Record {
                name: "second",
                score: 10,
            },
        ];

        assert_eq!(
            min_by_key(&records, |record| record.score),
            Some(Record {
                name: "first",
                score: 10,
            })
        );
    }

    #[test]
    fn handles_empty_and_single_item_inputs() {
        let empty: [i32; 0] = [];

        assert_eq!(min_by_key(&empty, |number| *number), None);
        assert_eq!(min_by_key(&[42], |number| *number), Some(42));
    }

    #[test]
    fn supports_custom_key_types() {
        #[derive(Debug, PartialEq, Clone)]
        struct Person {
            name: String,
            age: u32,
        }

        let people = [
            Person {
                name: "Alice".to_string(),
                age: 30,
            },
            Person {
                name: "Bob".to_string(),
                age: 25,
            },
            Person {
                name: "Carol".to_string(),
                age: 35,
            },
        ];

        assert_eq!(
            min_by_key(&people, |person| person.age),
            Some(Person {
                name: "Bob".to_string(),
                age: 25,
            })
        );
    }
}
