//! Union-Find (Disjoint Set Union) data structures.
//!
//! Union-Find is a data structure that tracks a set of elements partitioned into
//! disjoint (non-overlapping) subsets. It provides near-constant-time operations
//! to merge sets and determine whether elements are in the same set.
//!
//! This module provides three implementations:
//!
//! - **QuickFindUF**: Fast find O(1), slow union O(n)
//! - **QuickUnionUF**: Both operations O(n) worst case
//! - **WeightedQuickUnionUF**: Both operations O(log n) with path compression (recommended)
//!
//! # Use Cases
//!
//! - **Network connectivity**: Determine if two nodes are connected
//! - **Percolation**: Model physical systems
//! - **Kruskal's MST**: Minimum spanning tree algorithm
//! - **Image processing**: Connected component labeling
//!
//! # Examples
//!
//! ```
//! use algs4_fundamentals::union_find::WeightedQuickUnionUF;
//!
//! let mut uf = WeightedQuickUnionUF::new(10);
//!
//! // Connect some elements
//! uf.union(0, 1);
//! uf.union(2, 3);
//! uf.union(0, 2);
//!
//! // Check connectivity
//! assert!(uf.connected(0, 3)); // 0-1-2-3 are all connected
//! assert!(!uf.connected(0, 4)); // 4 is separate
//!
//! // Count components
//! assert_eq!(uf.count(), 7); // 7 disjoint sets
//! ```
//!
//! **Reference:** <https://algs4.cs.princeton.edu/15uf>

pub mod quick_find_uf;
pub mod quick_union_uf;
pub mod weighted_quick_union_uf;

pub use quick_find_uf::QuickFindUF;
pub use quick_union_uf::QuickUnionUF;
pub use weighted_quick_union_uf::WeightedQuickUnionUF;
