//! Binary search symbol table (ordered array-based).
//!
//! This symbol table implementation uses two parallel sorted arrays for keys and values.
//! Binary search provides O(log n) search time, but insertions and deletions require
//! array shifting, resulting in O(n) time for those operations.
//!
//! # Examples
//!
//! ```
//! use algs4_searching::BinarySearchST;
//!
//! let mut st = BinarySearchST::new();
//! st.put("apple", 1);
//! st.put("banana", 2);
//! st.put("cherry", 3);
//!
//! assert_eq!(st.get(&"banana"), Some(&2));
//! assert_eq!(st.min(), Some(&"apple"));
//! assert_eq!(st.max(), Some(&"cherry"));
//! assert_eq!(st.rank(&"banana"), 1);
//! ```
//!
//! # References
//!
//! - Algorithms, 4th Edition: Section 3.1
//! - Original Java: `BinarySearchST.java`

use std::fmt;

/// An ordered symbol table implemented with parallel sorted arrays.
///
/// This implementation maintains keys in sorted order, enabling efficient
/// binary search and ordered operations like min, max, floor, and ceiling.
///
/// # Type Parameters
///
/// * `K` - The key type, must implement `Ord` for ordering
/// * `V` - The value type
///
/// # Performance
///
/// * Search: O(log n)
/// * Insert: O(n) (requires array shifting)
/// * Delete: O(n) (requires array shifting)
/// * Min/Max: O(1)
/// * Floor/Ceiling: O(log n)
/// * Space: O(n)
#[derive(Debug, Clone)]
pub struct BinarySearchST<K, V> {
    keys: Vec<K>,
    vals: Vec<V>,
}

