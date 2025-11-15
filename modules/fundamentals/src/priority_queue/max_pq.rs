//! Maximum priority queue implementation using a binary heap.
//!
//! The `MaxPQ` data structure represents a priority queue of generic keys.
//! It supports inserting keys and deleting the maximum key.
//!
//! This implementation uses a binary heap, which provides O(log n) time complexity
//! for insert and delete-max operations, and O(1) time for peek operations.
//!
//! # Examples
//!
//! ```
//! use algs4_fundamentals::priority_queue::MaxPQ;
//!
//! let mut pq = MaxPQ::new();
//! pq.insert(5);
//! pq.insert(3);
//! pq.insert(7);
//! pq.insert(1);
//!
//! assert_eq!(pq.max(), Some(&7));
//! assert_eq!(pq.del_max(), Some(7));
//! assert_eq!(pq.del_max(), Some(5));
//! assert_eq!(pq.size(), 2);
//! ```
//!
//! **Reference:** <https://algs4.cs.princeton.edu/24pq>

use std::fmt;

/// A maximum priority queue implemented with a binary heap.
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
/// * Delete max: O(log n) amortized
/// * Max (peek): O(1)
/// * Is empty: O(1)
/// * Size: O(1)
#[derive(Debug, Clone)]
pub struct MaxPQ<T> {
    /// Heap-ordered complete binary tree stored in array indices 1..=n
    /// Index 0 is unused to simplify parent/child calculations
    pq: Vec<Option<T>>,
    /// Number of items in the priority queue
    n: usize,
}

