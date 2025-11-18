//! # algs4-searching
//!
//! Searching algorithms and symbol tables from *Algorithms, 4th Edition*
//! by Robert Sedgewick and Kevin Wayne.
//!
//! This module contains:
//! - Binary search
//! - Binary search trees (BST, Red-Black BST, AVL, B-trees)
//! - Hash tables (separate chaining, linear probing)
//! - Tries (R-way, Patricia)
//! - Applications (frequency counter, dedup, file index, etc.)
//!
//! ## Example
//!
//! ```
//! use algs4_searching::{RedBlackBST, AVLTreeST};
//!
//! let mut rb = RedBlackBST::new();
//! rb.put("hello", 42);
//! assert_eq!(rb.get(&"hello"), Some(&42));
//!
//! let mut avl = AVLTreeST::new();
//! avl.put(1, "one");
//! assert_eq!(avl.get(&1), Some(&"one"));
//! ```

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

// Binary search algorithm
pub mod binary_search;
pub use binary_search::{binary_search, rank};

// Symbol tables - Basic
pub mod sequential_search_st;
pub use sequential_search_st::SequentialSearchST;

pub mod binary_search_st;
pub use binary_search_st::BinarySearchST;

// Symbol tables - Trees
pub mod bst;
pub use bst::BST;

pub mod red_black_bst;
pub use red_black_bst::RedBlackBST;

/// AVL tree symbol table implementation
pub mod avl_tree_st;
pub use avl_tree_st::AVLTreeST;

/// B-tree implementation
pub mod btree;
pub use btree::BTree;

// Symbol tables - Hash tables
pub mod separate_chaining_hash_st;
pub use separate_chaining_hash_st::SeparateChainingHashST;

pub mod linear_probing_hash_st;
pub use linear_probing_hash_st::LinearProbingHashST;

// Symbol tables - Tries
pub mod trie_st;
pub use trie_st::TrieST;

pub mod trie_set;
pub use trie_set::TrieSET;

/// Patricia trie symbol table implementation
pub mod patricia_st;
pub use patricia_st::PatriciaST;

/// Patricia trie set implementation
pub mod patricia_set;
pub use patricia_set::PatriciaSET;

// Applications
/// Count occurrences application
pub mod count;
/// Remove duplicates application
pub mod dedup;
/// File indexing application
pub mod file_index;
/// Frequency counter application
pub mod frequency_counter;
/// Keyword in context (KWIK) application
pub mod kwik;
/// CSV lookup application
pub mod lookup_csv;
/// Index lookup application
pub mod lookup_index;
