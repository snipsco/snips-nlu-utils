use crate::trie::Trie;
use crate::StringTrieMap;

trait ToVal {
    fn to_val(&self) -> Vec<i64>;
}

impl ToVal for i32 {
    fn to_val(&self) -> Vec<i64> {
        vec![*self as i64]
    }
}

impl ToVal for &str {
    fn to_val(&self) -> Vec<i64> {
        let s = *self;
        s.bytes().map(|x| x as i64).collect::<Vec<i64>>()
    }
}

const TEST_DATA: [(&'static str, i32); 7] = [
    ("abcdefgh", 19),
    ("abcdef", 18),
    ("abcd", 17),
    ("ab", 16),
    ("a", 15),
    ("acbdef", 30),
    ("bcdefgh", 29),
];

fn dummy_trie() -> Trie {
    let mut t = Trie::new();
    for &(k, v) in &TEST_DATA {
        t.insert(k, v.to_val());
    }
    assert!(t.check_integrity());

    t
}

#[test]
fn get_nonexistant() {
    let trie = dummy_trie();
    assert!(trie.get("nonexistant").is_none());
    assert!(trie.get("").is_none());
}

#[test]
fn unicode() {
    let mut trie = Trie::new();
    trie.insert("bär", vec![1]);
    trie.insert("bären", vec![2]);

    assert_eq!(*trie.get("bär").unwrap(), vec![1]);
}

#[test]
fn empty_key() {
    let mut trie = dummy_trie();
    trie.insert("", vec![1]);
    assert_eq!(*trie.get("").unwrap(), vec![1]);
}

#[test]
fn insert() {
    let trie = dummy_trie();

    for &(key, val) in &TEST_DATA {
        assert_eq!(*trie.get(key).unwrap(), val.to_val());
    }

    assert!(trie.check_integrity());
    assert_eq!(trie.len(), TEST_DATA.len());
}

#[test]
fn insert_replace() {
    let mut trie = Trie::new();
    assert_eq!(trie.insert("haskell", vec![18]), None);
    let length = trie.len();
    assert_eq!(trie.insert("haskell", vec![36]), Some(vec![18]));
    assert_eq!(trie.len(), length);
}

#[test]
fn insert_similar() {
    let mut trie = Trie::new();
    trie.insert("a", vec![1, 4]);
    trie.insert("p", vec![1, 3]);
    assert_eq!(trie.get("a"), Some(&vec![1, 4]));
    assert_eq!(trie.get("p"), Some(&vec![1, 3]));
    trie.insert("a", vec![1, 5]);
    assert_eq!(trie.get("a"), Some(&vec![1, 5]));
    assert_eq!(trie.len(), 2);
    trie.remove("a");
    assert_eq!(trie.len(), 1);
    assert_eq!(trie.get("a"), None);
}

#[test]
fn remove() {
    let mut trie = dummy_trie();

    // Remove.
    for &(key, val) in &TEST_DATA {
        let res = trie.remove(key);
        assert_eq!(res, Some(val.to_val()));
        assert!(trie.check_integrity());
    }

    // Check non-existance.
    for &(key, _) in &TEST_DATA {
        assert!(trie.get(key).is_none());
    }
}

#[test]
fn remove_simple() {
    let mut trie = Trie::new();

    trie.insert("HELL", vec![66]);
    trie.insert("HELLO", vec![77]);
    let val = trie.remove("HELLO");
    assert_eq!(val, Some(vec![77]));
}

#[test]
fn remove_plus_insertion() {
    let mut trie = Trie::new();

    trie.insert("HELL", vec![66]);
    trie.insert("HELLO", vec![77]);
    let val = trie.remove("HELLO");
    trie.insert("HELLO", vec![88]);
    assert_eq!(val, Some(vec![77]));
    assert_eq!(trie.get("HELLO"), Some(&vec![88]));
}

#[test]
fn remove_non_existent() {
    let mut trie = Trie::new();

    trie.insert("acab", vec![1]);

    assert_eq!(trie.remove("abc"), None);
    assert_eq!(trie.remove("acaba"), None);
    assert_eq!(trie.remove("a"), None);
    assert_eq!(trie.remove(""), None);
    assert_eq!(trie.len(), 1);

    trie.insert("acaz", vec![1]);

    assert_eq!(trie.remove("acb"), None);
    assert_eq!(trie.remove("acaca"), None);
    assert_eq!(trie.remove("aca"), None);
    assert_eq!(trie.len(), 2);
}

#[test]
fn get_prefix_bug() {
    let mut trie = Trie::new();
    trie.insert("abdc", vec![5]);
    trie.insert("abde", vec![6]);
    assert!(trie.get("abc").is_none());
}

#[test]
fn root_replace_bug() {
    let mut trie = Trie::new();
    trie.insert("a", "".to_val());
    trie.insert("p", "".to_val());
    dbg!(&trie);
    dbg!(&trie.len());
    trie.remove("a");
    dbg!(&trie.len());
    assert_eq!(trie.len(), 1);
    trie.remove("p");
    assert_eq!(trie.len(), 0);
}

#[test]
fn test_get_borrow() {
    let mut trie = Trie::new();
    trie.insert("/boot".to_string(), "dir".to_val());
    assert_eq!(*trie.get("/boot").unwrap(), "dir".to_val());
}

#[test]
fn test_remove_borrow() {
    let mut trie = Trie::new();
    trie.insert("/boot".to_string(), "dir".to_val());
    assert_eq!(trie.remove("/boot").unwrap(), "dir".to_val());
}

#[test]
fn test_trie_map() {
    let mut map = StringTrieMap::new();
    map.insert("/boot", "dir");
    assert_eq!(map.get("/boot"), Some("dir".to_string()));
    map.insert("alpha", "beta");
    assert_eq!(map.get("alpha"), Some("beta".to_string()));
    assert_eq!(map.len(), 2);
    map.remove("alpha");
    assert_eq!(map.get("alpha"), None);
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("/boot"), Some("dir".to_string()));
}
