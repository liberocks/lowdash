/// Clamps a value between a minimum and maximum value.
/// If the value is less than the minimum, returns the minimum.
/// If the value is greater than the maximum, returns the maximum.
/// Otherwise, returns the value unchanged.
///
/// # Arguments
/// * `value` - The value to clamp.
/// * `min` - The minimum allowed value.
/// * `max` - The maximum allowed value.
///
/// # Returns
/// * `T` - The clamped value.
///
/// # Examples
/// ```rust
/// use lowdash::clamp;
/// assert_eq!(clamp(5, 0, 10), 5);  // Value within range
/// assert_eq!(clamp(-5, 0, 10), 0); // Value below minimum
/// assert_eq!(clamp(15, 0, 10), 10); // Value above maximum
/// ```
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp_within_range() {
        assert_eq!(clamp(5, 0, 10), 5);
        assert_eq!(clamp(3.5, 0.0, 10.0), 3.5);
    }

    #[test]
    fn test_clamp_below_minimum() {
        assert_eq!(clamp(-5, 0, 10), 0);
        assert_eq!(clamp(-3.5, 0.0, 10.0), 0.0);
    }

    #[test]
    fn test_clamp_above_maximum() {
        assert_eq!(clamp(15, 0, 10), 10);
        assert_eq!(clamp(15.5, 0.0, 10.0), 10.0);
    }

    #[test]
    fn test_clamp_equal_to_bounds() {
        assert_eq!(clamp(0, 0, 10), 0);
        assert_eq!(clamp(10, 0, 10), 10);
    }

    #[test]
    fn test_clamp_with_characters() {
        assert_eq!(clamp('b', 'a', 'c'), 'b');
        assert_eq!(clamp('x', 'a', 'c'), 'c');
        assert_eq!(clamp('a', 'b', 'c'), 'b');
    }
}
