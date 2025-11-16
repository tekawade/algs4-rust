//! Hash table with separate chaining.
//!
//! This implementation uses separate chaining to handle hash collisions.
//! Each bucket contains a linked list (SequentialSearchST) of key-value pairs.
//! The table automatically resizes to maintain performance.
//!
//! # Examples
//!
//! ```
//! use algs4_searching::SeparateChainingHashST;
//!
//! let mut st = SeparateChainingHashST::new();
//! st.put("apple", 1);
//! st.put("banana", 2);
//! st.put("cherry", 3);
//!
//! assert_eq!(st.get(&"banana"), Some(&2));
//! assert_eq!(st.size(), 3);
//!
//! st.delete(&"banana");
//! assert_eq!(st.get(&"banana"), None);
//! ```
//!
//! # References
//!
//! - Algorithms, 4th Edition: Section 3.4
//! - Original Java: `SeparateChainingHashST.java`

use crate::SequentialSearchST;
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};

const INIT_CAPACITY: usize = 4;

/// A symbol table implemented with a hash table using separate chaining.
///
/// This implementation uses an array of `SequentialSearchST` instances
/// to handle collisions. The table resizes dynamically to maintain
/// good performance as the load factor changes.
///
/// # Type Parameters
///
/// * `K` - The key type, must implement `Hash` and `Eq`
/// * `V` - The value type
///
/// # Performance
///
/// * Search: O(1) average case, O(n) worst case
/// * Insert: O(1) average case, O(n) worst case
/// * Delete: O(1) average case, O(n) worst case
/// * Space: O(n + m) where m is the number of chains
///
/// # Load Factor
///
/// The implementation maintains an average load factor α = n/m between
/// approximately 2 and 8 through dynamic resizing.
#[derive(Debug, Clone)]
pub struct SeparateChainingHashST<K, V> {
    chains: Vec<SequentialSearchST<K, V>>,
    n: usize, // Total number of key-value pairs
    m: usize, // Number of chains (hash table size)
}

