use algs4_strings::pattern_matching::kmp::KMP;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn pattern_matching_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("pattern_matching");

    for size in [1000, 5000].iter() {
        let text: String = (0..*size)
            .map(|_| {
                thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(1)
                    .map(char::from)
                    .collect::<String>()
            })
            .collect();
        let pattern = "ABC"; // Simple pattern

        group.bench_with_input(BenchmarkId::new("kmp_search", size), &text, |b, txt| {
            b.iter(|| {
                let kmp = KMP::new(black_box(pattern));
                kmp.search(black_box(txt));
            });
        });
    }
    group.finish();
}

criterion_group!(benches, pattern_matching_benchmark);
criterion_main!(benches);
