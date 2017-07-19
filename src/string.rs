use std::ops::Range;


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
}
