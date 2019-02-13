mod destroy;
mod string;
mod token;
mod types;

use failure::{Fallible, ResultExt};
use ffi_utils::*;
use snips_nlu_utils::StringTrieMap;

type Result<T> = ::std::result::Result<T, ::failure::Error>;

generate_error_handling!(snips_nlu_utils_get_last_error);

#[no_mangle]
/// create a new string trie map
pub extern "C" fn trie_map_new(map_ptr: *mut *const StringTrieMap) -> SNIPS_RESULT {
    let logic = || -> Fallible<()> {
        let map = StringTrieMap::new();
        unsafe { *map_ptr = map.into_raw_pointer() };
        Ok(())
    };
    wrap!(logic())
}

#[no_mangle]
/// insert string key-value into the map
pub unsafe extern "C" fn trie_map_insert(
    map_ptr: *mut StringTrieMap,
    k: *const libc::c_char,
    v: *const libc::c_char,
) -> SNIPS_RESULT {
    let logic = || -> Fallible<()> {
        let map = StringTrieMap::raw_borrow_mut(map_ptr)?;
        let key = create_rust_string_from!(k);
        let val = create_rust_string_from!(v);
        map.insert(key, val);
        Ok(())
    };

    wrap!(logic())
}

/// get value corresponding to key from the map
#[no_mangle]
pub unsafe extern "C" fn trie_map_get(
    map_ptr: *mut StringTrieMap,
    k: *const libc::c_char,
    v: *mut *const libc::c_char,
) -> SNIPS_RESULT {
    let logic = || -> Fallible<()> {
        let map = StringTrieMap::raw_borrow(map_ptr)?;
        let key = create_rust_string_from!(k);
        if let Some(val) = map.get(key) {
            *v = convert_to_c_string!(val);
        }
        Ok(())
    };

    wrap!(logic())
}

/// remove key from the map
#[no_mangle]
pub unsafe extern "C" fn trie_map_remove(
    map_ptr: *mut StringTrieMap,
    k: *const libc::c_char,
    v: *mut *const libc::c_char,
) -> SNIPS_RESULT {
    let logic = || -> Fallible<()> {
        let map = StringTrieMap::raw_borrow_mut(map_ptr)?;
        let key = create_rust_string_from!(k);
        if let Some(val) = map.remove(key) {
            *v = convert_to_c_string!(val);
        }
        Ok(())
    };

    wrap!(logic())
}

#[no_mangle]
/// dump trie map to the file system
pub extern "C" fn trie_map_dump(
    map_ptr: *mut StringTrieMap,
    path: *const libc::c_char,
) -> SNIPS_RESULT {
    let logic = || -> Fallible<()> {
        let map = unsafe { StringTrieMap::raw_borrow(map_ptr)? };
        map.dump(create_rust_string_from!(path))?;
        Ok(())
    };

    wrap!(logic())
}

#[no_mangle]
/// load trie map from the file system
pub extern "C" fn trie_map_load(
    map_ptr: *mut *const StringTrieMap,
    path: *const libc::c_char,
) -> SNIPS_RESULT {
    let logic = || -> Fallible<()> {
        let map = StringTrieMap::load(create_rust_string_from!(path))?;
        unsafe { *map_ptr = map.into_raw_pointer() };
        Ok(())
    };

    wrap!(logic())
}

#[no_mangle]
/// length of the trie map
pub unsafe extern "C" fn trie_map_len(
    symt_ptr: *mut StringTrieMap,
    length: *mut libc::size_t,
) -> SNIPS_RESULT {
    let logic = || -> Fallible<()> {
        let map = StringTrieMap::raw_borrow(symt_ptr)?;
        *length = map.len();
        Ok(())
    };

    wrap!(logic())
}

#[no_mangle]
/// drop trie map
pub unsafe extern "C" fn trie_map_drop(map_ptr: *mut StringTrieMap) -> SNIPS_RESULT {
    wrap!(StringTrieMap::from_raw_pointer(map_ptr))
}

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
