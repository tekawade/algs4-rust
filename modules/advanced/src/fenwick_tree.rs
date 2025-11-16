//! Binary Indexed Tree (Fenwick Tree) for efficient prefix sum queries.
//!
//! This module provides a Fenwick Tree data structure that supports:
//! - Prefix sum query in O(log n)
//! - Point update in O(log n)
//! - Range sum query in O(log n)
//!
//! The implementation uses 1-based indexing internally for simpler bit manipulation.
//!
//! # Examples
//!
//! ```
//! use algs4_advanced::FenwickTree;
//!
//! let array = vec![3, 2, -1, 6, 5, 4, -3, 3, 7, 2];
//! let mut tree = FenwickTree::new(array);
//!
//! // Prefix sum query: sum of elements from index 0 to 5
//! assert_eq!(tree.rsq(5), 19); // 3 + 2 - 1 + 6 + 5 + 4
//!
//! // Range sum query: sum of elements from index 2 to 5
//! assert_eq!(tree.rsq_range(2, 5), 14); // -1 + 6 + 5 + 4
//!
//! // Update: set element at index 3 to 10 (was 6)
//! tree.update(3, 10);
//! assert_eq!(tree.rsq(5), 23); // 3 + 2 - 1 + 10 + 5 + 4
//! ```

use std::fmt;

/// Binary Indexed Tree (Fenwick Tree) for efficient prefix sums.
///
/// Uses 1-based indexing internally for simpler implementation.
#[derive(Debug, Clone)]
pub struct FenwickTree {
    /// The BIT array (1-indexed)
    tree: Vec<i64>,
    /// Original array (0-indexed)
    array: Vec<i64>,
}

impl FenwickTree {
    /// Creates a new Fenwick tree from the given array.
    ///
    /// # Arguments
    ///
    /// * `array` - The input array
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::FenwickTree;
    ///
    /// let array = vec![3, 2, -1, 6, 5, 4, -3, 3, 7, 2];
    /// let tree = FenwickTree::new(array);
    /// ```
    pub fn new(array: Vec<i64>) -> Self {
        let n = array.len();
        assert!(n > 0, "Array cannot be empty");

        let tree = vec![0; n + 1]; // 1-indexed
        let mut ft = FenwickTree {
            tree,
            array: array.clone(),
        };

        // Build the tree
        #[allow(clippy::needless_range_loop)]
        for i in 0..n {
            ft.add(i, array[i]);
        }

        ft
    }

    /// Internal method to add a value at index (0-indexed).
    fn add(&mut self, idx: usize, delta: i64) {
        let mut i = (idx + 1) as i64; // Convert to 1-indexed
        while (i as usize) < self.tree.len() {
            self.tree[i as usize] += delta;
            i += i & (-i); // Add last set bit
        }
    }

    /// Returns the prefix sum from index 0 to ind (inclusive).
    ///
    /// # Arguments
    ///
    /// * `ind` - The end index (0-indexed, inclusive)
    ///
    /// # Panics
    ///
    /// Panics if index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::FenwickTree;
    ///
    /// let array = vec![3, 2, -1, 6, 5, 4];
    /// let tree = FenwickTree::new(array);
    ///
    /// assert_eq!(tree.rsq(0), 3);     // 3
    /// assert_eq!(tree.rsq(2), 4);     // 3 + 2 - 1
    /// assert_eq!(tree.rsq(5), 19);    // 3 + 2 - 1 + 6 + 5 + 4
    /// ```
    pub fn rsq(&self, ind: usize) -> i64 {
        assert!(ind < self.array.len(), "Index out of bounds");
        let mut sum = 0;
        let mut i = (ind + 1) as i64; // Convert to 1-indexed

        while i > 0 {
            sum += self.tree[i as usize];
            i -= i & (-i); // Remove last set bit
        }

        sum
    }

    /// Returns the sum of elements in range [a, b] (inclusive).
    ///
    /// # Arguments
    ///
    /// * `a` - Start index (0-indexed, inclusive)
    /// * `b` - End index (0-indexed, inclusive)
    ///
    /// # Panics
    ///
    /// Panics if indices are out of bounds or if a > b.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::FenwickTree;
    ///
    /// let array = vec![3, 2, -1, 6, 5, 4];
    /// let tree = FenwickTree::new(array);
    ///
    /// assert_eq!(tree.rsq_range(0, 2), 4);    // 3 + 2 - 1
    /// assert_eq!(tree.rsq_range(2, 5), 14);   // -1 + 6 + 5 + 4
    /// assert_eq!(tree.rsq_range(1, 3), 7);    // 2 - 1 + 6
    /// ```
    pub fn rsq_range(&self, a: usize, b: usize) -> i64 {
        assert!(
            a < self.array.len() && b < self.array.len() && a <= b,
            "Invalid range"
        );

        if a == 0 {
            self.rsq(b)
        } else {
            self.rsq(b) - self.rsq(a - 1)
        }
    }

