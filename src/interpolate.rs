/// Returns a function that linearly interpolates from `start` to `end`.
/// Its input is clamped to `0.0..=1.0`.
///
/// # Arguments
/// * `start` - Starting value.
/// * `end` - Ending value.
///
/// # Returns
/// * `impl Fn(f64) -> f64` - The interpolation function.
///
/// # Examples
/// ```rust
/// use lowdash::interpolate;
/// let lerp = interpolate(0.0, 10.0);
/// assert_eq!(lerp(0.5), 5.0);
/// ```
///
/// ```rust
/// use lowdash::interpolate;
/// let lerp = interpolate(-10.0, 10.0);
/// assert_eq!(lerp(0.25), -5.0);
/// assert_eq!(lerp(0.75), 5.0);
/// ```
pub fn interpolate(start: f64, end: f64) -> impl Fn(f64) -> f64 {
    move |t| start + (end - start) * t.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolate_basic() {
        let lerp = interpolate(0.0, 10.0);
        assert!((lerp(0.0) - 0.0).abs() < f64::EPSILON);
        assert!((lerp(0.5) - 5.0).abs() < f64::EPSILON);
        assert!((lerp(1.0) - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_interpolate_negative() {
        let lerp = interpolate(-10.0, 10.0);
        assert!((lerp(0.0) - -10.0).abs() < f64::EPSILON);
        assert!((lerp(0.5) - 0.0).abs() < f64::EPSILON);
        assert!((lerp(1.0) - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_interpolate_reverse() {
        let lerp = interpolate(10.0, 0.0);
        assert!((lerp(0.0) - 10.0).abs() < f64::EPSILON);
        assert!((lerp(0.5) - 5.0).abs() < f64::EPSILON);
        assert!((lerp(1.0) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_interpolate_same_value() {
        let lerp = interpolate(5.0, 5.0);
        assert!((lerp(0.0) - 5.0).abs() < f64::EPSILON);
        assert!((lerp(0.5) - 5.0).abs() < f64::EPSILON);
        assert!((lerp(1.0) - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_interpolate_clamp() {
        let lerp = interpolate(0.0, 10.0);
        // Values less than 0.0 should clamp to 0.0
        assert!((lerp(-0.5) - 0.0).abs() < f64::EPSILON);
        // Values greater than 1.0 should clamp to 1.0
        assert!((lerp(1.5) - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_interpolate_precision() {
        let lerp = interpolate(0.0, 1.0);
        assert!((lerp(0.333) - 0.333).abs() < f64::EPSILON);
        assert!((lerp(0.667) - 0.667).abs() < f64::EPSILON);
    }

    #[test]
    fn test_interpolate_large_numbers() {
        let lerp = interpolate(1000.0, 2000.0);
        assert!((lerp(0.0) - 1000.0).abs() < f64::EPSILON);
        assert!((lerp(0.5) - 1500.0).abs() < f64::EPSILON);
        assert!((lerp(1.0) - 2000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_interpolate_nan_t() {
        let lerp = interpolate(0.0, 10.0);
        let result = lerp(f64::NAN);
        assert!(result.is_nan());
    }
}
