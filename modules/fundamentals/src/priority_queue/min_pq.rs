//! Minimum priority queue implementation using a binary heap.
//!
//! The `MinPQ` data structure represents a priority queue of generic keys.
//! It supports inserting keys and deleting the minimum key.
//!
//! This implementation uses a binary heap, which provides O(log n) time complexity
//! for insert and delete-min operations, and O(1) time for peek operations.
//!
//! # Examples
//!
//! ```
//! use algs4_fundamentals::priority_queue::MinPQ;
//!
//! let mut pq = MinPQ::new();
//! pq.insert(5);
//! pq.insert(3);
//! pq.insert(7);
//! pq.insert(1);
//!
//! assert_eq!(pq.min(), Some(&1));
//! assert_eq!(pq.del_min(), Some(1));
//! assert_eq!(pq.del_min(), Some(3));
//! assert_eq!(pq.size(), 2);
//! ```
//!
//! **Reference:** <https://algs4.cs.princeton.edu/24pq>

use std::fmt;

/// A minimum priority queue implemented with a binary heap.
///
/// The binary heap is represented as a one-based array where the root is at index 1.
/// For any node at position k:
/// - Parent is at k/2
/// - Left child is at 2k
/// - Right child is at 2k+1
///
/// # Type Parameters
///
/// * `T` - The type of elements stored in the priority queue. Must implement `Ord` for comparison.
///
/// # Performance
///
/// * Insert: O(log n) amortized (due to array resizing)
/// * Delete min: O(log n) amortized
/// * Min (peek): O(1)
/// * Is empty: O(1)
/// * Size: O(1)
#[derive(Debug, Clone)]
pub struct MinPQ<T> {
    /// Heap-ordered complete binary tree stored in array indices 1..=n
    /// Index 0 is unused to simplify parent/child calculations
    pq: Vec<Option<T>>,
    /// Number of items in the priority queue
    n: usize,
}

