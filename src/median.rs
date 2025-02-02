use crate::percentile;

/// Calculate the median value of a collection.
/// The median is the 50th percentile of a collection.
/// For collections with an even number of elements, the median is the average of the two middle values.
/// The collection will be sorted before calculation.
///
/// # Arguments
/// * `collection` - A slice of items to calculate the median from
///
/// # Returns
/// * `Option<f64>` - The median value, or None if the collection is empty
///
/// # Examples
/// ```rust
/// use lowdash::median;
/// let numbers = vec![1, 3, 5, 2, 4];
/// let result = median(&numbers);
/// assert!((result.unwrap() - 3.0).abs() < f64::EPSILON);
/// ```
///
/// ```rust
/// use lowdash::median;
/// let numbers = vec![1, 2, 3, 4];
/// let result = median(&numbers);
/// assert!((result.unwrap() - 2.5).abs() < f64::EPSILON);
/// ```
pub fn median<T>(collection: &[T]) -> Option<f64>
where
    T: Copy + Into<f64> + PartialOrd,
{
    percentile(collection, 50.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_odd() {
        let numbers = vec![1, 3, 5, 2, 4];
        let result = median(&numbers);
        assert!((result.unwrap() - 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_median_even() {
        let numbers = vec![1, 2, 3, 4];
        let result = median(&numbers);
        assert!((result.unwrap() - 2.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_median_empty() {
        let empty: Vec<i32> = vec![];
        assert_eq!(median(&empty), None);
    }

    #[test]
    fn test_median_single() {
        let numbers = vec![42];
        let result = median(&numbers);
        assert!((result.unwrap() - 42.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_median_unsorted() {
        let numbers = vec![5, 2, 1, 4, 3];
        let result = median(&numbers);
        assert!((result.unwrap() - 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_median_with_duplicates() {
        let numbers = vec![1, 2, 2, 3, 3];
        let result = median(&numbers);
        assert!((result.unwrap() - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_median_float() {
        let numbers = vec![1.5, 2.5, 3.5];
        let result = median(&numbers);
        assert!((result.unwrap() - 2.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_median_negative() {
        let numbers = vec![-5, -2, -1, -4, -3];
        let result = median(&numbers);
        assert!((result.unwrap() - (-3.0)).abs() < f64::EPSILON);
    }
}
