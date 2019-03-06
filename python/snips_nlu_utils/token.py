from _ctypes import pointer, byref
from ctypes import c_char_p

from snips_nlu_utils.utils import (
    string_array_pointer, CStringArray, lib, check_ffi_error, CTokenArray,
    token_array_pointer, ngram_array_pointer, CNgramArray)


def tokenize(input, language):
    with token_array_pointer(pointer(CTokenArray())) as ptr:
        exit_code = lib.snips_nlu_utils_tokenize(
            input.encode("utf8"), language.encode("utf8"), byref(ptr))
        check_ffi_error(exit_code,
                        "Something went wrong when tokenizing '%s'" % input)
        array = ptr.contents
        return array.to_pylist()


def tokenize_light(input, language):
    with string_array_pointer(pointer(CStringArray())) as ptr:
        exit_code = lib.snips_nlu_utils_tokenize_light(
            input.encode("utf8"), language.encode("utf8"), byref(ptr))
        check_ffi_error(exit_code,
                        "Something went wrong when tokenizing '%s'" % input)
        array = ptr.contents
        return array.to_pylist()


def compute_all_ngrams(tokens, max_ngram_size):
    with ngram_array_pointer(pointer(CNgramArray())) as ptr:
        nb_tokens = len(tokens)
        c_tokens = CStringArray()
        c_tokens.data = (c_char_p * nb_tokens)(*[token.encode("utf8") for token in tokens])
        c_tokens.size = nb_tokens
        exit_code = lib.snips_nlu_utils_compute_all_ngrams(
            byref(c_tokens), max_ngram_size, byref(ptr))
        check_ffi_error(exit_code,
                        "Something went wrong when computing all ngrams for '%s'" % tokens)
        array = ptr.contents
        return array.to_pylist()
