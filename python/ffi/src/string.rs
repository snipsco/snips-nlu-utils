use crate::Result;
use ffi_utils::{convert_to_c_string, CReprOf, RawPointerConverter};
use snips_nlu_utils::string::*;
use std::ffi::CStr;

pub fn remove_diacritics_c(
    input: *const ::libc::c_char,
    result: *mut *const ::libc::c_char,
) -> Result<()> {
    let str_input = unsafe { CStr::from_ptr(input) }.to_str()?;
    let cleaned_string = remove_diacritics(str_input);
    let cs = convert_to_c_string!(cleaned_string);
    unsafe { *result = cs };
    Ok(())
}

pub fn normalize_c(input: *const ::libc::c_char, result: *mut *const ::libc::c_char) -> Result<()> {
    let str_input = unsafe { CStr::from_ptr(input) }.to_str()?;
    let cleaned_string = normalize(str_input);
    let cs = convert_to_c_string!(cleaned_string);
    unsafe { *result = cs };
    Ok(())
}

pub fn get_shape_c(input: *const ::libc::c_char, result: *mut *const ::libc::c_char) -> Result<()> {
    let str_input = unsafe { CStr::from_ptr(input) }.to_str()?;
    let shape = get_shape(str_input).to_string();
    let cs = convert_to_c_string!(shape);
    unsafe { *result = cs };
    Ok(())
}

pub fn hash_str_to_i32_c(input: *const ::libc::c_char, result: *mut ::libc::c_int) -> Result<()> {
    let str_input = unsafe { CStr::from_ptr(input) }.to_str()?;
    let hash = hash_str_to_i32(str_input);
    unsafe { *result = hash };
    Ok(())
}
