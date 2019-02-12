mod destroy;
mod string;
mod token;
mod types;

pub use destroy::*;
pub use string::*;
pub use token::*;
pub use types::*;

type Result<T> = ::std::result::Result<T, ::failure::Error>;

#[macro_export]
macro_rules! export_nlu_utils_c_symbols {
    () => {
        #[no_mangle]
        pub extern "C" fn snips_nlu_utils_destroy_string(
            ptr: *mut libc::c_char,
        ) -> SNIPS_RESULT {
            wrap!($crate::destroy_string_c(ptr))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_utils_destroy_string_array(
            ptr: *mut ::ffi_utils::CStringArray,
        ) -> SNIPS_RESULT {
            wrap!($crate::destroy_string_array_c(ptr))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_utils_destroy_token_array(
            ptr: *mut $crate::CTokenArray,
        ) -> SNIPS_RESULT {
            wrap!($crate::destroy_token_array_c(ptr))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_utils_remove_diacritics(
            input: *const ::libc::c_char,
            result: *mut *const ::libc::c_char,
        ) -> SNIPS_RESULT {
            wrap!($crate::remove_diacritics_c(input, result))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_utils_normalize(
            input: *const ::libc::c_char,
            result: *mut *const ::libc::c_char,
        ) -> SNIPS_RESULT {
            wrap!($crate::normalize_c(input, result))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_utils_get_shape(
            input: *const ::libc::c_char,
            result: *mut *const ::libc::c_char,
        ) -> SNIPS_RESULT {
            wrap!($crate::get_shape_c(input, result))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_utils_tokenize(
            input: *const ::libc::c_char,
            language: *const ::libc::c_char,
            result: *mut *const $crate::CTokenArray,
        ) -> SNIPS_RESULT {
            wrap!($crate::tokenize_c(input, language, result))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_utils_tokenize_light(
            input: *const ::libc::c_char,
            language: *const ::libc::c_char,
            result: *mut *const ::ffi_utils::CStringArray,
        ) -> SNIPS_RESULT {
            wrap!($crate::tokenize_light_c(input, language, result))
        }
    };
}
