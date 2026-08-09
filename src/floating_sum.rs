/// Returns an accurately compensated sum of floating-point values.
///
/// Empty input returns `0.0`. A `NaN` input returns `NaN`; one-sided
/// infinities return that infinity, while both positive and negative infinity
/// return `NaN`. Finite overflow returns an infinity after cancellation is
/// accounted for where possible.
///
/// # Complexity
///
/// Runs in `O(n)` time and `O(1)` additional space.
///
/// # Examples
///
/// ```rust
/// use lowdash::floating_sum;
///
/// assert_eq!(floating_sum(&[1.0, 2.0, 3.0]), 6.0);
/// assert_eq!(floating_sum(&[]), 0.0);
/// ```
pub fn floating_sum(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }

    let mut has_positive_infinity = false;
    let mut has_negative_infinity = false;
    let mut maximum_magnitude = 0.0_f64;
    let mut sum = 0.0;
    let mut compensation = 0.0;

    for &value in values {
        if value.is_nan() {
            return f64::NAN;
        }
        if value == f64::INFINITY {
            has_positive_infinity = true;
        } else if value == f64::NEG_INFINITY {
            has_negative_infinity = true;
        } else {
            maximum_magnitude = maximum_magnitude.max(value.abs());
            let next = sum + value;
            if sum.abs() >= value.abs() {
                compensation += (sum - next) + value;
            } else {
                compensation += (value - next) + sum;
            }
            sum = next;
        }
    }

    if has_positive_infinity && has_negative_infinity {
        return f64::NAN;
    }
    if has_positive_infinity {
        return f64::INFINITY;
    }
    if has_negative_infinity {
        return f64::NEG_INFINITY;
    }
    if maximum_magnitude == 0.0 {
        return 0.0;
    }

    let result = sum + compensation;
    if result.is_finite() {
        result
    } else {
        compensated_sum(values, maximum_magnitude) * maximum_magnitude
    }
}

fn compensated_sum(values: &[f64], scale: f64) -> f64 {
    let mut sum = 0.0;
    let mut compensation = 0.0;

    for &value in values {
        if !value.is_finite() {
            continue;
        }

        let value = value / scale;
        let next = sum + value;
        if sum.abs() >= value.abs() {
            compensation += (sum - next) + value;
        } else {
            compensation += (value - next) + sum;
        }
        sum = next;
    }

    sum + compensation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floating_sum_empty_and_known_values() {
        assert_eq!(floating_sum(&[]), 0.0);
        assert_eq!(floating_sum(&[1.0, 2.0, 3.0, 4.0]), 10.0);
    }

    #[test]
    fn test_floating_sum_compensates_cancellation() {
        assert_eq!(floating_sum(&[1.0e16, 1.0, -1.0e16]), 1.0);
    }

    #[test]
    fn test_floating_sum_handles_finite_overflow_and_cancellation() {
        assert_eq!(floating_sum(&[f64::MAX, f64::MAX]), f64::INFINITY);
        assert_eq!(floating_sum(&[f64::MAX, f64::MAX, -f64::MAX]), f64::MAX);
    }

    #[test]
    fn test_floating_sum_nan_behavior() {
        assert!(floating_sum(&[1.0, f64::NAN]).is_nan());
    }

    #[test]
    fn test_floating_sum_infinity_behavior() {
        assert_eq!(floating_sum(&[f64::INFINITY, 1.0]), f64::INFINITY);
        assert_eq!(floating_sum(&[f64::NEG_INFINITY, -1.0]), f64::NEG_INFINITY);
        assert!(floating_sum(&[f64::INFINITY, f64::NEG_INFINITY]).is_nan());
    }
}