impl<K, V> BinarySearchST<K, V>
where
    K: Ord,
{
    /// Creates a new empty ordered symbol table.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BinarySearchST;
    ///
    /// let st: BinarySearchST<String, i32> = BinarySearchST::new();
    /// assert!(st.is_empty());
    /// ```
    pub fn new() -> Self {
        BinarySearchST {
            keys: Vec::new(),
            vals: Vec::new(),
        }
    }

    /// Creates a new ordered symbol table with the specified capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BinarySearchST;
    ///
    /// let st: BinarySearchST<String, i32> = BinarySearchST::with_capacity(100);
    /// assert!(st.is_empty());
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        BinarySearchST {
            keys: Vec::with_capacity(capacity),
            vals: Vec::with_capacity(capacity),
        }
    }

    /// Returns the number of key-value pairs in the symbol table.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BinarySearchST;
    ///
    /// let mut st = BinarySearchST::new();
    /// assert_eq!(st.size(), 0);
    ///
    /// st.put("key", 42);
    /// assert_eq!(st.size(), 1);
    /// ```
    pub fn size(&self) -> usize {
        self.keys.len()
    }

    /// Returns `true` if the symbol table is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BinarySearchST;
    ///
    /// let mut st = BinarySearchST::new();
    /// assert!(st.is_empty());
    ///
    /// st.put("key", 42);
    /// assert!(!st.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    /// Returns `true` if the symbol table contains the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BinarySearchST;
    ///
    /// let mut st = BinarySearchST::new();
    /// st.put("apple", 1);
    ///
    /// assert!(st.contains(&"apple"));
    /// assert!(!st.contains(&"banana"));
    /// ```
    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Returns the value associated with the specified key.
    ///
    /// Uses binary search to find the key in O(log n) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BinarySearchST;
    ///
    /// let mut st = BinarySearchST::new();
    /// st.put("apple", 1);
    /// st.put("banana", 2);
    ///
    /// assert_eq!(st.get(&"apple"), Some(&1));
    /// assert_eq!(st.get(&"cherry"), None);
    /// ```
    pub fn get(&self, key: &K) -> Option<&V> {
        if self.is_empty() {
            return None;
        }
        let i = self.rank(key);
        if i < self.size() && &self.keys[i] == key {
            Some(&self.vals[i])
        } else {
            None
        }
    }

    /// Returns a mutable reference to the value associated with the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BinarySearchST;
    ///
    /// let mut st = BinarySearchST::new();
    /// st.put("apple", 1);
    ///
    /// if let Some(val) = st.get_mut(&"apple") {
    ///     *val = 10;
    /// }
    ///
    /// assert_eq!(st.get(&"apple"), Some(&10));
    /// ```
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if self.is_empty() {
            return None;
        }
        let i = self.rank(key);
        if i < self.size() && &self.keys[i] == key {
            Some(&mut self.vals[i])
        } else {
            None
        }
    }

    /// Returns the number of keys strictly less than the specified key.
    ///
    /// This is also the index where the key would be inserted.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BinarySearchST;
    ///
    /// let mut st = BinarySearchST::new();
    /// st.put("apple", 1);
    /// st.put("banana", 2);
    /// st.put("date", 3);
    ///
    /// assert_eq!(st.rank(&"apple"), 0);
    /// assert_eq!(st.rank(&"banana"), 1);
    /// assert_eq!(st.rank(&"cherry"), 2);  // Between banana and date
    /// assert_eq!(st.rank(&"fig"), 3);     // After all keys
    /// ```
    pub fn rank(&self, key: &K) -> usize {
        let mut lo = 0;
        let mut hi = self.size();

        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            match key.cmp(&self.keys[mid]) {
                std::cmp::Ordering::Less => hi = mid,
                std::cmp::Ordering::Greater => lo = mid + 1,
                std::cmp::Ordering::Equal => return mid,
            }
        }
        lo
    }

    /// Inserts the specified key-value pair into the symbol table.
    ///
    /// If the key already exists, its value is updated. If the key is new,
    /// it is inserted at the appropriate position to maintain sorted order.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BinarySearchST;
    ///
    /// let mut st = BinarySearchST::new();
    /// st.put("banana", 2);
    /// st.put("apple", 1);
    /// st.put("cherry", 3);
    ///
    /// // Keys are kept in sorted order
    /// assert_eq!(st.select(0), Some(&"apple"));
    /// assert_eq!(st.select(1), Some(&"banana"));
    /// assert_eq!(st.select(2), Some(&"cherry"));
    /// ```
    pub fn put(&mut self, key: K, val: V) {
        let i = self.rank(&key);

        // Key already exists, update value
        if i < self.size() && self.keys[i] == key {
            self.vals[i] = val;
            return;
        }

        // Insert new key-value pair
        self.keys.insert(i, key);
        self.vals.insert(i, val);
    }

    /// Removes the specified key and its associated value from the symbol table.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BinarySearchST;
    ///
    /// let mut st = BinarySearchST::new();
    /// st.put("apple", 1);
    /// st.put("banana", 2);
    /// st.put("cherry", 3);
    ///
    /// st.delete(&"banana");
    /// assert_eq!(st.get(&"banana"), None);
    /// assert_eq!(st.size(), 2);
    /// ```
    pub fn delete(&mut self, key: &K) {
        if self.is_empty() {
            return;
        }

        let i = self.rank(key);
        if i < self.size() && &self.keys[i] == key {
            self.keys.remove(i);
            self.vals.remove(i);
        }
    }

    /// Returns the smallest key in the symbol table.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BinarySearchST;
    ///
    /// let mut st = BinarySearchST::new();
    /// st.put("banana", 2);
    /// st.put("apple", 1);
    /// st.put("cherry", 3);
    ///
    /// assert_eq!(st.min(), Some(&"apple"));
    /// ```
    pub fn min(&self) -> Option<&K> {
        self.keys.first()
    }

    /// Returns the largest key in the symbol table.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BinarySearchST;
    ///
    /// let mut st = BinarySearchST::new();
    /// st.put("banana", 2);
    /// st.put("apple", 1);
    /// st.put("cherry", 3);
    ///
    /// assert_eq!(st.max(), Some(&"cherry"));
    /// ```
    pub fn max(&self) -> Option<&K> {
        self.keys.last()
    }

    /// Returns the key of rank k (the k-th smallest key).
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BinarySearchST;
    ///
    /// let mut st = BinarySearchST::new();
    /// st.put("banana", 2);
    /// st.put("apple", 1);
    /// st.put("cherry", 3);
    ///
    /// assert_eq!(st.select(0), Some(&"apple"));
    /// assert_eq!(st.select(1), Some(&"banana"));
    /// assert_eq!(st.select(2), Some(&"cherry"));
    /// assert_eq!(st.select(3), None);
    /// ```
    pub fn select(&self, k: usize) -> Option<&K> {
        self.keys.get(k)
    }

    /// Returns the largest key less than or equal to the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BinarySearchST;
    ///
    /// let mut st = BinarySearchST::new();
    /// st.put("apple", 1);
    /// st.put("banana", 2);
    /// st.put("date", 3);
    ///
    /// assert_eq!(st.floor(&"cherry"), Some(&"banana"));
    /// assert_eq!(st.floor(&"banana"), Some(&"banana"));
    /// assert_eq!(st.floor(&"aaa"), None);
    /// ```
    pub fn floor(&self, key: &K) -> Option<&K> {
        let i = self.rank(key);
        if i < self.size() && &self.keys[i] == key {
            Some(&self.keys[i])
        } else if i == 0 {
            None
        } else {
            Some(&self.keys[i - 1])
        }
    }

    /// Returns the smallest key greater than or equal to the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BinarySearchST;
    ///
    /// let mut st = BinarySearchST::new();
    /// st.put("apple", 1);
    /// st.put("banana", 2);
    /// st.put("date", 3);
    ///
    /// assert_eq!(st.ceiling(&"cherry"), Some(&"date"));
    /// assert_eq!(st.ceiling(&"banana"), Some(&"banana"));
    /// assert_eq!(st.ceiling(&"fig"), None);
    /// ```
    pub fn ceiling(&self, key: &K) -> Option<&K> {
        let i = self.rank(key);
        self.keys.get(i)
    }

    /// Removes the smallest key and its associated value.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BinarySearchST;
    ///
    /// let mut st = BinarySearchST::new();
    /// st.put("banana", 2);
    /// st.put("apple", 1);
    /// st.put("cherry", 3);
    ///
    /// st.delete_min();
    /// assert_eq!(st.min(), Some(&"banana"));
    /// assert_eq!(st.size(), 2);
    /// ```
    pub fn delete_min(&mut self) {
        if !self.is_empty() {
            self.keys.remove(0);
            self.vals.remove(0);
        }
    }

    /// Removes the largest key and its associated value.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BinarySearchST;
    ///
    /// let mut st = BinarySearchST::new();
    /// st.put("banana", 2);
    /// st.put("apple", 1);
    /// st.put("cherry", 3);
    ///
    /// st.delete_max();
    /// assert_eq!(st.max(), Some(&"banana"));
    /// assert_eq!(st.size(), 2);
    /// ```
    pub fn delete_max(&mut self) {
        if !self.is_empty() {
            self.keys.pop();
            self.vals.pop();
        }
    }

    /// Returns the number of keys in the range `[lo, hi]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BinarySearchST;
    ///
    /// let mut st = BinarySearchST::new();
    /// st.put("apple", 1);
    /// st.put("banana", 2);
    /// st.put("cherry", 3);
    /// st.put("date", 4);
    ///
    /// assert_eq!(st.size_range(&"banana", &"cherry"), 2);
    /// assert_eq!(st.size_range(&"apple", &"date"), 4);
    /// ```
    pub fn size_range(&self, lo: &K, hi: &K) -> usize {
        if hi < lo {
            return 0;
        }
        let lo_rank = self.rank(lo);
        let hi_rank = self.rank(hi);
        if self.contains(hi) {
            hi_rank - lo_rank + 1
        } else {
            hi_rank - lo_rank
        }
    }

    /// Returns an iterator over all keys in the symbol table in sorted order.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BinarySearchST;
    ///
    /// let mut st = BinarySearchST::new();
    /// st.put("banana", 2);
    /// st.put("apple", 1);
    /// st.put("cherry", 3);
    ///
    /// let keys: Vec<_> = st.keys().collect();
    /// assert_eq!(keys, vec![&"apple", &"banana", &"cherry"]);
    /// ```
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.keys.iter()
    }

    /// Returns an iterator over keys in the range `[lo, hi]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BinarySearchST;
    ///
    /// let mut st = BinarySearchST::new();
    /// st.put("apple", 1);
    /// st.put("banana", 2);
    /// st.put("cherry", 3);
    /// st.put("date", 4);
    /// st.put("fig", 5);
    ///
    /// let keys: Vec<_> = st.range(&"banana", &"date").collect();
    /// assert_eq!(keys, vec![&"banana", &"cherry", &"date"]);
    /// ```
    pub fn range(&self, lo: &K, hi: &K) -> impl Iterator<Item = &K> {
        let lo_rank = self.rank(lo);
        let hi_rank = self.rank(hi);
        let end = if self.contains(hi) {
            hi_rank + 1
        } else {
            hi_rank
        };
        self.keys[lo_rank..end].iter()
    }
}

