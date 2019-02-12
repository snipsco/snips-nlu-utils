use crate::trie::Trie;
use crate::StringTrieMap;
use tempfile::tempdir;

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
        t.insert(k.to_val(), v.to_val());
    }
    assert!(t.check_integrity());

    t
}

#[test]
fn get_nonexistant() {
    let trie = dummy_trie();
    assert!(trie.get("nonexistant".to_val()).is_none());
    assert!(trie.get("".to_val()).is_none());
    assert!(trie.get("abcx".to_val()).is_none());
    assert!(trie.get("x".to_val()).is_none());
}

#[test]
fn unicode() {
    let mut trie = Trie::new();
    trie.insert("bär".to_val(), vec![1]);
    trie.insert("bären".to_val(), vec![2]);

    assert_eq!(*trie.get("bär".to_val()).unwrap(), vec![1]);
}

#[test]
fn insert_replace() {
    let mut trie = Trie::new();
    assert_eq!(trie.insert("haskell".to_val(), vec![18]), None);
    let length = trie.len();
    assert_eq!(trie.insert("haskell".to_val(), vec![36]), Some(vec![18]));
    assert_eq!(trie.len(), length);
}

#[test]
fn insert_similar() {
    let mut trie = Trie::new();
    trie.insert("a".to_val(), vec![1, 4]);
    trie.insert("p".to_val(), vec![1, 3]);
    assert_eq!(trie.get("a".to_val()), Some(&vec![1, 4]));
    assert_eq!(trie.get("p".to_val()), Some(&vec![1, 3]));
    trie.insert("a".to_val(), vec![1, 5]);
    assert_eq!(trie.get("a".to_val()), Some(&vec![1, 5]));
    assert_eq!(trie.len(), 2);
    trie.remove("a".to_val());
    assert_eq!(trie.len(), 1);
    assert_eq!(trie.get("a".to_val()), None);
}

#[test]
fn remove() {
    let mut trie = dummy_trie();

    // Remove.
    for &(key, val) in &TEST_DATA {
        let res = trie.remove(key.to_val());
        assert_eq!(
            res,
            Some(val.to_val()),
            "{}",
            format!("key: {:?} returned None \n {:#?}", key.to_val(), trie)
        );
        assert!(trie.check_integrity());
    }

    assert!(trie.is_empty());

    // Check non-existance.
    for &(key, _) in &TEST_DATA {
        assert!(
            trie.get(key.to_val()).is_none(),
            format!("key: `{:?}` is not None", &key)
        );
    }
}

#[test]
fn remove_simple() {
    let mut trie = Trie::new();

    trie.insert("HELL".to_val(), vec![66]);
    trie.insert("HELLO".to_val(), vec![77]);
    let val = trie.remove("HELLO".to_val());
    assert_eq!(val, Some(vec![77]));
}

#[test]
fn remove_plus_insertion() {
    let mut trie = Trie::new();

    trie.insert("HELL".to_val(), vec![66]);
    trie.insert("HELLO".to_val(), vec![77]);
    let val = trie.remove("HELLO".to_val());
    trie.insert("HELLO".to_val(), vec![88]);
    assert_eq!(val, Some(vec![77]));
    assert_eq!(trie.get("HELLO".to_val()), Some(&vec![88]));
}

#[test]
fn remove_non_existent() {
    let mut trie = Trie::new();

    trie.insert("acab".to_val(), vec![1]);

    assert_eq!(trie.remove("abc".to_val()), None);
    assert_eq!(trie.remove("acaba".to_val()), None);
    assert_eq!(trie.remove("a".to_val()), None);
    assert_eq!(trie.remove("".to_val()), None);
    assert_eq!(trie.len(), 1);

    trie.insert("acaz".to_val(), vec![1]);

    assert_eq!(trie.remove("acb".to_val()), None);
    assert_eq!(trie.remove("acaca".to_val()), None);
    assert_eq!(trie.remove("aca".to_val()), None);
    assert_eq!(trie.len(), 2);
}

#[test]
fn get_prefix_bug() {
    let mut trie = Trie::new();
    trie.insert("abdc".to_val(), vec![5]);
    trie.insert("abde".to_val(), vec![6]);
    assert!(trie.get("abc".to_val()).is_none());
}

#[test]
fn root_replace_bug() {
    let mut trie = Trie::new();
    trie.insert("a".to_val(), "".to_val());
    trie.insert("p".to_val(), "".to_val());
    trie.remove("a".to_val());
    assert_eq!(trie.len(), 1);
    trie.remove("p".to_val());
    assert_eq!(trie.len(), 0);
}

#[test]
fn test_get_borrow() {
    let mut trie = Trie::new();
    trie.insert("/boot".to_val(), "dir".to_val());
    assert_eq!(*trie.get("/boot".to_val()).unwrap(), "dir".to_val());
}

#[test]
fn test_remove_borrow() {
    let mut trie = Trie::new();
    trie.insert("/boot".to_val(), "dir".to_val());
    assert_eq!(trie.remove("/boot".to_val()).unwrap(), "dir".to_val());
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
    map.insert("gamma  rho", "sigma");
    assert_eq!(map.get("gamma  rho"), Some("sigma".to_string()));
    map.insert("gamma", "sigma  sigma");
    assert_eq!(map.get("gamma"), Some("sigma  sigma".to_string()));

    // test dump and load
    let dir = tempdir().unwrap();
    map.dump(dir.path()).unwrap();

    let loaded_map = StringTrieMap::load(dir.path()).unwrap();
    assert_eq!(map, loaded_map);

    dir.close().unwrap();
}
