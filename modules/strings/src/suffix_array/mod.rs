//! Suffix array data structures.
//!
//! This module provides two implementations of suffix arrays:
//! - **SuffixArray**: Basic implementation with O(N^2 log N) construction
//! - **SuffixArrayX**: Optimized implementation with O(N log N) construction

#[allow(clippy::module_inception)]
pub mod suffix_array;
pub mod suffix_array_x;

pub use suffix_array::SuffixArray;
pub use suffix_array_x::SuffixArrayX;
