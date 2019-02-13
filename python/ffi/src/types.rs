use failure::{Error, ResultExt};
use ffi_utils::{convert_to_c_string, take_back_c_string, CReprOf, RawPointerConverter};
use snips_nlu_utils::token::Token;

#[repr(C)]
#[derive(Debug)]
pub struct CToken {
    pub value: *const libc::c_char,
    pub range_start: libc::c_uint,
    pub range_end: libc::c_uint,
    pub char_range_start: libc::c_uint,
    pub char_range_end: libc::c_uint,
}

impl CReprOf<Token> for CToken {
    fn c_repr_of(input: Token) -> Result<Self, Error> {
        let value = convert_to_c_string!(input.value);
        Ok(Self {
            value,
            range_start: input.range.start as libc::c_uint,
            range_end: input.range.end as libc::c_uint,
            char_range_start: input.char_range.start as libc::c_uint,
            char_range_end: input.char_range.end as libc::c_uint,
        })
    }
}

impl Drop for CToken {
    fn drop(&mut self) {
        take_back_c_string!(self.value);
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CTokenArray {
    pub data: *const CToken,
    pub size: libc::c_int,
}

impl CReprOf<Vec<Token>> for CTokenArray {
    fn c_repr_of(input: Vec<Token>) -> Result<Self, Error> {
        Ok(Self {
            size: input.len() as libc::c_int,
            data: Box::into_raw(
                input
                    .into_iter()
                    .map(|token| CToken::c_repr_of(token))
                    .collect::<Result<Vec<CToken>, _>>()
                    .context("Could not convert Vector of Token to C Repr")?
                    .into_boxed_slice(),
            ) as *const CToken,
        })
    }
}

impl Drop for CTokenArray {
    fn drop(&mut self) {
        let _ = unsafe {
            Box::from_raw(std::slice::from_raw_parts_mut(
                self.data as *mut CToken,
                self.size as usize,
            ))
        };
    }
}
