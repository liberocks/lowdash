/// Calculates the sum of all elements in a collection.
/// Works with any numeric type that implements `std::ops::Add` and can be copied.
///
/// # Arguments
/// * `collection` - A slice of numeric values.
///
/// # Returns
/// * `T` - The sum of all elements in the collection.
///
/// # Examples
/// ```rust
/// use lowdash::sum;
///
/// // Integer sum
/// assert_eq!(sum(&[1, 2, 3, 4, 5]), 15);
///
/// // Float sum
/// assert_eq!(sum(&[1.1, 2.2, 3.3]), 6.6);
/// ```
pub fn sum<T>(collection: &[T]) -> T
where
    T: std::ops::Add<Output = T> + Copy + Default,
{
    collection.iter().fold(T::default(), |acc, &x| acc + x)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::EPSILON;

    #[test]
    fn test_sum_integers() {
        assert_eq!(sum(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(sum(&[-1, -2, -3]), -6);
        assert_eq!(sum(&[0, 0, 0]), 0);
    }

    #[test]
    fn test_sum_floats() {
        let result: f64 = sum(&[1.1, 2.2, 3.3]);
        assert!((result - 6.6).abs() < EPSILON);

        let result: f64 = sum(&[-1.5, 2.5, -3.5]);
        assert!((result - (-2.5)).abs() < EPSILON);
    }

    #[test]
    fn test_sum_empty() {
        assert_eq!(sum::<i32>(&[]), 0);
        assert_eq!(sum::<f64>(&[]), 0.0);
    }

    #[test]
    fn test_sum_single_element() {
        assert_eq!(sum(&[42]), 42);
        assert_eq!(sum(&[3.14]), 3.14);
    }
}
