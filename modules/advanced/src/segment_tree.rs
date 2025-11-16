//! Segment tree with lazy propagation for range queries and updates.
//!
//! This module provides a segment tree data structure that supports:
//! - Range sum query (RSQ)
//! - Range minimum query (RMQ)
//! - Range update with lazy propagation
//!
//! All operations run in O(log n) time.
//!
//! # Examples
//!
//! ```
//! use algs4_advanced::SegmentTree;
//!
//! let array = vec![1, 3, 5, 7, 9, 11];
//! let mut tree = SegmentTree::new(array);
//!
//! // Range sum query: sum of elements from index 1 to 3
//! assert_eq!(tree.rsq(1, 3), 15); // 3 + 5 + 7 = 15
//!
//! // Range minimum query: minimum element from index 1 to 3
//! assert_eq!(tree.r_min_q(1, 3), 3);
//!
//! // Range update: add 10 to elements from index 1 to 3
//! tree.update(1, 3, 10);
//! assert_eq!(tree.rsq(1, 3), 45); // (3+10) + (5+10) + (7+10) = 45
//! ```

use std::fmt;

/// Node in the segment tree.
#[derive(Debug, Clone, Copy)]
struct Node {
    /// Sum of elements in this range
    sum: i64,
    /// Minimum element in this range
    min: i64,
    /// Pending update value (lazy propagation)
    pending: i64,
}

impl Node {
    fn new() -> Self {
        Node {
            sum: 0,
            min: i64::MAX,
            pending: 0,
        }
    }
}

/// Segment tree with lazy propagation.
///
/// Supports range sum query, range minimum query, and range update operations
/// in O(log n) time.
#[derive(Debug, Clone)]
pub struct SegmentTree {
    /// The segment tree nodes
    tree: Vec<Node>,
    /// Size of the original array
    n: usize,
}

impl SegmentTree {
    /// Creates a new segment tree from the given array.
    ///
    /// # Arguments
    ///
    /// * `array` - The input array
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::SegmentTree;
    ///
    /// let array = vec![1, 3, 5, 7, 9, 11];
    /// let tree = SegmentTree::new(array);
    /// ```
    pub fn new(array: Vec<i64>) -> Self {
        let n = array.len();
        assert!(n > 0, "Array cannot be empty");

        // Segment tree needs 4*n space
        let tree = vec![Node::new(); 4 * n];
        let mut st = SegmentTree { tree, n };

        if n > 0 {
            st.build(&array, 0, 0, n - 1);
        }

        st
    }

    /// Builds the segment tree recursively.
    fn build(&mut self, array: &[i64], node: usize, start: usize, end: usize) {
        if start == end {
            // Leaf node
            self.tree[node].sum = array[start];
            self.tree[node].min = array[start];
        } else {
            let mid = (start + end) / 2;
            let left_child = 2 * node + 1;
            let right_child = 2 * node + 2;

            self.build(array, left_child, start, mid);
            self.build(array, right_child, mid + 1, end);

            self.tree[node].sum = self.tree[left_child].sum + self.tree[right_child].sum;
            self.tree[node].min = self.tree[left_child].min.min(self.tree[right_child].min);
        }
    }

    /// Applies pending updates to a node and propagates to children.
    fn propagate(&mut self, node: usize, start: usize, end: usize) {
        if self.tree[node].pending != 0 {
            let pending = self.tree[node].pending;
            let range_len = (end - start + 1) as i64;

            self.tree[node].sum += pending * range_len;
            self.tree[node].min += pending;

            if start != end {
                let left_child = 2 * node + 1;
                let right_child = 2 * node + 2;
                self.tree[left_child].pending += pending;
                self.tree[right_child].pending += pending;
            }

            self.tree[node].pending = 0;
        }
    }

