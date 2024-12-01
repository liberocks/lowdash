/// Extracts a substring from the given string based on the specified offset and length.
///
/// This function handles negative offsets by counting from the end of the string. It ensures that
/// the resulting substring contains up to `length` non-null characters, skipping any null characters (`\x00`).
/// If the offset is out of bounds, it returns an empty string.
///
/// # Arguments
/// * `str_input` - The input string from which to extract the substring.
/// * `offset` - The starting position for the substring. Can be negative to indicate an offset from the end.
/// * `length` - The number of non-null characters to include in the substring.
///
/// # Returns
/// * `String` - The extracted substring with null characters removed.
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
    let rs: Vec<char> = str_input.chars().collect();
    let size = rs.len();

    // Adjust the offset for negative values
    let idx = if offset < 0 {
        let new_offset = size as i32 + offset;
        if new_offset < 0 {
            0
        } else {
            new_offset as usize
        }
    } else {
        offset as usize
    };

    // If the adjusted offset is beyond the string length, return an empty string
    if idx >= size {
        return String::new();
    }

    let mut collected = 0;
    let mut result = String::new();
    let mut current_idx = idx;

    while current_idx < size && collected < (length as usize) {
        let c = rs[current_idx];
        if c != '\x00' {
            result.push(c);
            collected += 1;
        }
        current_idx += 1;
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
}
