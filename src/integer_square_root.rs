/// Returns the exact floor of the square root of `n`.
///
/// Newton iteration uses integer division instead of multiplying candidate
/// values, so it is safe for every `u128` input, including `u128::MAX`.
///
/// # Complexity
///
/// Runs in `O(log(log(n)))` time and `O(1)` space.
///
/// # Examples
///
/// ```rust
/// use lowdash::integer_square_root;
///
/// assert_eq!(integer_square_root(0), 0);
/// assert_eq!(integer_square_root(16), 4);
/// assert_eq!(integer_square_root(17), 4);
/// ```
pub fn integer_square_root(n: u128) -> u128 {
    if n < 2 {
        return n;
    }

    let shift = (128 - n.leading_zeros()).div_ceil(2);
    let mut root = 1_u128 << shift;

    loop {
        let next = (root + n / root) / 2;
        if next >= root {
            return root;
        }
        root = next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_square_root_zero_and_one() {
        assert_eq!(integer_square_root(0), 0);
        assert_eq!(integer_square_root(1), 1);
    }

    #[test]
    fn test_integer_square_root_between_squares() {
        assert_eq!(integer_square_root(2), 1);
        assert_eq!(integer_square_root(3), 1);
        assert_eq!(integer_square_root(15), 3);
        assert_eq!(integer_square_root(17), 4);
    }

    #[test]
    fn test_integer_square_root_exact_squares() {
        assert_eq!(integer_square_root(4), 2);
        assert_eq!(integer_square_root(16), 4);
        assert_eq!(integer_square_root(10_000), 100);
    }

    #[test]
    fn test_integer_square_root_u128_boundary() {
        assert_eq!(integer_square_root(u128::MAX), u64::MAX as u128);
        assert_eq!(integer_square_root(u128::MAX - 1), u64::MAX as u128);
    }
}
