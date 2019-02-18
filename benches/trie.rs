extern crate bencher;

use bencher::{benchmark_group, benchmark_main, Bencher};
use rand::{thread_rng, Rng};
use snips_nlu_utils::trie::Trie;
use std::collections::{HashMap,BTreeMap};

#[inline(always)]
fn random_vec() -> Vec<i64> {
    let mut v = vec![];
    let mut rng = thread_rng();
    for _ in 0..rng.gen_range(10, 1000){
        v.push(rng.gen::<i64>());
    }
    v
}
fn trie(b: &mut Bencher) {
    let mut t = Trie::new();
    b.iter(|| {
        let rv = random_vec();
        t.insert(rv.clone(), rv.clone());
        t.get(rv);
    });
}

fn btree(b: &mut Bencher) {
    let mut t = BTreeMap::new();
    b.iter(|| {
        let rv = random_vec();
        t.insert(rv.clone(), rv.clone());
        t.get(&rv);
    });
}

fn hmap(b: &mut Bencher) {
    let mut t = HashMap::new();
    b.iter(|| {
        let rv = random_vec();
        t.insert(rv.clone(), rv.clone());
        t.get(&rv);
    });
}

benchmark_group!(benches, trie, btree, hmap);
benchmark_main!(benches);
