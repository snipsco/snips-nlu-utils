use failure::Fallible;
use fnv::FnvHasher;
use rmp_serde as rmps;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::hash::Hasher;
use std::io::{BufReader, Write};
use std::mem;
use std::path::Path;

const BRANCH_FACTOR: usize = 16;
const DEFAULT_KEY: u64 = 0xcbf2_9ce4_8422_2325;

type V = Vec<i64>;

macro_rules! no_kids {
    () => {
        [
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ]
    };
}

#[derive(Debug)]
/// variants for key matching
/// since the keys are sequences there can be full, partial or
/// no matches.
pub enum KeyMatch {
    /// partial key match: only part of the sequence match
    Partial(usize),
    /// full key sequence match
    Full,
    /// no match
    NoMatch,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
/// trie key
pub struct Key {
    data: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
struct Node {
    key: Key,
    val: Option<V>,
    children: [Option<Box<Node>>; BRANCH_FACTOR],
    child_count: i32,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
/// A recursive trie datastructure
pub struct Trie {
    length: usize,
    root: Node,
}

impl Key {
    /// get the internal bucket to which this key belongs, usually
    /// and index between 0 and 15
    pub fn get_bucket(&self) -> usize {
        debug_assert!(!self.is_empty());
        let entry = match self.data.len() % 2 {
            0 => self.data[0] >> 4,
            _ => self.data[0] & 0x0F,
        };

        entry as usize
    }

    /// split key at a specified index
    pub fn split(&mut self, idx: usize) -> Key {
        debug_assert!(self.data.len() > idx);
        Key {
            data: self.data.split_off(idx),
        }
    }

    /// join this key with another key: the other key becomes the suffix
    pub fn merge(&mut self, other: &mut Key) {
        self.data.append(&mut other.data);
    }

    /// length of the key: it's length of the underlying data
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// match two keys
    pub fn match_key(&self, other: &Key) -> KeyMatch {
        let min_len = ::std::cmp::min(self.len(), other.len());
        for i in 0..min_len {
            if self.data[i] != other.data[i] {
                if i == 0 {
                    // no match if index is zero
                    return KeyMatch::NoMatch;
                }
                return KeyMatch::Partial(i);
            }
        }

        // it's a Full match IFF both keys are of equal length
        if self.len() == other.len() {
            KeyMatch::Full
        } else {
            KeyMatch::Partial(min_len)
        }

        //KeyMatch::Full
    }

    /// check if key is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// create an empty key: used to initialize the root node.
    pub fn empty() -> Key {
        Key { data: vec![] }
    }
}

impl From<V> for Key {
    fn from(v: V) -> Key {
        let mut hasher = FnvHasher::with_key(DEFAULT_KEY);
        for i in v {
            hasher.write_i64(i);
        }
        let hash = hasher.finish();

        Key {
            data: hash.to_be_bytes().to_vec(),
        }
    }
}

impl From<&[u8]> for Key {
    fn from(v: &[u8]) -> Key {
        let mut hasher = FnvHasher::with_key(DEFAULT_KEY);
        hasher.write(v);
        let hash = hasher.finish();

        Key {
            data: hash.to_be_bytes().to_vec(),
        }
    }
}

impl From<&[i64]> for Key {
    fn from(v: &[i64]) -> Key {
        let mut hasher = FnvHasher::with_key(DEFAULT_KEY);
        for i in v {
            hasher.write_i64(*i);
        }
        let hash = hasher.finish();

        Key {
            data: hash.to_be_bytes().to_vec(),
        }
    }
}

impl From<String> for Key {
    fn from(input: String) -> Key {
        let mut hasher = FnvHasher::with_key(DEFAULT_KEY);
        hasher.write(input.as_bytes());
        let hash = hasher.finish();
        Key {
            data: hash.to_be_bytes().to_vec(),
        }
    }
}

impl From<&str> for Key {
    fn from(input: &str) -> Key {
        let mut hasher = FnvHasher::with_key(DEFAULT_KEY);
        hasher.write(input.as_bytes());
        let hash = hasher.finish();
        Key {
            data: hash.to_be_bytes().to_vec(),
        }
    }
}

impl Node {
    pub fn with_keyval(k: Key, v: Option<V>) -> Node {
        Node {
            key: k,
            val: v,
            children: no_kids!(),
            child_count: 0,
        }
    }

    pub fn get(&self, mut k: Key) -> Option<&V> {
        debug_assert!(!k.is_empty());
        let bkt = k.get_bucket();
        if let Some(ref child) = self.children[bkt] {
            match child.key.match_key(&k) {
                KeyMatch::Partial(idx) => {
                    return child.get(k.split(idx));
                }
                KeyMatch::Full => match child.val {
                    Some(ref val) => return Some(val),
                    None => return None,
                },
                KeyMatch::NoMatch => {
                    return None;
                }
            }
        } else if self.key == k {
            self.val.as_ref()
        } else {
            None
        }
    }

