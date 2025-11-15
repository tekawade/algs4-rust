//! Indexed minimum priority queue implementation using a binary heap.
//!
//! The `IndexMinPQ` data structure represents an indexed priority queue of generic keys.
//! It supports the usual insert and delete-the-minimum operations, along with delete and
//! change-the-key methods. An integer between 0 and maxN-1 is associated with each key,
//! allowing the client to refer to items on the priority queue.
//!
//! # Examples
//!
//! ```
//! use algs4_fundamentals::priority_queue::IndexMinPQ;
//!
//! let mut pq = IndexMinPQ::new(10);
//! pq.insert(0, 5);
//! pq.insert(1, 3);
//! pq.insert(2, 7);
//!
//! assert_eq!(pq.min_index(), Some(1));
//! assert_eq!(pq.min_key(), Some(&3));
//! assert_eq!(pq.del_min(), Some(1));
//! ```
//!
//! **Reference:** <https://algs4.cs.princeton.edu/24pq>

use std::fmt;

/// An indexed minimum priority queue implemented with a binary heap.
///
/// Associates keys with integer indices in the range [0, max_n).
/// Maintains three arrays:
/// - `pq`: binary heap of indices (1-based)
/// - `qp`: inverse of pq, qp[pq[i]] = i
/// - `keys`: keys[i] = priority of index i
///
/// # Type Parameters
///
/// * `T` - The type of keys stored in the priority queue. Must implement `Ord` for comparison.
///
/// # Performance
///
/// * Insert: O(log n)
/// * Delete min: O(log n)
/// * Change key: O(log n)
/// * Contains: O(1)
/// * Is empty: O(1)
#[derive(Debug, Clone)]
pub struct IndexMinPQ<T> {
    max_n: usize,               // maximum number of elements
    n: usize,                   // current number of elements
    pq: Vec<usize>,             // binary heap using 1-based indexing (stores indices)
    qp: Vec<Option<usize>>,     // inverse: qp[pq[i]] = i (None if index not in PQ)
    keys: Vec<Option<T>>,       // keys[i] = priority of i
}

impl<T: Ord> IndexMinPQ<T> {
    /// Creates an empty indexed priority queue with indices from 0 to max_n-1.
    ///
    /// # Arguments
    ///
    /// * `max_n` - Maximum number of keys (indices range from 0 to max_n-1)
    ///
    /// # Panics
    ///
    /// Panics if max_n is 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::priority_queue::IndexMinPQ;
    ///
    /// let pq: IndexMinPQ<i32> = IndexMinPQ::new(10);
    /// assert!(pq.is_empty());
    /// ```
    pub fn new(max_n: usize) -> Self {
        assert!(max_n > 0, "max_n must be positive");
        let keys = vec![None; max_n];
        IndexMinPQ {
            max_n,
            n: 0,
            pq: vec![0; max_n + 1],       // 1-based indexing
            qp: vec![None; max_n],
            keys,
        }
    }

    /// Returns `true` if this priority queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::priority_queue::IndexMinPQ;
    ///
    /// let mut pq = IndexMinPQ::new(10);
    /// assert!(pq.is_empty());
    /// pq.insert(0, 42);
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
    /// use algs4_fundamentals::priority_queue::IndexMinPQ;
    ///
    /// let mut pq = IndexMinPQ::new(10);
    /// assert_eq!(pq.size(), 0);
    /// pq.insert(0, 1);
    /// assert_eq!(pq.size(), 1);
    /// ```
    pub fn size(&self) -> usize {
        self.n
    }

    /// Returns `true` if i is an index on this priority queue.
    ///
    /// # Arguments
    ///
    /// * `i` - An index to check
    ///
    /// # Panics
    ///
    /// Panics if i >= max_n.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::priority_queue::IndexMinPQ;
    ///
    /// let mut pq = IndexMinPQ::new(10);
    /// assert!(!pq.contains(0));
    /// pq.insert(0, 42);
    /// assert!(pq.contains(0));
    /// ```
    pub fn contains(&self, i: usize) -> bool {
        self.validate_index(i);
        self.qp[i].is_some()
    }