impl<T: Ord> MaxPQ<T> {
    /// Creates an empty priority queue with the given initial capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - Initial capacity for the underlying array
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::priority_queue::MaxPQ;
    ///
    /// let pq: MaxPQ<i32> = MaxPQ::with_capacity(10);
    /// assert!(pq.is_empty());
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        let mut pq = Vec::with_capacity(capacity + 1);
        pq.push(None); // Index 0 is unused
        MaxPQ { pq, n: 0 }
    }

    /// Creates an empty priority queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::priority_queue::MaxPQ;
    ///
    /// let mut pq = MaxPQ::new();
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
    /// use algs4_fundamentals::priority_queue::MaxPQ;
    ///
    /// let pq = MaxPQ::from_slice(&[3, 1, 4, 1, 5, 9, 2, 6]);
    /// assert_eq!(pq.size(), 8);
    /// assert_eq!(pq.max(), Some(&9));
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

        let mut max_pq = MaxPQ { pq, n };

        // Heapify using sink operations from bottom up
        for k in (1..=n / 2).rev() {
            max_pq.sink(k);
        }

        debug_assert!(max_pq.is_max_heap());
        max_pq
    }

    /// Returns `true` if this priority queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::priority_queue::MaxPQ;
    ///
    /// let mut pq = MaxPQ::new();
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
    /// use algs4_fundamentals::priority_queue::MaxPQ;
    ///
    /// let mut pq = MaxPQ::new();
    /// assert_eq!(pq.size(), 0);
    /// pq.insert(1);
    /// pq.insert(2);
    /// assert_eq!(pq.size(), 2);
    /// ```
    pub fn size(&self) -> usize {
        self.n
    }

    /// Returns a reference to the largest key on this priority queue.
    ///
    /// Returns `None` if the priority queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::priority_queue::MaxPQ;
    ///
    /// let mut pq = MaxPQ::new();
    /// assert_eq!(pq.max(), None);
    /// pq.insert(5);
    /// pq.insert(3);
    /// assert_eq!(pq.max(), Some(&5));
    /// ```
    pub fn max(&self) -> Option<&T> {
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
    /// use algs4_fundamentals::priority_queue::MaxPQ;
    ///
    /// let mut pq = MaxPQ::new();
    /// pq.insert(5);
    /// pq.insert(3);
    /// pq.insert(7);
    /// assert_eq!(pq.max(), Some(&7));
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
        debug_assert!(self.is_max_heap());
    }

    /// Removes and returns the largest key on this priority queue.
    ///
    /// Returns `None` if the priority queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::priority_queue::MaxPQ;
    ///
    /// let mut pq = MaxPQ::new();
    /// pq.insert(5);
    /// pq.insert(3);
    /// pq.insert(7);
    /// assert_eq!(pq.del_max(), Some(7));
    /// assert_eq!(pq.del_max(), Some(5));
    /// assert_eq!(pq.del_max(), Some(3));
    /// assert_eq!(pq.del_max(), None);
    /// ```
    pub fn del_max(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let max = self.pq[1].take();
        self.pq.swap(1, self.n);
        self.n -= 1;
        self.sink(1);

        // Avoid loitering
        self.pq[self.n + 1] = None;

        // Shrink array if necessary
        if self.n > 0 && self.n == (self.pq.len() - 1) / 4 {
            self.resize(self.pq.len() / 2);
        }

        debug_assert!(self.is_max_heap());
        max
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
        while k > 1 && self.less(k / 2, k) {
            self.pq.swap(k / 2, k);
            k /= 2;
        }
    }

    /// Restores the heap invariant by moving the key at position k downward.
    fn sink(&mut self, mut k: usize) {
        while 2 * k <= self.n {
            let mut j = 2 * k;
            if j < self.n && self.less(j, j + 1) {
                j += 1;
            }
            if !self.less(k, j) {
                break;
            }
            self.pq.swap(k, j);
            k = j;
        }
    }

    /// Returns `true` if the key at position i is less than the key at position j.
    fn less(&self, i: usize, j: usize) -> bool {
        match (&self.pq[i], &self.pq[j]) {
            (Some(a), Some(b)) => a < b,
            _ => false,
        }
    }

    /// Checks if the heap invariant is satisfied (for debugging).
    fn is_max_heap(&self) -> bool {
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

        self.is_max_heap_ordered(1)
    }

    /// Checks if the subtree rooted at k is a max heap.
    fn is_max_heap_ordered(&self, k: usize) -> bool {
        if k > self.n {
            return true;
        }

        let left = 2 * k;
        let right = 2 * k + 1;

        if left <= self.n && self.less(k, left) {
            return false;
        }
        if right <= self.n && self.less(k, right) {
            return false;
        }

        self.is_max_heap_ordered(left) && self.is_max_heap_ordered(right)
    }

    /// Returns an iterator that iterates over the keys in descending order.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::priority_queue::MaxPQ;
    ///
    /// let mut pq = MaxPQ::new();
    /// pq.insert(3);
    /// pq.insert(1);
    /// pq.insert(4);
    ///
    /// let items: Vec<_> = pq.iter().collect();
    /// assert_eq!(items, vec![&4, &3, &1]);
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

impl<T: Ord> Default for MaxPQ<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord + fmt::Display> fmt::Display for MaxPQ<T> {
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

/// An iterator over items in descending order.
#[derive(Debug)]
pub struct Iter<T> {
    copy: MaxPQ<T>,
}

impl<T: Ord + Clone> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.copy.del_max()
    }
}

impl<'a, T: Ord + Clone> IntoIterator for &'a MaxPQ<T> {
    type Item = T;
    type IntoIter = Iter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// A consuming iterator over items in descending order.
#[derive(Debug)]
pub struct IntoIter<T> {
    pq: MaxPQ<T>,
}

impl<T: Ord> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pq.del_max()
    }
}

