/// Splits a string into words based on casing, digits, and separators.
///
/// Processes the input string by inserting spaces between words based on uppercase letters,
/// digits, and other non-alphanumeric characters, then splits on spaces to extract words.
///
/// # Arguments
/// * `str_input` - The input string to split into words
///
/// # Returns
/// * `Vec<String>` - A vector of words extracted from the input string
///
/// # Examples
/// ```rust
/// use lowdash::words;
///
/// let result = words("Int8Value");
/// assert_eq!(result, vec!["Int", "8", "Value"]);
///
/// let result = words("hello_world");
/// assert_eq!(result, vec!["hello", "world"]);
///
/// let result = words("fooBarBazHello");
/// assert_eq!(result, vec!["foo", "Bar", "Baz", "Hello"]);
/// ```
pub fn words(str_input: &str) -> Vec<String> {
    if str_input.is_empty() {
        return Vec::new();
    }

    let mut words = Vec::new();
    let mut current_word = String::new();
    let mut prev_char = '\0';

    let chars: Vec<char> = str_input.chars().collect();
    let len = chars.len();

    for i in 0..len {
        let c = chars[i];

        // Look ahead to the next character if it exists
        let next_char = if i + 1 < len {
            Some(chars[i + 1])
        } else {
            None
        };

        if c.is_uppercase() {
            if prev_char.is_lowercase()
                || prev_char.is_ascii_digit()
                || prev_char == ' '
                || prev_char == '-'
                || prev_char == '_'
            {
                if !current_word.is_empty() {
                    words.push(current_word.clone());
                    current_word.clear();
                }
            } else if let Some(next) = next_char {
                if next.is_lowercase() && !current_word.is_empty() {
                    words.push(current_word.clone());
                    current_word.clear();
                }
            }
        } else if c.is_ascii_digit() && !prev_char.is_ascii_digit() {
            if !current_word.is_empty() {
                words.push(current_word.clone());
                current_word.clear();
            }
        } else if !c.is_alphanumeric() {
            if !current_word.is_empty() {
                words.push(current_word.clone());
                current_word.clear();
            }
            prev_char = c;
            continue;
        }

        if c.is_alphanumeric() {
            current_word.push(c);
        }

        prev_char = c;
    }

    if !current_word.is_empty() {
        words.push(current_word);
    }

    words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let result = words("");
        let expected: Vec<String> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_word() {
        let result = words("hello");
        let expected = vec!["hello".to_string()];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_space_separated() {
        let result = words("hello world");
        let expected = vec!["hello".to_string(), "world".to_string()];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_hyphen_separated() {
        let result = words("foo-bar");
        let expected = vec!["foo".to_string(), "bar".to_string()];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_underscore_separated() {
        let result = words("lorem_ipsum");
        let expected = vec!["lorem".to_string(), "ipsum".to_string()];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mixed_separators() {
        let result = words("foo-bar_baz hello");
        let expected = vec![
            "foo".to_string(),
            "bar".to_string(),
            "baz".to_string(),
            "hello".to_string(),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_pascal_case() {
        let result = words("FooBarBazHello");
        let expected = vec![
            "Foo".to_string(),
            "Bar".to_string(),
            "Baz".to_string(),
            "Hello".to_string(),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_camel_case() {
        let result = words("fooBarBazHello");
        let expected = vec![
            "foo".to_string(),
            "Bar".to_string(),
            "Baz".to_string(),
            "Hello".to_string(),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_with_numbers() {
        let result = words("Int8Value");
        let expected = vec!["Int".to_string(), "8".to_string(), "Value".to_string()];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_with_special_characters() {
        let result = words("hello@world#2023");
        let expected = vec!["hello".to_string(), "world".to_string(), "2023".to_string()];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_unicode_characters() {
        let result = words("こんにちは世界");
        let expected = vec!["こんにちは世界".to_string()];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiple_uppercase_letters() {
        let result = words("HTTPRequest");
        let expected = vec!["HTTP".to_string(), "Request".to_string()];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_consecutive_digits() {
        let result = words("version2Release10");
        let expected = vec![
            "version".to_string(),
            "2".to_string(),
            "Release".to_string(),
            "10".to_string(),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_leading_and_trailing_separators() {
        let result = words("_startMiddle_end_");
        let expected = vec!["start".to_string(), "Middle".to_string(), "end".to_string()];
        assert_eq!(result, expected);
    }
}
