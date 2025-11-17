//! Hash table with linear probing (open addressing).
//!
//! This implementation uses linear probing to handle hash collisions.
//! All key-value pairs are stored directly in parallel arrays, and collisions
//! are resolved by probing the next available slot.
//!
//! # Examples
//!
//! ```
//! use algs4_searching::LinearProbingHashST;
//!
//! let mut st = LinearProbingHashST::new();
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
//! - Original Java: `LinearProbingHashST.java`

use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};

const INIT_CAPACITY: usize = 4;

/// A symbol table implemented with a hash table using linear probing.
///
/// This implementation uses open addressing with linear probing to resolve
/// collisions. The table resizes dynamically to maintain good performance
/// as the load factor changes.
///
/// # Type Parameters
///
/// * `K` - The key type, must implement `Hash` and `Eq`
/// * `V` - The value type
///
/// # Performance
///
/// * Search: O(1) average case (with low load factor)
/// * Insert: O(1) average case (with low load factor)
/// * Delete: O(1) average case (with low load factor)
/// * Space: O(m) where m is the table size
///
/// # Load Factor
///
/// The implementation maintains a load factor α = n/m between 0.125 and 0.5
/// through dynamic resizing.
#[derive(Debug, Clone)]
pub struct LinearProbingHashST<K, V> {
    keys: Vec<Option<K>>,
    vals: Vec<Option<V>>,
    n: usize, // Number of key-value pairs
    m: usize, // Hash table size
}

