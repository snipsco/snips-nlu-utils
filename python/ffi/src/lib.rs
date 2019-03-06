mod destroy;
mod string;
mod token;
mod types;

use crate::types::CNgramArray;
use ffi_utils::*;

type Result<T> = ::std::result::Result<T, ::failure::Error>;

generate_error_handling!(snips_nlu_utils_get_last_error);

#[no_mangle]
pub extern "C" fn snips_nlu_utils_destroy_string(ptr: *mut libc::c_char) -> SNIPS_RESULT {
    wrap!(destroy::destroy_string_c(ptr))
}

#[no_mangle]
pub extern "C" fn snips_nlu_utils_destroy_string_array(
    ptr: *mut ::ffi_utils::CStringArray,
) -> SNIPS_RESULT {
    wrap!(destroy::destroy_string_array_c(ptr))
}

#[no_mangle]
pub extern "C" fn snips_nlu_utils_destroy_ngram_array(
    ptr: *mut types::CNgramArray,
) -> SNIPS_RESULT {
    wrap!(destroy::destroy_ngram_array_c(ptr))
}

#[no_mangle]
pub extern "C" fn snips_nlu_utils_destroy_token_array(
    ptr: *mut types::CTokenArray,
) -> SNIPS_RESULT {
    wrap!(destroy::destroy_token_array_c(ptr))
}

#[no_mangle]
pub extern "C" fn snips_nlu_utils_remove_diacritics(
    input: *const ::libc::c_char,
    result: *mut *const ::libc::c_char,
) -> SNIPS_RESULT {
    wrap!(string::remove_diacritics_c(input, result))
}

#[no_mangle]
pub extern "C" fn snips_nlu_utils_normalize(
    input: *const ::libc::c_char,
    result: *mut *const ::libc::c_char,
) -> SNIPS_RESULT {
    wrap!(string::normalize_c(input, result))
}

#[no_mangle]
pub extern "C" fn snips_nlu_utils_get_shape(
    input: *const ::libc::c_char,
    result: *mut *const ::libc::c_char,
) -> SNIPS_RESULT {
    wrap!(string::get_shape_c(input, result))
}

#[no_mangle]
pub extern "C" fn snips_nlu_utils_hash_str_to_i32(
    input: *const ::libc::c_char,
    result: *mut ::libc::c_int,
) -> SNIPS_RESULT {
    wrap!(string::hash_str_to_i32_c(input, result))
}

#[no_mangle]
pub extern "C" fn snips_nlu_utils_tokenize(
    input: *const ::libc::c_char,
    language: *const ::libc::c_char,
    result: *mut *const types::CTokenArray,
) -> SNIPS_RESULT {
    wrap!(token::tokenize_c(input, language, result))
}

#[no_mangle]
pub extern "C" fn snips_nlu_utils_tokenize_light(
    input: *const ::libc::c_char,
    language: *const ::libc::c_char,
    result: *mut *const ::ffi_utils::CStringArray,
) -> SNIPS_RESULT {
    wrap!(token::tokenize_light_c(input, language, result))
}

#[no_mangle]
pub extern "C" fn snips_nlu_utils_compute_all_ngrams(
    tokens: *const CStringArray,
    max_ngram_size: libc::c_uint,
    result: *mut *const CNgramArray,
) -> SNIPS_RESULT {
    wrap!(token::compute_all_ngrams_c(tokens, max_ngram_size, result))
}
