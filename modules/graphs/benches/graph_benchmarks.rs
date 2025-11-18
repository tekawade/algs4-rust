use algs4_graphs::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn graph_creation_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_creation");

    for size in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("simple", size), size, |b, &size| {
            b.iter(|| {
                let edges = size * 2;
                GraphGenerator::simple(black_box(size), black_box(edges))
            });
        });

        group.bench_with_input(BenchmarkId::new("complete", size), size, |b, &size| {
            b.iter(|| GraphGenerator::complete(black_box(size)));
        });

        group.bench_with_input(BenchmarkId::new("tree", size), size, |b, &size| {
            b.iter(|| GraphGenerator::tree(black_box(size)));
        });
    }

    group.finish();
}

fn digraph_creation_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("digraph_creation");

    for size in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("simple", size), size, |b, &size| {
            b.iter(|| {
                let edges = size * 2;
                DigraphGenerator::simple(black_box(size), black_box(edges))
            });
        });

        group.bench_with_input(BenchmarkId::new("dag", size), size, |b, &size| {
            b.iter(|| {
                let edges = size * 2;
                DigraphGenerator::dag(black_box(size), black_box(edges))
            });
        });

        group.bench_with_input(BenchmarkId::new("strong", size), size, |b, &size| {
            b.iter(|| {
                let edges = size * 2;
                DigraphGenerator::strong(black_box(size), black_box(edges))
            });
        });
    }

    group.finish();
}

fn dfs_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("depth_first_search");

    for size in [100, 500, 1000].iter() {
        let graph = GraphGenerator::simple(*size, size * 2);

        group.bench_with_input(BenchmarkId::new("dfs_paths", size), &graph, |b, g| {
            b.iter(|| {
                let dfs = DepthFirstPaths::new(black_box(g), black_box(0));
                dfs.has_path_to(black_box(g.v() - 1))
            });
        });
    }

    group.finish();
}

fn bfs_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("breadth_first_search");

    for size in [100, 500, 1000].iter() {
        let graph = GraphGenerator::simple(*size, size * 2);

        group.bench_with_input(BenchmarkId::new("bfs_paths", size), &graph, |b, g| {
            b.iter(|| {
                let bfs = BreadthFirstPaths::new(black_box(g), black_box(0));
                bfs.has_path_to(black_box(g.v() - 1))
            });
        });
    }

    group.finish();
}

fn connected_components_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("connected_components");

    for size in [100, 500, 1000].iter() {
        let graph = GraphGenerator::simple(*size, size * 2);

        group.bench_with_input(BenchmarkId::new("cc", size), &graph, |b, g| {
            b.iter(|| {
                let cc = CC::new(black_box(g));
                cc.count()
            });
        });
    }

    group.finish();
}

fn dijkstra_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("shortest_path");

    for size in [100, 500, 1000].iter() {
        let mut ewg = EdgeWeightedGraph::new(*size);
        for _ in 0..(size * 2) {
            let v = rand::random::<usize>() % size;
            let w = rand::random::<usize>() % size;
            if v != w {
                let weight = rand::random::<f64>() * 100.0;
                ewg.add_edge(Edge::new(v, w, weight));
            }
        }

        group.bench_with_input(BenchmarkId::new("dijkstra", size), &ewg, |b, g| {
            b.iter(|| {
                let sp = DijkstraUndirectedSP::new(black_box(g), black_box(0));
                sp.has_path_to(black_box(g.v() - 1))
            });
        });
    }

    group.finish();
}

fn mst_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("minimum_spanning_tree");

    for size in [100, 500, 1000].iter() {
        let mut ewg = EdgeWeightedGraph::new(*size);
        for i in 0..*size {
            for j in (i + 1)..*size {
                if rand::random::<f64>() < 0.1 {
                    let weight = rand::random::<f64>() * 100.0;
                    ewg.add_edge(Edge::new(i, j, weight));
                }
            }
        }

        group.bench_with_input(BenchmarkId::new("kruskal", size), &ewg, |b, g| {
            b.iter(|| {
                let mst = KruskalMST::new(black_box(g));
                mst.weight()
            });
        });

        group.bench_with_input(BenchmarkId::new("prim", size), &ewg, |b, g| {
            b.iter(|| {
                let mst = PrimMST::new(black_box(g));
                mst.weight()
            });
        });

        group.bench_with_input(BenchmarkId::new("lazy_prim", size), &ewg, |b, g| {
            b.iter(|| {
                let mst = LazyPrimMST::new(black_box(g));
                mst.weight()
            });
        });
    }

    group.finish();
}

fn topological_sort_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("topological_sort");

    for size in [100, 500, 1000].iter() {
        let dag = DigraphGenerator::dag(*size, size * 2);

        group.bench_with_input(BenchmarkId::new("topological", size), &dag, |b, g| {
            b.iter(|| {
                let topo = Topological::new(black_box(g));
                topo.has_order()
            });
        });
    }

    group.finish();
}

fn strongly_connected_components_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("strongly_connected_components");

    for size in [100, 500, 1000].iter() {
        let digraph = DigraphGenerator::simple(*size, size * 3);

        group.bench_with_input(
            BenchmarkId::new("kosaraju_sharir", size),
            &digraph,
            |b, g| {
                b.iter(|| {
                    let scc = KosarajuSharirSCC::new(black_box(g));
                    scc.count()
                });
            },
        );

        group.bench_with_input(BenchmarkId::new("tarjan", size), &digraph, |b, g| {
            b.iter(|| {
                let scc = TarjanSCC::new(black_box(g));
                scc.count()
            });
        });

        group.bench_with_input(BenchmarkId::new("gabow", size), &digraph, |b, g| {
            b.iter(|| {
                let scc = GabowSCC::new(black_box(g));
                scc.count()
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    graph_creation_benchmark,
    digraph_creation_benchmark,
    dfs_benchmark,
    bfs_benchmark,
    connected_components_benchmark,
    dijkstra_benchmark,
    mst_benchmark,
    topological_sort_benchmark,
    strongly_connected_components_benchmark
);
criterion_main!(benches);