impl<K, V> LinearProbingHashST<K, V>
where
    K: Hash + Eq,
{
    /// Creates a new empty hash table.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::LinearProbingHashST;
    ///
    /// let st: LinearProbingHashST<String, i32> = LinearProbingHashST::new();
    /// assert!(st.is_empty());
    /// ```
    pub fn new() -> Self {
        Self::with_capacity(INIT_CAPACITY)
    }

    /// Creates a new hash table with the specified capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::LinearProbingHashST;
    ///
    /// let st: LinearProbingHashST<String, i32> = LinearProbingHashST::with_capacity(100);
    /// assert!(st.is_empty());
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        let m = capacity;
        let mut keys = Vec::with_capacity(m);
        let mut vals = Vec::with_capacity(m);
        for _ in 0..m {
            keys.push(None);
            vals.push(None);
        }
        LinearProbingHashST {
            keys,
            vals,
            n: 0,
            m,
        }
    }

    /// Returns the number of key-value pairs in the hash table.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::LinearProbingHashST;
    ///
    /// let mut st = LinearProbingHashST::new();
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
    /// use algs4_searching::LinearProbingHashST;
    ///
    /// let mut st = LinearProbingHashST::new();
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
    /// use algs4_searching::LinearProbingHashST;
    ///
    /// let mut st = LinearProbingHashST::new();
    /// st.put("apple", 1);
    ///
    /// assert!(st.contains(&"apple"));
    /// assert!(!st.contains(&"banana"));
    /// ```
    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Computes the hash value for the key.
    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        (hash as usize) % self.m
    }

    /// Returns the value associated with the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::LinearProbingHashST;
    ///
    /// let mut st = LinearProbingHashST::new();
    /// st.put("apple", 1);
    /// st.put("banana", 2);
    ///
    /// assert_eq!(st.get(&"apple"), Some(&1));
    /// assert_eq!(st.get(&"cherry"), None);
    /// ```
    pub fn get(&self, key: &K) -> Option<&V> {
        let mut i = self.hash(key);
        while self.keys[i].is_some() {
            if self.keys[i].as_ref() == Some(key) {
                return self.vals[i].as_ref();
            }
            i = (i + 1) % self.m;
        }
        None
    }

    /// Inserts the specified key-value pair into the hash table.
    ///
    /// If the key already exists, its value is updated. The table is
    /// resized if the load factor becomes too high.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::LinearProbingHashST;
    ///
    /// let mut st = LinearProbingHashST::new();
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
        // Double table size if load factor >= 0.5
        if self.n >= self.m / 2 {
            self.resize(2 * self.m);
        }

        let mut i = self.hash(&key);
        while self.keys[i].is_some() {
            if self.keys[i].as_ref() == Some(&key) {
                self.vals[i] = Some(val);
                return;
            }
            i = (i + 1) % self.m;
        }

        self.keys[i] = Some(key);
        self.vals[i] = Some(val);
        self.n += 1;
    }

    /// Removes the specified key and its associated value from the hash table.
    ///
    /// After deletion, rehashes all keys in the same cluster to ensure
    /// that future lookups work correctly.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::LinearProbingHashST;
    ///
    /// let mut st = LinearProbingHashST::new();
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
        if !self.contains(key) {
            return;
        }

        // Find position of key
        let mut i = self.hash(key);
        while self.keys[i].as_ref() != Some(key) {
            i = (i + 1) % self.m;
        }

        // Delete key and value
        self.keys[i] = None;
        self.vals[i] = None;

        // Rehash all keys in same cluster
        i = (i + 1) % self.m;
        while self.keys[i].is_some() {
            let key_to_rehash = self.keys[i].take().unwrap();
            let val_to_rehash = self.vals[i].take().unwrap();

            // Reinsert without modifying the counter
            let mut j = self.hash(&key_to_rehash);
            while self.keys[j].is_some() {
                j = (j + 1) % self.m;
            }
            self.keys[j] = Some(key_to_rehash);
            self.vals[j] = Some(val_to_rehash);

            i = (i + 1) % self.m;
        }

        self.n -= 1;

        // Halve table size if load factor <= 0.125
        if self.n > 0 && self.n <= self.m / 8 {
            self.resize(self.m / 2);
        }
    }

    /// Resizes the hash table to the specified capacity.
    fn resize(&mut self, capacity: usize)
    where
        K: Clone,
        V: Clone,
    {
        let mut temp = Self::with_capacity(capacity);

        for i in 0..self.m {
            if let Some(key) = self.keys[i].take() {
                if let Some(val) = self.vals[i].take() {
                    temp.put(key, val);
                }
            }
        }

        self.keys = temp.keys;
        self.vals = temp.vals;
        self.m = temp.m;
    }

    /// Returns an iterator over all keys in the hash table.
    ///
    /// The keys are returned in no particular order.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::LinearProbingHashST;
    ///
    /// let mut st = LinearProbingHashST::new();
    /// st.put("apple", 1);
    /// st.put("banana", 2);
    /// st.put("cherry", 3);
    ///
    /// let keys = st.keys();
    /// assert_eq!(keys.len(), 3);
    /// ```
    pub fn keys(&self) -> Vec<&K> {
        let mut keys = Vec::with_capacity(self.n);
        for k in self.keys.iter().flatten() {
            keys.push(k);
        }
        keys
    }

    /// Returns the capacity of the hash table.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::LinearProbingHashST;
    ///
    /// let st: LinearProbingHashST<String, i32> = LinearProbingHashST::with_capacity(16);
    /// assert_eq!(st.capacity(), 16);
    /// ```
    pub fn capacity(&self) -> usize {
        self.m
    }

    /// Returns the current load factor (n/m).
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::LinearProbingHashST;
    ///
    /// let mut st = LinearProbingHashST::with_capacity(10);
    /// st.put("apple", 1);
    /// st.put("banana", 2);
    ///
    /// assert_eq!(st.load_factor(), 0.2);
    /// ```
    pub fn load_factor(&self) -> f64 {
        self.n as f64 / self.m as f64
    }
}

impl<K, V> Default for LinearProbingHashST<K, V>
where
    K: Hash + Eq,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> fmt::Display for LinearProbingHashST<K, V>