    /// Range sum query: returns the sum of elements in range [a, b].
    ///
    /// # Arguments
    ///
    /// * `a` - Start index (inclusive)
    /// * `b` - End index (inclusive)
    ///
    /// # Panics
    ///
    /// Panics if indices are out of bounds or if a > b.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::SegmentTree;
    ///
    /// let array = vec![1, 3, 5, 7, 9, 11];
    /// let tree = SegmentTree::new(array);
    ///
    /// assert_eq!(tree.rsq(1, 3), 15); // 3 + 5 + 7 = 15
    /// assert_eq!(tree.rsq(0, 5), 36); // 1 + 3 + 5 + 7 + 9 + 11 = 36
    /// ```
    pub fn rsq(&mut self, a: usize, b: usize) -> i64 {
        assert!(a < self.n && b < self.n && a <= b, "Invalid range");
        self.query_sum(0, 0, self.n - 1, a, b)
    }

    /// Internal method for range sum query.
    fn query_sum(&mut self, node: usize, start: usize, end: usize, l: usize, r: usize) -> i64 {
        self.propagate(node, start, end);

        if r < start || l > end {
            // No overlap
            return 0;
        }

        if l <= start && end <= r {
            // Complete overlap
            return self.tree[node].sum;
        }

        // Partial overlap
        let mid = (start + end) / 2;
        let left_child = 2 * node + 1;
        let right_child = 2 * node + 2;

        let left_sum = self.query_sum(left_child, start, mid, l, r);
        let right_sum = self.query_sum(right_child, mid + 1, end, l, r);

        left_sum + right_sum
    }

    /// Range minimum query: returns the minimum element in range [a, b].
    ///
    /// # Arguments
    ///
    /// * `a` - Start index (inclusive)
    /// * `b` - End index (inclusive)
    ///
    /// # Panics
    ///
    /// Panics if indices are out of bounds or if a > b.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::SegmentTree;
    ///
    /// let array = vec![1, 3, 5, 7, 9, 11];
    /// let tree = SegmentTree::new(array);
    ///
    /// assert_eq!(tree.r_min_q(1, 3), 3);
    /// assert_eq!(tree.r_min_q(0, 5), 1);
    /// ```
    pub fn r_min_q(&mut self, a: usize, b: usize) -> i64 {
        assert!(a < self.n && b < self.n && a <= b, "Invalid range");
        self.query_min(0, 0, self.n - 1, a, b)
    }

    /// Internal method for range minimum query.
    fn query_min(&mut self, node: usize, start: usize, end: usize, l: usize, r: usize) -> i64 {
        self.propagate(node, start, end);

        if r < start || l > end {
            // No overlap
            return i64::MAX;
        }

        if l <= start && end <= r {
            // Complete overlap
            return self.tree[node].min;
        }

        // Partial overlap
        let mid = (start + end) / 2;
        let left_child = 2 * node + 1;
        let right_child = 2 * node + 2;

        let left_min = self.query_min(left_child, start, mid, l, r);
        let right_min = self.query_min(right_child, mid + 1, end, l, r);

        left_min.min(right_min)
    }

    /// Range update: adds value to all elements in range [a, b].
    ///
    /// Uses lazy propagation for efficiency.
    ///
    /// # Arguments
    ///
    /// * `a` - Start index (inclusive)
    /// * `b` - End index (inclusive)
    /// * `value` - Value to add to each element
    ///
    /// # Panics
    ///
    /// Panics if indices are out of bounds or if a > b.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::SegmentTree;
    ///
    /// let array = vec![1, 3, 5, 7, 9, 11];
    /// let mut tree = SegmentTree::new(array);
    ///
    /// tree.update(1, 3, 10);
    /// assert_eq!(tree.rsq(1, 3), 45); // (3+10) + (5+10) + (7+10)
    /// ```
    pub fn update(&mut self, a: usize, b: usize, value: i64) {
        assert!(a < self.n && b < self.n && a <= b, "Invalid range");
        self.update_range(0, 0, self.n - 1, a, b, value);
    }

    /// Internal method for range update.
    fn update_range(
        &mut self,
        node: usize,
        start: usize,
        end: usize,
        l: usize,
        r: usize,
        value: i64,
    ) {
        self.propagate(node, start, end);

        if r < start || l > end {
            // No overlap
            return;
        }

        if l <= start && end <= r {
            // Complete overlap
            self.tree[node].pending = value;
            self.propagate(node, start, end);
            return;
        }

        // Partial overlap
        let mid = (start + end) / 2;
        let left_child = 2 * node + 1;
        let right_child = 2 * node + 2;

        self.update_range(left_child, start, mid, l, r, value);
        self.update_range(right_child, mid + 1, end, l, r, value);

        self.propagate(left_child, start, mid);
        self.propagate(right_child, mid + 1, end);

        self.tree[node].sum = self.tree[left_child].sum + self.tree[right_child].sum;
        self.tree[node].min = self.tree[left_child].min.min(self.tree[right_child].min);
    }