impl<T: Ord> MinPQ<T> {
    /// Creates an empty priority queue with the given initial capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - Initial capacity for the underlying array
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::priority_queue::MinPQ;
    ///
    /// let pq: MinPQ<i32> = MinPQ::with_capacity(10);
    /// assert!(pq.is_empty());
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        let mut pq = Vec::with_capacity(capacity + 1);
        pq.push(None); // Index 0 is unused
        MinPQ { pq, n: 0 }
    }

    /// Creates an empty priority queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::priority_queue::MinPQ;
    ///
    /// let mut pq = MinPQ::new();
    /// pq.insert(42);
    /// assert_eq!(pq.size(), 1);
    /// ```
    pub fn new() -> Self {
        Self::with_capacity(1)
    }

    /// Creates a priority queue from an array of keys.
    ///
    /// Takes time proportional to the number of keys, using sink-based heap construction.
    ///
    /// # Arguments
    ///
    /// * `keys` - Slice of keys to initialize the priority queue
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::priority_queue::MinPQ;
    ///
    /// let pq = MinPQ::from_slice(&[3, 1, 4, 1, 5, 9, 2, 6]);
    /// assert_eq!(pq.size(), 8);
    /// assert_eq!(pq.min(), Some(&1));
    /// ```
    pub fn from_slice(keys: &[T]) -> Self
    where
        T: Clone,
    {
        let n = keys.len();
        let mut pq = Vec::with_capacity(n + 1);
        pq.push(None); // Index 0 is unused

        for key in keys {
            pq.push(Some(key.clone()));
        }

        let mut min_pq = MinPQ { pq, n };

        // Heapify using sink operations from bottom up
        for k in (1..=n / 2).rev() {
            min_pq.sink(k);
        }

        debug_assert!(min_pq.is_min_heap());
        min_pq
    }

    /// Returns `true` if this priority queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::priority_queue::MinPQ;
    ///
    /// let mut pq = MinPQ::new();
    /// assert!(pq.is_empty());
    /// pq.insert(1);
    /// assert!(!pq.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Returns the number of keys on this priority queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::priority_queue::MinPQ;
    ///
    /// let mut pq = MinPQ::new();
    /// assert_eq!(pq.size(), 0);
    /// pq.insert(1);
    /// pq.insert(2);
    /// assert_eq!(pq.size(), 2);
    /// ```
    pub fn size(&self) -> usize {
        self.n
    }

    /// Returns a reference to the smallest key on this priority queue.
    ///
    /// Returns `None` if the priority queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::priority_queue::MinPQ;
    ///
    /// let mut pq = MinPQ::new();
    /// assert_eq!(pq.min(), None);
    /// pq.insert(5);
    /// pq.insert(3);
    /// assert_eq!(pq.min(), Some(&3));
    /// ```
    pub fn min(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.pq[1].as_ref()
        }
    }

    /// Adds a new key to this priority queue.
    ///
    /// # Arguments
    ///
    /// * `x` - The key to add
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::priority_queue::MinPQ;
    ///
    /// let mut pq = MinPQ::new();
    /// pq.insert(5);
    /// pq.insert(3);
    /// pq.insert(7);
    /// assert_eq!(pq.min(), Some(&3));
    /// ```
    pub fn insert(&mut self, x: T) {
        // Double size of array if necessary
        if self.n == self.pq.len() - 1 {
            self.resize(2 * self.pq.len());
        }

        // Add x and percolate it up to maintain heap invariant
        self.n += 1;
        if self.n < self.pq.len() {
            self.pq[self.n] = Some(x);
        } else {
            self.pq.push(Some(x));
        }
        self.swim(self.n);
        debug_assert!(self.is_min_heap());
    }

    /// Removes and returns the smallest key on this priority queue.
    ///
    /// Returns `None` if the priority queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::priority_queue::MinPQ;
    ///
    /// let mut pq = MinPQ::new();
    /// pq.insert(5);
    /// pq.insert(3);
    /// pq.insert(7);
    /// assert_eq!(pq.del_min(), Some(3));
    /// assert_eq!(pq.del_min(), Some(5));
    /// assert_eq!(pq.del_min(), Some(7));
    /// assert_eq!(pq.del_min(), None);
    /// ```
    pub fn del_min(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let min = self.pq[1].take();
        self.pq.swap(1, self.n);
        self.n -= 1;
        self.sink(1);

        // Avoid loitering
        self.pq[self.n + 1] = None;

        // Shrink array if necessary
        if self.n > 0 && self.n == (self.pq.len() - 1) / 4 {
            self.resize(self.pq.len() / 2);
        }

        debug_assert!(self.is_min_heap());
        min
    }

    /// Resizes the underlying array to the given capacity.
    fn resize(&mut self, capacity: usize) {
        debug_assert!(capacity > self.n);
        let mut temp = Vec::with_capacity(capacity);
        temp.push(None); // Index 0 is unused

        for i in 1..=self.n {
            temp.push(self.pq[i].take());
        }

        self.pq = temp;
    }

    /// Restores the heap invariant by moving the key at position k upward.
    fn swim(&mut self, mut k: usize) {
        while k > 1 && self.greater(k / 2, k) {
            self.pq.swap(k / 2, k);
            k /= 2;
        }
    }

    /// Restores the heap invariant by moving the key at position k downward.
    fn sink(&mut self, mut k: usize) {
        while 2 * k <= self.n {
            let mut j = 2 * k;
            if j < self.n && self.greater(j, j + 1) {
                j += 1;
            }
            if !self.greater(k, j) {
                break;
            }
            self.pq.swap(k, j);
            k = j;
        }
    }

    /// Returns `true` if the key at position i is greater than the key at position j.
    fn greater(&self, i: usize, j: usize) -> bool {
        match (&self.pq[i], &self.pq[j]) {
            (Some(a), Some(b)) => a > b,
            _ => false,
        }
    }

    /// Checks if the heap invariant is satisfied (for debugging).
    fn is_min_heap(&self) -> bool {
        // Check that indices 1..=n are not None
        for i in 1..=self.n {
            if self.pq[i].is_none() {
                return false;
            }
        }

        // Check that indices n+1.. are None
        for i in (self.n + 1)..self.pq.len() {
            if self.pq[i].is_some() {
                return false;
            }
        }

        // Check that index 0 is None
        if self.pq[0].is_some() {
            return false;
        }

        self.is_min_heap_ordered(1)
    }

    /// Checks if the subtree rooted at k is a min heap.
    fn is_min_heap_ordered(&self, k: usize) -> bool {
        if k > self.n {
            return true;
        }

        let left = 2 * k;
        let right = 2 * k + 1;

        if left <= self.n && self.greater(k, left) {
            return false;
        }
        if right <= self.n && self.greater(k, right) {
            return false;
        }

        self.is_min_heap_ordered(left) && self.is_min_heap_ordered(right)
    }

    /// Returns an iterator that iterates over the keys in ascending order.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::priority_queue::MinPQ;
    ///
    /// let mut pq = MinPQ::new();
    /// pq.insert(3);
    /// pq.insert(1);
    /// pq.insert(4);
    ///
    /// let items: Vec<_> = pq.iter().collect();
    /// assert_eq!(items, vec![&1, &3, &4]);
    /// ```
    pub fn iter(&self) -> Iter<T>
    where
        T: Clone,
    {
        Iter {
            copy: self.clone(),
        }
    }
}

impl<T: Ord> Default for MinPQ<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord + fmt::Display> fmt::Display for MinPQ<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for i in 1..=self.n {
            if let Some(ref item) = self.pq[i] {
                if i > 1 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", item)?;
            }
        }
        write!(f, "]")
    }
}

/// An iterator over items in ascending order.
#[derive(Debug)]
pub struct Iter<T> {
    copy: MinPQ<T>,
}

