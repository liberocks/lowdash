/// Returns whether two floating-point values are within the supplied
/// relative or absolute tolerance.
///
/// Tolerances must be finite and nonnegative. NaN values return `false`, while
/// matching positive or negative infinities return `true`. With both
/// tolerances set to zero, only exactly equal finite values (including signed
/// zero) and matching infinities are close.
///
/// # Complexity
///
/// Runs in `O(1)` time and uses `O(1)` space.
///
/// # Examples
///
/// ```rust
/// use lowdash::is_close;
///
/// assert!(is_close(100.0, 101.0, 0.01, 0.0));
/// assert!(is_close(0.0, 1.0e-9, 0.0, 1.0e-8));
/// assert!(!is_close(0.0, 1.0, 0.0, 1.0e-8));
/// ```
pub fn is_close(a: f64, b: f64, relative_tolerance: f64, absolute_tolerance: f64) -> bool {
    if !relative_tolerance.is_finite()
        || !absolute_tolerance.is_finite()
        || relative_tolerance < 0.0
        || absolute_tolerance < 0.0
    {
        return false;
    }

    if a.is_nan() || b.is_nan() {
        return false;
    }

    if a == b {
        return true;
    }

    if a.is_infinite() || b.is_infinite() {
        return false;
    }

    let difference = (a - b).abs();
    let scale = a.abs().max(b.abs());
    let tolerance = (relative_tolerance * scale).max(absolute_tolerance);

    difference <= tolerance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_close_exact_and_zero_tolerance_behavior() {
        assert!(is_close(1.0, 1.0, 0.0, 0.0));
        assert!(is_close(0.0, -0.0, 0.0, 0.0));
        assert!(!is_close(1.0, 1.0 + f64::EPSILON * 2.0, 0.0, 0.0));
    }

    #[test]
    fn test_is_close_relative_tolerance() {
        assert!(is_close(100.0, 101.0, 0.01, 0.0));
        assert!(!is_close(100.0, 101.0, 0.009, 0.0));
    }

    #[test]
    fn test_is_close_absolute_tolerance_near_zero() {
        assert!(is_close(0.0, 1.0e-9, 0.0, 1.0e-8));
        assert!(!is_close(0.0, 1.0e-7, 0.0, 1.0e-8));
    }

    #[test]
    fn test_is_close_nan_and_infinity_behavior() {
        assert!(!is_close(f64::NAN, 1.0, 1.0, 1.0));
        assert!(!is_close(1.0, f64::NAN, 1.0, 1.0));
        assert!(is_close(f64::INFINITY, f64::INFINITY, 0.0, 0.0));
        assert!(is_close(f64::NEG_INFINITY, f64::NEG_INFINITY, 0.0, 0.0));
        assert!(!is_close(f64::INFINITY, f64::NEG_INFINITY, 1.0, 1.0));
        assert!(!is_close(f64::INFINITY, 1.0, 1.0, 1.0));
    }

    #[test]
    fn test_is_close_rejects_invalid_tolerances() {
        assert!(!is_close(1.0, 1.0, -0.1, 0.0));
        assert!(!is_close(1.0, 1.0, 0.0, -0.1));
        assert!(!is_close(1.0, 1.0, f64::NAN, 0.0));
        assert!(!is_close(1.0, 1.0, 0.0, f64::INFINITY));
        assert!(!is_close(f64::INFINITY, f64::INFINITY, -0.1, 0.0));
    }
}
