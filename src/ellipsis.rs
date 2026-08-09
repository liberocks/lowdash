/// Trims a string and appends `"..."` when it exceeds `length`.
/// If truncation is needed and either length is below 3, returns `"..."`.
///
/// # Arguments
///
/// * `s` - String to trim and truncate.
/// * `length` - Maximum output length.
///
/// # Returns
///
/// * `String` - The trimmed or truncated string.
///
/// # Examples
///
/// ```rust
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
    if length <= 3 {
        let trimmed_len = trimmed.chars().count();
        return if trimmed_len > length {
            "...".to_string()
        } else {
            trimmed.to_string()
        };
    }

    let trunc_length = length.saturating_sub(3);
    let mut trunc_end = None;

    for (index, (byte_index, _)) in trimmed.char_indices().enumerate() {
        if index == trunc_length {
            trunc_end = Some(byte_index);
        }
        if index == length {
            let end = trunc_end.expect("truncation boundary must exist");
            let mut result = String::with_capacity(end.saturating_add(3));
            result.push_str(&trimmed[..end]);
            result.push_str("...");
            return result;
        }
    }

    trimmed.to_owned()
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
