/// Truncates a string and appends an ellipsis (`"..."`) if it exceeds the specified length.
///
/// This function trims the input string of leading and trailing whitespace. If the trimmed
/// string's length exceeds the specified `length`, it truncates the string to `length - 3`
/// characters and appends an ellipsis. If the trimmed string is shorter than or equal to
/// `length`, it returns the trimmed string as is. If either the trimmed string or the
/// specified `length` is less than 3, it returns `"..."`.
///
/// # Arguments
///
/// * `s` - The input string to potentially truncate.
/// * `length` - The maximum allowed length of the returned string.
///
/// # Returns
///
/// * `String` - The possibly truncated string with an ellipsis appended.
///
/// # Examples
///
/// ```
/// use lowdash::ellipsis;
///
/// let result = ellipsis("Hello, World!", 10);
/// assert_eq!(result, "Hello, ...");
///
/// let result = ellipsis("Short", 10);
/// assert_eq!(result, "Short");
///
/// let result = ellipsis("ExactLength", 11);
/// assert_eq!(result, "ExactLength");
///
/// let result = ellipsis("  Trimmed  ", 6);
/// assert_eq!(result, "Tri...");
///
/// let result = ellipsis("Hi", 2);
/// assert_eq!(result, "Hi");
/// ```
pub fn ellipsis(s: &str, length: usize) -> String {
    let trimmed = s.trim();
    let trimmed_len = trimmed.chars().count();

    if trimmed_len > length {
        if trimmed_len < 3 || length < 3 {
            return "...".to_string();
        }
        let trunc_length = length.saturating_sub(3);
        let truncated: String = trimmed.chars().take(trunc_length).collect();
        truncated + "..."
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ellipsis_truncate() {
        let result = ellipsis("Hello, World!", 10);
        assert_eq!(result, "Hello, ...");
    }

    #[test]
    fn test_ellipsis_no_truncate() {
        let result = ellipsis("Short", 10);
        assert_eq!(result, "Short");
    }

    #[test]
    fn test_ellipsis_exact_length() {
        let result = ellipsis("ExactLength", 11);
        assert_eq!(result, "ExactLength");
    }

    #[test]
    fn test_ellipsis_with_whitespace() {
        let result = ellipsis("  Trimmed  ", 6);
        assert_eq!(result, "Tri...");
    }

    #[test]
    fn test_ellipsis_short_length() {
        let result = ellipsis("Hi", 2);
        assert_eq!(result, "Hi");
    }

    #[test]
    fn test_ellipsis_length_less_than_three() {
        let result = ellipsis("Hello", 2);
        assert_eq!(result, "...");
    }

    #[test]
    fn test_ellipsis_empty_string() {
        let result = ellipsis("   ", 5);
        assert_eq!(result, "");
    }

    #[test]
    fn test_ellipsis_length_zero() {
        let result = ellipsis("Hello", 0);
        assert_eq!(result, "...");
    }

    #[test]
    fn test_ellipsis_unicode_characters() {
        let result = ellipsis("こんにちは世界", 5);
        assert_eq!(result, "こん...");
    }

    #[test]
    fn test_ellipsis_multibyte_characters() {
        let result = ellipsis("😀😃😄😁😆", 4);
        assert_eq!(result, "😀...");
    }
}
