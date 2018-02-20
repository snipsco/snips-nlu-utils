use std::ops::Range;
use unicode_normalization::char::{decompose_canonical, compose, is_combining_mark};

pub fn convert_to_char_range(string: &str, range: &Range<usize>) -> Range<usize> {
    Range {
        start: convert_to_char_index(string, range.start),
        end: convert_to_char_index(string, range.end),
    }
}

pub fn convert_to_byte_range(string: &str, range: &Range<usize>) -> Range<usize> {
    Range {
        start: convert_to_byte_index(string, range.start),
        end: convert_to_byte_index(string, range.end),
    }
}

pub fn convert_to_char_index(string: &str, byte_index: usize) -> usize {
    if string.is_empty() {
        return 0;
    }
    let mut acc = 0;
    let mut last_char_index = 0;
    for (char_index, char) in string.chars().enumerate() {
        if byte_index <= acc {
            return char_index;
        }
        acc += char.len_utf8();
        last_char_index = char_index;
    }
    last_char_index + 1
}

pub fn convert_to_byte_index(string: &str, char_index: usize) -> usize {
    let mut result = 0;
    for (current_char_index, char) in string.chars().enumerate() {
        if current_char_index == char_index {
            return result;
        }
        result += char.len_utf8()
    }
    result
}

pub fn substring_with_char_range(string: String, range: &Range<usize>) -> String {
    string
        .chars()
        .skip(range.start)
        .take(range.end - range.start)
        .collect()
}

pub fn prefix_until_char_index(string: String, index: usize) -> String {
    substring_with_char_range(string, &(0..index))
}

pub fn suffix_from_char_index(string: String, index: usize) -> String {
    let length = string.len();
    substring_with_char_range(string, &(index..length))
}

/// Apply the following normalization successively:
/// 1) trim
/// 2) remove diacritics
/// 3) lowercase
///
/// # Examples
///
/// ```
/// use snips_nlu_utils::string::normalize;
/// 
/// assert_eq!("heloa".to_string(), normalize("  HelöÀ "));
/// ```
pub fn normalize(string: &str) -> String {
    remove_diacritics(string.trim()).to_lowercase()
}

/// Remove common accentuations and diacritics
///
/// More specifically, remove all combination marks (https://en.wikipedia.org/wiki/Combining_character)
///
/// # Examples
///
/// ```
/// use snips_nlu_utils::string::remove_diacritics;
///
/// assert_eq!("ceaA".to_owned(), remove_diacritics("çéaÀ"));
/// ```
pub fn remove_diacritics(string: &str) -> String {
    string.chars().flat_map(|c| remove_combination_marks(c)).collect()
}

fn remove_combination_marks(character: char) -> Option<char> {
    let mut decomposition = Vec::<char>::new();
    decompose_canonical(character, |c| {
        if !is_combining_mark(c) {
            decomposition.push(c)
        }
    });
    let first_char = decomposition.first().map(|c| c.to_owned());
    decomposition.into_iter().skip(1).fold(first_char, |opt_acc, c| {
        opt_acc.map(|acc| compose(acc, c)).unwrap_or(None)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn substring_with_char_range_works() {
        // Given
        let text = "Hellö !!".to_string();
        let char_range = 2..5;

        // When
        let substring = substring_with_char_range(text, &char_range);

        // Then
        assert_eq!("llö", &substring);
    }

    #[test]
    fn prefix_works() {
        // Given
        let text = "Hellö !!".to_string();

        // When
        let prefix = prefix_until_char_index(text, 5);

        // Then
        assert_eq!("Hellö", &prefix);
    }

    #[test]
    fn suffix_works() {
        // Given
        let text = "Hellö !!".to_string();

        // When
        let suffix = suffix_from_char_index(text, 4);

        // Then
        assert_eq!("ö !!", &suffix);
    }

    #[test]
    fn remove_combination_marks_works() {
        assert_eq!(Some('c'.to_owned()), remove_combination_marks('ç'));
        assert_eq!(Some('e'.to_owned()), remove_combination_marks('ë'));
        assert_eq!(Some('안'.to_owned()), remove_combination_marks('안'));
    }

    #[test]
    fn remove_diacritics_works() {
        assert_eq!("".to_owned(), remove_diacritics(""));
        assert_eq!("ceaA".to_owned(), remove_diacritics("çéaÀ"));
    }

    #[test]
    fn normalize_works() {
        assert_eq!("heloa".to_string(), normalize("  HelöÀ "));
    }
}
