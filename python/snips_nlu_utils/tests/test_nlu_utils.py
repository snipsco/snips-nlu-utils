# coding=utf-8
from __future__ import unicode_literals

import unittest

from snips_nlu_utils import remove_diacritics, get_shape, normalize, tokenize_light, tokenize
from snips_nlu_utils.token import compute_all_ngrams


class TestNluUtils(unittest.TestCase):
    def test_should_tokenize(self):
        # Given
        u = "foo bär baz"
        language = "en"

        # When
        tokens = tokenize(u, language)

        # Then
        expected_tokens = [
            {
                "value": "foo",
                "range": {
                    "start": 0,
                    "end": 3
                },
                "char_range": {
                    "start": 0,
                    "end": 3
                }
            },
            {
                "value": "bär",
                "range": {
                    "start": 4,
                    "end": 8
                },
                "char_range": {
                    "start": 4,
                    "end": 7
                }
            },
            {
                "value": "baz",
                "range": {
                    "start": 9,
                    "end": 12
                },
                "char_range": {
                    "start": 8,
                    "end": 11
                }
            },
        ]
        self.assertListEqual(expected_tokens, tokens)

    def test_should_tokenize_empty_string(self):
        self.assertListEqual([], tokenize("", "en"))

    def test_should_tokenize_light(self):
        # Given
        u = "foo' bär baz"
        language = "en"

        # When
        tokens = tokenize_light(u, language)

        # Then
        expected_tokens = ["foo", "bär", "baz"]
        self.assertListEqual(expected_tokens, tokens)

    def test_should_tokenize_light_empty_string(self):
        self.assertListEqual([], tokenize_light("", "en"))

    def test_should_remove_diacritics(self):
        self.assertEqual("Hello", remove_diacritics("Hëllo"))

    def test_should_normalize(self):
        self.assertEqual("hello", normalize("Hëllo"))

    def test_should_compute_all_ngrams(self):
        # Given
        tokens = ["hello", "beautiful", "world", "!"]

        # When
        ngrams = compute_all_ngrams(tokens, 3)

        # Then
        expected_ngrams = [
            {'ngram': 'hello', 'token_indexes': [0]},
            {'ngram': 'hello beautiful', 'token_indexes': [0, 1]},
            {'ngram': 'hello beautiful world', 'token_indexes': [0, 1, 2]},
            {'ngram': 'beautiful', 'token_indexes': [1]},
            {'ngram': 'beautiful world', 'token_indexes': [1, 2]},
            {'ngram': 'beautiful world !', 'token_indexes': [1, 2, 3]},
            {'ngram': 'world', 'token_indexes': [2]},
            {'ngram': 'world !', 'token_indexes': [2, 3]},
            {'ngram': '!', 'token_indexes': [3]}
        ]
        self.assertListEqual(expected_ngrams, ngrams)

    def test_should_get_shape(self):
        self.assertEqual("xxx", get_shape("hello"))
        self.assertEqual("XXX", get_shape("HELLO"))
        self.assertEqual("Xxx", get_shape("Hello"))
        self.assertEqual("xX", get_shape("hEllo"))
