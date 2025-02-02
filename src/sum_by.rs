use std::ops::Add;

/// Calculates the sum of values obtained by applying a function to each element in a collection.
///
/// # Arguments
/// * `collection` - A slice of items to process.
/// * `iteratee` - A function that maps each item to a numeric value.
///
/// # Returns
/// * `R` - The sum of all values produced by the iteratee function.
///
/// # Examples
/// ```rust
/// use lowdash::sum_by;
///
/// let numbers = vec![1, 2, 3, 4];
/// let result = sum_by(&numbers, |x| x * 2);
/// assert_eq!(result, 20); // (1*2 + 2*2 + 3*2 + 4*2)
/// ```
///
/// ```rust
/// use lowdash::sum_by;
///
/// #[derive(Debug)]
/// struct Person {
///     age: u32,
///     score: f64,
/// }
///
/// let people = vec![
///     Person { age: 25, score: 4.5 },
///     Person { age: 30, score: 3.7 },
///     Person { age: 35, score: 4.2 },
/// ];
///
/// let total_age = sum_by(&people, |p| p.age);
/// assert_eq!(total_age, 90);
///
/// let total_score = sum_by(&people, |p| p.score);
/// assert!((total_score - 12.4).abs() < f64::EPSILON);
/// ```
pub fn sum_by<T, R, F>(collection: &[T], iteratee: F) -> R
where
    F: Fn(&T) -> R,
    R: Add<Output = R> + Default + Copy,
{
    collection
        .iter()
        .fold(R::default(), |acc, item| acc + iteratee(item))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::EPSILON;

    #[test]
    fn test_sum_by_integers() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = sum_by(&numbers, |&x| x * 2);
        assert_eq!(result, 30);
    }

    #[test]
    fn test_sum_by_empty() {
        let numbers: Vec<i32> = vec![];
        let result = sum_by(&numbers, |&x| x * 2);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_sum_by_floats() {
        let numbers: Vec<f64> = vec![1.5, 2.5, 3.5];
        let result = sum_by(&numbers, |&x| x * 2.0);
        assert!((result - 15.0).abs() < EPSILON);
    }

    #[test]
    fn test_sum_by_with_struct() {
        struct Person {
            age: u32,
            score: f64,
        }

        let people = vec![
            Person {
                age: 25,
                score: 4.5,
            },
            Person {
                age: 30,
                score: 3.7,
            },
            Person {
                age: 35,
                score: 4.2,
            },
        ];

        let total_age = sum_by(&people, |p| p.age);
        assert_eq!(total_age, 90);

        let total_score = sum_by(&people, |p| p.score);
        assert!((total_score - 12.4).abs() < 1e-10);
    }

    #[test]
    fn test_sum_by_with_type_conversion() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = sum_by(&numbers, |&x| x as f64 / 2.0);
        assert!((result - 7.5).abs() < EPSILON);
    }

    #[test]
    fn test_sum_by_with_complex_calculation() {
        struct Item {
            quantity: i32,
            price: f64,
        }

        let items = vec![
            Item {
                quantity: 2,
                price: 10.5,
            },
            Item {
                quantity: 3,
                price: 8.75,
            },
            Item {
                quantity: 1,
                price: 15.0,
            },
        ];

        let total_cost = sum_by(&items, |item| item.quantity as f64 * item.price);

        println!("Total cost: {}", total_cost);
        assert!((total_cost - 62.25).abs() < EPSILON);
    }
}
