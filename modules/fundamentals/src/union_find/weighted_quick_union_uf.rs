//! Weighted quick-union with path compression implementation.
//!
//! This implementation uses weighted quick-union by size with path compression.
//! The find operation uses path compression to flatten the tree, and the union
//! operation always links the smaller tree to the larger tree.
//!
//! # Performance
//!
//! - Constructor: Θ(n) time
//! - Union, find, connected: Θ(log n) time in worst case
//! - Count: Θ(1) time
//!
//! # Examples
//!
//! ```
//! use algs4_fundamentals::union_find::WeightedQuickUnionUF;
//!
//! let mut uf = WeightedQuickUnionUF::new(10);
//! uf.union(0, 1);
//! uf.union(2, 3);
//! uf.union(0, 2);
//!
//! assert!(uf.connected(0, 3));
//! assert!(!uf.connected(0, 4));
//! assert_eq!(uf.count(), 7);
//! ```
//!
//! **Reference:** <https://algs4.cs.princeton.edu/15uf>

use std::fmt;

/// Weighted quick-union with path compression.
///
/// This data structure supports union-find operations for determining
/// connectivity among a set of n elements. The weighted quick-union
/// algorithm maintains tree balance by always linking the smaller tree
/// to the larger tree, ensuring logarithmic height.
#[derive(Debug, Clone)]
pub struct WeightedQuickUnionUF {
    parent: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl WeightedQuickUnionUF {
    /// Initializes an empty union-find data structure with n elements (0 through n-1).
    ///
    /// Initially, each element is in its own component.
    ///
    /// # Arguments
    ///
    /// * `n` - the number of elements
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::union_find::WeightedQuickUnionUF;
    ///
    /// let uf = WeightedQuickUnionUF::new(10);
    /// assert_eq!(uf.count(), 10);
    /// ```
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            count: n,
        }
    }

    /// Returns the number of components.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::union_find::WeightedQuickUnionUF;
    ///
    /// let mut uf = WeightedQuickUnionUF::new(5);
    /// assert_eq!(uf.count(), 5);
    ///
    /// uf.union(0, 1);
    /// assert_eq!(uf.count(), 4);
    /// ```
    pub fn count(&self) -> usize {
        self.count
    }

    /// Returns the component identifier for the component containing element p.
    ///
    /// Uses path compression to flatten the tree.
    ///
    /// # Arguments
    ///
    /// * `p` - the element
    ///
    /// # Panics
    ///
    /// Panics if p is not between 0 and n-1.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::union_find::WeightedQuickUnionUF;
    ///
    /// let mut uf = WeightedQuickUnionUF::new(5);
    /// uf.union(0, 1);
    /// assert_eq!(uf.find(0), uf.find(1));
    /// ```
    pub fn find(&mut self, mut p: usize) -> usize {
        self.validate(p);

        // Path compression: make every node point to its grandparent
        while p != self.parent[p] {
            self.parent[p] = self.parent[self.parent[p]];
            p = self.parent[p];
        }
        p
    }

    /// Returns true if the two elements are in the same component.
    ///
    /// # Arguments
    ///
    /// * `p` - one element
    /// * `q` - the other element
    ///
    /// # Panics
    ///
    /// Panics if either p or q is not between 0 and n-1.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::union_find::WeightedQuickUnionUF;
    ///
    /// let mut uf = WeightedQuickUnionUF::new(5);
    /// assert!(!uf.connected(0, 1));
    ///
    /// uf.union(0, 1);
    /// assert!(uf.connected(0, 1));
    /// ```
    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    /// Merges the component containing element p with the component containing element q.
    ///
    /// Uses union by size: always links the smaller tree to the larger tree.
    ///
    /// # Arguments
    ///
    /// * `p` - one element
    /// * `q` - the other element
    ///
    /// # Panics
    ///
    /// Panics if either p or q is not between 0 and n-1.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::union_find::WeightedQuickUnionUF;
    ///
    /// let mut uf = WeightedQuickUnionUF::new(5);
    /// uf.union(0, 1);
    /// uf.union(2, 3);
    /// assert_eq!(uf.count(), 3);
    /// ```
    pub fn union(&mut self, p: usize, q: usize) {
        let root_p = self.find(p);
        let root_q = self.find(q);

        if root_p == root_q {
            return;
        }

        // Make smaller tree point to larger tree
        if self.size[root_p] < self.size[root_q] {
            self.parent[root_p] = root_q;
            self.size[root_q] += self.size[root_p];
        } else {
            self.parent[root_q] = root_p;
            self.size[root_p] += self.size[root_q];
        }

        self.count -= 1;
    }

    /// Validates that p is a valid index.
    fn validate(&self, p: usize) {
        if p >= self.parent.len() {
            panic!("index {} is not between 0 and {}", p, self.parent.len() - 1);
        }
    }
}

