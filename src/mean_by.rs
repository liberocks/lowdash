/// Calculates the mean value of a collection after applying a transformation function to each element.
///
/// # Arguments
/// * `collection` - A slice of items to calculate the mean from
/// * `iteratee` - A function that transforms each item before calculating the mean
///
/// # Returns
/// The mean value after applying the transformation
///
/// # Examples
/// ```rust
/// use lowdash::mean_by;
///
/// let objects = vec![(1, 4), (2, 6), (3, 8)];
/// let mean = mean_by(&objects, |&(x, _)| x as f64);
/// assert!((mean - 2.0).abs() < f64::EPSILON);
///
/// // Calculate mean of y coordinates
/// let mean_y = mean_by(&objects, |&(_, y)| y as f64);
/// assert!((mean_y - 6.0).abs() < f64::EPSILON);
/// ```
pub fn mean_by<T, F>(collection: &[T], iteratee: F) -> f64
where
    F: Fn(&T) -> f64,
{
    let length = collection.len();
    if length == 0 {
        return 0.0;
    }

    let sum = collection
        .iter()
        .fold(0.0, |acc, item| acc + iteratee(item));

    sum / length as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::EPSILON;

    #[test]
    fn test_mean_by_numbers() {
        let numbers = vec![(1, 10), (2, 20), (3, 30)];

        // Test x coordinates
        let mean_x = mean_by(&numbers, |&(x, _)| x as f64);
        assert!((mean_x - 2.0).abs() < EPSILON);

        // Test y coordinates
        let mean_y = mean_by(&numbers, |&(_, y)| y as f64);
        assert!((mean_y - 20.0).abs() < EPSILON);
    }

    #[test]
    fn test_mean_by_empty() {
        let empty: Vec<(i32, i32)> = vec![];
        let mean = mean_by(&empty, |&(x, _)| x as f64);
        assert_eq!(mean, 0.0);
    }

    #[test]
    fn test_mean_by_single_element() {
        let numbers = vec![(5, 10)];
        let mean = mean_by(&numbers, |&(x, _)| x as f64);
        assert!((mean - 5.0).abs() < EPSILON);
    }
}