    /// Returns the size of the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::SegmentTree;
    ///
    /// let array = vec![1, 3, 5, 7, 9, 11];
    /// let tree = SegmentTree::new(array);
    ///
    /// assert_eq!(tree.size(), 6);
    /// ```
    pub fn size(&self) -> usize {
        self.n
    }
}

impl fmt::Display for SegmentTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SegmentTree[size={}]", self.n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let array = vec![1, 3, 5, 7, 9, 11];
        let tree = SegmentTree::new(array);
        assert_eq!(tree.size(), 6);
    }

    #[test]
    fn test_rsq_basic() {
        let array = vec![1, 3, 5, 7, 9, 11];
        let mut tree = SegmentTree::new(array);

        assert_eq!(tree.rsq(0, 0), 1);
        assert_eq!(tree.rsq(1, 3), 15); // 3 + 5 + 7
        assert_eq!(tree.rsq(0, 5), 36); // 1 + 3 + 5 + 7 + 9 + 11
    }

    #[test]
    fn test_r_min_q_basic() {
        let array = vec![5, 3, 7, 1, 9, 2];
        let mut tree = SegmentTree::new(array);

        assert_eq!(tree.r_min_q(0, 0), 5);
        assert_eq!(tree.r_min_q(1, 3), 1);
        assert_eq!(tree.r_min_q(0, 5), 1);
        assert_eq!(tree.r_min_q(4, 5), 2);
    }

    #[test]
    fn test_update_and_query() {
        let array = vec![1, 3, 5, 7, 9, 11];
        let mut tree = SegmentTree::new(array);

        // Update range [1, 3] by adding 10
        tree.update(1, 3, 10);

        // New values: [1, 13, 15, 17, 9, 11]
        assert_eq!(tree.rsq(1, 3), 45); // 13 + 15 + 17
        assert_eq!(tree.rsq(0, 5), 66); // 1 + 13 + 15 + 17 + 9 + 11
    }

    #[test]
    fn test_multiple_updates() {
        let array = vec![1, 2, 3, 4, 5];
        let mut tree = SegmentTree::new(array);

        tree.update(0, 2, 5);
        // [6, 7, 8, 4, 5]
        assert_eq!(tree.rsq(0, 2), 21);

        tree.update(1, 3, 3);
        // [6, 10, 11, 7, 5]
        assert_eq!(tree.rsq(1, 3), 28);

        assert_eq!(tree.r_min_q(0, 4), 5);
    }

    #[test]
    fn test_single_element() {
        let array = vec![42];
        let mut tree = SegmentTree::new(array);

        assert_eq!(tree.rsq(0, 0), 42);
        assert_eq!(tree.r_min_q(0, 0), 42);

        tree.update(0, 0, 8);
        assert_eq!(tree.rsq(0, 0), 50);
    }

    #[test]
    fn test_negative_values() {
        let array = vec![-5, 3, -2, 8, -1];
        let mut tree = SegmentTree::new(array);

        assert_eq!(tree.rsq(0, 4), 3); // -5 + 3 - 2 + 8 - 1
        assert_eq!(tree.r_min_q(0, 4), -5);

        tree.update(0, 2, 10);
        // [5, 13, 8, 8, -1]
        assert_eq!(tree.rsq(0, 2), 26);
        assert_eq!(tree.r_min_q(0, 4), -1);
    }

    #[test]
    fn test_overlapping_updates() {
        let array = vec![1, 2, 3, 4, 5];
        let mut tree = SegmentTree::new(array);

        tree.update(0, 2, 1);
        tree.update(2, 4, 2);
        // [2, 3, 6, 6, 7]
        assert_eq!(tree.rsq(0, 4), 24);
        assert_eq!(tree.r_min_q(0, 4), 2);
    }
}
