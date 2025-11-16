//! Priority queue data structures.
//!
//! This module contains priority queue implementations from *Algorithms, 4th Edition*.
//!
//! Priority queues support efficient insertion and deletion of the maximum (or minimum) element.
//! They are fundamental data structures used in graph algorithms, event-driven simulation,
//! and various scheduling applications.
//!
//! # Implementations
//!
//! - **Basic Priority Queues**: Binary heap-based implementations
//!   - [`MaxPQ`] - Maximum priority queue
//!   - [`MinPQ`] - Minimum priority queue
//!
//! - **Indexed Priority Queues**: Allow changing the priority of elements
//!   - [`IndexMaxPQ`] - Indexed maximum priority queue
//!   - [`IndexMinPQ`] - Indexed minimum priority queue
//!
//! - **Advanced Priority Queues**: More sophisticated heap structures
//!   - [`BinomialMinPQ`] - Binomial heap
//!   - [`FibonacciMinPQ`] - Fibonacci heap
//!   - [`MultiwayMinPQ`] - Multiway heap
//!
//! # Examples
//!
//! ```
//! use algs4_fundamentals::priority_queue::MaxPQ;
//!
//! let mut pq = MaxPQ::new();
//! pq.insert(5);
//! pq.insert(3);
//! pq.insert(7);
//!
//! assert_eq!(pq.del_max(), Some(7));
//! assert_eq!(pq.del_max(), Some(5));
//! ```
//!
//! **Reference:** <https://algs4.cs.princeton.edu/24pq>

pub mod index_max_pq;
pub mod index_min_pq;
pub mod max_pq;
pub mod min_pq;

pub use index_max_pq::IndexMaxPQ;
pub use index_min_pq::IndexMinPQ;
pub use max_pq::MaxPQ;
pub use min_pq::MinPQ;

// To be implemented:
// pub mod binomial_min_pq;
// pub mod fibonacci_min_pq;
// pub mod index_binomial_min_pq;
// pub mod index_fibonacci_min_pq;
// pub mod multiway_min_pq;
// pub mod index_multiway_min_pq;

// pub use binomial_min_pq::BinomialMinPQ;
// pub use fibonacci_min_pq::FibonacciMinPQ;
// pub use index_binomial_min_pq::IndexBinomialMinPQ;
// pub use index_fibonacci_min_pq::IndexFibonacciMinPQ;
// pub use multiway_min_pq::MultiwayMinPQ;
// pub use index_multiway_min_pq::IndexMultiwayMinPQ;
