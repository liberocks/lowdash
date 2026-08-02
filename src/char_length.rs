/// Returns the number of Unicode scalar values in a string.
///
/// # Arguments
/// * `str_input` - String to count.
///
/// # Returns
/// * `usize` - The character count.
///
/// # Examples
/// ```rust
/// use lowdash::char_length;
///
/// assert_eq!(char_length("hello"), 5);
/// assert_eq!(char_length("🌍world"), 6);
/// assert_eq!(char_length("こんにちは"), 5);
/// ```
pub fn char_length(str_input: &str) -> usize {
    str_input.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_string() {
        assert_eq!(char_length("hello"), 5);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(char_length(""), 0);
    }

    #[test]
    fn test_unicode_string() {
        assert_eq!(char_length("こんにちは"), 5);
    }

    #[test]
    fn test_mixed_string() {
        assert_eq!(char_length("hello🌍world"), 11);
    }

    #[test]
    fn test_emoji_string() {
        assert_eq!(char_length("✨🌟💫"), 3);
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(char_length("a b c"), 5);
    }

    #[test]
    fn test_special_characters() {
        assert_eq!(char_length("a\nb\tc"), 5);
    }
}
