use algs4_geometry::graham_scan::GrahamScan;
use algs4_geometry::point2d::Point2D;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::{thread_rng, Rng};

fn geometry_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("geometry");

    for size in [1000, 5000].iter() {
        let mut rng = thread_rng();
        let points: Vec<Point2D> = (0..*size)
            .map(|_| Point2D::new(rng.gen_range(0.0..100.0), rng.gen_range(0.0..100.0)))
            .collect();

        group.bench_with_input(BenchmarkId::new("graham_scan", size), &points, |b, pts| {
            b.iter(|| {
                let mut p = pts.clone();
                GrahamScan::new(black_box(&mut p));
            });
        });
    }
    group.finish();
}

criterion_group!(benches, geometry_benchmark);
criterion_main!(benches);
