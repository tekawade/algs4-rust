//! Quick-find implementation of union-find.
//!
//! This implementation uses a flat array where each element stores its component ID.
//! The find operation is constant time, but union is linear time.
//!
//! # Performance
//!
//! - Constructor: Θ(n) time
//! - Find, connected, count: Θ(1) time
//! - Union: Θ(n) time
//!
//! # Examples
//!
//! ```
//! use algs4_fundamentals::union_find::QuickFindUF;
//!
//! let mut uf = QuickFindUF::new(10);
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

/// Quick-find implementation of union-find.
///
/// This implementation uses a flat array where id[i] represents the component
/// identifier for element i. All elements in the same component have the same ID.
#[derive(Debug, Clone)]
pub struct QuickFindUF {
    id: Vec<usize>,
    count: usize,
}

impl QuickFindUF {
    /// Initializes an empty union-find data structure with n elements (0 through n-1).
    ///
    /// Initially, each element is in its own component.
    ///
    /// # Arguments
    ///
    /// * `n` - the number of elements
    pub fn new(n: usize) -> Self {
        Self {
            id: (0..n).collect(),
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
    pub fn find(&self, p: usize) -> usize {
        self.validate(p);
        self.id[p]
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
        self.validate(p);
        self.validate(q);

        let pid = self.id[p];
        let qid = self.id[q];

        if pid == qid {
            return;
        }

        // Change all entries with id[p] to id[q]
        for i in 0..self.id.len() {
            if self.id[i] == pid {
                self.id[i] = qid;
            }
        }

        self.count -= 1;
    }

    /// Validates that p is a valid index.
    fn validate(&self, p: usize) {
        if p >= self.id.len() {
            panic!("index {} is not between 0 and {}", p, self.id.len() - 1);
        }
    }
}

impl fmt::Display for QuickFindUF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} components", self.count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let uf = QuickFindUF::new(10);
        assert_eq!(uf.count(), 10);
    }

    #[test]
    fn test_union_connected() {
        let mut uf = QuickFindUF::new(10);

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
        let mut uf = QuickFindUF::new(10);

        uf.union(0, 1);
        uf.union(2, 3);
        uf.union(0, 2);

        // All of 0, 1, 2, 3 should have the same ID
        let id0 = uf.find(0);
        assert_eq!(uf.find(1), id0);
        assert_eq!(uf.find(2), id0);
        assert_eq!(uf.find(3), id0);

        // 4 should have a different ID
        assert_ne!(uf.find(4), id0);
    }

    #[test]
    fn test_textbook_example() {
        let mut uf = QuickFindUF::new(10);

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
