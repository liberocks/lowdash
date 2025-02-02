/// Calculates the specified percentile of a collection.
/// The percentile should be a value between 0 and 100.
/// The collection will be sorted before calculation.
/// Uses linear interpolation between closest ranks for non-integer results.
///
/// # Arguments
/// * `collection` - A slice of items to calculate the percentile from
/// * `p` - The percentile to calculate (0-100)
///
/// # Returns
/// * `Option<f64>` - The calculated percentile value, or None if the collection is empty
///
/// # Examples
/// ```rust
/// use lowdash::percentile;
/// let numbers = vec![1, 2, 3, 4, 5];
/// let result = percentile(&numbers, 50.0);
/// assert!((result.unwrap() - 3.0).abs() < f64::EPSILON);
/// ```
///
/// ```rust
/// use lowdash::percentile;
/// let numbers = vec![1, 2, 3, 4];
/// let result = percentile(&numbers, 75.0);
/// assert!((result.unwrap() - 3.25).abs() < f64::EPSILON);
/// ```
pub fn percentile<T>(collection: &[T], p: f64) -> Option<f64>
where
    T: Copy + Into<f64> + PartialOrd,
{
    if collection.is_empty() {
        return None;
    }

    if p < 0.0 || p > 100.0 {
        return None;
    }

    let mut sorted = collection.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    if p == 0.0 {
        return Some(sorted[0].into());
    }
    if p == 100.0 {
        return Some(sorted[sorted.len() - 1].into());
    }

    let rank = (p / 100.0) * (sorted.len() - 1) as f64;
    let lower_idx = rank.floor() as usize;
    let upper_idx = rank.ceil() as usize;

    if lower_idx == upper_idx {
        return Some(sorted[lower_idx].into());
    }

    let lower_value: f64 = sorted[lower_idx].into();
    let upper_value: f64 = sorted[upper_idx].into();
    let fraction = rank - lower_idx as f64;

    Some(lower_value + (upper_value - lower_value) * fraction)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percentile_empty() {
        let empty: Vec<i32> = vec![];
        assert_eq!(percentile(&empty, 50.0), None);
    }

    #[test]
    fn test_percentile_invalid() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(percentile(&numbers, -1.0), None);
        assert_eq!(percentile(&numbers, 101.0), None);
    }

    #[test]
    fn test_percentile_median() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = percentile(&numbers, 50.0).unwrap();
        assert!((result - 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_percentile_quartiles() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let q1 = percentile(&numbers, 25.0).unwrap();
        let q3 = percentile(&numbers, 75.0).unwrap();
        assert!((q1 - 2.75).abs() < f64::EPSILON);
        assert!((q3 - 6.25).abs() < f64::EPSILON);
    }

    #[test]
    fn test_percentile_extremes() {
        let numbers = vec![1, 2, 3, 4, 5];
        let min = percentile(&numbers, 0.0).unwrap();
        let max = percentile(&numbers, 100.0).unwrap();
        assert!((min - 1.0).abs() < f64::EPSILON);
        assert!((max - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_percentile_unsorted() {
        let numbers = vec![5, 2, 1, 4, 3];
        let result = percentile(&numbers, 50.0).unwrap();
        assert!((result - 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_percentile_with_duplicates() {
        let numbers = vec![1, 2, 2, 3, 3, 4, 5];
        let result = percentile(&numbers, 50.0).unwrap();
        assert!((result - 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_percentile_float() {
        let numbers = vec![1.5, 2.5, 3.5, 4.5, 5.5];
        let result = percentile(&numbers, 50.0).unwrap();
        assert!((result - 3.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_percentile_interpolation() {
        let numbers = vec![1, 2, 3, 4];
        let result = percentile(&numbers, 75.0).unwrap();
        println!("{}", result);
        assert!((result - 3.25).abs() < f64::EPSILON);
    }

    #[test]
    fn test_percentile_single_value() {
        let numbers = vec![42];
        let result = percentile(&numbers, 50.0).unwrap();
        assert!((result - 42.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_percentile_two_values() {
        let numbers = vec![1, 2];
        let result = percentile(&numbers, 25.0).unwrap();
        assert!((result - 1.25).abs() < f64::EPSILON);
    }
}
