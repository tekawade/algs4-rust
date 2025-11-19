use algs4_fundamentals::collections::linked_bag::LinkedBag;
use algs4_fundamentals::collections::linked_queue::LinkedQueue;
use algs4_fundamentals::collections::linked_stack::LinkedStack;
use algs4_fundamentals::collections::resizing_array_bag::ResizingArrayBag;
use algs4_fundamentals::collections::resizing_array_queue::ResizingArrayQueue;
use algs4_fundamentals::collections::resizing_array_stack::ResizingArrayStack;
use algs4_fundamentals::union_find::quick_find_uf::QuickFindUF;
use algs4_fundamentals::union_find::quick_union_uf::QuickUnionUF;
use algs4_fundamentals::union_find::weighted_quick_union_uf::WeightedQuickUnionUF;
use algs4_fundamentals::union_find::weighted_quick_union_uf::WeightedQuickUnionUF as UF; // UF trait might not be public or exist as named
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn bag_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("bag");

    for size in [1000, 10000].iter() {
        group.bench_with_input(
            BenchmarkId::new("linked_bag_add", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut bag = LinkedBag::new();
                    for i in 0..size {
                        bag.add(black_box(i));
                    }
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("resizing_array_bag_add", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut bag = ResizingArrayBag::new();
                    for i in 0..size {
                        bag.add(black_box(i));
                    }
                });
            },
        );
    }
    group.finish();
}

fn queue_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("queue");

    for size in [1000, 10000].iter() {
        group.bench_with_input(
            BenchmarkId::new("linked_queue_enqueue_dequeue", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut queue = LinkedQueue::new();
                    for i in 0..size {
                        queue.enqueue(black_box(i));
                    }
                    for _ in 0..size {
                        queue.dequeue();
                    }
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("resizing_array_queue_enqueue_dequeue", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut queue = ResizingArrayQueue::new();
                    for i in 0..size {
                        queue.enqueue(black_box(i));
                    }
                    for _ in 0..size {
                        queue.dequeue();
                    }
                });
            },
        );
    }
    group.finish();
}

fn stack_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("stack");

    for size in [1000, 10000].iter() {
        group.bench_with_input(
            BenchmarkId::new("linked_stack_push_pop", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut stack = LinkedStack::new();
                    for i in 0..size {
                        stack.push(black_box(i));
                    }
                    for _ in 0..size {
                        stack.pop();
                    }
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("resizing_array_stack_push_pop", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut stack = ResizingArrayStack::new();
                    for i in 0..size {
                        stack.push(black_box(i));
                    }
                    for _ in 0..size {
                        stack.pop();
                    }
                });
            },
        );
    }
    group.finish();
}

fn union_find_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("union_find");

    for size in [1000, 5000].iter() {
        let ops = size * 2;

        group.bench_with_input(BenchmarkId::new("quick_find", size), size, |b, &size| {
            b.iter(|| {
                let mut uf = QuickFindUF::new(black_box(size));
                for i in 0..size - 1 {
                    uf.union(i, i + 1);
                }
                uf.connected(0, size - 1)
            });
        });

        group.bench_with_input(BenchmarkId::new("quick_union", size), size, |b, &size| {
            b.iter(|| {
                let mut uf = QuickUnionUF::new(black_box(size));
                for i in 0..size - 1 {
                    uf.union(i, i + 1);
                }
                uf.connected(0, size - 1)
            });
        });

        group.bench_with_input(
            BenchmarkId::new("weighted_quick_union", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut uf = WeightedQuickUnionUF::new(black_box(size));
                    for i in 0..size - 1 {
                        uf.union(i, i + 1);
                    }
                    uf.connected(0, size - 1)
                });
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bag_benchmark,
    queue_benchmark,
    stack_benchmark,
    union_find_benchmark
);
criterion_main!(benches);
