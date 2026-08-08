/// Returns the N-dimensional Euclidean norm of `values`.
///
/// Empty input returns `0.0`. NaN inputs return `NaN`, and any infinity returns
/// positive infinity, following ordinary IEEE-754 propagation.
///
/// A scaled sum of squares avoids unnecessary intermediate overflow and
/// underflow for finite values.
///
/// # Complexity
///
/// Runs in `O(n)` time and `O(1)` additional space.
///
/// # Examples
///
/// ```rust
/// use lowdash::euclidean_norm;
///
/// assert_eq!(euclidean_norm(&[3.0, 4.0]), 5.0);
/// assert_eq!(euclidean_norm(&[]), 0.0);
/// ```
pub fn euclidean_norm(values: &[f64]) -> f64 {
    let mut scale = 0.0_f64;
    let mut scaled_sum = 0.0_f64;
    let mut has_infinity = false;

    for &value in values {
        if value.is_nan() {
            return f64::NAN;
        }

        let magnitude = value.abs();
        if magnitude.is_infinite() {
            has_infinity = true;
            continue;
        }
        if magnitude == 0.0 {
            continue;
        }

        if magnitude > scale {
            let ratio = scale / magnitude;
            scaled_sum = 1.0 + scaled_sum * ratio * ratio;
            scale = magnitude;
        } else {
            let ratio = magnitude / scale;
            scaled_sum += ratio * ratio;
        }
    }

    if has_infinity {
        return f64::INFINITY;
    }

    scale * scaled_sum.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_norm_empty_zero_and_known_values() {
        assert_eq!(euclidean_norm(&[]), 0.0);
        assert_eq!(euclidean_norm(&[0.0, -0.0]), 0.0);
        assert_eq!(euclidean_norm(&[3.0, 4.0]), 5.0);
    }

    #[test]
    fn test_euclidean_norm_negative_values() {
        assert_eq!(euclidean_norm(&[-3.0, -4.0]), 5.0);
    }

    #[test]
    fn test_euclidean_norm_handles_large_finite_values() {
        let result = euclidean_norm(&[1.0e200, 1.0e200]);
        let expected = 2.0_f64.sqrt() * 1.0e200;

        assert!((result - expected).abs() / expected < 1.0e-15);
    }

    #[test]
    fn test_euclidean_norm_follows_infinity_behavior() {
        assert_eq!(euclidean_norm(&[f64::INFINITY, 1.0]), f64::INFINITY);
        assert_eq!(euclidean_norm(&[f64::NEG_INFINITY]), f64::INFINITY);
        assert_eq!(euclidean_norm(&[f64::MAX, f64::MAX]), f64::INFINITY);
    }

    #[test]
    fn test_euclidean_norm_nan_behavior() {
        assert!(euclidean_norm(&[1.0, f64::NAN]).is_nan());
        assert!(euclidean_norm(&[f64::INFINITY, f64::NAN]).is_nan());
    }
}
