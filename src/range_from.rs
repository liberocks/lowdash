/// Generates `element_num` values starting at `start`, stepping by `1` or `-1`.
///
/// # Arguments
/// * `start` - First value.
/// * `element_num` - Number of values; negative means descending.
///
/// # Returns
/// * `Vec<T>` - The generated range.
///
/// # Examples
/// ```rust
/// use lowdash::range_from;
/// let result = range_from(5, 3);
/// assert_eq!(result, vec![5, 6, 7]);
/// ```
///
/// ```rust
/// use lowdash::range_from;
/// let result = range_from(5, -3);
/// assert_eq!(result, vec![5, 4, 3]);
/// ```
pub fn range_from<T>(start: T, element_num: i32) -> Vec<T>
where
    T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + From<i32>,
{
    let length = if element_num < 0 {
        (-element_num) as usize
    } else {
        element_num as usize
    };
    let step: T = if element_num < 0 {
        (-1).into()
    } else {
        1.into()
    };
    let mut result = Vec::with_capacity(length);
    let mut current = start;

    for _ in 0..length {
        result.push(current);
        current = current + step;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_from_positive() {
        let result = range_from(5, 3);
        assert_eq!(result, vec![5, 6, 7]);
    }

    #[test]
    fn test_range_from_negative() {
        let result = range_from(5, -3);
        assert_eq!(result, vec![5, 4, 3]);
    }

    #[test]
    fn test_range_from_zero() {
        let result: Vec<i32> = range_from(5, 0);
        assert_eq!(result, Vec::new());
    }

    #[test]
    fn test_range_from_float() {
        let result = range_from(1.0, 3);
        assert_eq!(result, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_range_from_float_negative() {
        let result = range_from(3.0, -3);
        assert_eq!(result, vec![3.0, 2.0, 1.0]);
    }

    #[test]
    fn test_range_from_large() {
        let result = range_from(998, 3);
        assert_eq!(result, vec![998, 999, 1000]);
    }

    #[test]
    fn test_range_from_large_negative() {
        let result = range_from(1000, -3);
        assert_eq!(result, vec![1000, 999, 998]);
    }
}