impl<K, V> SeparateChainingHashST<K, V>
where
    K: Hash + Eq + PartialEq,
{
    /// Creates a new empty hash table.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::SeparateChainingHashST;
    ///
    /// let st: SeparateChainingHashST<String, i32> = SeparateChainingHashST::new();
    /// assert!(st.is_empty());
    /// ```
    pub fn new() -> Self {
        Self::with_capacity(INIT_CAPACITY)
    }

    /// Creates a new hash table with the specified number of chains.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::SeparateChainingHashST;
    ///
    /// let st: SeparateChainingHashST<String, i32> = SeparateChainingHashST::with_capacity(100);
    /// assert!(st.is_empty());
    /// ```
    pub fn with_capacity(m: usize) -> Self {
        let mut chains = Vec::with_capacity(m);
        for _ in 0..m {
            chains.push(SequentialSearchST::new());
        }
        SeparateChainingHashST { chains, n: 0, m }
    }

    /// Returns the number of key-value pairs in the hash table.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::SeparateChainingHashST;
    ///
    /// let mut st = SeparateChainingHashST::new();
    /// assert_eq!(st.size(), 0);
    ///
    /// st.put("key", 42);
    /// assert_eq!(st.size(), 1);
    /// ```
    pub fn size(&self) -> usize {
        self.n
    }

    /// Returns `true` if the hash table is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::SeparateChainingHashST;
    ///
    /// let mut st = SeparateChainingHashST::new();
    /// assert!(st.is_empty());
    ///
    /// st.put("key", 42);
    /// assert!(!st.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Returns `true` if the hash table contains the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::SeparateChainingHashST;
    ///
    /// let mut st = SeparateChainingHashST::new();
    /// st.put("apple", 1);
    ///
    /// assert!(st.contains(&"apple"));
    /// assert!(!st.contains(&"banana"));
    /// ```
    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Computes the hash value for the key.
    ///
    /// Maps the key to a bucket index in the range [0, m).
    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        // Use bitwise AND with max value to ensure positive,
        // then take modulo to fit in range
        (hash as usize) % self.m
    }

    /// Returns the value associated with the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::SeparateChainingHashST;
    ///
    /// let mut st = SeparateChainingHashST::new();
    /// st.put("apple", 1);
    /// st.put("banana", 2);
    ///
    /// assert_eq!(st.get(&"apple"), Some(&1));
    /// assert_eq!(st.get(&"cherry"), None);
    /// ```
    pub fn get(&self, key: &K) -> Option<&V> {
        let i = self.hash(key);
        self.chains[i].get(key)
    }

    /// Returns a mutable reference to the value associated with the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::SeparateChainingHashST;
    ///
    /// let mut st = SeparateChainingHashST::new();
    /// st.put("apple", 1);
    ///
    /// if let Some(val) = st.get_mut(&"apple") {
    ///     *val = 10;
    /// }
    ///
    /// assert_eq!(st.get(&"apple"), Some(&10));
    /// ```
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let i = self.hash(key);
        self.chains[i].get_mut(key)
    }

    /// Inserts the specified key-value pair into the hash table.
    ///
    /// If the key already exists, its value is updated. The table is
    /// resized if the load factor becomes too high.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::SeparateChainingHashST;
    ///
    /// let mut st = SeparateChainingHashST::new();
    /// st.put("apple", 1);
    /// st.put("banana", 2);
    ///
    /// assert_eq!(st.get(&"apple"), Some(&1));
    /// assert_eq!(st.size(), 2);
    /// ```
    pub fn put(&mut self, key: K, val: V)
    where
        K: Clone,
        V: Clone,
    {
        // Double table size if average chain length >= 10
        if self.n >= 10 * self.m {
            self.resize(2 * self.m);
        }

        let i = self.hash(&key);
        let old_size = self.chains[i].size();
        self.chains[i].put(key, val);
        let new_size = self.chains[i].size();

        // Update total size if this was a new key
        if new_size > old_size {
            self.n += 1;
        }
    }

    /// Removes the specified key and its associated value from the hash table.
    ///
    /// The table is resized if the load factor becomes too low.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::SeparateChainingHashST;
    ///
    /// let mut st = SeparateChainingHashST::new();
    /// st.put("apple", 1);
    /// st.put("banana", 2);
    ///
    /// st.delete(&"apple");
    /// assert_eq!(st.get(&"apple"), None);
    /// assert_eq!(st.size(), 1);
    /// ```
    pub fn delete(&mut self, key: &K)
    where
        K: Clone,
        V: Clone,
    {
        let i = self.hash(key);
        let old_size = self.chains[i].size();
        self.chains[i].delete(key);
        let new_size = self.chains[i].size();

        // Update total size if key was deleted
        if new_size < old_size {
            self.n -= 1;
        }

        // Halve table size if average chain length <= 2 (and m > INIT_CAPACITY)
        if self.m > INIT_CAPACITY && self.n <= 2 * self.m {
            self.resize(self.m / 2);
        }
    }

    /// Resizes the hash table to the specified number of chains.
    fn resize(&mut self, new_m: usize)
    where
        K: Clone + PartialEq,
        V: Clone,
    {
        let mut temp = Self::with_capacity(new_m);

        // Rehash all keys
        for chain in &self.chains {
            for key in chain.keys() {
                if let Some(val) = chain.get(key) {
                    temp.put(key.clone(), val.clone());
                }
            }
        }

        self.chains = temp.chains;
        self.m = temp.m;
    }

    /// Returns an iterator over all keys in the hash table.
    ///
    /// The keys are returned in no particular order.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::SeparateChainingHashST;
    ///
    /// let mut st = SeparateChainingHashST::new();
    /// st.put("apple", 1);
    /// st.put("banana", 2);
    /// st.put("cherry", 3);
    ///
    /// let keys = st.keys();
    /// assert_eq!(keys.len(), 3);
    /// ```
    pub fn keys(&self) -> Vec<&K>
    where
        K: PartialEq,
    {
        let mut keys = Vec::with_capacity(self.n);
        for chain in &self.chains {
            for key in chain.keys() {
                keys.push(key);
            }
        }
        keys
    }

    /// Returns the number of chains in the hash table.
    ///
    /// This is useful for analyzing the performance of the hash table.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::SeparateChainingHashST;
    ///
    /// let st: SeparateChainingHashST<String, i32> = SeparateChainingHashST::with_capacity(16);
    /// assert_eq!(st.capacity(), 16);
    /// ```
    pub fn capacity(&self) -> usize {
        self.m
    }

    /// Returns the average chain length (load factor).
    ///
    /// This is n/m where n is the number of keys and m is the number of chains.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::SeparateChainingHashST;
    ///
    /// let mut st = SeparateChainingHashST::with_capacity(4);
    /// st.put("apple", 1);
    /// st.put("banana", 2);
    /// st.put("cherry", 3);
    /// st.put("date", 4);
    ///
    /// assert_eq!(st.load_factor(), 1.0);
    /// ```
    pub fn load_factor(&self) -> f64 {
        self.n as f64 / self.m as f64
    }
}

impl<K, V> Default for SeparateChainingHashST<K, V>
where
    K: Hash + Eq,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> fmt::Display for SeparateChainingHashST<K, V>
