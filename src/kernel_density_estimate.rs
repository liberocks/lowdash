/// Builds a Gaussian kernel density estimate from `values`.
///
/// The estimator is Gaussian-only. Input data must be nonempty and finite;
/// `bandwidth` must be finite and positive. Invalid input returns `None`.
/// The returned closure owns a cloned copy of the data, returns `NaN` for a
/// nonfinite query, and evaluates the normalized PDF for any finite query.
///
/// # Complexity
///
/// Construction uses `O(n)` time and space. Each closure evaluation uses
/// `O(n)` time and `O(1)` additional space.
///
/// # Examples
///
/// ```rust
/// use lowdash::kernel_density_estimate;
///
/// let estimate = kernel_density_estimate(&[0.0], 1.0).unwrap();
/// let density_at_zero = estimate(0.0);
/// assert!((density_at_zero - 0.39894228).abs() < 1.0e-7);
/// ```
pub fn kernel_density_estimate(values: &[f64], bandwidth: f64) -> Option<impl Fn(f64) -> f64> {
    if values.is_empty()
        || !bandwidth.is_finite()
        || bandwidth <= 0.0
        || values.iter().any(|value| !value.is_finite())
    {
        return None;
    }

    let data = values.to_vec();
    let sample_count = data.len() as f64;
    let gaussian_normalization = 1.0 / (2.0 * std::f64::consts::PI).sqrt();
    let normalization = gaussian_normalization / (sample_count * bandwidth);

    Some(move |x: f64| {
        if !x.is_finite() {
            return f64::NAN;
        }

        let mut kernel_sum = 0.0_f64;
        for &value in &data {
            let standardized = (x - value) / bandwidth;
            kernel_sum += (-0.5 * standardized * standardized).exp();
        }

        if kernel_sum == 0.0 {
            0.0
        } else {
            kernel_sum * normalization
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_density_estimate_known_density() {
        let estimate = kernel_density_estimate(&[0.0], 1.0).unwrap();
        let expected_at_zero = 1.0 / (2.0 * std::f64::consts::PI).sqrt();
        let expected_at_one = expected_at_zero * (-0.5_f64).exp();

        assert!((estimate(0.0) - expected_at_zero).abs() < 1.0e-15);
        assert!((estimate(1.0) - expected_at_one).abs() < 1.0e-15);
    }

    #[test]
    fn test_kernel_density_estimate_is_symmetric_and_normalized() {
        let estimate = kernel_density_estimate(&[0.0], 1.0).unwrap();
        assert_eq!(estimate(-0.75), estimate(0.75));

        let step = 0.1;
        let area: f64 = (-50..=50)
            .map(|index| estimate(index as f64 * step) * step)
            .sum();
        assert!((area - 1.0).abs() < 0.02);
    }

    #[test]
    fn test_kernel_density_estimate_rejects_invalid_input() {
        assert!(kernel_density_estimate(&[], 1.0).is_none());
        assert!(kernel_density_estimate(&[0.0, f64::NAN], 1.0).is_none());
        assert!(kernel_density_estimate(&[0.0, f64::INFINITY], 1.0).is_none());
        assert!(kernel_density_estimate(&[0.0], 0.0).is_none());
        assert!(kernel_density_estimate(&[0.0], -1.0).is_none());
        assert!(kernel_density_estimate(&[0.0], f64::NAN).is_none());
        assert!(kernel_density_estimate(&[0.0], f64::INFINITY).is_none());
    }

    #[test]
    fn test_kernel_density_estimate_deterministic_queries() {
        let estimate = kernel_density_estimate(&[-1.0, 1.0], 0.5).unwrap();
        assert_eq!(estimate(0.25), estimate(0.25));
        assert!(estimate(f64::NAN).is_nan());
        assert!(estimate(f64::INFINITY).is_nan());
    }
}
