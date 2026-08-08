/// Returns the population variance using a stable one-pass calculation.
///
/// An empty collection returns `None`; a singleton has variance `0.0`.
/// Values are converted to `f64` before accumulation.
///
/// **Time Complexity:** O(n) with O(1) additional space.
///
/// # Examples
/// ```rust
/// use lowdash::variance;
///
/// assert_eq!(variance(&[1.0, 2.0, 3.0, 4.0, 5.0]), Some(2.0));
/// ```
pub fn variance<T>(collection: &[T]) -> Option<f64>
where
    T: Copy + Into<f64>,
{
    if collection.is_empty() {
        return None;
    }

    let mut count = 0.0;
    let mut mean = 0.0;
    let mut sum_squared_differences = 0.0;

    for &item in collection {
        let value = item.into();
        count += 1.0;
        let difference_from_mean = value - mean;
        mean += difference_from_mean / count;
        let updated_difference = value - mean;
        sum_squared_differences += difference_from_mean * updated_difference;
    }

    Some(sum_squared_differences / count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_population_variance() {
        assert_eq!(variance(&[1.0, 2.0, 3.0, 4.0, 5.0]), Some(2.0));
    }

    #[test]
    fn returns_none_for_empty_input() {
        let empty: [f64; 0] = [];

        assert_eq!(variance(&empty), None);
    }

    #[test]
    fn returns_zero_for_a_singleton_and_constant_values() {
        assert_eq!(variance(&[42.0]), Some(0.0));
        assert_eq!(variance(&[7.0, 7.0, 7.0]), Some(0.0));
    }

    #[test]
    fn uses_population_not_sample_variance() {
        let values = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];

        assert!((variance(&values).unwrap() - 4.0).abs() < 1e-12);
    }

    #[test]
    fn remains_accurate_for_values_with_a_large_offset() {
        let values = [1_000_000_001.0, 1_000_000_002.0, 1_000_000_003.0];

        assert!((variance(&values).unwrap() - 2.0 / 3.0).abs() < 1e-12);
    }

    #[test]
    fn accepts_f32_and_custom_copy_types() {
        #[derive(Clone, Copy)]
        struct Reading(f64);

        impl From<Reading> for f64 {
            fn from(reading: Reading) -> Self {
                reading.0
            }
        }

        let readings = [Reading(1.0), Reading(2.0), Reading(3.0)];
        assert!((variance(&readings).unwrap() - 2.0 / 3.0).abs() < 1e-12);

        let float_values = [1.0_f32, 2.0, 3.0];
        assert!((variance(&float_values).unwrap() - 2.0 / 3.0).abs() < 1e-6);
    }
}
