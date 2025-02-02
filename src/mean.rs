use std::ops::{Add, Div};

/// Calculates the arithmetic mean of a collection of numbers.
/// If the collection is empty, returns zero.
///
/// # Arguments
/// * `collection` - A slice of numbers.
///
/// # Returns
/// * `T` - The arithmetic mean of the collection.
///
/// # Examples
/// ```rust
/// use lowdash::mean;
/// let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let result = mean(&numbers);
/// assert_eq!(result, 3.0);
/// ```
///
/// ```rust
/// use lowdash::mean;
/// let numbers = vec![1, 2, 3, 4, 5];
/// let result = mean(&numbers);
/// assert_eq!(result, 3);
/// ```
///
/// ```rust
/// use lowdash::mean;
/// let empty: Vec<f64> = vec![];
/// let result = mean(&empty);
/// assert_eq!(result, 0.0);
/// ```
pub fn mean<T>(collection: &[T]) -> T
where
    T: Add<Output = T> + Div<Output = T> + From<u8> + Copy,
{
    let length = collection.len();
    if length == 0 {
        return T::from(0);
    }

    let sum = collection.iter().fold(T::from(0), |acc, &x| acc + x);

    sum / T::from(length as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean_integers() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(mean(&numbers), 3);
    }

    #[test]
    fn test_mean_floats() {
        let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(mean(&numbers), 3.0);
    }

    #[test]
    fn test_mean_empty() {
        let empty: Vec<f64> = vec![];
        assert_eq!(mean(&empty), 0.0);
    }

    #[test]
    fn test_mean_single_element() {
        let numbers = vec![42];
        assert_eq!(mean(&numbers), 42);
    }

    #[test]
    fn test_mean_negative_numbers() {
        let numbers = vec![-1.0, -2.0, -3.0, -4.0, -5.0];
        assert_eq!(mean(&numbers), -3.0);
    }

    #[test]
    fn test_mean_mixed_positive_negative() {
        let numbers = vec![-2, -1, 0, 1, 2];
        assert_eq!(mean(&numbers), 0);
    }

    #[test]
    fn test_mean_decimal_result() {
        let numbers = vec![1.0, 2.0, 3.0];
        assert_eq!(mean(&numbers), 2.0);
    }
}
