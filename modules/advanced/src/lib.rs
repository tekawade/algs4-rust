//! # algs4-advanced
//!
//! Advanced algorithms from *Algorithms, 4th Edition*
//! by Robert Sedgewick and Kevin Wayne.
//!
//! This module contains:
//! - Linear algebra (vectors, matrices, FFT)
//! - Linear programming (simplex)
//! - Advanced data structures (segment tree, Fenwick tree)
//!
//! ## Example
//!
//! ```
//! use algs4_advanced::{Complex, Vector, SegmentTree, FenwickTree};
//!
//! // Complex numbers
//! let z = Complex::new(3.0, 4.0);
//! println!("|z| = {}", z.abs()); // Magnitude: 5.0
//!
//! // Segment tree for range queries
//! let mut tree = SegmentTree::new(vec![1, 3, 5, 7, 9, 11]);
//! println!("Sum [1,3] = {}", tree.rsq(1, 3)); // 3 + 5 + 7 = 15
//! ```

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

// Linear algebra
pub mod complex;
pub mod fft;
pub mod polynomial;
pub mod sparse_vector;
pub mod vector;

// Matrix operations and linear systems
pub mod gauss_jordan_elimination;
pub mod gaussian_elimination;

// Optimization
pub mod linear_programming;

// Advanced data structures
pub mod fenwick_tree;
pub mod segment_tree;

// Algorithms
pub mod three_sum;
pub mod three_sum_fast;

// Applications and utilities
pub mod allow_filter;
pub mod block_filter;
pub mod top_m;

// Re-export main types for convenience
pub use allow_filter::AllowFilter;
pub use block_filter::BlockFilter;
pub use complex::Complex;
pub use fenwick_tree::FenwickTree;
pub use gauss_jordan_elimination::GaussJordanElimination;
pub use gaussian_elimination::GaussianElimination;
pub use linear_programming::LinearProgramming;
pub use polynomial::Polynomial;
pub use segment_tree::SegmentTree;
pub use sparse_vector::SparseVector;
pub use top_m::TopM;
pub use vector::Vector;
