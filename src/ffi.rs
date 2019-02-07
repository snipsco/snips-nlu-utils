use crate::trie::Trie;
use crate::SymbolTable;
use failure::Fallible;
use failure::ResultExt;
use ffi_utils::*;
use libc::{c_long, size_t};

generate_error_handling!(ffi_get_last_error);

#[no_mangle]
/// create a new trie
pub extern "C" fn trie_new(trie_ptr: *mut *const Trie) -> SNIPS_RESULT {
    let logic = || -> Fallible<()> {
        let trie = Trie::new();
        unsafe { *trie_ptr = trie.into_raw_pointer() };
        Ok(())
    };
    wrap!(logic())
}

#[no_mangle]
/// insert a key-value into the trie
pub extern "C" fn trie_insert(
    trie_ptr: *mut Trie,
    key: *const c_long,
    keylen: size_t,
    val: *const c_long,
    val_len: size_t,
) -> SNIPS_RESULT {
    let logic = || -> Fallible<()> {
        unsafe {
            let trie = Trie::raw_borrow_mut(trie_ptr)?;
            let k = std::slice::from_raw_parts(key, keylen);
            let v = std::slice::from_raw_parts(val, val_len);
            trie.insert(k, v);
        }
        Ok(())
    };

    wrap!(logic())
}

#[no_mangle]
/// get value matching key from trie
pub extern "C" fn trie_get(
    trie_ptr: *mut Trie,
    key: *const c_long,
    keylen: size_t,
    val: *mut *const c_long,
    val_len: *mut size_t,
) -> SNIPS_RESULT {
    let logic = || -> Fallible<()> {
        unsafe {
            let trie = Trie::raw_borrow_mut(trie_ptr)?;
            let k = std::slice::from_raw_parts(key, keylen);
            match trie.get(k) {
                Some(v) => {
                    *val_len = v.len();
                    *val = v.as_ptr();
                }
                None => {
                    *val_len = 0;
                    *val = std::ptr::null();
                }
            }
        }
        Ok(())
    };

    wrap!(logic())
}

#[no_mangle]
/// dump trie to the file system
pub extern "C" fn trie_dump(trie_ptr: *mut Trie, path: *const i8) -> SNIPS_RESULT {
    let logic = || -> Fallible<()> {
        let trie = unsafe { Trie::raw_borrow(trie_ptr)? };
        trie.dump(create_rust_string_from!(path))?;
        Ok(())
    };

    wrap!(logic())
}

#[no_mangle]
/// load trie from the file system
pub extern "C" fn trie_load(trie_ptr: *mut *const Trie, path: *const i8) -> SNIPS_RESULT {
    let logic = || -> Fallible<()> {
        let trie = Trie::load(create_rust_string_from!(path))?;
        unsafe { *trie_ptr = trie.into_raw_pointer() };
        Ok(())
    };

    wrap!(logic())
}

#[no_mangle]
/// check if two trie are equal
pub unsafe extern "C" fn trie_eq(
    trie_a_ptr: *mut Trie,
    trie_b_ptr: *mut Trie,
    eq: *mut size_t,
) -> SNIPS_RESULT {
    let logic = || -> Fallible<()> {
        let trie_a = Trie::raw_borrow(trie_a_ptr)?;
        let trie_b = Trie::raw_borrow(trie_b_ptr)?;
        *eq = (trie_a == trie_b) as usize;
        Ok(())
    };

    wrap!(logic())
}

#[no_mangle]
/// length of the trie
pub unsafe extern "C" fn trie_len(trie_ptr: *mut Trie, length: *mut size_t) -> SNIPS_RESULT {
    let logic = || -> Fallible<()> {
        let trie = Trie::raw_borrow(trie_ptr)?;
        *length = trie.len() as usize;
        Ok(())
    };

    wrap!(logic())
}

#[no_mangle]
/// drop a trie
pub unsafe extern "C" fn trie_drop(trie_ptr: *mut Trie) -> SNIPS_RESULT {
    wrap!(Trie::from_raw_pointer(trie_ptr))
}

