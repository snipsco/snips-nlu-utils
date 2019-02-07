pub mod language;
pub mod range;
pub mod string;
pub mod token;

#[deny(missing_docs)]
/// A string => int symbol table
pub mod symboltable;

#[deny(missing_docs)]
/// A non-generic trie map implementation 
pub mod trie;

pub use symboltable::SymbolTable;
pub use trie::Trie;

#[cfg(test)]
mod tests;

#[deny(missing_docs)]
/// a string => string map
pub mod string_trie_map;
pub use string_trie_map::StringTrieMap;
