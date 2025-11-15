//! Phase 1 demonstration - I/O utilities and basic types
//!
//! This example demonstrates the utilities implemented in Phase 1:
//! - Stopwatch: measuring elapsed time
//! - Counter: counting with named counters
//! - Accumulator: running statistics
//! - StdRandom: random number generation
//!
//! Run with: cargo run --example phase1_demo

use algs4_fundamentals::io::{stdout, stdrandom};
use algs4_fundamentals::util::{Accumulator, Counter, Stopwatch};

fn main() {
    println!("=== Phase 1 Demo: algs4-rust Utilities ===\n");

    // Demonstrate Stopwatch
    println!("1. Stopwatch Demo:");
    let timer = Stopwatch::new();
    std::thread::sleep(std::time::Duration::from_millis(100));
    println!("   Elapsed time: {:.3} seconds\n", timer.elapsed());

    // Demonstrate Counter
    println!("2. Counter Demo:");
    let mut heads = Counter::new("heads");
    let mut tails = Counter::new("tails");

    stdrandom::set_seed(12345); // For reproducibility
    for _ in 0..10 {
        if stdrandom::bernoulli(0.5) {
            heads.increment();
        } else {
            tails.increment();
        }
    }
    println!("   {}", heads);
    println!("   {}\n", tails);

    // Demonstrate Accumulator
    println!("3. Accumulator Demo (running statistics):");
    let mut stats = Accumulator::new();
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    for &x in &data {
        stats.add_data_value(x);
    }
    println!("   Data: {:?}", data);
    println!("   {}\n", stats);

    // Demonstrate StdRandom
    println!("4. StdRandom Demo:");
    stdrandom::set_seed(42);
    println!("   Uniform [0,1): {:.4}", stdrandom::uniform_f64());
    println!("   Uniform int [0,100): {}", stdrandom::uniform_i32(100));
    println!("   Gaussian N(0,1): {:.4}", stdrandom::gaussian());

    let mut arr = vec![1, 2, 3, 4, 5];
    stdrandom::shuffle(&mut arr);
    println!("   Shuffled [1,2,3,4,5]: {:?}\n", arr);

    // Demonstrate formatted output
    println!("5. Formatted Output Demo:");
    stdout::printf("   Using printf: {} + {} = {}\n", &[&2, &3, &5]);
    stdout::println("   Using println: Hello, algs4-rust!");

    println!("\n=== Phase 1 Demo Complete ===");
}
