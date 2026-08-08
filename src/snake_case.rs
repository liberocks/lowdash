/// Converts a string to `snake_case` using separators and case boundaries.
///
/// # Arguments
/// * `str_input` - String to convert.
///
/// # Returns
/// * `String` - The converted string.
///
/// # Examples
/// ```rust
/// use lowdash::snake_case;
///
/// assert_eq!(snake_case("hello world"), "hello_world");
/// assert_eq!(snake_case("foo-bar"), "foo_bar");
/// assert_eq!(snake_case("lorem_ipsum"), "lorem_ipsum");
/// assert_eq!(snake_case("FooBarBazHello"), "foo_bar_baz_hello");
/// assert_eq!(snake_case("fooBarBazHello"), "foo_bar_baz_hello");
/// ```
pub fn snake_case(str_input: &str) -> String {
    if str_input.is_empty() {
        return String::new();
    }

    let mut result = String::with_capacity(str_input.len());
    let mut in_word = false;
    let mut needs_separator = false;
    let mut prev_char = ' ';

    for c in str_input.chars() {
        if c.is_uppercase()
            && (prev_char.is_lowercase()
                || prev_char == ' '
                || prev_char == '-'
                || prev_char == '_')
        {
            if in_word {
                needs_separator = true;
            }
            if needs_separator && !result.is_empty() {
                result.push('_');
                needs_separator = false;
            }
            result.extend(c.to_lowercase());
            in_word = true;
        } else if c.is_whitespace() || c == '-' || c == '_' {
            if in_word {
                needs_separator = true;
                in_word = false;
            }
        } else {
            if needs_separator && !result.is_empty() {
                result.push('_');
                needs_separator = false;
            }
            result.extend(c.to_lowercase());
            in_word = true;
        }
        prev_char = c;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_space_separated() {
        assert_eq!(snake_case("hello world"), "hello_world");
    }

    #[test]
    fn test_hyphen_separated() {
        assert_eq!(snake_case("foo-bar"), "foo_bar");
    }

    #[test]
    fn test_underscore_separated() {
        assert_eq!(snake_case("lorem_ipsum"), "lorem_ipsum");
    }

    #[test]
    fn test_mixed_separators() {
        assert_eq!(snake_case("foo-bar_baz hello"), "foo_bar_baz_hello");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(snake_case(""), "");
    }

    #[test]
    fn test_single_word() {
        assert_eq!(snake_case("hello"), "hello");
    }

    #[test]
    fn test_already_snake_case() {
        assert_eq!(snake_case("hello_world"), "hello_world");
    }

    #[test]
    fn test_pascal_case() {
        assert_eq!(snake_case("HelloWorld"), "hello_world");
    }

    #[test]
    fn test_camel_case() {
        assert_eq!(snake_case("helloWorld"), "hello_world");
    }

    #[test]
    fn test_multiple_spaces() {
        assert_eq!(snake_case("hello   world"), "hello_world");
    }

    #[test]
    fn test_multiple_separators() {
        assert_eq!(snake_case("hello---world___test"), "hello_world_test");
    }

    #[test]
    fn test_with_numbers() {
        assert_eq!(snake_case("hello2world"), "hello2world");
    }

    #[test]
    fn test_with_special_characters() {
        assert_eq!(snake_case("hello!world"), "hello!world");
    }

    #[test]
    fn test_unicode_characters() {
        assert_eq!(snake_case("hello_世界"), "hello_世界");
    }

    #[test]
    fn test_multiple_camel_case_words() {
        assert_eq!(snake_case("fooBarBaz"), "foo_bar_baz");
        assert_eq!(snake_case("helloWorldTest"), "hello_world_test");
    }

    #[test]
    fn test_multiple_pascal_case_words() {
        assert_eq!(snake_case("FooBarBaz"), "foo_bar_baz");
        assert_eq!(snake_case("HelloWorldTest"), "hello_world_test");
    }

    #[test]
    fn test_uppercase_after_separator() {
        assert_eq!(snake_case("hello-World"), "hello_world");
        assert_eq!(snake_case("hello_World"), "hello_world");
        assert_eq!(snake_case("hello World"), "hello_world");
    }
}
