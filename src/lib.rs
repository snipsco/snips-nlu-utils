pub mod language;
pub mod range;
pub mod string;
pub mod token;

#[deny(missing_docs)]
/// FFI interface module
pub mod ffi;

#[deny(missing_docs)]
/// A string => int symbol table
pub mod symboltable;

#[deny(missing_docs)]
/// A non-generic trie map implementation 
pub mod trie;

pub use symboltable::SymbolTable;

#[cfg(test)]
mod tests;
