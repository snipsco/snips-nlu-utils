from __future__ import unicode_literals

import unittest
from builtins import str, bytes

from snips_nlu_utils import (
    compute_all_ngrams, get_shape, normalize, remove_diacritics, tokenize,
    tokenize_light)


class TestNLUUtils(unittest.TestCase):
    def test_should_tokenize(self):
        # Given
        u = "let's eat food tonight"
        language = "en"

        # When
        tokens = tokenize(u, language)

        # Then
        self.assertGreater(len(tokens), 0)
        self.assertTrue(all(isinstance(t, dict) for t in tokens))

    def test_should_tokenize_light(self):
        # Given
        u = "let's eat food tonight"
        language = "en"

        # When
        tokens = tokenize_light(u, language)

        # Then
        self.assertGreater(len(tokens), 0)
        self.assertTrue(all(isinstance(t, str) for t in tokens))

    def test_tokenize_should_raise_on_string(self):
        # Given
        s = bytes(b"let's eat food tonight")
        language = bytes(b"en")

        # When / Then
        with self.assertRaises(TypeError):
            tokenize(s, language)

    def test_tokenize_light_should_raise_on_string(self):
        # Given
        s = bytes(b"let's eat food tonight")
        language = bytes(b"en")

        # When / Then
        with self.assertRaises(TypeError):
            tokenize_light(s, language)

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
