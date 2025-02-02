use std::ops::{Add, Sub};

/// Generate a range of numbers from start to end (exclusive) with a specified step.
///
/// # Arguments
/// * `start` - The starting value of the range.
/// * `end` - The ending value of the range (exclusive).
/// * `step` - The increment/decrement value between elements.
///
/// # Returns
/// * `Vec<T>` - A vector containing the range of numbers.
///
/// # Examples
/// ```rust
/// use lowdash::range_with_steps;
/// let result = range_with_steps(1, 5, 1);
/// assert_eq!(result, vec![1, 2, 3, 4]);
/// ```
///
/// ```rust
/// use lowdash::range_with_steps;
/// let result = range_with_steps(5.0, 2.0, -1.0);
/// assert_eq!(result, vec![5.0, 4.0, 3.0]);
/// ```
///
/// ```rust
/// use lowdash::range_with_steps;
/// let result = range_with_steps(1, 1, 1);  // Empty range
/// assert_eq!(result, Vec::<i32>::new());
/// ```
pub fn range_with_steps<T>(start: T, end: T, step: T) -> Vec<T>
where
    T: Copy + PartialOrd + Add<Output = T> + Sub<Output = T> + Default,
{
    let mut result = Vec::new();

    if start == end || step == T::default() {
        return result;
    }

    if start < end {
        if step < T::default() {
            return result;
        }
        let mut current = start;
        while current < end {
            result.push(current);
            current = current + step;
        }
    } else {
        if step > T::default() {
            return result;
        }
        let mut current = start;
        while current > end {
            result.push(current);
            current = current + step;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_with_steps_positive() {
        let result = range_with_steps(1, 5, 1);
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_range_with_steps_negative() {
        let result = range_with_steps(5, 1, -1);
        assert_eq!(result, vec![5, 4, 3, 2]);
    }

    #[test]
    fn test_range_with_steps_empty_same_start_end() {
        let result = range_with_steps(1, 1, 1);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_range_with_steps_empty_zero_step() {
        let result = range_with_steps(1, 5, 0);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_range_with_steps_empty_wrong_direction() {
        let result = range_with_steps(1, 5, -1);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_range_with_steps_float() {
        let result = range_with_steps(1.0, 3.0, 0.5);
        assert_eq!(result, vec![1.0, 1.5, 2.0, 2.5]);
    }

    #[test]
    fn test_range_with_steps_float_negative() {
        let result = range_with_steps(3.0, 1.0, -0.5);
        assert_eq!(result, vec![3.0, 2.5, 2.0, 1.5]);
    }

    #[test]
    fn test_range_with_steps_float_empty() {
        let result = range_with_steps(1.0, 1.0, 0.5);
        assert_eq!(result, Vec::<f64>::new());
    }

    #[test]
    fn test_range_with_steps_large_step() {
        let result = range_with_steps(1, 10, 3);
        assert_eq!(result, vec![1, 4, 7]);
    }

    #[test]
    fn test_range_with_steps_large_negative_step() {
        let result = range_with_steps(10, 1, -3);
        assert_eq!(result, vec![10, 7, 4]);
    }
}
