use std::collections::HashMap;
use std::hash::Hash;

/// Merges maps while keeping the first value for duplicate keys.
///
/// Later maps only provide keys that are not already present. Map iteration
/// order does not affect the result for distinct keys.
///
/// **Time Complexity:** O(n) expected, where `n` is the total number of map
/// entries, with O(n) result space.
///
/// # Examples
/// ```rust
/// use lowdash::defaults;
/// use std::collections::HashMap;
///
/// let mut primary = HashMap::new();
/// primary.insert("color", "blue");
/// let mut fallback = HashMap::new();
/// fallback.insert("color", "red");
/// fallback.insert("size", "large");
///
/// let result = defaults(&[primary, fallback]);
/// assert_eq!(result["color"], "blue");
/// assert_eq!(result["size"], "large");
/// ```
pub fn defaults<K, V>(maps: &[HashMap<K, V>]) -> HashMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    let capacity = maps
        .iter()
        .fold(0usize, |total, map| total.saturating_add(map.len()));
    let mut result = HashMap::with_capacity(capacity);

    for map in maps {
        for (key, value) in map {
            result.entry(key.clone()).or_insert_with(|| value.clone());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_the_first_value_for_duplicate_keys() {
        let mut first = HashMap::new();
        first.insert("a", 1);
        let mut second = HashMap::new();
        second.insert("a", 2);
        second.insert("b", 3);

        let result = defaults(&[first, second]);

        assert_eq!(result.get("a"), Some(&1));
        assert_eq!(result.get("b"), Some(&3));
    }

    #[test]
    fn fills_missing_keys_from_later_maps() {
        let mut first = HashMap::new();
        first.insert("a", 1);
        let mut second = HashMap::new();
        second.insert("b", 2);
        let mut third = HashMap::new();
        third.insert("c", 3);

        let result = defaults(&[first, second, third]);

        assert_eq!(result.len(), 3);
        assert_eq!(result.get("a"), Some(&1));
        assert_eq!(result.get("b"), Some(&2));
        assert_eq!(result.get("c"), Some(&3));
    }

    #[test]
    fn handles_empty_map_lists_and_maps() {
        let empty_maps: Vec<HashMap<&str, i32>> = Vec::new();
        assert!(defaults(&empty_maps).is_empty());

        let empty: HashMap<&str, i32> = HashMap::new();
        assert!(defaults(&[empty]).is_empty());
    }

    #[test]
    fn works_with_custom_keys_and_values() {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        struct Key(u8);

        #[derive(Debug, Clone, PartialEq)]
        struct Value {
            name: &'static str,
        }

        let mut first = HashMap::new();
        first.insert(Key(1), Value { name: "first" });
        let mut second = HashMap::new();
        second.insert(Key(1), Value { name: "second" });
        second.insert(Key(2), Value { name: "other" });

        let result = defaults(&[first, second]);

        assert_eq!(result.get(&Key(1)), Some(&Value { name: "first" }));
        assert_eq!(result.get(&Key(2)), Some(&Value { name: "other" }));
    }
}
