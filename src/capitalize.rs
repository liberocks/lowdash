/// Capitalizes the first letter of the input string and converts the rest to lowercase.
///
/// # Arguments
/// * `str_input` - The input string to capitalize
///
/// # Returns
/// * `String` - The capitalized string
///
/// # Examples
/// ```rust
/// use lowdash::capitalize;
///
/// assert_eq!(capitalize("hello"), "Hello");
/// assert_eq!(capitalize("WORLD"), "World");
/// assert_eq!(capitalize("rUsT"), "Rust");
/// ```
pub fn capitalize(str_input: &str) -> String {
    let mut chars = str_input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            first.to_uppercase().collect::<String>() + chars.as_str().to_lowercase().as_str()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize_lowercase() {
        assert_eq!(capitalize("hello"), "Hello");
    }

    #[test]
    fn test_capitalize_uppercase() {
        assert_eq!(capitalize("WORLD"), "World");
    }

    #[test]
    fn test_capitalize_mixed_case() {
        assert_eq!(capitalize("rUsT"), "Rust");
    }

    #[test]
    fn test_capitalize_single_character() {
        assert_eq!(capitalize("a"), "A");
    }

    #[test]
    fn test_capitalize_empty_string() {
        assert_eq!(capitalize(""), "");
    }

    #[test]
    fn test_capitalize_with_numbers() {
        assert_eq!(capitalize("123abc"), "123abc");
    }

    #[test]
    fn test_capitalize_with_special_characters() {
        assert_eq!(capitalize("!hello"), "!hello");
    }

    #[test]
    fn test_capitalize_unicode() {
        assert_eq!(capitalize("こんにちは"), "こんにちは");
    }

    #[test]
    fn test_capitalize_with_whitespace() {
        assert_eq!(capitalize(" hello"), " hello");
    }
}
