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
/// ```
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

    str_input
        .split(|c: char| c.is_whitespace() || c == '-' || c == '_')
        .filter(|s| !s.is_empty())
        .map(|word| {
            let mut chars: Vec<char> = word.chars().collect();
            if let Some(first) = chars.first_mut() {
                *first = first.to_uppercase().next().unwrap_or(*first);
            }
            chars.into_iter().collect::<String>()
        })
        .collect()
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
