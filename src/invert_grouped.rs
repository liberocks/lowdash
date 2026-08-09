use std::collections::HashMap;
use std::hash::Hash;

/// Groups every input key under its value.
///
/// Unlike a simple inversion, repeated values retain all of their keys. The
/// order of keys in each output vector follows the source map's iteration
/// order, which is unspecified for `HashMap`.
///
/// **Time Complexity:** O(n) expected for `n` input entries, with O(n) output
/// space.
///
/// # Examples
/// ```rust
/// use lowdash::invert_grouped;
/// use std::collections::HashMap;
///
/// let mut input = HashMap::new();
/// input.insert("a", 1);
/// input.insert("b", 1);
/// input.insert("c", 2);
///
/// let result = invert_grouped(&input);
/// assert_eq!(result.get(&1).map(Vec::len), Some(2));
/// assert_eq!(result.get(&2), Some(&vec!["c"]));
/// ```
pub fn invert_grouped<K, V>(input: &HashMap<K, V>) -> HashMap<V, Vec<K>>
where
    K: Clone,
    V: Eq + Hash + Clone,
{
    let mut result: HashMap<V, Vec<K>> = HashMap::with_capacity(input.len());

    for (key, value) in input {
        result.entry(value.clone()).or_default().push(key.clone());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_all_keys_for_repeated_values() {
        let mut input = HashMap::new();
        input.insert("a", 1);
        input.insert("b", 1);
        input.insert("c", 2);

        let result = invert_grouped(&input);
        let ones = result.get(&1).expect("value 1 should be present");

        assert_eq!(ones.len(), 2);
        assert!(ones.contains(&"a"));
        assert!(ones.contains(&"b"));
        assert_eq!(result.get(&2), Some(&vec!["c"]));
    }

    #[test]
    fn handles_empty_maps() {
        let input: HashMap<&str, i32> = HashMap::new();

        assert!(invert_grouped(&input).is_empty());
    }

    #[test]
    fn keeps_one_key_values_in_their_group() {
        let mut input = HashMap::new();
        input.insert(1, "one");
        input.insert(2, "two");

        let result = invert_grouped(&input);

        assert_eq!(result.get("one"), Some(&vec![1]));
        assert_eq!(result.get("two"), Some(&vec![2]));
    }

    #[test]
    fn works_with_custom_keys_and_values() {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        struct Key(u8);

        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        struct Value(&'static str);

        let mut input = HashMap::new();
        input.insert(Key(1), Value("shared"));
        input.insert(Key(2), Value("shared"));
        input.insert(Key(3), Value("other"));

        let result = invert_grouped(&input);
        let shared = result
            .get(&Value("shared"))
            .expect("shared value should be present");

        assert_eq!(shared.len(), 2);
        assert!(shared.contains(&Key(1)));
        assert!(shared.contains(&Key(2)));
        assert_eq!(result.get(&Value("other")), Some(&vec![Key(3)]));
    }
}
