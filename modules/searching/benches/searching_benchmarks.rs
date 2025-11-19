use algs4_searching::bst::BST;
use algs4_searching::linear_probing_hash_st::LinearProbingHashST;
use algs4_searching::red_black_bst::RedBlackBST;
use algs4_searching::separate_chaining_hash_st::SeparateChainingHashST;
use algs4_searching::trie_st::TrieST;
// use algs4_searching::btree::BTree; // BTree might be complex to init, skipping for now if not simple
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::distributions::Alphanumeric;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use rand::Rng;

fn searching_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("searching");

    for size in [1000, 5000].iter() {
        let mut rng = thread_rng();
        let mut keys: Vec<i32> = (0..*size as i32).collect();
        keys.shuffle(&mut rng);

        group.bench_with_input(BenchmarkId::new("bst_put_get", size), size, |b, &size| {
            b.iter(|| {
                let mut st = BST::new();
                for &key in &keys {
                    st.put(key, key);
                }
                for &key in &keys {
                    st.get(&key);
                }
            });
        });

        group.bench_with_input(
            BenchmarkId::new("red_black_bst_put_get", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut st = RedBlackBST::new();
                    for &key in &keys {
                        st.put(key, key);
                    }
                    for &key in &keys {
                        st.get(&key);
                    }
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("linear_probing_hash_st_put_get", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut st = LinearProbingHashST::new();
                    for &key in &keys {
                        st.put(key, key);
                    }
                    for &key in &keys {
                        st.get(&key);
                    }
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("separate_chaining_hash_st_put_get", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut st = SeparateChainingHashST::new();
                    for &key in &keys {
                        st.put(key, key);
                    }
                    for &key in &keys {
                        st.get(&key);
                    }
                });
            },
        );
    }

    // Trie benchmark
    for size in [1000, 5000].iter() {
        let keys: Vec<String> = (0..*size)
            .map(|_| {
                thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(10)
                    .map(char::from)
                    .collect()
            })
            .collect();

        group.bench_with_input(
            BenchmarkId::new("trie_st_put_get", size),
            size,
            |b, &_size| {
                b.iter(|| {
                    let mut st = TrieST::new();
                    for (i, key) in keys.iter().enumerate() {
                        st.put(key, i);
                    }
                    for key in &keys {
                        st.get(key);
                    }
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, searching_benchmark);
criterion_main!(benches);
