/// Converts a string to snake_case.
///
/// Splits the input string into words based on spaces, hyphens, underscores, and capitalization,
/// then converts each word to lowercase and joins them with underscores.
///
/// # Arguments
/// * `str_input` - The input string to convert
///
/// # Returns
/// * `String` - The converted string in snake_case
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

    let mut words = Vec::new();
    let mut current_word = String::new();
    let mut prev_char = ' ';

    for c in str_input.chars() {
        if c.is_uppercase()
            && (prev_char.is_lowercase()
                || prev_char == ' '
                || prev_char == '-'
                || prev_char == '_')
        {
            if !current_word.is_empty() {
                words.push(current_word.to_lowercase());
                current_word.clear();
            }
            current_word.push(c);
        } else if c.is_whitespace() || c == '-' || c == '_' {
            if !current_word.is_empty() {
                words.push(current_word.to_lowercase());
                current_word.clear();
            }
        } else {
            current_word.push(c);
        }
        prev_char = c;
    }

    if !current_word.is_empty() {
        words.push(current_word.to_lowercase());
    }

    words.join("_")
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
}
