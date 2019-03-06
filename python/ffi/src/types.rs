use failure::{Error, ResultExt};
use ffi_utils::{convert_to_c_string, take_back_c_string, CReprOf, RawPointerConverter};
use snips_nlu_utils::token::{Ngram, Token};

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
    pub size: libc::c_uint,
}

impl CReprOf<Vec<Token>> for CTokenArray {
    fn c_repr_of(input: Vec<Token>) -> Result<Self, Error> {
        Ok(Self {
            size: input.len() as libc::c_uint,
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

#[repr(C)]
#[derive(Debug)]
pub struct CNgram {
    pub ngram: *const libc::c_char,
    pub token_indexes: *const libc::c_uint,
    pub nb_token_indexes: libc::c_uint,
}

impl CReprOf<Ngram> for CNgram {
    fn c_repr_of(input: Ngram) -> Result<Self, Error> {
        let ngram = convert_to_c_string!(input.0);
        let nb_token_indexes = input.1.len() as libc::c_uint;
        let token_indexes = Box::into_raw(
            input
                .1
                .into_iter()
                .map(|index| index as libc::c_uint)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        ) as *const libc::c_uint;
        Ok(Self {
            ngram,
            token_indexes,
            nb_token_indexes,
        })
    }
}

impl Drop for CNgram {
    fn drop(&mut self) {
        take_back_c_string!(self.ngram);
        let _ = unsafe {
            Box::from_raw(std::slice::from_raw_parts_mut(
                self.token_indexes as *mut libc::c_uint,
                self.nb_token_indexes as usize,
            ))
        };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CNgramArray {
    pub data: *const CNgram,
    pub size: libc::c_uint,
}

impl CReprOf<Vec<Ngram>> for CNgramArray {
    fn c_repr_of(input: Vec<Ngram>) -> Result<Self, Error> {
        Ok(Self {
            size: input.len() as libc::c_uint,
            data: Box::into_raw(
                input
                    .into_iter()
                    .map(|ngram| CNgram::c_repr_of(ngram))
                    .collect::<Result<Vec<CNgram>, _>>()
                    .context("Could not convert Vector of Ngram to C Repr")?
                    .into_boxed_slice(),
            ) as *const CNgram,
        })
    }
}

impl Drop for CNgramArray {
    fn drop(&mut self) {
        let _ = unsafe {
            Box::from_raw(std::slice::from_raw_parts_mut(
                self.data as *mut CNgram,
                self.size as usize,
            ))
        };
    }
}
