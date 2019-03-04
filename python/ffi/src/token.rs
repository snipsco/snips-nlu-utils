use crate::types::CNgramArray;
use crate::types::CTokenArray;
use crate::Result;
use ffi_utils::{convert_to_c_string_array, AsRust, CReprOf, CStringArray, RawPointerConverter};
use snips_nlu_utils::language::Language;
use snips_nlu_utils::token::*;
use std::ffi::CStr;
use std::str::FromStr;

pub fn tokenize_c(
    input: *const libc::c_char,
    language: *const libc::c_char,
    result: *mut *const CTokenArray,
) -> Result<()> {
    let str_input = unsafe { CStr::from_ptr(input) }.to_str()?;
    let str_lang = unsafe { CStr::from_ptr(language) }.to_str()?;
    let language = Language::from_str(str_lang)?;
    let tokens = tokenize(str_input, language);
    let c_token_array = CTokenArray::c_repr_of(tokens)?.into_raw_pointer();
    unsafe { *result = c_token_array };
    Ok(())
}

pub fn tokenize_light_c(
    input: *const libc::c_char,
    language: *const libc::c_char,
    result: *mut *const CStringArray,
) -> Result<()> {
    let str_input = unsafe { CStr::from_ptr(input) }.to_str()?;
    let str_lang = unsafe { CStr::from_ptr(language) }.to_str()?;
    let language = Language::from_str(str_lang)?;
    let tokens = tokenize_light(str_input, language);
    let cs = convert_to_c_string_array!(tokens);
    unsafe { *result = cs };
    Ok(())
}

pub fn compute_all_ngrams_c(
    tokens: *const CStringArray,
    max_ngram_size: libc::c_uint,
    result: *mut *const CNgramArray,
) -> Result<()> {
    let tokens_vec = unsafe { (*tokens).as_rust()? };
    let tokens_slice: Vec<&str> = tokens_vec.iter().map(|token| &**token).collect();
    let ngrams = compute_all_ngrams(&tokens_slice, max_ngram_size as usize);
    let c_ngrams = CNgramArray::c_repr_of(ngrams)?.into_raw_pointer();
    unsafe { *result = c_ngrams };
    Ok(())
}
