//! # algs4-fundamentals
//!
//! Fundamental data structures and utilities from *Algorithms, 4th Edition*
//! by Robert Sedgewick and Kevin Wayne.
//!
//! This module contains:
//! - Basic collections (Bag, Queue, Stack) - **Phase 2** ✓
//! - Union-Find data structures - **Phase 2** ✓
//! - I/O utilities - **Phase 1** ✓ (StdIn, StdOut, StdRandom)
//! - Statistical functions - **Phase 1** ✓ (Stopwatch, Counter, Accumulator)
//! - Priority queues - Coming in Phase 4
//!
//! ## Example
//!
//! ```
//! use algs4_fundamentals::util::{Stopwatch, Counter, Accumulator};
//! use algs4_fundamentals::io::stdrandom;
//!
//! // Use a stopwatch
//! let timer = Stopwatch::new();
//! // ... do some work ...
//! println!("Elapsed: {:.2}s", timer.elapsed());
//!
//! // Use a counter
//! let mut counter = Counter::new("operations");
//! counter.increment();
//! println!("{}", counter); // Prints "1 operations"
//!
//! // Generate random numbers
//! stdrandom::set_seed(12345);
//! let x = stdrandom::uniform_f64();
//! println!("Random: {}", x);
//! ```

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

// Phase 1: I/O and utilities
pub mod io;
pub mod util;

// Phase 2: Basic collections and union-find
pub mod collections;
pub mod union_find;

// Phase 4: Priority queues (to be implemented)
// pub mod priority_queue;