    /// Associates key with index i.
    ///
    /// # Arguments
    ///
    /// * `i` - The index
    /// * `key` - The key to associate with index i
    ///
    /// # Panics
    ///
    /// Panics if i >= max_n or if there's already a key associated with index i.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::priority_queue::IndexMinPQ;
    ///
    /// let mut pq = IndexMinPQ::new(10);
    /// pq.insert(0, 42);
    /// pq.insert(1, 17);
    /// assert_eq!(pq.size(), 2);
    /// ```
    pub fn insert(&mut self, i: usize, key: T) {
        self.validate_index(i);
        assert!(!self.contains(i), "index is already in the priority queue");

        self.n += 1;
        self.qp[i] = Some(self.n);
        self.pq[self.n] = i;
        self.keys[i] = Some(key);
        self.swim(self.n);
    }

    /// Returns an index associated with a minimum key.
    ///
    /// Returns `None` if the priority queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::priority_queue::IndexMinPQ;
    ///
    /// let mut pq = IndexMinPQ::new(10);
    /// pq.insert(0, 5);
    /// pq.insert(1, 7);
    /// pq.insert(2, 3);
    /// assert_eq!(pq.min_index(), Some(2));
    /// ```
    pub fn min_index(&self) -> Option<usize> {
        if self.n == 0 {
            None
        } else {
            Some(self.pq[1])
        }
    }

    /// Returns a reference to a minimum key.
    ///
    /// Returns `None` if the priority queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::priority_queue::IndexMinPQ;
    ///
    /// let mut pq = IndexMinPQ::new(10);
    /// pq.insert(0, 5);
    /// pq.insert(1, 7);
    /// assert_eq!(pq.min_key(), Some(&5));
    /// ```
    pub fn min_key(&self) -> Option<&T> {
        if self.n == 0 {
            None
        } else {
            self.keys[self.pq[1]].as_ref()
        }
    }

    /// Removes a minimum key and returns its associated index.
    ///
    /// Returns `None` if the priority queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::priority_queue::IndexMinPQ;
    ///
    /// let mut pq = IndexMinPQ::new(10);
    /// pq.insert(0, 5);
    /// pq.insert(1, 7);
    /// pq.insert(2, 3);
    /// assert_eq!(pq.del_min(), Some(2));
    /// assert_eq!(pq.del_min(), Some(0));
    /// ```
    pub fn del_min(&mut self) -> Option<usize> {
        if self.n == 0 {
            return None;
        }

        let min = self.pq[1];
        self.exch(1, self.n);
        self.n -= 1;
        self.sink(1);

        debug_assert_eq!(self.pq[self.n + 1], min);
        self.qp[min] = None;
        self.keys[min] = None;

        Some(min)
    }

    /// Returns the key associated with index i.
    ///
    /// # Arguments
    ///
    /// * `i` - The index
    ///
    /// # Panics
    ///
    /// Panics if i >= max_n or if no key is associated with index i.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::priority_queue::IndexMinPQ;
    ///
    /// let mut pq = IndexMinPQ::new(10);
    /// pq.insert(0, 42);
    /// assert_eq!(pq.key_of(0), Some(&42));
    /// ```
    pub fn key_of(&self, i: usize) -> Option<&T> {
        self.validate_index(i);
        if !self.contains(i) {
            None
        } else {
            self.keys[i].as_ref()
        }
    }

    /// Changes the key associated with index i to the specified value.
    ///
    /// # Arguments
    ///
    /// * `i` - The index
    /// * `key` - The new key value
    ///
    /// # Panics
    ///
    /// Panics if i >= max_n or if no key is associated with index i.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::priority_queue::IndexMinPQ;
    ///
    /// let mut pq = IndexMinPQ::new(10);
    /// pq.insert(0, 42);
    /// pq.change_key(0, 100);
    /// assert_eq!(pq.key_of(0), Some(&100));
    /// ```
    pub fn change_key(&mut self, i: usize, key: T) {
        self.validate_index(i);
        assert!(self.contains(i), "index is not in the priority queue");

        self.keys[i] = Some(key);
        let pos = self.qp[i].unwrap();
        self.swim(pos);
        self.sink(pos);
    }