impl<K, V> Default for BinarySearchST<K, V>
where
    K: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> fmt::Display for BinarySearchST<K, V>
where
    K: fmt::Display,
    V: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ")?;
        for (i, (k, v)) in self.keys.iter().zip(self.vals.iter()).enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", k, v)?;
        }
        write!(f, " }}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let st: BinarySearchST<String, i32> = BinarySearchST::new();
        assert!(st.is_empty());
        assert_eq!(st.size(), 0);
    }

    #[test]
    fn test_put_and_get() {
        let mut st = BinarySearchST::new();
        st.put("banana", 2);
        st.put("apple", 1);
        st.put("cherry", 3);

        assert_eq!(st.get(&"apple"), Some(&1));
        assert_eq!(st.get(&"banana"), Some(&2));
        assert_eq!(st.get(&"cherry"), Some(&3));
        assert_eq!(st.get(&"date"), None);
        assert_eq!(st.size(), 3);
    }

    #[test]
    fn test_ordered_keys() {
        let mut st = BinarySearchST::new();
        st.put("banana", 2);
        st.put("apple", 1);
        st.put("cherry", 3);

        let keys: Vec<_> = st.keys().copied().collect();
        assert_eq!(keys, vec!["apple", "banana", "cherry"]);
    }

    #[test]
    fn test_rank() {
        let mut st = BinarySearchST::new();
        st.put("apple", 1);
        st.put("banana", 2);
        st.put("date", 4);

        assert_eq!(st.rank(&"apple"), 0);
        assert_eq!(st.rank(&"banana"), 1);
        assert_eq!(st.rank(&"cherry"), 2); // Between banana and date
        assert_eq!(st.rank(&"date"), 2);
        assert_eq!(st.rank(&"fig"), 3);
    }

    #[test]
    fn test_min_max() {
        let mut st = BinarySearchST::new();
        st.put("banana", 2);
        st.put("apple", 1);
        st.put("cherry", 3);

        assert_eq!(st.min(), Some(&"apple"));
        assert_eq!(st.max(), Some(&"cherry"));
    }

    #[test]
    fn test_select() {
        let mut st = BinarySearchST::new();
        st.put("banana", 2);
        st.put("apple", 1);
        st.put("cherry", 3);

        assert_eq!(st.select(0), Some(&"apple"));
        assert_eq!(st.select(1), Some(&"banana"));
        assert_eq!(st.select(2), Some(&"cherry"));
        assert_eq!(st.select(3), None);
    }

    #[test]
    fn test_floor() {
        let mut st = BinarySearchST::new();
        st.put("apple", 1);
        st.put("banana", 2);
        st.put("date", 4);

        assert_eq!(st.floor(&"cherry"), Some(&"banana"));
        assert_eq!(st.floor(&"banana"), Some(&"banana"));
        assert_eq!(st.floor(&"aaa"), None);
        assert_eq!(st.floor(&"zzz"), Some(&"date"));
    }

    #[test]
    fn test_ceiling() {
        let mut st = BinarySearchST::new();
        st.put("apple", 1);
        st.put("banana", 2);
        st.put("date", 4);

        assert_eq!(st.ceiling(&"cherry"), Some(&"date"));
        assert_eq!(st.ceiling(&"banana"), Some(&"banana"));
        assert_eq!(st.ceiling(&"aaa"), Some(&"apple"));
        assert_eq!(st.ceiling(&"zzz"), None);
    }

    #[test]
    fn test_delete() {
        let mut st = BinarySearchST::new();
        st.put("apple", 1);
        st.put("banana", 2);
        st.put("cherry", 3);

        st.delete(&"banana");
        assert_eq!(st.get(&"banana"), None);
        assert_eq!(st.size(), 2);

        let keys: Vec<_> = st.keys().copied().collect();
        assert_eq!(keys, vec!["apple", "cherry"]);
    }

    #[test]
    fn test_delete_min_max() {
        let mut st = BinarySearchST::new();
        st.put("banana", 2);
        st.put("apple", 1);
        st.put("cherry", 3);

        st.delete_min();
        assert_eq!(st.min(), Some(&"banana"));
        assert_eq!(st.size(), 2);

        st.delete_max();
        assert_eq!(st.max(), Some(&"banana"));
        assert_eq!(st.size(), 1);
    }

    #[test]
    fn test_size_range() {
        let mut st = BinarySearchST::new();
        st.put("apple", 1);
        st.put("banana", 2);
        st.put("cherry", 3);
        st.put("date", 4);

        assert_eq!(st.size_range(&"banana", &"cherry"), 2);
        assert_eq!(st.size_range(&"apple", &"date"), 4);
        assert_eq!(st.size_range(&"cherry", &"banana"), 0);
    }

    #[test]
    fn test_range() {
        let mut st = BinarySearchST::new();
        st.put("apple", 1);
        st.put("banana", 2);
        st.put("cherry", 3);
        st.put("date", 4);
        st.put("fig", 5);

        let keys: Vec<_> = st.range(&"banana", &"date").collect();
        assert_eq!(keys, vec![&"banana", &"cherry", &"date"]);

        let keys: Vec<_> = st.range(&"apple", &"cherry").collect();
        assert_eq!(keys, vec![&"apple", &"banana", &"cherry"]);
    }

    #[test]
    fn test_put_update() {
        let mut st = BinarySearchST::new();
        st.put("apple", 1);
        assert_eq!(st.get(&"apple"), Some(&1));
        assert_eq!(st.size(), 1);

        st.put("apple", 10);
        assert_eq!(st.get(&"apple"), Some(&10));
        assert_eq!(st.size(), 1);
    }

    #[test]
    fn test_with_integers() {
        let mut st = BinarySearchST::new();
        st.put(5, "five");
        st.put(1, "one");
        st.put(9, "nine");
        st.put(3, "three");

        assert_eq!(st.min(), Some(&1));
        assert_eq!(st.max(), Some(&9));
        assert_eq!(st.select(1), Some(&3));
    }

    #[test]
    fn test_display() {
        let mut st = BinarySearchST::new();
        st.put("apple", 1);
        st.put("banana", 2);

        let display = format!("{}", st);
        assert!(display.contains("apple"));
        assert!(display.contains("banana"));
    }
}