where
    K: fmt::Display,
    V: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ")?;
        let mut first = true;
        for i in 0..self.m {
            if let Some(ref key) = self.keys[i] {
                if let Some(ref val) = self.vals[i] {
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
        let st: LinearProbingHashST<String, i32> = LinearProbingHashST::new();
        assert!(st.is_empty());
        assert_eq!(st.size(), 0);
    }

    #[test]
    fn test_put_and_get() {
        let mut st = LinearProbingHashST::new();
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
        let mut st = LinearProbingHashST::new();
        st.put("apple", 1);
        assert_eq!(st.get(&"apple"), Some(&1));
        assert_eq!(st.size(), 1);

        st.put("apple", 10);
        assert_eq!(st.get(&"apple"), Some(&10));
        assert_eq!(st.size(), 1);
    }

    #[test]
    fn test_delete() {
        let mut st = LinearProbingHashST::new();
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
        let mut st = LinearProbingHashST::new();
        st.put("apple", 1);

        st.delete(&"banana");
        assert_eq!(st.size(), 1);
        assert_eq!(st.get(&"apple"), Some(&1));
    }

    #[test]
    fn test_contains() {
        let mut st = LinearProbingHashST::new();
        st.put("apple", 1);

        assert!(st.contains(&"apple"));
        assert!(!st.contains(&"banana"));
    }

    #[test]
    fn test_keys() {
        let mut st = LinearProbingHashST::new();
        st.put("apple", 1);
        st.put("banana", 2);
        st.put("cherry", 3);

        let keys: Vec<_> = st.keys().into_iter().copied().collect();
        assert_eq!(keys.len(), 3);

        assert!(keys.contains(&"apple"));
        assert!(keys.contains(&"banana"));
        assert!(keys.contains(&"cherry"));
    }

    #[test]
    fn test_resize_up() {
        let mut st = LinearProbingHashST::new();
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
        let mut st = LinearProbingHashST::new();

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
        let mut st = LinearProbingHashST::with_capacity(10);
        st.put("apple", 1);
        st.put("banana", 2);

        assert_eq!(st.load_factor(), 0.2);
    }

    #[test]
    fn test_with_integers() {
        let mut st = LinearProbingHashST::new();
        st.put(1, "one");
        st.put(2, "two");
        st.put(3, "three");

        assert_eq!(st.get(&2), Some(&"two"));
        assert_eq!(st.size(), 3);
    }

    #[test]
    fn test_collision_handling() {
        let mut st = LinearProbingHashST::with_capacity(4);

        // Add multiple elements (likely to cause collisions in small table)
        for i in 0..3 {
            st.put(i, i * 10);
        }

        // Verify all elements are accessible
        for i in 0..3 {
            assert_eq!(st.get(&i), Some(&(i * 10)));
        }

        assert_eq!(st.size(), 3);
    }

    #[test]
    fn test_delete_with_cluster() {
        let mut st = LinearProbingHashST::with_capacity(8);

        // Create a cluster
        st.put("a", 1);
        st.put("b", 2);
        st.put("c", 3);
        st.put("d", 4);

        // Delete from middle of cluster
        st.delete(&"b");

        // Ensure other elements still accessible
        assert_eq!(st.get(&"a"), Some(&1));
        assert_eq!(st.get(&"c"), Some(&3));
        assert_eq!(st.get(&"d"), Some(&4));
        assert_eq!(st.get(&"b"), None);
    }

    #[test]
    fn test_empty_operations() {
        let st: LinearProbingHashST<&str, i32> = LinearProbingHashST::new();

        assert_eq!(st.get(&"key"), None);
        assert!(!st.contains(&"key"));
        assert_eq!(st.keys().len(), 0);
    }

    #[test]
    fn test_display() {
        let mut st = LinearProbingHashST::new();
        st.put("apple", 1);
        st.put("banana", 2);

        let display = format!("{}", st);
        assert!(display.contains("apple"));
        assert!(display.contains("banana"));
    }
}
