use crate::common;

/// Generates a random string of a specified size using the provided charset.
///
/// # Arguments
///
/// * `size` - The length of the generated string. Must be greater than 0.
/// * `charset` - A slice of characters to use for generating the string. Must not be empty.
///
/// # Panics
///
/// * If `size` is less than or equal to 0.
/// * If `charset` is empty.
///
/// # Examples
///
/// ```rust
/// use lowdash::common::ALPHANUMERIC_CHARSET;
/// use lowdash::random_string;
///
/// let charset = ALPHANUMERIC_CHARSET;
/// let random_str = random_string(10, charset);
/// assert_eq!(random_str.len(), 10);
/// for c in random_str.chars() {
///     assert!(charset.contains(&c));
/// }
/// ```
pub fn random_string(size: usize, charset: &[char]) -> String {
    if size == 0 {
        panic!("common::random_string: Size parameter must be greater than 0");
    }
    if charset.is_empty() {
        panic!("common::random_string: Charset parameter must not be empty");
    }

    // Calculate the number of bits required to represent the charset
    let charset_len = charset.len();
    let letter_id_bits = common::ceil_log2(charset_len);
    let letter_id_mask = (1 << letter_id_bits) - 1;
    let letter_id_max = 63 / letter_id_bits;

    let mut result = String::with_capacity(size);
    let mut bits_remaining = 0;
    let mut cache: u64 = 0;

    for _ in 0..size {
        if bits_remaining == 0 {
            cache = common::random_u64();
            bits_remaining = letter_id_max;
        }

        let idx = (cache & letter_id_mask as u64) as usize;
        cache >>= letter_id_bits;
        bits_remaining -= 1;

        if idx < charset_len {
            result.push(charset[idx]);
        } else {
            // If the index is out of range, retry with a new random number
            let new_cache = common::random_u64();
            let new_idx = (new_cache & letter_id_mask as u64) as usize;
            if new_idx < charset_len {
                result.push(charset[new_idx]);
            } else {
                // Fallback to the first character if all else fails
                result.push(charset[0]);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::common;

    use super::*;

    #[test]
    fn test_random_string_length() {
        let charset = common::ALPHANUMERIC_CHARSET;
        let size = 12;
        let random_str = random_string(size, charset);
        assert_eq!(random_str.len(), size);
    }

    #[test]
    fn test_random_string_characters() {
        let charset = common::ALPHANUMERIC_CHARSET;
        let size = 20;
        let random_str = random_string(size, charset);
        for c in random_str.chars() {
            assert!(charset.contains(&c));
        }
    }

    #[test]
    #[should_panic(expected = "common::random_string: Size parameter must be greater than 0")]
    fn test_random_string_size_zero() {
        let charset = common::ALPHANUMERIC_CHARSET;
        let _ = random_string(0, charset);
    }

    #[test]
    #[should_panic(expected = "common::random_string: Charset parameter must not be empty")]
    fn test_random_string_empty_charset() {
        let charset: &[char] = &[];
        let _ = random_string(10, charset);
    }

    #[test]
    fn test_random_string_variety() {
        let charset = common::ALPHANUMERIC_CHARSET;
        let size = 15;
        let random_str1 = random_string(size, charset);
        let random_str2 = random_string(size, charset);
        assert_ne!(
            random_str1, random_str2,
            "Two random strings should not be identical"
        );
    }

    #[test]
    fn test_random_string_with_special_characters() {
        let charset = common::SPECIAL_CHARSET;
        let size = 10;
        let random_str = random_string(size, charset);
        for c in random_str.chars() {
            assert!(charset.contains(&c));
        }
    }

    #[test]
    fn test_random_string_full_charset() {
        let charset = common::ALL_CHARSET;
        let size = 25;
        let random_str = random_string(size, charset);
        for c in random_str.chars() {
            assert!(charset.contains(&c));
        }
    }
}
