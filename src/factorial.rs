/// Computes the factorial of `n` using checked `u128` multiplication.
///
/// The factorial of zero is `Some(1)`. Returns `None` when the result cannot
/// be represented by `u128`.
///
/// # Complexity
///
/// Runs in `O(n)` time until overflow and uses `O(1)` space.
///
/// # Examples
///
/// ```rust
/// use lowdash::factorial;
///
/// assert_eq!(factorial(0), Some(1));
/// assert_eq!(factorial(5), Some(120));
/// ```
pub fn factorial(n: u64) -> Option<u128> {
    (1..=n).try_fold(1_u128, |result, value| result.checked_mul(value as u128))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial_zero() {
        assert_eq!(factorial(0), Some(1));
    }

    #[test]
    fn test_factorial_one() {
        assert_eq!(factorial(1), Some(1));
    }

    #[test]
    fn test_factorial_small_value() {
        assert_eq!(factorial(5), Some(120));
    }

    #[test]
    fn test_factorial_largest_u128_value() {
        assert_eq!(
            factorial(34),
            Some(295_232_799_039_604_140_847_618_609_643_520_000_000)
        );
    }

    #[test]
    fn test_factorial_overflow() {
        assert_eq!(factorial(35), None);
        assert_eq!(factorial(u64::MAX), None);
    }
}
