use crate::{SymbolTable, Trie};
use failure::Fallible;

#[derive(Debug)]
/// a string-to-string key-value mapping
/// build using the Trie and SymbolTable
/// data structures.
pub struct StringTrieMap {
    trie: Trie,
    table: SymbolTable,
}

impl StringTrieMap {
    /// create a new map
    pub fn new() -> StringTrieMap {
        StringTrieMap {
            trie: Trie::new(),
            table: SymbolTable::new(),
        }
    }

    /// insert a key-value pair into the map
    pub fn insert<S: Into<String>>(&mut self, k: S, v: S) {
        let key = k
            .into()
            .split_whitespace()
            .map(|x| self.table.add_symbol(x))
            .collect::<Vec<i64>>();
        let val = v
            .into()
            .split_whitespace()
            .map(|x| self.table.add_symbol(x))
            .collect::<Vec<i64>>();
        self.trie.insert(key, val);
    }

    /// get the value corresponding to the given key
    pub fn get<S: Into<String>>(&self, s: S) -> Option<String> {
        self.string_to_symbols(s).and_then(|key| {
            self.trie.get(key).and_then(|trie_val| {
                let mut res = vec![];
                for frag in trie_val {
                    if let Some(val) = self.table.get_symbol(*frag) {
                        res.push(val.clone());
                    } else {
                        return None;
                    }
                }
                Some(res.join("  "))
            })
        })
    }

    /// remove key from trie
    ///
    /// NOTE: we do not touch the symbol table pontentially
    /// leaving unused symbols. If more expensive to track
    /// and remove unused symbols therefore the ideal usecase
    /// you avoid removing keys.
    pub fn remove<S: Into<String>>(&mut self, s: S) -> Option<String> {
        // create key from string
        self.string_to_symbols(s)
            .and_then(|key| match self.trie.remove(key) {
                Some(val) => self.trie_val_to_string(val),
                None => None,
            })
    }

    /// dump the map to the filesystem
    pub fn dump(&self, path: String) -> Fallible<()> {
        let table_path = format!("{}.table", path);
        let trie_path = format!("{}.trie", path);
        self.table.dump(table_path)?;
        self.trie.dump(trie_path)?;
        Ok(())
    }

    /// deserialize a map from the filesystem
    pub fn load(path: String) -> Fallible<(StringTrieMap)> {
        let table_path = format!("{}.table", path);
        let trie_path = format!("{}.trie", path);
        let table = SymbolTable::load(table_path)?;
        let trie = Trie::load(trie_path)?;
        Ok(Self { trie, table })
    }

    /// length of the key-value map
    pub fn len(&self) -> usize {
        self.trie.len()
    }

    fn trie_val_to_string(&self, val: Vec<i64>) -> Option<String> {
        let mut res = vec![];
        for frag in val {
            if let Some(val) = self.table.get_symbol(frag) {
                res.push(val.clone());
            } else {
                return None;
            }
        }
        Some(res.join("  "))
    }

    fn string_to_symbols<S: Into<String>>(&self, s: S) -> Option<Vec<i64>> {
        let mut syms = vec![];
        for frag in s.into().split_whitespace() {
            if let Some(res) = self.table.get_key(frag) {
                syms.push(res);
            } else {
                return None;
            }
        }
        Some(syms)
    }
}
