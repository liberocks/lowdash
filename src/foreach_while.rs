/// Execute a function on each item in a collection until the iteratee returns `false`.
///
/// This function iterates over a collection, applying the provided `iteratee` function
/// to each item along with its index. If the `iteratee` returns `false`, the iteration stops.
///
/// # Arguments
/// * `collection` - A slice of items.
/// * `iteratee` - A function that takes a reference to an item and its index, returning a boolean.
///                If `false` is returned, the iteration stops.
///
/// # Examples
/// ```rust
/// use lowdash::foreach_while;
/// let numbers = vec![1, 2, 3, 4, 5];
/// let mut sum = 0;
/// foreach_while(&numbers, |x, _| {
///     sum += x;
///     true
/// });
/// assert_eq!(sum, 15);
/// ```
///
/// ```rust
/// use lowdash::foreach_while;
///
/// #[derive(Debug, PartialEq)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let people = vec![
///     Person { name: "Alice".to_string(), age: 25 },
///     Person { name: "Bob".to_string(), age: 30 },
///     Person { name: "Carol".to_string(), age: 35 },
/// ];
///
/// let mut names = Vec::new();
/// foreach_while(&people, |p, _| {
///     names.push(p.name.clone());
///     true
/// });
/// assert_eq!(names, vec!["Alice", "Bob", "Carol"]);
/// ```
///
/// ```rust
/// use lowdash::foreach_while;
///
/// let numbers = vec![10, 20, 30, 40, 50];
/// let mut collected = Vec::new();
/// foreach_while(&numbers, |x, index| {
///     if *x < 35 {
///         collected.push((*x, index));
///         true
///     } else {
///         false
///     }
/// });
/// assert_eq!(collected, vec![(10, 0), (20, 1), (30, 2)]);
/// ```
pub fn foreach_while<T, F>(collection: &[T], mut iteratee: F)
where
    F: FnMut(&T, usize) -> bool,
{
    for (index, item) in collection.iter().enumerate() {
        if !iteratee(item, index) {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ... other tests ...

    #[test]
    fn test_foreach_while_with_floats() {
        let float_collection = vec![1.1, 2.2, 3.3, 4.4];
        let mut product: f64 = 1.0;
        foreach_while(&float_collection, |x, _| {
            product *= x;
            true
        });
        assert!((product - 35.1384).abs() < 1e-10); // Corrected expected product: 35.1384
    }

    // ... remaining tests ...
}
