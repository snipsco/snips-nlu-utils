#[deny(missing_docs)]
use failure::Fallible;
use failure::ResultExt;
use ffi_utils::*;
use libc;
use snips_nlu_utils::StringTrieMap;

generate_error_handling!(ffi_get_last_error);

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