    /// Increases the key associated with index i to the specified value.
    ///
    /// # Arguments
    ///
    /// * `i` - The index
    /// * `key` - The new key value (must be >= current key)
    ///
    /// # Panics
    ///
    /// Panics if i >= max_n, if no key is associated with index i,
    /// or if the new key is less than the current key.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::priority_queue::IndexMinPQ;
    ///
    /// let mut pq = IndexMinPQ::new(10);
    /// pq.insert(0, 42);
    /// pq.increase_key(0, 100);
    /// assert_eq!(pq.key_of(0), Some(&100));
    /// ```
    pub fn increase_key(&mut self, i: usize, key: T) {
        self.validate_index(i);
        assert!(self.contains(i), "index is not in the priority queue");

        if let Some(ref old_key) = self.keys[i] {
            assert!(
                &key >= old_key,
                "new key must be greater than or equal to old key"
            );
        }

        self.keys[i] = Some(key);
        self.sink(self.qp[i].unwrap());
    }

    /// Decreases the key associated with index i to the specified value.
    ///
    /// # Arguments
    ///
    /// * `i` - The index
    /// * `key` - The new key value (must be <= current key)
    ///
    /// # Panics
    ///
    /// Panics if i >= max_n, if no key is associated with index i,
    /// or if the new key is greater than the current key.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::priority_queue::IndexMinPQ;
    ///
    /// let mut pq = IndexMinPQ::new(10);
    /// pq.insert(0, 100);
    /// pq.decrease_key(0, 42);
    /// assert_eq!(pq.key_of(0), Some(&42));
    /// ```
    pub fn decrease_key(&mut self, i: usize, key: T) {
        self.validate_index(i);
        assert!(self.contains(i), "index is not in the priority queue");

        if let Some(ref old_key) = self.keys[i] {
            assert!(
                &key <= old_key,
                "new key must be less than or equal to old key"
            );
        }

        self.keys[i] = Some(key);
        self.swim(self.qp[i].unwrap());
    }

    /// Removes the key associated with index i.
    ///
    /// # Arguments
    ///
    /// * `i` - The index
    ///
    /// # Panics
    ///
    /// Panics if i >= max_n or if no key is associated with index i.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::priority_queue::IndexMinPQ;
    ///
    /// let mut pq = IndexMinPQ::new(10);
    /// pq.insert(0, 42);
    /// pq.insert(1, 17);
    /// pq.delete(0);
    /// assert!(!pq.contains(0));
    /// assert_eq!(pq.size(), 1);
    /// ```
    pub fn delete(&mut self, i: usize) {
        self.validate_index(i);
        assert!(self.contains(i), "index is not in the priority queue");

        let index = self.qp[i].unwrap();
        self.exch(index, self.n);
        self.n -= 1;
        self.swim(index);
        self.sink(index);
        self.keys[i] = None;
        self.qp[i] = None;
    }

    /// Validates that the given index is in range.
    fn validate_index(&self, i: usize) {
        assert!(i < self.max_n, "index {} is out of bounds (max_n = {})", i, self.max_n);
    }

    /// Compares keys at heap positions i and j.
    fn greater(&self, i: usize, j: usize) -> bool {
        let key_i = &self.keys[self.pq[i]];
        let key_j = &self.keys[self.pq[j]];
        match (key_i, key_j) {
            (Some(a), Some(b)) => a > b,
            _ => false,
        }
    }
    }

    /// Exchanges heap positions i and j (also updates qp).
    fn exch(&mut self, i: usize, j: usize) {
        self.pq.swap(i, j);
        self.qp[self.pq[i]] = Some(i);
        self.qp[self.pq[j]] = Some(j);
    }

    /// Restores heap invariant by moving up.
    fn swim(&mut self, mut k: usize) {
        while k > 1 && self.less(k / 2, k) {
            self.exch(k / 2, k);
            k /= 2;
        }
    }

    /// Restores heap invariant by moving down.
    fn sink(&mut self, mut k: usize) {
        while 2 * k <= self.n {
            let mut j = 2 * k;
            if j < self.n && self.less(j, j + 1) {
                j += 1;
            }
            if !self.less(k, j) {
                break;
            }
            self.exch(k, j);
            k = j;
        }
    }
}

