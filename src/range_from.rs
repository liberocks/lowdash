/// Generate a range of numbers starting from a given value.
/// If `element_num` is negative, generate a range with a step of -1.
///
/// # Arguments
/// * `start` - The starting value of the range.
/// * `element_num` - The number of elements in the range. If negative, generates a descending range.
///
/// # Returns
/// * `Vec<T>` - A vector containing the range of numbers.
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
    let length = if element_num < 0 { -element_num } else { element_num } as usize;
    let step: T = if element_num < 0 { (-1).into() } else { 1.into() };
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
