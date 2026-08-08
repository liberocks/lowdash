use std::collections::HashMap;
use std::hash::Hash;

/// Merges maps with a resolver for duplicate keys.
///
/// The resolver receives the incoming key, the value already in the result,
/// and the incoming value. Returning the incoming value gives ordinary
/// later-map-wins behavior. The key already stored in the result is retained.
///
/// **Time Complexity:** O(n) expected, where `n` is the total number of map
/// entries, with O(n) result space.
///
/// # Examples
/// ```rust
/// use lowdash::assign_with;
/// use std::collections::HashMap;
///
/// let mut first = HashMap::new();
/// first.insert("a", 1);
/// let mut second = HashMap::new();
/// second.insert("a", 2);
/// second.insert("b", 3);
///
/// let merged = assign_with(&[first, second], |_, existing, incoming| {
///     existing + incoming
/// });
/// assert_eq!(merged.get("a"), Some(&3));
/// assert_eq!(merged.get("b"), Some(&3));
/// ```
pub fn assign_with<K, V, F>(maps: &[HashMap<K, V>], mut resolver: F) -> HashMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
    F: FnMut(&K, &V, &V) -> V,
{
    let capacity = maps.iter().map(HashMap::len).sum();
    let mut result = HashMap::with_capacity(capacity);

    for map in maps {
        for (key, incoming) in map {
            if let Some(existing) = result.get(key) {
                let resolved = resolver(key, existing, incoming);
                result.insert(key.clone(), resolved);
            } else {
                result.insert(key.clone(), incoming.clone());
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combines_collisions_with_the_resolver() {
        let mut first = HashMap::new();
        first.insert("a", 1);
        let mut second = HashMap::new();
        second.insert("a", 2);
        second.insert("b", 3);

        let merged = assign_with(&[first, second], |_, existing, incoming| {
            existing + incoming
        });

        assert_eq!(merged.get("a"), Some(&3));
        assert_eq!(merged.get("b"), Some(&3));
    }

    #[test]
    fn can_implement_later_map_wins() {
        let mut first = HashMap::new();
        first.insert("a", 1);
        let mut second = HashMap::new();
        second.insert("a", 2);

        let merged = assign_with(&[first, second], |_, _, incoming| incoming.clone());

        assert_eq!(merged.get("a"), Some(&2));
    }

    #[test]
    fn passes_the_key_and_both_values_to_the_resolver() {
        let mut first = HashMap::new();
        first.insert("a", 1);
        let mut second = HashMap::new();
        second.insert("a", 2);

        let merged = assign_with(&[first, second], |key, existing, incoming| {
            assert_eq!(*key, "a");
            assert_eq!(*existing, 1);
            assert_eq!(*incoming, 2);
            existing + incoming
        });

        assert_eq!(merged.get("a"), Some(&3));
    }

    #[test]
    fn handles_empty_and_non_overlapping_maps() {
        let empty: Vec<HashMap<&str, i32>> = Vec::new();
        assert!(assign_with(&empty, |_, _, incoming| incoming.clone()).is_empty());

        let mut first = HashMap::new();
        first.insert("a", 1);
        let mut second = HashMap::new();
        second.insert("b", 2);

        let merged = assign_with(&[first, second], |_, _, incoming| incoming.clone());
        assert_eq!(merged.len(), 2);
        assert_eq!(merged.get("a"), Some(&1));
        assert_eq!(merged.get("b"), Some(&2));
    }

    #[test]
    fn works_with_custom_keys_and_values() {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        struct Key(u8);

        #[derive(Debug, Clone, PartialEq)]
        struct Value {
            amount: i32,
        }

        let mut first = HashMap::new();
        first.insert(Key(1), Value { amount: 4 });
        let mut second = HashMap::new();
        second.insert(Key(1), Value { amount: 5 });

        let merged = assign_with(&[first, second], |_, existing, incoming| Value {
            amount: existing.amount + incoming.amount,
        });

        assert_eq!(merged.get(&Key(1)), Some(&Value { amount: 9 }));
    }
}
