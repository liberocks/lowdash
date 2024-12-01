/// Converts a string to kebab-case.
///
/// Splits the input string into words based on spaces, hyphens, underscores, and capitalization,
/// then converts each word to lowercase and joins them with hyphens.
///
/// # Arguments
/// * `str_input` - The input string to convert
///
/// # Returns
/// * `String` - The converted string in kebab-case
///
/// # Examples
/// ```rust
/// use lowdash::kebab_case;
///
/// assert_eq!(kebab_case("hello world"), "hello-world");
/// assert_eq!(kebab_case("foo-bar"), "foo-bar");
/// assert_eq!(kebab_case("lorem_ipsum"), "lorem-ipsum");
/// assert_eq!(kebab_case("FooBarBazHello"), "foo-bar-baz-hello");
/// assert_eq!(kebab_case("helloWorld"), "hello-world");
/// ```
pub fn kebab_case(str_input: &str) -> String {
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

    words.join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_space_separated() {
        assert_eq!(kebab_case("hello world"), "hello-world");
    }

    #[test]
    fn test_hyphen_separated() {
        assert_eq!(kebab_case("foo-bar"), "foo-bar");
    }

    #[test]
    fn test_underscore_separated() {
        assert_eq!(kebab_case("lorem_ipsum"), "lorem-ipsum");
    }

    #[test]
    fn test_mixed_separators() {
        assert_eq!(kebab_case("foo-bar_baz hello"), "foo-bar-baz-hello");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(kebab_case(""), "");
    }

    #[test]
    fn test_single_word() {
        assert_eq!(kebab_case("hello"), "hello");
    }

    #[test]
    fn teat_pascal_case() {
        assert_eq!(kebab_case("HelloWorld"), "hello-world");
    }

    #[test]
    fn teat_camel_case() {
        assert_eq!(kebab_case("helloWorld"), "hello-world");
    }

    #[test]
    fn test_already_kebab_case() {
        assert_eq!(kebab_case("hello-world"), "hello-world");
    }

    #[test]
    fn test_multiple_spaces() {
        assert_eq!(kebab_case("hello   world"), "hello-world");
    }

    #[test]
    fn test_multiple_separators() {
        assert_eq!(kebab_case("hello---world___test"), "hello-world-test");
    }

    #[test]
    fn test_with_numbers() {
        assert_eq!(kebab_case("hello2world"), "hello2world");
    }

    #[test]
    fn test_with_special_characters() {
        assert_eq!(kebab_case("hello!world"), "hello!world");
    }

    #[test]
    fn test_unicode_characters() {
        assert_eq!(kebab_case("hello_世界"), "hello-世界");
    }
}
