use failure::Fallible;
use fnv::FnvHasher;
use rmp_serde as rmps;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hasher;
use std::io::{BufReader, Write};
use std::path::Path;

const DEFAULT_KEY: u64 = 0xcbf2_9ce4_8422_2325;

#[derive(Debug, Default, Deserialize, Serialize)]
/// A string to i64 symbol table
pub struct SymbolTable {
    table: HashMap<i64, String>,
    count: i32,
}

impl SymbolTable {
    /// new empty symbol table
    pub fn new() -> SymbolTable {
        SymbolTable {
            table: HashMap::new(),
            count: 0,
        }
    }

    /// add symbol to table
    pub fn add_symbol<S: Into<String>>(&mut self, symbol: S) -> i64 {
        let symbol = symbol.into();
        let mut hasher = FnvHasher::with_key(DEFAULT_KEY);
        // can we avoid this clone?
        hasher.write(&symbol.clone().into_bytes());

        let hash = hasher.finish();

        self.table.insert(hash as i64, symbol);
        self.count += 1;
        hash as i64
    }

    /// get symbol matching given key
    pub fn get_symbol(&self, key: i64) -> Option<&String> {
        self.table.get(&key)
    }

    /// get key matching given symbol
    pub fn get_key<S: Into<String>>(&self, symbol: S) -> Option<i64> {
        let symbol = symbol.into();
        let mut hasher = FnvHasher::with_key(DEFAULT_KEY);
        hasher.write(&symbol.into_bytes());
        let hash = hasher.finish() as i64;

        if self.table.contains_key(&hash) {
            Some(hash)
        } else {
            None
        }
    }

    /// serialize symbol table onto a file using the MessagePack format
    pub fn dump<P: AsRef<Path>>(&self, filename: P) -> Fallible<()> {
        let fname = filename.as_ref();
        let mut ofile = File::create(fname)?;
        ofile.write_all(&rmps::encode::to_vec(&self)?)?;
        Ok(())
    }

    /// load a symbol table from a file
    pub fn load<P: AsRef<Path>>(filepath: P) -> Fallible<SymbolTable> {
        let fpath = filepath.as_ref();
        let ifile = File::open(fpath)?;
        let rdr = BufReader::new(ifile);
        let table: SymbolTable = rmps::decode::from_read(rdr)?;

        Ok(table)
    }

    /// length of the symbol table
    pub fn len(&self) -> i32 {
        self.count
    }

    /// check if symbol table is empty
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

impl PartialEq for SymbolTable {
    fn eq(&self, other: &SymbolTable) -> bool {
        self.count == other.count && self.table == other.table
    }
}

impl Eq for SymbolTable {}
