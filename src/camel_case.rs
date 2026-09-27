/// Converts a string to `camelCase` using word boundaries.
///
/// # Arguments
/// * `str_input` - String to convert.
///
/// # Returns
/// * `String` - The converted string.
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

    let mut result = String::with_capacity(str_input.len());
    let mut in_word = false;
    let mut has_first_character = false;

    for c in str_input.chars() {
        if c.is_whitespace() || c == '-' || c == '_' {
            in_word = false;
        } else if !in_word {
            let first = c.to_uppercase().next().unwrap_or(c);
            if has_first_character {
                result.push(first);
            } else {
                result.extend(first.to_lowercase());
                has_first_character = true;
            }
            in_word = true;
        } else {
            result.push(c);
        }
    }

    result
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

    #[test]
    fn preserves_unicode_case_expansion_behavior() {
        for input in ["İSTANBUL", "ßeta", "ὈΔΥΣΣΕΎΣ", "___éclair"] {
            let pascal = crate::pascal_case::pascal_case(input);
            let mut chars = pascal.chars();
            let first = chars.next().expect("sample has a word");
            let mut expected = String::with_capacity(pascal.len());
            expected.extend(first.to_lowercase());
            expected.push_str(chars.as_str());

            assert_eq!(camel_case(input), expected);
        }
    }

    #[test]
    fn test_separator_only() {
        assert_eq!(camel_case("   "), "");
        assert_eq!(camel_case("---"), "");
        assert_eq!(camel_case("___"), "");
    }
}
