/// Performs linear interpolation between two values.
///
/// # Arguments
/// * `start` - The starting value
/// * `end` - The ending value
/// * `t` - The interpolation factor between 0.0 and 1.0
///
/// # Returns
/// The interpolated value between start and end
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
    use std::f64::EPSILON;

    #[test]
    fn test_interpolate_basic() {
        let lerp = interpolate(0.0, 10.0);
        assert!((lerp(0.0) - 0.0).abs() < EPSILON);
        assert!((lerp(0.5) - 5.0).abs() < EPSILON);
        assert!((lerp(1.0) - 10.0).abs() < EPSILON);
    }

    #[test]
    fn test_interpolate_negative() {
        let lerp = interpolate(-10.0, 10.0);
        assert!((lerp(0.0) - -10.0).abs() < EPSILON);
        assert!((lerp(0.5) - 0.0).abs() < EPSILON);
        assert!((lerp(1.0) - 10.0).abs() < EPSILON);
    }

    #[test]
    fn test_interpolate_reverse() {
        let lerp = interpolate(10.0, 0.0);
        assert!((lerp(0.0) - 10.0).abs() < EPSILON);
        assert!((lerp(0.5) - 5.0).abs() < EPSILON);
        assert!((lerp(1.0) - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_interpolate_same_value() {
        let lerp = interpolate(5.0, 5.0);
        assert!((lerp(0.0) - 5.0).abs() < EPSILON);
        assert!((lerp(0.5) - 5.0).abs() < EPSILON);
        assert!((lerp(1.0) - 5.0).abs() < EPSILON);
    }

    #[test]
    fn test_interpolate_clamp() {
        let lerp = interpolate(0.0, 10.0);
        // Values less than 0.0 should clamp to 0.0
        assert!((lerp(-0.5) - 0.0).abs() < EPSILON);
        // Values greater than 1.0 should clamp to 1.0
        assert!((lerp(1.5) - 10.0).abs() < EPSILON);
    }

    #[test]
    fn test_interpolate_precision() {
        let lerp = interpolate(0.0, 1.0);
        assert!((lerp(0.333) - 0.333).abs() < EPSILON);
        assert!((lerp(0.667) - 0.667).abs() < EPSILON);
    }

    #[test]
    fn test_interpolate_large_numbers() {
        let lerp = interpolate(1000.0, 2000.0);
        assert!((lerp(0.0) - 1000.0).abs() < EPSILON);
        assert!((lerp(0.5) - 1500.0).abs() < EPSILON);
        assert!((lerp(1.0) - 2000.0).abs() < EPSILON);
    }
}
