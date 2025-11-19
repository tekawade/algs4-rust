use algs4_advanced::fenwick_tree::FenwickTree;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::SliceRandom;
use rand::thread_rng;

fn advanced_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("advanced");

    for size in [1000, 5000].iter() {
        let mut rng = thread_rng();
        let mut data: Vec<i32> = (0..*size as i32).collect();
        data.shuffle(&mut rng);

        group.bench_with_input(
            BenchmarkId::new("fenwick_tree_sum", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let data = vec![1; size];
                    let ft = FenwickTree::new(black_box(data));
                    ft.rsq(size - 1)
                });
            },
        );
    }
    group.finish();
}

criterion_group!(benches, advanced_benchmark);
criterion_main!(benches);