impl<T: Ord> IntoIterator for MaxPQ<T> {
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
        let pq: MaxPQ<i32> = MaxPQ::new();
        assert!(pq.is_empty());
        assert_eq!(pq.size(), 0);
        assert_eq!(pq.max(), None);
    }

    #[test]
    fn test_with_capacity() {
        let pq: MaxPQ<i32> = MaxPQ::with_capacity(10);
        assert!(pq.is_empty());
        assert_eq!(pq.size(), 0);
    }

    #[test]
    fn test_insert_and_max() {
        let mut pq = MaxPQ::new();
        pq.insert(5);
        assert_eq!(pq.max(), Some(&5));
        assert_eq!(pq.size(), 1);

        pq.insert(3);
        assert_eq!(pq.max(), Some(&5));
        assert_eq!(pq.size(), 2);

        pq.insert(7);
        assert_eq!(pq.max(), Some(&7));
        assert_eq!(pq.size(), 3);
    }

    #[test]
    fn test_del_max() {
        let mut pq = MaxPQ::new();
        pq.insert(5);
        pq.insert(3);
        pq.insert(7);
        pq.insert(1);
        pq.insert(9);

        assert_eq!(pq.del_max(), Some(9));
        assert_eq!(pq.del_max(), Some(7));
        assert_eq!(pq.del_max(), Some(5));
        assert_eq!(pq.del_max(), Some(3));
        assert_eq!(pq.del_max(), Some(1));
        assert_eq!(pq.del_max(), None);
        assert!(pq.is_empty());
    }

    #[test]
    fn test_from_slice() {
        let pq = MaxPQ::from_slice(&[3, 1, 4, 1, 5, 9, 2, 6]);
        assert_eq!(pq.size(), 8);
        assert_eq!(pq.max(), Some(&9));
    }

    #[test]
    fn test_from_slice_sorted_order() {
        let pq = MaxPQ::from_slice(&[1, 2, 3, 4, 5]);
        let mut sorted: Vec<_> = pq.into_iter().collect();
        sorted.reverse();
        assert_eq!(sorted, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_iter() {
        let mut pq = MaxPQ::new();
        pq.insert(3);
        pq.insert(1);
        pq.insert(4);
        pq.insert(1);
        pq.insert(5);

        let items: Vec<_> = pq.iter().collect();
        assert_eq!(items, vec![5, 4, 3, 1, 1]);
    }

    #[test]
    fn test_into_iter() {
        let mut pq = MaxPQ::new();
        pq.insert(3);
        pq.insert(1);
        pq.insert(4);

        let items: Vec<_> = pq.into_iter().collect();
        assert_eq!(items, vec![4, 3, 1]);
    }

    #[test]
    fn test_resizing() {
        let mut pq = MaxPQ::new();

        // Insert many items to trigger resize
        for i in 0..100 {
            pq.insert(i);
        }
        assert_eq!(pq.size(), 100);
        assert_eq!(pq.max(), Some(&99));

        // Delete many items to trigger shrink
        for _ in 0..90 {
            pq.del_max();
        }
        assert_eq!(pq.size(), 10);
        assert_eq!(pq.max(), Some(&9));
    }

    #[test]
    fn test_strings() {
        let mut pq = MaxPQ::new();
        pq.insert("apple".to_string());
        pq.insert("banana".to_string());
        pq.insert("cherry".to_string());

        assert_eq!(pq.del_max(), Some("cherry".to_string()));
        assert_eq!(pq.del_max(), Some("banana".to_string()));
        assert_eq!(pq.del_max(), Some("apple".to_string()));
    }

    #[test]
    fn test_heap_property() {
        let mut pq = MaxPQ::new();
        for i in 0..20 {
            pq.insert(i);
            assert!(pq.is_max_heap());
        }

        for _ in 0..20 {
            pq.del_max();
            assert!(pq.is_max_heap());
        }
    }

    #[test]
    fn test_display() {
        let mut pq = MaxPQ::new();
        pq.insert(3);
        pq.insert(1);
        pq.insert(4);

        let s = format!("{}", pq);
        assert!(s.contains("4"));
        assert!(s.contains("3"));
        assert!(s.contains("1"));
    }

    #[test]
    fn test_default() {
        let pq: MaxPQ<i32> = MaxPQ::default();
        assert!(pq.is_empty());
    }
}
