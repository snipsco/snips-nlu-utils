from _ctypes import byref, Structure, POINTER
from contextlib import contextmanager
from ctypes import c_char_p, cdll, string_at, c_int
from pathlib import Path

PACKAGE_PATH = Path(__file__).absolute().parent

dylib_dir = PACKAGE_PATH / "dylib"
dylib_path = list(dylib_dir.glob("libsnips_nlu_utils_rs*"))[0]
lib = cdll.LoadLibrary(str(dylib_path))


class CStringArray(Structure):
    _fields_ = [
        ("data", POINTER(c_char_p)),
        ("size", c_int)
    ]

    def to_pylist(self):
        return [self.data[i].decode("utf8") for i in range(self.size)]


class CToken(Structure):
    _fields_ = [
        ("value", c_char_p),
        ("range_start", c_int),
        ("range_end", c_int),
        ("char_range_start", c_int),
        ("char_range_end", c_int),
    ]

    def to_pytoken(self):
        return {
            "value": self.value.decode("utf8"),
            "range": {
                "start": self.range_start,
                "end": self.range_end
            },
            "char_range": {
                "start": self.char_range_start,
                "end": self.char_range_end
            }
        }


class CTokenArray(Structure):
    _fields_ = [
        ("data", POINTER(CToken)),
        ("size", c_int)
    ]

    def to_pylist(self):
        return [self.data[i].to_pytoken() for i in range(self.size)]


class CNgram(Structure):
    _fields_ = [
        ("ngram", c_char_p),
        ("token_indexes", POINTER(c_int)),
        ("nb_token_indexes", c_int)
    ]

    def to_pytoken(self):
        return {
            "ngram": self.ngram.decode("utf8"),
            "token_indexes": [self.token_indexes[i] for i in range(self.nb_token_indexes)]
        }


class CNgramArray(Structure):
    _fields_ = [
        ("data", POINTER(CNgram)),
        ("size", c_int)
    ]

    def to_pylist(self):
        return [self.data[i].to_pytoken() for i in range(self.size)]


@contextmanager
def string_pointer(ptr):
    try:
        yield ptr
    finally:
        if ptr:
            lib.snips_nlu_utils_destroy_string(ptr)


@contextmanager
def string_array_pointer(ptr):
    try:
        yield ptr
    finally:
        if ptr and ptr.contents.data:
            lib.snips_nlu_utils_destroy_string_array(ptr)


@contextmanager
def token_array_pointer(ptr):
    try:
        yield ptr
    finally:
        if ptr and ptr.contents.data:
            lib.snips_nlu_utils_destroy_token_array(ptr)


@contextmanager
def ngram_array_pointer(ptr):
    try:
        yield ptr
    finally:
        if ptr and ptr.contents.data:
            lib.snips_nlu_utils_destroy_ngram_array(ptr)


def check_ffi_error(exit_code, error_context_msg):
    if exit_code != 0:
        with string_pointer(c_char_p()) as ptr:
            if lib.snips_nlu_utils_get_last_error(byref(ptr)) == 0:
                ffi_error_message = string_at(ptr).decode("utf8")
            else:
                ffi_error_message = "see stderr"
        raise ValueError("%s: %s" % (error_context_msg, ffi_error_message))