#[no_mangle]
/// create a new symbol table
pub extern "C" fn symt_new(symt_ptr: *mut *const SymbolTable) -> SNIPS_RESULT {
    let logic = || -> Fallible<()> {
        let table = SymbolTable::new();
        unsafe { *symt_ptr = table.into_raw_pointer() };
        Ok(())
    };

    wrap!(logic())
}

#[no_mangle]
/// insert symbol into the symbol table
pub extern "C" fn symt_add_symbol(
    symt_ptr: *mut SymbolTable,
    sym: *const i8,
    key_ptr: *mut c_long,
) -> SNIPS_RESULT {
    let logic = || -> Fallible<()> {
        let table = unsafe { SymbolTable::raw_borrow_mut(symt_ptr)? };
        let key = table.add_symbol(create_rust_string_from!(sym));
        unsafe { *key_ptr = key }
        Ok(())
    };

    wrap!(logic())
}

#[no_mangle]
/// get symbol matching the given key from the table
pub unsafe extern "C" fn symt_get_symbol(
    symt_ptr: *mut SymbolTable,
    key: c_long,
    symbol_ptr: *mut *const i8,
) -> SNIPS_RESULT {
    let logic = || -> Fallible<()> {
        let table = SymbolTable::raw_borrow(symt_ptr)?;
        let symbol = table.get_symbol(key);
        if let Some(sym) = symbol {
            *symbol_ptr = convert_to_c_string!(sym.to_string())
        }
        Ok(())
    };

    wrap!(logic())
}

#[no_mangle]
/// get key matching given symbol from the table
pub unsafe extern "C" fn symt_get_key(
    symt_ptr: *mut SymbolTable,
    sym: *const i8,
    key_ptr: *mut c_long,
) -> SNIPS_RESULT {
    let logic = || -> Fallible<()> {
        let table = SymbolTable::raw_borrow(symt_ptr)?;
        let key = table.get_key(create_rust_string_from!(sym));
        if let Some(val) = key {
            *key_ptr = val
        }
        Ok(())
    };

    wrap!(logic())
}

#[no_mangle]
/// dump the symbol table to the file system
pub extern "C" fn symt_dump(symt_ptr: *mut SymbolTable, path: *const i8) -> SNIPS_RESULT {
    let logic = || -> Fallible<()> {
        let table = unsafe { SymbolTable::raw_borrow(symt_ptr)? };
        table.dump(create_rust_string_from!(path))?;
        Ok(())
    };

    wrap!(logic())
}

#[no_mangle]
/// load a symbol table from the file system
pub extern "C" fn symt_load(symt_ptr: *mut *const SymbolTable, path: *const i8) -> SNIPS_RESULT {
    let logic = || -> Fallible<()> {
        let table = SymbolTable::load(create_rust_string_from!(path))?;
        unsafe { *symt_ptr = table.into_raw_pointer() };
        Ok(())
    };

    wrap!(logic())
}

#[no_mangle]
/// length of the symbol table
pub unsafe extern "C" fn symt_len(symt_ptr: *mut SymbolTable, length: *mut size_t) -> SNIPS_RESULT {
    let logic = || -> Fallible<()> {
        let table = SymbolTable::raw_borrow(symt_ptr)?;
        *length = table.len() as usize;
        Ok(())
    };

    wrap!(logic())
}

#[no_mangle]
/// check if two symbol tables are equal
pub unsafe extern "C" fn symt_eq(
    symt_a_ptr: *mut SymbolTable,
    symt_b_ptr: *mut SymbolTable,
    eq: *mut size_t,
) -> SNIPS_RESULT {
    let logic = || -> Fallible<()> {
        let table_a = SymbolTable::raw_borrow(symt_a_ptr)?;
        let table_b = SymbolTable::raw_borrow(symt_b_ptr)?;
        *eq = (table_a == table_b) as usize;
        Ok(())
    };

    wrap!(logic())
}

#[no_mangle]
/// drop symbol table
pub unsafe extern "C" fn symt_drop(symt_ptr: *mut SymbolTable) -> SNIPS_RESULT {
    wrap!(SymbolTable::from_raw_pointer(symt_ptr))
}
