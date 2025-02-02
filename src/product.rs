/// Calculate the product of all elements in a collection.
/// If the collection is empty, returns 1 (multiplicative identity).
/// Works with any numeric type that implements `std::ops::Mul` and can be copied.
///
/// # Arguments
/// * `collection` - A slice of numbers.
///
/// # Returns
/// * `T` - The product of all numbers in the collection.
///
/// # Examples
/// ```rust
/// use lowdash::product;
///
/// // Integer product
/// let numbers = vec![1, 2, 3, 4, 5];
/// assert_eq!(product(&numbers), 120);
/// ```
///
/// ```rust
/// use lowdash::product;
///
/// // Float product
/// let numbers = vec![1.5, 2.0, 3.0];
/// assert_eq!(product(&numbers), 9.0);
/// ```
///
/// ```rust
/// use lowdash::product;
///
/// // Empty collection returns 1
/// let empty: Vec<i32> = vec![];
/// assert_eq!(product(&empty), 1);
/// ```
pub fn product<T>(collection: &[T]) -> T
where
    T: std::ops::Mul<Output = T> + Copy + From<u8>,
{
    if collection.is_empty() {
        return T::from(1);
    }

    collection.iter().fold(T::from(1), |acc, &x| acc * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_integers() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(product(&numbers), 120);
    }

    #[test]
    fn test_product_floats() {
        let numbers = vec![1.5, 2.0, 3.0];
        assert_eq!(product(&numbers), 9.0);
    }

    #[test]
    fn test_product_empty() {
        let empty: Vec<i32> = vec![];
        assert_eq!(product(&empty), 1);
    }

    #[test]
    fn test_product_single_element() {
        let numbers = vec![42];
        assert_eq!(product(&numbers), 42);
    }

    #[test]
    fn test_product_with_zero() {
        let numbers = vec![1, 2, 0, 4, 5];
        assert_eq!(product(&numbers), 0);
    }

    #[test]
    fn test_product_negative_numbers() {
        let numbers = vec![-2, 3, -4];
        assert_eq!(product(&numbers), 24);
    }

    #[test]
    fn test_product_complex_floats() {
        let numbers = vec![0.5, 2.5, -1.5];
        assert_eq!(product(&numbers), -1.875);
    }
}
