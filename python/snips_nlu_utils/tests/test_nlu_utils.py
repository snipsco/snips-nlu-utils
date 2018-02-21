from __future__ import unicode_literals

import unittest

from builtins import str, bytes

from snips_nlu_utils import tokenize, tokenize_light


class TestNLUUtils(unittest.TestCase):
    def test_package_should_tokenize(self):
        # Given
        u = "let's eat food tonight"
        language = "en"

        # When
        tokens = tokenize(u, language)

        # Then
        self.assertGreater(len(tokens), 0)
        self.assertTrue(all(isinstance(t, dict) for t in tokens))

    def test_package_should_tokenize_light(self):
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