    /// Updates the element at index ind to the new value.
    ///
    /// # Arguments
    ///
    /// * `ind` - The index to update (0-indexed)
    /// * `value` - The new value
    ///
    /// # Panics
    ///
    /// Panics if index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::FenwickTree;
    ///
    /// let array = vec![3, 2, -1, 6, 5, 4];
    /// let mut tree = FenwickTree::new(array);
    ///
    /// assert_eq!(tree.rsq(5), 19);
    ///
    /// tree.update(3, 10); // Change 6 to 10
    /// assert_eq!(tree.rsq(5), 23); // 3 + 2 - 1 + 10 + 5 + 4
    /// ```
    pub fn update(&mut self, ind: usize, value: i64) {
        assert!(ind < self.array.len(), "Index out of bounds");

        let delta = value - self.array[ind];
        self.array[ind] = value;
        self.add(ind, delta);
    }

    /// Returns the size of the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::FenwickTree;
    ///
    /// let array = vec![3, 2, -1, 6, 5, 4];
    /// let tree = FenwickTree::new(array);
    ///
    /// assert_eq!(tree.size(), 6);
    /// ```
    pub fn size(&self) -> usize {
        self.array.len()
    }
}

impl fmt::Display for FenwickTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FenwickTree[size={}, array={:?}]",
            self.array.len(),
            self.array
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let array = vec![3, 2, -1, 6, 5, 4];
        let tree = FenwickTree::new(array);
        assert_eq!(tree.size(), 6);
    }

    #[test]
    fn test_rsq_basic() {
        let array = vec![3, 2, -1, 6, 5, 4, -3, 3, 7, 2];
        let tree = FenwickTree::new(array);

        assert_eq!(tree.rsq(0), 3);
        assert_eq!(tree.rsq(1), 5); // 3 + 2
        assert_eq!(tree.rsq(2), 4); // 3 + 2 - 1
        assert_eq!(tree.rsq(5), 19); // 3 + 2 - 1 + 6 + 5 + 4
        assert_eq!(tree.rsq(9), 28); // sum of all
    }

    #[test]
    fn test_rsq_range() {
        let array = vec![3, 2, -1, 6, 5, 4, -3, 3, 7, 2];
        let tree = FenwickTree::new(array);

        assert_eq!(tree.rsq_range(0, 2), 4); // 3 + 2 - 1
        assert_eq!(tree.rsq_range(2, 5), 14); // -1 + 6 + 5 + 4
        assert_eq!(tree.rsq_range(0, 9), 28); // sum of all
        assert_eq!(tree.rsq_range(3, 3), 6); // single element
    }

    #[test]
    fn test_update() {
        let array = vec![3, 2, -1, 6, 5, 4];
        let mut tree = FenwickTree::new(array);

        assert_eq!(tree.rsq(5), 19);

        // Update index 3 from 6 to 10
        tree.update(3, 10);
        assert_eq!(tree.rsq(5), 23); // 3 + 2 - 1 + 10 + 5 + 4
        assert_eq!(tree.rsq(2), 4); // Unchanged
        assert_eq!(tree.rsq_range(3, 5), 19); // 10 + 5 + 4
    }

    #[test]
    fn test_multiple_updates() {
        let array = vec![1, 2, 3, 4, 5];
        let mut tree = FenwickTree::new(array);

        assert_eq!(tree.rsq(4), 15); // 1 + 2 + 3 + 4 + 5

        tree.update(0, 10);
        assert_eq!(tree.rsq(4), 24); // 10 + 2 + 3 + 4 + 5

        tree.update(2, 0);
        assert_eq!(tree.rsq(4), 21); // 10 + 2 + 0 + 4 + 5

        tree.update(4, 15);
        assert_eq!(tree.rsq(4), 31); // 10 + 2 + 0 + 4 + 15
    }

    #[test]
    fn test_single_element() {
        let array = vec![42];
        let mut tree = FenwickTree::new(array);

        assert_eq!(tree.rsq(0), 42);
        assert_eq!(tree.rsq_range(0, 0), 42);

        tree.update(0, 100);
        assert_eq!(tree.rsq(0), 100);
    }

    #[test]
    fn test_negative_values() {
        let array = vec![-5, 3, -2, 8, -1];
        let mut tree = FenwickTree::new(array);

        assert_eq!(tree.rsq(4), 3); // -5 + 3 - 2 + 8 - 1
        assert_eq!(tree.rsq_range(1, 3), 9); // 3 - 2 + 8

        tree.update(2, 5);
        assert_eq!(tree.rsq(4), 10); // -5 + 3 + 5 + 8 - 1
    }

    #[test]
    fn test_all_zeros() {
        let array = vec![0, 0, 0, 0, 0];
        let mut tree = FenwickTree::new(array);

        assert_eq!(tree.rsq(4), 0);
        assert_eq!(tree.rsq_range(1, 3), 0);

        tree.update(2, 5);
        assert_eq!(tree.rsq(4), 5);
        assert_eq!(tree.rsq_range(2, 2), 5);
    }

    #[test]
    fn test_large_values() {
        let array = vec![1_000_000, 2_000_000, 3_000_000];
        let tree = FenwickTree::new(array);

        assert_eq!(tree.rsq(0), 1_000_000);
        assert_eq!(tree.rsq(2), 6_000_000);
        assert_eq!(tree.rsq_range(1, 2), 5_000_000);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_rsq_out_of_bounds() {
        let array = vec![1, 2, 3];
        let tree = FenwickTree::new(array);
        tree.rsq(3);
    }

    #[test]
    #[should_panic(expected = "Invalid range")]
    fn test_rsq_range_invalid() {
        let array = vec![1, 2, 3];
        let tree = FenwickTree::new(array);
        tree.rsq_range(2, 1); // a > b
    }
}
