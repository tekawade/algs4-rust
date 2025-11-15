//! Quick-union implementation of union-find.
//!
//! This implementation uses a parent-pointer tree representation.
//! Both find and union operations can take linear time in the worst case.
//!
//! # Performance
//!
//! - Constructor: Θ(n) time
//! - Union, find, connected: Θ(n) time in worst case
//! - Count: Θ(1) time
//!
//! # Examples
//!
//! ```
//! use algs4_fundamentals::union_find::QuickUnionUF;
//!
//! let mut uf = QuickUnionUF::new(10);
//! uf.union(0, 1);
//! uf.union(2, 3);
//!
//! assert!(uf.connected(0, 1));
//! assert!(!uf.connected(0, 2));
//! assert_eq!(uf.count(), 8);
//! ```
//!
//! **Reference:** <https://algs4.cs.princeton.edu/15uf>

use std::fmt;

/// Quick-union implementation of union-find.
///
/// This implementation uses a parent-pointer tree where parent[i] is the
/// parent of element i. The root of the tree is the component identifier.
#[derive(Debug, Clone)]
pub struct QuickUnionUF {
    parent: Vec<usize>,
    count: usize,
}

impl QuickUnionUF {
    /// Initializes an empty union-find data structure with n elements (0 through n-1).
    ///
    /// Initially, each element is in its own component.
    ///
    /// # Arguments
    ///
    /// * `n` - the number of elements
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            count: n,
        }
    }

    /// Returns the number of components.
    pub fn count(&self) -> usize {
        self.count
    }

    /// Returns the component identifier for the component containing element p.
    ///
    /// # Arguments
    ///
    /// * `p` - the element
    ///
    /// # Panics
    ///
    /// Panics if p is not between 0 and n-1.
    pub fn find(&self, mut p: usize) -> usize {
        self.validate(p);

        while p != self.parent[p] {
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
    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    /// Merges the component containing element p with the component containing element q.
    ///
    /// # Arguments
    ///
    /// * `p` - one element
    /// * `q` - the other element
    ///
    /// # Panics
    ///
    /// Panics if either p or q is not between 0 and n-1.
    pub fn union(&mut self, p: usize, q: usize) {
        let root_p = self.find(p);
        let root_q = self.find(q);

        if root_p == root_q {
            return;
        }

        self.parent[root_p] = root_q;
        self.count -= 1;
    }

    /// Validates that p is a valid index.
    fn validate(&self, p: usize) {
        if p >= self.parent.len() {
            panic!("index {} is not between 0 and {}", p, self.parent.len() - 1);
        }
    }
}

impl fmt::Display for QuickUnionUF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} components", self.count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let uf = QuickUnionUF::new(10);
        assert_eq!(uf.count(), 10);
    }

    #[test]
    fn test_union_connected() {
        let mut uf = QuickUnionUF::new(10);

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
        let mut uf = QuickUnionUF::new(10);

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
    fn test_textbook_example() {
        let mut uf = QuickUnionUF::new(10);

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
