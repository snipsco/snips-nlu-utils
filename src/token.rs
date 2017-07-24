use std::cmp::min;
use std::ops::Range;
use itertools::Itertools;

use regex::{Regex, RegexBuilder};
use string::{convert_to_char_range, normalize};
use range::ranges_overlap;

pub type Ngrams = (String, Vec<usize>);

const CURRENCIES: &str = "$؋ƒ៛¥₡₱£€¢﷼₪₩₭₨₮₦₽฿₴₫";

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub value: String,
    pub range: Range<usize>,
    pub char_range: Range<usize>,
    _normalized: Option<String>,
}

impl Token {
    pub fn new(value: String, range: Range<usize>, char_range: Range<usize>) -> Self {
        Token {value, range, char_range, _normalized: None}
    }

    pub fn normalized_value(&mut self) -> String {
        if let Some(ref normalized) = self._normalized {
            normalized.to_string()
        } else {
            self._normalized = Some(normalize(&self.value));
            self._normalized.clone().unwrap()
        }
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    lazy_static! {
        static ref WORD_REGEX: Regex = RegexBuilder::new(r"\w+").unicode(true).build().unwrap();
        static ref SYMBOL_REGEX: Regex = RegexBuilder::new(&format!("[?!&%{}]+", CURRENCIES)).unicode(true).build().unwrap();
    }
    _tokenize(input, &[&WORD_REGEX, &SYMBOL_REGEX])
}

pub fn tokenize_light(input: &str) -> Vec<String> {
    tokenize(input).into_iter().map(|t| t.value).collect_vec()
}

fn _tokenize(input: &str, regexes: &[&Regex]) -> Vec<Token> {
    let mut non_overlapping_tokens: Vec<Token> = vec![];

    for r in regexes {
        let mut tokens: Vec<Token> = r
            .find_iter(input)
            .map(|m| {
                let range = m.start()..m.end();
                let value = m.as_str().to_string();
                Token {
                    char_range: convert_to_char_range(input, &range),
                    value,
                    range,
                    _normalized: None
                }
            })
            .filter(|t| non_overlapping_tokens.iter().find(|t2| ranges_overlap(&t.range, &t2.range)).is_none())
            .collect();

        non_overlapping_tokens.append(&mut tokens);
    }

    non_overlapping_tokens.sort_by_key(|t| t.range.start);

    non_overlapping_tokens
}

pub fn compute_all_ngrams(tokens: &[&str], max_ngram_size: usize) -> Vec<Ngrams> {
    let mut ngrams: Vec<Ngrams> = Vec::new();

    for start in 0..tokens.len() {
        let mut local_ngrams: Vec<Ngrams> = Vec::new();
        let mut last_ngram_item: Option<Ngrams> = None;
        let max_end = min(tokens.len(), start + max_ngram_size);

        for end in start..max_end {
            let ngram_item = if let Some(last_ngram_item) = last_ngram_item {
                (format!("{} {}", last_ngram_item.0, tokens[end]),
                 consume_and_concat(last_ngram_item.1, vec![end]))
            } else {
                (tokens[start].to_string(), vec![start])
            };
            last_ngram_item = Some(ngram_item.clone());
            local_ngrams.push(ngram_item);
        }
        ngrams.extend_from_slice(&local_ngrams);
    }

    ngrams
}

fn consume_and_concat<T>(mut vec1: Vec<T>, vec2: Vec<T>) -> Vec<T> {
    vec1.extend(vec2);
    vec1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_empty_string_works() {
        let text = "";
        let retrieved = tokenize(text);
        assert_eq!(retrieved, vec![]);
    }

    #[test]
    fn tokenize_only_whitespaces_works() {
        let text = "                ";
        let retrieved = tokenize(text);
        assert_eq!(retrieved, vec![]);
    }

    #[test]
    fn tokenize_literals_works() {
        let text = "hello World";
        let retrieved = tokenize(text);
        let expected = vec![
            Token {
                value: "hello".to_string(),
                range: 0..5,
                char_range: 0..5,
                _normalized: None,
            },
            Token {
                value: "World".to_string(),
                range: 6..11,
                char_range: 6..11,
                _normalized: None,
            }
        ];
        assert_eq!(retrieved, expected);
    }

    #[test]
    fn tokenize_symbols_works() {
        let text = "$$ % !!";
        let retrieved = tokenize(text);
        let expected = vec![
            Token {
                value: "$$".to_string(),
                range: 0..2,
                char_range: 0..2,
                _normalized: None,
            },
            Token {
                value: "%".to_string(),
                range: 3..4,
                char_range: 3..4,
                _normalized: None,
            },
            Token {
                value: "!!".to_string(),
                range: 5..7,
                char_range: 5..7,
                _normalized: None,
            },
        ];
        assert_eq!(retrieved, expected);
    }

    #[test]
    fn compute_all_ngrams_works() {
        let result = compute_all_ngrams(&vec!["a", "b", "c"], 3);
        let expected: Vec<Ngrams> = vec![("a".to_string(), vec![0]),
                                         ("a b".to_string(), vec![0, 1]),
                                         ("a b c".to_string(), vec![0, 1, 2]),
                                         ("b".to_string(), vec![1]),
                                         ("b c".to_string(), vec![1, 2]),
                                         ("c".to_string(), vec![2])];
        assert_eq!(result, expected)
    }

    #[test]
    fn normalized_value_should_be_lazy() {
        // Given
        let mut token = Token {
            value: "HellÖ".to_string(),
            range: 0..6,
            char_range: 0..5,
            _normalized: None
        };

        // When
        let normalized_value = token.normalized_value();

        // Then
        assert_eq!("hello".to_string(), normalized_value);
        assert_eq!(Some("hello".to_string()), token._normalized);
    }
}