impl<T: Ord + fmt::Display> fmt::Display for IndexMinPQ<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let mut first = true;
        for i in 1..=self.n {
            let idx = self.pq[i];
            if let Some(ref key) = self.keys[idx] {
                if !first {
                    write!(f, ", ")?;
                }
                write!(f, "{}:{}", idx, key)?;
                first = false;
            }
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pq: IndexMinPQ<i32> = IndexMinPQ::new(10);
        assert!(pq.is_empty());
        assert_eq!(pq.size(), 0);
    }

    #[test]
    fn test_insert_and_min() {
        let mut pq = IndexMinPQ::new(10);
        pq.insert(0, 5);
        pq.insert(1, 3);
        pq.insert(2, 7);

        assert_eq!(pq.min_index(), Some(1));
        assert_eq!(pq.min_key(), Some(&3));
        assert_eq!(pq.size(), 3);
    }

    #[test]
    fn test_del_min() {
        let mut pq = IndexMinPQ::new(10);
        pq.insert(0, 5);
        pq.insert(1, 3);
        pq.insert(2, 7);
        pq.insert(3, 1);

        assert_eq!(pq.del_min(), Some(3));
        assert_eq!(pq.del_min(), Some(1));
        assert_eq!(pq.del_min(), Some(0));
        assert_eq!(pq.del_min(), Some(2));
        assert_eq!(pq.del_min(), None);
    }

    #[test]
    fn test_contains() {
        let mut pq = IndexMinPQ::new(10);
        assert!(!pq.contains(0));
        pq.insert(0, 42);
        assert!(pq.contains(0));
        assert!(!pq.contains(1));
    }

    #[test]
    fn test_key_of() {
        let mut pq = IndexMinPQ::new(10);
        pq.insert(0, 42);
        pq.insert(1, 17);
        assert_eq!(pq.key_of(0), Some(&42));
        assert_eq!(pq.key_of(1), Some(&17));
        assert_eq!(pq.key_of(2), None);
    }

    #[test]
    fn test_change_key() {
        let mut pq = IndexMinPQ::new(10);
        pq.insert(0, 42);
        pq.insert(1, 17);
        pq.insert(2, 99);

        pq.change_key(2, 5);
        assert_eq!(pq.min_index(), Some(2));
        assert_eq!(pq.min_key(), Some(&5));
    }

    #[test]
    fn test_increase_key() {
        let mut pq = IndexMinPQ::new(10);
        pq.insert(0, 42);
        pq.insert(1, 17);

        pq.increase_key(1, 100);
        assert_eq!(pq.min_index(), Some(0));
    }

    #[test]
    fn test_decrease_key() {
        let mut pq = IndexMinPQ::new(10);
        pq.insert(0, 100);
        pq.insert(1, 17);

        pq.decrease_key(0, 10);
        assert_eq!(pq.min_index(), Some(0));
    }

    #[test]
    fn test_delete() {
        let mut pq = IndexMinPQ::new(10);
        pq.insert(0, 5);
        pq.insert(1, 3);
        pq.insert(2, 7);

        pq.delete(1);
        assert!(!pq.contains(1));
        assert_eq!(pq.size(), 2);
        assert_eq!(pq.min_index(), Some(0));
    }

    #[test]
    fn test_strings() {
        let mut pq = IndexMinPQ::new(5);
        pq.insert(0, "apple".to_string());
        pq.insert(1, "banana".to_string());
        pq.insert(2, "cherry".to_string());

        assert_eq!(pq.min_key(), Some(&"apple".to_string()));
        assert_eq!(pq.del_min(), Some(0));
    }

    #[test]
    fn test_complex_scenario() {
        let mut pq = IndexMinPQ::new(10);

        // Insert several items (values: 0, 10, 20, 30, 40)
        for i in 0..5 {
            pq.insert(i, i * 10);
        }

        // Min should be index 0 with value 0
        assert_eq!(pq.min_index(), Some(0));

        // Delete the minimum
        pq.delete(0);
        // Now min should be index 1 with value 10
        assert_eq!(pq.min_index(), Some(1));

        // Insert a new minimum
        pq.insert(7, 3);
        // Now min should be index 7 with value 3
        assert_eq!(pq.min_index(), Some(7));

        // Change a key to be smaller
        pq.change_key(2, 2);
        // Now min should be index 2 with value 2
        assert_eq!(pq.min_index(), Some(2));
    }
}
