use algs4_sorting::heap;
use algs4_sorting::insertion;
use algs4_sorting::lsd;
use algs4_sorting::merge;
use algs4_sorting::msd;
use algs4_sorting::quick;
use algs4_sorting::quick_3string;
use algs4_sorting::selection;
use algs4_sorting::shell;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::distributions::Alphanumeric;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

fn sorting_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sorting");

    for size in [100, 1000, 5000].iter() {
        let mut rng = thread_rng();
        let mut data: Vec<i32> = (0..*size as i32).collect();
        data.shuffle(&mut rng);

        group.bench_with_input(BenchmarkId::new("selection", size), &data, |b, d| {
            b.iter(|| {
                let mut arr = d.clone();
                selection::sort(black_box(&mut arr));
            });
        });

        group.bench_with_input(BenchmarkId::new("insertion", size), &data, |b, d| {
            b.iter(|| {
                let mut arr = d.clone();
                insertion::sort(black_box(&mut arr));
            });
        });

        group.bench_with_input(BenchmarkId::new("shell", size), &data, |b, d| {
            b.iter(|| {
                let mut arr = d.clone();
                shell::sort(black_box(&mut arr));
            });
        });

        group.bench_with_input(BenchmarkId::new("merge", size), &data, |b, d| {
            b.iter(|| {
                let mut arr = d.clone();
                merge::sort(black_box(&mut arr));
            });
        });

        group.bench_with_input(BenchmarkId::new("quick", size), &data, |b, d| {
            b.iter(|| {
                let mut arr = d.clone();
                quick::sort(black_box(&mut arr));
            });
        });

        group.bench_with_input(BenchmarkId::new("heap", size), &data, |b, d| {
            b.iter(|| {
                let mut arr = d.clone();
                heap::sort(black_box(&mut arr));
            });
        });
    }

    // String sorts
    for size in [1000, 5000].iter() {
        let data: Vec<String> = (0..*size)
            .map(|_| {
                thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(10)
                    .map(char::from)
                    .collect()
            })
            .collect();

        group.bench_with_input(BenchmarkId::new("lsd", size), &data, |b, d| {
            b.iter(|| {
                let mut arr = d.clone();
                lsd::sort(&mut arr, 10);
            });
        });

        group.bench_with_input(BenchmarkId::new("msd", size), &data, |b, d| {
            b.iter(|| {
                let mut arr = d.clone();
                msd::sort(&mut arr);
            });
        });

        group.bench_with_input(BenchmarkId::new("quick3string", size), &data, |b, d| {
            b.iter(|| {
                let mut arr = d.clone();
                quick_3string::sort(&mut arr);
            });
        });
    }

    group.finish();
}

criterion_group!(benches, sorting_benchmark);
criterion_main!(benches);