where
    K: fmt::Display + PartialEq,
    V: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ")?;
        let mut first = true;
        for chain in &self.chains {
            for key in chain.keys() {
                if let Some(val) = chain.get(key) {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", key, val)?;
                    first = false;
                }
            }
        }
        write!(f, " }}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let st: SeparateChainingHashST<String, i32> = SeparateChainingHashST::new();
        assert!(st.is_empty());
        assert_eq!(st.size(), 0);
        assert_eq!(st.capacity(), INIT_CAPACITY);
    }

    #[test]
    fn test_put_and_get() {
        let mut st = SeparateChainingHashST::new();
        st.put("apple", 1);
        st.put("banana", 2);
        st.put("cherry", 3);

        assert_eq!(st.get(&"apple"), Some(&1));
        assert_eq!(st.get(&"banana"), Some(&2));
        assert_eq!(st.get(&"cherry"), Some(&3));
        assert_eq!(st.get(&"date"), None);
        assert_eq!(st.size(), 3);
    }

    #[test]
    fn test_put_update() {
        let mut st = SeparateChainingHashST::new();
        st.put("apple", 1);
        assert_eq!(st.get(&"apple"), Some(&1));
        assert_eq!(st.size(), 1);

        st.put("apple", 10);
        assert_eq!(st.get(&"apple"), Some(&10));
        assert_eq!(st.size(), 1); // Size should not change
    }

    #[test]
    fn test_delete() {
        let mut st = SeparateChainingHashST::new();
        st.put("apple", 1);
        st.put("banana", 2);
        st.put("cherry", 3);

        st.delete(&"banana");
        assert_eq!(st.get(&"banana"), None);
        assert_eq!(st.get(&"apple"), Some(&1));
        assert_eq!(st.get(&"cherry"), Some(&3));
        assert_eq!(st.size(), 2);
    }

    #[test]
    fn test_delete_nonexistent() {
        let mut st = SeparateChainingHashST::new();
        st.put("apple", 1);

        st.delete(&"banana");
        assert_eq!(st.size(), 1);
        assert_eq!(st.get(&"apple"), Some(&1));
    }

    #[test]
    fn test_contains() {
        let mut st = SeparateChainingHashST::new();
        st.put("apple", 1);

        assert!(st.contains(&"apple"));
        assert!(!st.contains(&"banana"));
    }

    #[test]
    fn test_get_mut() {
        let mut st = SeparateChainingHashST::new();
        st.put("apple", 1);

        if let Some(val) = st.get_mut(&"apple") {
            *val = 100;
        }

        assert_eq!(st.get(&"apple"), Some(&100));
    }

    #[test]
    fn test_keys() {
        let mut st = SeparateChainingHashST::new();
        st.put("apple", 1);
        st.put("banana", 2);
        st.put("cherry", 3);

        let keys: Vec<_> = st.keys().into_iter().copied().collect();
        assert_eq!(keys.len(), 3);

        // Order is not guaranteed in a hash table
        assert!(keys.contains(&"apple"));
        assert!(keys.contains(&"banana"));
        assert!(keys.contains(&"cherry"));
    }

    #[test]
    fn test_resize_up() {
        let mut st = SeparateChainingHashST::new();
        let initial_capacity = st.capacity();

        // Add enough elements to trigger resize
        for i in 0..100 {
            st.put(format!("key{}", i), i);
        }

        assert_eq!(st.size(), 100);
        assert!(st.capacity() > initial_capacity);

        // Verify all elements are still accessible
        for i in 0..100 {
            assert_eq!(st.get(&format!("key{}", i)), Some(&i));
        }
    }

    #[test]
    fn test_resize_down() {
        let mut st = SeparateChainingHashST::new();

        // Add many elements
        for i in 0..100 {
            st.put(format!("key{}", i), i);
        }
        let max_capacity = st.capacity();

        // Delete most of them
        for i in 0..95 {
            st.delete(&format!("key{}", i));
        }

        assert_eq!(st.size(), 5);
        assert!(st.capacity() < max_capacity);

        // Verify remaining elements are still accessible
        for i in 95..100 {
            assert_eq!(st.get(&format!("key{}", i)), Some(&i));
        }
    }

    #[test]
    fn test_load_factor() {
        let mut st = SeparateChainingHashST::with_capacity(4);
        st.put("apple", 1);
        st.put("banana", 2);
        st.put("cherry", 3);
        st.put("date", 4);

        assert_eq!(st.load_factor(), 1.0);
    }

    #[test]
    fn test_with_integers() {
        let mut st = SeparateChainingHashST::new();
        st.put(1, "one");
        st.put(2, "two");
        st.put(3, "three");

        assert_eq!(st.get(&2), Some(&"two"));
        assert_eq!(st.size(), 3);
    }

    #[test]
    fn test_collision_handling() {
        // Create a small hash table to ensure collisions
        let mut st = SeparateChainingHashST::with_capacity(2);

        // Add multiple elements (likely to cause collisions)
        for i in 0..10 {
            st.put(i, i * 10);
        }

        // Verify all elements are accessible
        for i in 0..10 {
            assert_eq!(st.get(&i), Some(&(i * 10)));
        }

        assert_eq!(st.size(), 10);
    }

    #[test]
    fn test_empty_operations() {
        let st: SeparateChainingHashST<&str, i32> = SeparateChainingHashST::new();

        assert_eq!(st.get(&"key"), None);
        assert!(!st.contains(&"key"));
        assert_eq!(st.keys().len(), 0);
    }

    #[test]
    fn test_display() {
        let mut st = SeparateChainingHashST::new();
        st.put("apple", 1);
        st.put("banana", 2);

        let display = format!("{}", st);
        assert!(display.contains("apple"));
        assert!(display.contains("banana"));
    }
}
