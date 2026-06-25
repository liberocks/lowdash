/// Converts a string to PascalCase.
///
/// Splits the input string into words based on spaces, hyphens, and underscores,
/// then capitalizes the first letter of each word and joins them together.
///
/// # Arguments
/// * `str_input` - The input string to convert
///
/// # Returns
/// * `String` - The converted string in PascalCase
///
/// # Examples
/// ```rust
/// use lowdash::pascal_case;
///
/// assert_eq!(pascal_case("hello world"), "HelloWorld");
/// assert_eq!(pascal_case("foo-bar"), "FooBar");
/// assert_eq!(pascal_case("lorem_ipsum"), "LoremIpsum");
/// ```
pub fn pascal_case(str_input: &str) -> String {
    if str_input.is_empty() {
        return String::new();
    }

    let mut result = String::with_capacity(str_input.len());
    let mut in_word = false;

    for c in str_input.chars() {
        if c.is_whitespace() || c == '-' || c == '_' {
            in_word = false;
        } else {
            if !in_word {
                result.push(c.to_uppercase().next().unwrap_or(c));
                in_word = true;
            } else {
                result.push(c);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_space_separated() {
        assert_eq!(pascal_case("hello world"), "HelloWorld");
    }

    #[test]
    fn test_hyphen_separated() {
        assert_eq!(pascal_case("foo-bar"), "FooBar");
    }

    #[test]
    fn test_underscore_separated() {
        assert_eq!(pascal_case("lorem_ipsum"), "LoremIpsum");
    }

    #[test]
    fn test_mixed_separators() {
        assert_eq!(pascal_case("foo-bar_baz hello"), "FooBarBazHello");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(pascal_case(""), "");
    }

    #[test]
    fn test_single_word() {
        assert_eq!(pascal_case("hello"), "Hello");
    }

    #[test]
    fn test_already_pascal_case() {
        assert_eq!(pascal_case("HelloWorld"), "HelloWorld");
    }

    #[test]
    fn test_multiple_spaces() {
        assert_eq!(pascal_case("hello   world"), "HelloWorld");
    }

    #[test]
    fn test_multiple_separators() {
        assert_eq!(pascal_case("hello---world___test"), "HelloWorldTest");
    }

    #[test]
    fn test_with_numbers() {
        assert_eq!(pascal_case("hello2world"), "Hello2world");
    }

    #[test]
    fn test_with_special_characters() {
        assert_eq!(pascal_case("hello!world"), "Hello!world");
    }

    #[test]
    fn test_unicode_characters() {
        assert_eq!(pascal_case("hello_世界"), "Hello世界");
    }
}
