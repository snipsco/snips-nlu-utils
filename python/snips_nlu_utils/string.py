from _ctypes import byref
from ctypes import c_char_p, string_at

from snips_nlu_utils.utils import lib, string_pointer, check_ffi_error


def remove_diacritics(string):
    with string_pointer(c_char_p()) as ptr:
        exit_code = lib.snips_nlu_utils_remove_diacritics(
            string.encode("utf8"), byref(ptr))
        check_ffi_error(exit_code,
                        "Something went wrong when removing diacritics from '%s'" % string)
        result = string_at(ptr).decode("utf8")
        return result


def normalize(string):
    with string_pointer(c_char_p()) as ptr:
        exit_code = lib.snips_nlu_utils_normalize(
            string.encode("utf8"), byref(ptr))
        check_ffi_error(exit_code,
                        "Something went wrong when normalizing '%s'" % string)
        result = string_at(ptr).decode("utf8")
        return result


def get_shape(string):
    with string_pointer(c_char_p()) as ptr:
        exit_code = lib.snips_nlu_utils_get_shape(
            string.encode("utf8"), byref(ptr))
        check_ffi_error(exit_code,
                        "Something went wrong when getting shape of '%s'" % string)
        result = string_at(ptr).decode("utf8")
        return result
