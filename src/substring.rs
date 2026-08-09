/// Extracts up to `length` non-null characters from `offset`.
/// Negative offsets count from the end; an out-of-bounds offset returns an empty string.
///
/// # Arguments
/// * `str_input` - String to read.
/// * `offset` - Start offset, possibly negative.
/// * `length` - Maximum number of non-null characters.
///
/// # Returns
/// * `String` - The extracted string with null characters removed.
///
/// # Examples
/// ```rust
/// use lowdash::substring;
///
/// let s = String::from("Hello, World!");
/// assert_eq!(substring(&s, 7, 5), "World");
///
/// let s = String::from("Hello, World!");
/// assert_eq!(substring(&s, -6, 5), "World");
///
/// let s = String::from("Hello, World!");
/// assert_eq!(substring(&s, 100, 5), "");
///
/// let s = String::from("Hello\x00World!");
/// assert_eq!(substring(&s, 0, 10), "HelloWorld");
/// ```
pub fn substring(str_input: &str, offset: i32, length: u32) -> String {
    if length == 0 {
        return String::new();
    }

    let start = if offset < 0 {
        let from_end = offset.unsigned_abs() as usize;
        if from_end == 0 {
            0
        } else {
            str_input
                .char_indices()
                .rev()
                .nth(from_end - 1)
                .map(|(index, _)| index)
                .unwrap_or(0)
        }
    } else {
        match str_input.char_indices().nth(offset as usize) {
            Some((index, _)) => index,
            None => return String::new(),
        }
    };

    let capacity = (length as usize).min(str_input.len() - start);
    let mut result = String::with_capacity(capacity);
    let mut count = 0;
    for character in str_input[start..].chars() {
        if character != '\x00' {
            result.push(character);
            count += 1;
            if count == length {
                break;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substring_positive_offset_within_bounds() {
        let s = "Hello, World!";
        let result = substring(s, 7, 5);
        assert_eq!(result, "World");
    }

    #[test]
    fn test_substring_negative_offset_within_bounds() {
        let s = "Hello, World!";
        let result = substring(s, -6, 5);
        assert_eq!(result, "World");
    }

    #[test]
    fn test_substring_offset_zero() {
        let s = "Hello, World!";
        let result = substring(s, 0, 5);
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_substring_offset_exceeds_length() {
        let s = "Hello, World!";
        let result = substring(s, 100, 5);
        assert_eq!(result, "");
    }

    #[test]
    fn test_substring_negative_offset_exceeds_length() {
        let s = "Hello, World!";
        let result = substring(s, -20, 5);
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_substring_length_exceeds_remaining() {
        let s = "Hello, World!";
        let result = substring(s, 7, 50);
        assert_eq!(result, "World!");
    }

    #[test]
    fn test_substring_zero_length() {
        let s = "Hello, World!";
        let result = substring(s, 7, 0);
        assert_eq!(result, "");
    }

    #[test]
    fn test_substring_with_null_characters() {
        let s = "Hello\x00World!";
        let result = substring(s, 0, 10);
        assert_eq!(result, "HelloWorld");
    }

    #[test]
    fn test_substring_entire_string() {
        let s = "Hello, World!";
        let result = substring(s, 0, 13);
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_substring_partial_string() {
        let s = "Rust Programming Language";
        let result = substring(s, 5, 11);
        assert_eq!(result, "Programming");
    }

    #[test]
    fn test_substring_negative_offset_partial() {
        let s = "Rust Programming Language";
        let result = substring(s, -8, 8);
        assert_eq!(result, "Language");
    }
    #[test]
    fn test_substring_entire_string_with_nulls() {
        let s = "\x00Rust Programming\x00Language\x00";
        let result = substring(s, 0, 24);
        assert_eq!(result, "Rust ProgrammingLanguage");
    }

    #[test]
    fn test_substring_max_length() {
        let s = "Short";
        let result = substring(s, 0, 100);
        assert_eq!(result, "Short");
    }

    #[test]
    fn test_substring_large_length_does_not_overallocate() {
        let result = substring("Short", 0, u32::MAX);
        assert_eq!(result, "Short");
        assert!(result.capacity() <= "Short".len());
    }
}
