/// Generates `0..element_num`, or a descending range from `0` for a negative
/// `element_num`.
///
/// # Arguments
/// * `element_num` - Number of elements and direction.
///
/// # Returns
/// * `Vec<i32>` - The generated range.
///
/// # Examples
/// ```rust
/// use lowdash::range;
/// let result = range(5);
/// assert_eq!(result, vec![0, 1, 2, 3, 4]);
/// ```
///
/// ```rust
/// use lowdash::range;
/// let result = range(-5);
/// assert_eq!(result, vec![0, -1, -2, -3, -4]);
/// ```
pub fn range(element_num: i32) -> Vec<i32> {
    let length = if element_num < 0 {
        -element_num
    } else {
        element_num
    } as usize;
    let step = if element_num < 0 { -1 } else { 1 };
    let mut result = Vec::with_capacity(length);
    for i in 0..length {
        result.push((i as i32) * step);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_positive() {
        let result = range(5);
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_range_negative() {
        let result = range(-5);
        assert_eq!(result, vec![0, -1, -2, -3, -4]);
    }

    #[test]
    fn test_range_zero() {
        let result = range(0);
        assert_eq!(result, Vec::<i32>::new());
    }
}
