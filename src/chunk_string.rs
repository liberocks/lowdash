/// Splits a string into chunks of specified size.
///
/// This function takes a string and divides it into chunks of the specified size.
/// The last chunk may be shorter than the specified size if the string length
/// is not evenly divisible by the chunk size.
///
/// # Arguments
/// * `str_input` - The input string to be chunked
/// * `size` - The size of each chunk
///
/// # Returns
/// * `Vec<String>` - A vector containing the chunked strings
///
/// # Panics
/// Panics if the chunk size is less than or equal to 0
///
/// # Examples
/// ```rust
/// use lowdash::chunk_string;
///
/// let result = chunk_string("hello", 2);
/// assert_eq!(result, vec!["he", "ll", "o"]);
/// ```
pub fn chunk_string(str_input: &str, size: i32) -> Vec<String> {
    if size <= 0 {
        panic!("chunk_string: Size parameter must be greater than 0");
    }

    if str_input.is_empty() {
        return vec![String::from("")];
    }

    let chars: Vec<char> = str_input.chars().collect();
    let size = size as usize;

    if size >= chars.len() {
        return vec![str_input.to_string()];
    }

    chars
        .chunks(size)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_chunking() {
        let result = chunk_string("hello", 2);
        assert_eq!(result, vec!["he", "ll", "o"]);
    }

    #[test]
    fn test_exact_chunks() {
        let result = chunk_string("abcd", 2);
        assert_eq!(result, vec!["ab", "cd"]);
    }

    #[test]
    fn test_larger_than_string_size() {
        let result = chunk_string("rust", 10);
        assert_eq!(result, vec!["rust"]);
    }

    #[test]
    fn test_empty_string() {
        let result = chunk_string("", 2);
        assert_eq!(result, vec![""]);
    }

    #[test]
    fn test_single_character_chunks() {
        let result = chunk_string("rust", 1);
        assert_eq!(result, vec!["r", "u", "s", "t"]);
    }

    #[test]
    #[should_panic(expected = "chunk_string: Size parameter must be greater than 0")]
    fn test_zero_size_panic() {
        chunk_string("hello", 0);
    }

    #[test]
    #[should_panic(expected = "chunk_string: Size parameter must be greater than 0")]
    fn test_negative_size_panic() {
        chunk_string("hello", -1);
    }

    #[test]
    fn test_multibyte_characters() {
        let result = chunk_string("こんにちは", 2);
        assert_eq!(result, vec!["こん", "にち", "は"]);
    }

    #[test]
    fn test_mixed_ascii_unicode() {
        let result = chunk_string("ab🌟cde", 2);
        assert_eq!(result, vec!["ab", "🌟c", "de"]);
    }

    #[test]
    fn test_unicode_characters() {
        let result = chunk_string("hello🌍world!", 5);
        assert_eq!(result, vec!["hello", "🌍worl", "d!"]);
    }
}
