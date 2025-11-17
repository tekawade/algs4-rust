//! Pattern matching algorithms for substring search.
//!
//! This module provides three classic string matching algorithms:
//! - **KMP (Knuth-Morris-Pratt)**: Uses a DFA for linear-time matching
//! - **Boyer-Moore**: Uses bad character heuristic for sublinear performance
//! - **Rabin-Karp**: Uses rolling hash for efficient pattern matching

pub mod boyer_moore;
pub mod kmp;
pub mod rabin_karp;

pub use boyer_moore::BoyerMoore;
pub use kmp::KMP;
pub use rabin_karp::RabinKarp;
