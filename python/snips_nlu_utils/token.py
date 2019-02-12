from _ctypes import pointer, byref

from snips_nlu_utils.utils import string_array_pointer, CStringArray, lib, check_ffi_error, CTokenArray, \
    token_array_pointer


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
