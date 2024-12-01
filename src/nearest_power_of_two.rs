/// Calculates the smallest power of two greater than or equal to the given capacity.
///
/// This function determines the nearest power of two that is not less than the provided `cap`. If
/// the calculated power of two exceeds the predefined `MAXIMUM_CAPACITY`, it returns `MAXIMUM_CAPACITY`.
///
/// # Arguments
///
/// * `cap` - The target capacity. Must be a non-negative integer.
///
/// # Returns
///
/// * `usize` - The nearest power of two greater than or equal to `cap`.
///
/// # Panics
///
/// * This function does not panic.
///
/// # Examples
///
/// ```rust
/// use lowdash::nearest_power_of_two;
///
/// assert_eq!(nearest_power_of_two(0), 1);
/// assert_eq!(nearest_power_of_two(1), 1);
/// assert_eq!(nearest_power_of_two(5), 8);
/// assert_eq!(nearest_power_of_two(16), 16);
/// assert_eq!(nearest_power_of_two(17), 32);
/// ```
pub fn nearest_power_of_two(cap: usize) -> usize {
    const MAXIMUM_CAPACITY: usize = 1 << 30; // Example maximum capacity (adjust as needed)

    if cap == 0 {
        return 1;
    }

    let mut n = cap - 1;
    n |= n >> 1;
    n |= n >> 2;
    n |= n >> 4;
    n |= n >> 8;
    n |= n >> 16;

    // For 64-bit architectures, include additional shifts
    #[cfg(target_pointer_width = "64")]
    {
        n |= n >> 32;
    }

    if n >= MAXIMUM_CAPACITY {
        MAXIMUM_CAPACITY
    } else {
        n + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nearest_power_of_two_zero() {
        assert_eq!(nearest_power_of_two(0), 1);
    }

    #[test]
    fn test_nearest_power_of_two_one() {
        assert_eq!(nearest_power_of_two(1), 1);
    }

    #[test]
    fn test_nearest_power_of_two_two() {
        assert_eq!(nearest_power_of_two(2), 2);
    }

    #[test]
    fn test_nearest_power_of_two_three() {
        assert_eq!(nearest_power_of_two(3), 4);
    }

    #[test]
    fn test_nearest_power_of_two_five() {
        assert_eq!(nearest_power_of_two(5), 8);
    }

    #[test]
    fn test_nearest_power_of_two_sixteen() {
        assert_eq!(nearest_power_of_two(16), 16);
    }

    #[test]
    fn test_nearest_power_of_two_seventeen() {
        assert_eq!(nearest_power_of_two(17), 32);
    }

    #[test]
    fn test_nearest_power_of_two_maximum_capacity() {
        const MAXIMUM_CAPACITY: usize = 1 << 30; // Must match the constant in the function

        assert_eq!(nearest_power_of_two(MAXIMUM_CAPACITY - 1), MAXIMUM_CAPACITY);
        assert_eq!(nearest_power_of_two(MAXIMUM_CAPACITY), MAXIMUM_CAPACITY);
        assert_eq!(nearest_power_of_two(MAXIMUM_CAPACITY + 1), MAXIMUM_CAPACITY);
    }

    #[test]
    fn test_nearest_power_of_two_large_values() {
        assert_eq!(nearest_power_of_two(1 << 25), 1 << 25);
        assert_eq!(nearest_power_of_two((1 << 25) + 1), 1 << 26);
    }

    #[test]
    fn test_nearest_power_of_two_boundary() {
        // Just below a power of two
        assert_eq!(nearest_power_of_two((1 << 10) - 1), 1 << 10);

        // Exactly a power of two
        assert_eq!(nearest_power_of_two(1 << 20), 1 << 20);

        // Just above a power of two
        assert_eq!(nearest_power_of_two((1 << 15) + 1), 1 << 16);
    }
}
