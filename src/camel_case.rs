use crate::pascal_case;

/// Converts a string to camelCase.
///
/// Splits the input string into words based on spaces, hyphens, and underscores,
/// then converts the first word to lowercase and capitalizes the first letter of each subsequent word,
/// and joins them together.
///
/// # Arguments
/// * `str_input` - The input string to convert
///
/// # Returns
/// * `String` - The converted string in camelCase
///
/// # Examples
/// ```rust
/// use lowdash::camel_case;
///
/// assert_eq!(camel_case("hello world"), "helloWorld");
/// assert_eq!(camel_case("foo-bar"), "fooBar");
/// assert_eq!(camel_case("lorem_ipsum"), "loremIpsum");
/// assert_eq!(camel_case("FooBarBazHello"), "fooBarBazHello");
/// ```
pub fn camel_case(str_input: &str) -> String {
    if str_input.is_empty() {
        return String::new();
    }

    let pascal = pascal_case(str_input);
    let mut chars = pascal.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_lowercase().collect::<String>() + chars.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_space_separated() {
        assert_eq!(camel_case("hello world"), "helloWorld");
    }

    #[test]
    fn test_hyphen_separated() {
        assert_eq!(camel_case("foo-bar"), "fooBar");
    }

    #[test]
    fn test_underscore_separated() {
        assert_eq!(camel_case("lorem_ipsum"), "loremIpsum");
    }

    #[test]
    fn test_mixed_separators() {
        assert_eq!(camel_case("foo-bar_baz hello"), "fooBarBazHello");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(camel_case(""), "");
    }

    #[test]
    fn test_single_word() {
        assert_eq!(camel_case("hello"), "hello");
    }

    #[test]
    fn test_already_camel_case() {
        assert_eq!(camel_case("helloWorld"), "helloWorld");
    }

    #[test]
    fn test_multiple_spaces() {
        assert_eq!(camel_case("hello   world"), "helloWorld");
    }

    #[test]
    fn test_multiple_separators() {
        assert_eq!(camel_case("hello---world___test"), "helloWorldTest");
    }

    #[test]
    fn test_with_numbers() {
        assert_eq!(camel_case("hello2world"), "hello2world");
    }

    #[test]
    fn test_with_special_characters() {
        assert_eq!(camel_case("hello!world"), "hello!world");
    }

    #[test]
    fn test_unicode_characters() {
        assert_eq!(camel_case("hello_世界"), "hello世界");
    }
}
