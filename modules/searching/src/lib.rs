//! # algs4-searching
//!
//! Searching algorithms and symbol tables from *Algorithms, 4th Edition*
//! by Robert Sedgewick and Kevin Wayne.
//!
//! This module contains:
//! - Binary search
//! - Binary search trees (BST, Red-Black BST, AVL)
//! - Hash tables (separate chaining, linear probing)
//! - Tries (R-way, TST, Patricia)
//!
//! ## Example
//!
//! ```
//! // Examples will be added as implementations are completed
//! ```

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

// Binary search algorithm
pub mod binary_search;
pub use binary_search::{binary_search, rank};

// Symbol tables
pub mod sequential_search_st;
pub use sequential_search_st::SequentialSearchST;

pub mod binary_search_st;
pub use binary_search_st::BinarySearchST;

pub mod bst;
pub use bst::BST;

pub mod separate_chaining_hash_st;
pub use separate_chaining_hash_st::SeparateChainingHashST;

pub mod red_black_bst;
pub use red_black_bst::RedBlackBST;

pub mod linear_probing_hash_st;
pub use linear_probing_hash_st::LinearProbingHashST;

// Symbol tables (to be implemented)
// pub mod trie_st;
// pub mod trie_set;