    pub fn insert(&mut self, mut k: Key, v: V) -> Option<V> {
        debug_assert!(!k.is_empty());
        let bkt = k.get_bucket();
        if let Some(ref mut child) = self.children[bkt] {
            match child.key.match_key(&k) {
                KeyMatch::Partial(idx) => {
                    if child.val.is_some() {
                        child.split(idx);
                    }
                    return child.insert(k.split(idx), v);
                }
                KeyMatch::Full => {
                    return child.replace_value(v);
                }
                KeyMatch::NoMatch => {
                    return child.insert(k, v);
                }
            }
        } else if self.key == k {
            self.replace_value(v)
        } else {
            self.add_child(bkt, k, v)
        }
    }

    pub fn remove(&mut self, mut k: Key) -> Option<V> {
        let bkt = k.get_bucket();
        if let Some(ref mut child) = self.children[bkt] {
            match child.key.match_key(&k) {
                KeyMatch::Full => {
                    let res = child.val.take();
                    self.children[bkt] = None;
                    self.child_count -= 1;
                    self.prune();
                    res
                }
                KeyMatch::Partial(idx) => {
                    return child.remove(k.split(idx));
                }
                KeyMatch::NoMatch => return None,
            }
        } else if self.key == k {
            self.val.take()
        } else {
            None
        }
    }

    fn prune(&mut self) {
        if self.child_count == 1 && self.val == None {
            // find the child: expensive op. optimize??
            let mut idx: usize = 0;
            for (i, val) in self.children.iter().enumerate() {
                if val.is_some() {
                    idx = i;
                    break;
                }
            }

            if let Some(ref mut child) = self.children[idx] {
                self.key.merge(&mut child.key);
                self.val = child.val.take();
            }

            std::mem::replace(&mut self.children[idx], None);
        }
    }

    pub fn split(&mut self, idx: usize) {
        debug_assert!(self.val.is_some());
        let suffix = self.key.split(idx);
        let val = self.val.take();
        self.insert(suffix, val.unwrap());
    }

    fn check_integrity(&self) -> bool {
        // check if non-root nodes with single child have values
        if !self.key.is_empty()
            && self.key.len() != 8
            && self.child_count == 1
            && self.val.is_none()
        {
            return false;
        }
        for i in 0..BRANCH_FACTOR {
            if let Some(ref child) = self.children[i] {
                match child.check_integrity() {
                    false => return false,
                    true => {}
                }
            }
        }
        true
    }

    fn add_child(&mut self, bucket: usize, k: Key, v: V) -> Option<V> {
        self.children[bucket] = Some(Box::new(Node::with_keyval(k, Some(v))));
        self.child_count += 1;
        None
    }

    fn replace_value(&mut self, v: V) -> Option<V> {
        match self.val {
            Some(ref mut val) => Some(mem::replace(val, v)),
            _ => unreachable!(),
        }
    }
}

impl Trie {
    /// create a new empty trie
    pub fn new() -> Trie {
        Trie {
            length: 0,
            root: Node {
                key: Key::empty(),
                val: None,
                children: no_kids!(),
                child_count: 0,
            },
        }
    }

    /// insert key-value into the trie
    pub fn insert<K: Into<Key>, J: Into<V>>(&mut self, key: K, val: J) -> Option<V> {
        let key = key.into();
        let val = val.into();
        match self.root.insert(key, val) {
            Some(res) => Some(res),
            None => {
                self.length += 1;
                None
            }
        }
    }

    /// remove key from the trie
    pub fn remove<K: Into<Key>>(&mut self, k: K) -> Option<V> {
        let k = k.into();
        match self.root.remove(k) {
            Some(v) => {
                self.length -= 1;
                Some(v)
            }
            None => None,
        }
    }

    /// retrieve the value matching the given key
    pub fn get<K: Into<Key>>(&self, key: K) -> Option<&V> {
        self.root.get(key.into())
    }

    /// check if trie is empty
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// length of the trie
    pub fn len(&self) -> usize {
        self.length
    }

    /// check if the trie obeys expected invariants / validity checks
    pub fn check_integrity(&self) -> bool {
        self.root.check_integrity()
    }

    /// serialize trie onto a file
    pub fn dump<P: AsRef<Path>>(&self, filename: P) -> Fallible<()> {
        let fname = filename.as_ref();
        let mut ofile = File::create(fname)?;
        ofile.write_all(&rmps::encode::to_vec(&self)?)?;
        Ok(())
    }

    /// load trie from a file
    pub fn load<P: AsRef<Path>>(filepath: P) -> Fallible<Trie> {
        let fpath = filepath.as_ref();
        let ifile = File::open(fpath)?;
        let rdr = BufReader::new(ifile);
        let trie: Trie = rmps::decode::from_read(rdr)?;
        Ok(trie)
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}