impl<T: Ord + Clone> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.copy.del_min()
    }
}

impl<'a, T: Ord + Clone> IntoIterator for &'a MinPQ<T> {
    type Item = T;
    type IntoIter = Iter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// A consuming iterator over items in ascending order.
#[derive(Debug)]
pub struct IntoIter<T> {
    pq: MinPQ<T>,
}

impl<T: Ord> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pq.del_min()
    }
}

impl<T: Ord> IntoIterator for MinPQ<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { pq: self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pq: MinPQ<i32> = MinPQ::new();
        assert!(pq.is_empty());
        assert_eq!(pq.size(), 0);
        assert_eq!(pq.min(), None);
    }

    #[test]
    fn test_with_capacity() {
        let pq: MinPQ<i32> = MinPQ::with_capacity(10);
        assert!(pq.is_empty());
        assert_eq!(pq.size(), 0);
    }

    #[test]
    fn test_insert_and_min() {
        let mut pq = MinPQ::new();
        pq.insert(5);
        assert_eq!(pq.min(), Some(&5));
        assert_eq!(pq.size(), 1);

        pq.insert(3);
        assert_eq!(pq.min(), Some(&3));
        assert_eq!(pq.size(), 2);

        pq.insert(7);
        assert_eq!(pq.min(), Some(&3));
        assert_eq!(pq.size(), 3);

        pq.insert(1);
        assert_eq!(pq.min(), Some(&1));
        assert_eq!(pq.size(), 4);
    }

    #[test]
    fn test_del_min() {
        let mut pq = MinPQ::new();
        pq.insert(5);
        pq.insert(3);
        pq.insert(7);
        pq.insert(1);
        pq.insert(9);

        assert_eq!(pq.del_min(), Some(1));
        assert_eq!(pq.del_min(), Some(3));
        assert_eq!(pq.del_min(), Some(5));
        assert_eq!(pq.del_min(), Some(7));
        assert_eq!(pq.del_min(), Some(9));
        assert_eq!(pq.del_min(), None);
        assert!(pq.is_empty());
    }

    #[test]
    fn test_from_slice() {
        let pq = MinPQ::from_slice(&[3, 1, 4, 1, 5, 9, 2, 6]);
        assert_eq!(pq.size(), 8);
        assert_eq!(pq.min(), Some(&1));
    }

    #[test]
    fn test_from_slice_sorted_order() {
        let pq = MinPQ::from_slice(&[5, 4, 3, 2, 1]);
        let sorted: Vec<_> = pq.into_iter().collect();
        assert_eq!(sorted, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_iter() {
        let mut pq = MinPQ::new();
        pq.insert(3);
        pq.insert(1);
        pq.insert(4);
        pq.insert(1);
        pq.insert(5);

        let items: Vec<_> = pq.iter().collect();
        assert_eq!(items, vec![1, 1, 3, 4, 5]);
    }

    #[test]
    fn test_into_iter() {
        let mut pq = MinPQ::new();
        pq.insert(3);
        pq.insert(1);
        pq.insert(4);

        let items: Vec<_> = pq.into_iter().collect();
        assert_eq!(items, vec![1, 3, 4]);
    }

    #[test]
    fn test_resizing() {
        let mut pq = MinPQ::new();

        // Insert many items to trigger resize
        for i in (0..100).rev() {
            pq.insert(i);
        }
        assert_eq!(pq.size(), 100);
        assert_eq!(pq.min(), Some(&0));

        // Delete many items to trigger shrink
        for i in 0..90 {
            assert_eq!(pq.del_min(), Some(i));
        }
        assert_eq!(pq.size(), 10);
        assert_eq!(pq.min(), Some(&90));
    }

    #[test]
    fn test_strings() {
        let mut pq = MinPQ::new();
        pq.insert("cherry".to_string());
        pq.insert("banana".to_string());
        pq.insert("apple".to_string());

        assert_eq!(pq.del_min(), Some("apple".to_string()));
        assert_eq!(pq.del_min(), Some("banana".to_string()));
        assert_eq!(pq.del_min(), Some("cherry".to_string()));
    }

    #[test]
    fn test_heap_property() {
        let mut pq = MinPQ::new();
        for i in (0..20).rev() {
            pq.insert(i);
            assert!(pq.is_min_heap());
        }

        for _ in 0..20 {
            pq.del_min();
            assert!(pq.is_min_heap());
        }
    }

    #[test]
    fn test_display() {
        let mut pq = MinPQ::new();
        pq.insert(3);
        pq.insert(1);
        pq.insert(4);

        let s = format!("{}", pq);
        assert!(s.contains("1"));
        assert!(s.contains("3"));
        assert!(s.contains("4"));
    }

    #[test]
    fn test_default() {
        let pq: MinPQ<i32> = MinPQ::default();
        assert!(pq.is_empty());
    }
}
