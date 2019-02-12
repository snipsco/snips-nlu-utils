use crate::Result;
use ffi_utils::{take_back_c_string, take_back_c_string_array, CStringArray, RawPointerConverter};
use crate::types::CTokenArray;

pub fn destroy_string_c(ptr: *mut libc::c_char) -> Result<()> {
    take_back_c_string!(ptr);
    Ok(())
}

pub fn destroy_string_array_c(ptr: *mut CStringArray) -> Result<()> {
    take_back_c_string_array!(ptr);
    Ok(())
}

pub fn destroy_token_array_c(ptr: *mut CTokenArray) -> Result<()> {
    let _ = unsafe { CTokenArray::from_raw_pointer(ptr) };
    Ok(())
}