impl fmt::Display for WeightedQuickUnionUF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} components", self.count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let uf = WeightedQuickUnionUF::new(10);
        assert_eq!(uf.count(), 10);
    }

    #[test]
    fn test_union_connected() {
        let mut uf = WeightedQuickUnionUF::new(10);

        // Initially, all elements are in separate components
        for i in 0..10 {
            for j in 0..10 {
                if i == j {
                    assert!(uf.connected(i, j));
                } else {
                    assert!(!uf.connected(i, j));
                }
            }
        }

        // Union some elements
        uf.union(0, 1);
        assert!(uf.connected(0, 1));
        assert_eq!(uf.count(), 9);

        uf.union(2, 3);
        assert!(uf.connected(2, 3));
        assert!(!uf.connected(0, 2));
        assert_eq!(uf.count(), 8);

        uf.union(0, 2);
        assert!(uf.connected(0, 3));
        assert!(uf.connected(1, 3));
        assert_eq!(uf.count(), 7);
    }

    #[test]
    fn test_find() {
        let mut uf = WeightedQuickUnionUF::new(10);

        uf.union(0, 1);
        uf.union(2, 3);
        uf.union(0, 2);

        // All of 0, 1, 2, 3 should have the same root
        let root0 = uf.find(0);
        assert_eq!(uf.find(1), root0);
        assert_eq!(uf.find(2), root0);
        assert_eq!(uf.find(3), root0);

        // 4 should have a different root
        assert_ne!(uf.find(4), root0);
    }

    #[test]
    fn test_repeated_union() {
        let mut uf = WeightedQuickUnionUF::new(5);

        uf.union(0, 1);
        assert_eq!(uf.count(), 4);

        // Unioning already connected elements shouldn't change count
        uf.union(0, 1);
        assert_eq!(uf.count(), 4);

        uf.union(1, 0);
        assert_eq!(uf.count(), 4);
    }

    #[test]
    fn test_large_uf() {
        let mut uf = WeightedQuickUnionUF::new(1000);

        // Create a chain: 0-1-2-3-...-999
        for i in 0..999 {
            uf.union(i, i + 1);
        }

        assert_eq!(uf.count(), 1);
        assert!(uf.connected(0, 999));
        assert!(uf.connected(500, 750));
    }

    #[test]
    fn test_path_compression() {
        let mut uf = WeightedQuickUnionUF::new(10);

        // Create a deep tree
        uf.union(0, 1);
        uf.union(1, 2);
        uf.union(2, 3);
        uf.union(3, 4);

        // Find should compress the path
        let root = uf.find(0);
        assert_eq!(uf.find(4), root);

        // After path compression, 0 should point closer to the root
        // (This is a bit tricky to test directly, but we can at least
        // verify that all elements still have the same root)
        for i in 0..=4 {
            assert_eq!(uf.find(i), root);
        }
    }

    #[test]
    #[should_panic(expected = "index 10 is not between 0 and 9")]
    fn test_validate_out_of_bounds() {
        let mut uf = WeightedQuickUnionUF::new(10);
        uf.find(10);
    }

    #[test]
    fn test_display() {
        let uf = WeightedQuickUnionUF::new(5);
        assert_eq!(uf.to_string(), "5 components");
    }

    #[test]
    fn test_textbook_example() {
        // Example from textbook: tinyUF.txt
        let mut uf = WeightedQuickUnionUF::new(10);

        let pairs = vec![
            (4, 3),
            (3, 8),
            (6, 5),
            (9, 4),
            (2, 1),
            (8, 9),
            (5, 0),
            (7, 2),
            (6, 1),
            (1, 0),
            (6, 7),
        ];

        for (p, q) in pairs {
            if !uf.connected(p, q) {
                uf.union(p, q);
            }
        }

        assert_eq!(uf.count(), 2);
    }
}
