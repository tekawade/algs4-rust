//! Sequential search symbol table (unordered linked list).
//!
//! This is the simplest symbol table implementation using an unordered singly-linked list.
//! All operations are O(n) in the worst case, making it suitable only for small tables
//! or as a baseline for comparison.
//!
//! # Examples
//!
//! ```
//! use algs4_searching::SequentialSearchST;
//!
//! let mut st = SequentialSearchST::new();
//! st.put("apple", 1);
//! st.put("banana", 2);
//! st.put("cherry", 3);
//!
//! assert_eq!(st.get(&"banana"), Some(&2));
//! assert_eq!(st.size(), 3);
//!
//! st.delete(&"banana");
//! assert_eq!(st.get(&"banana"), None);
//! assert_eq!(st.size(), 2);
//! ```
//!
//! # References
//!
//! - Algorithms, 4th Edition: Section 3.1
//! - Original Java: `SequentialSearchST.java`

use std::fmt;

/// A symbol table implemented with a singly-linked list of key-value pairs.
///
/// This implementation uses an unordered linked list, resulting in O(n) performance
/// for all operations. It serves as a baseline for comparing more efficient implementations.
///
/// # Type Parameters
///
/// * `K` - The key type, must implement `PartialEq` for searching
/// * `V` - The value type
///
/// # Performance
///
/// * Search: O(n) worst case, O(n/2) average case
/// * Insert: O(n) (need to search for existing key)
/// * Delete: O(n)
/// * Space: O(n)
#[derive(Debug, Clone)]
pub struct SequentialSearchST<K, V> {
    head: Option<Box<Node<K, V>>>,
    n: usize,
}

#[derive(Debug, Clone)]
struct Node<K, V> {
    key: K,
    val: V,
    next: Option<Box<Node<K, V>>>,
}

impl<K, V> SequentialSearchST<K, V>
where
    K: PartialEq,
{
    /// Creates a new empty symbol table.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::SequentialSearchST;
    ///
    /// let st: SequentialSearchST<String, i32> = SequentialSearchST::new();
    /// assert!(st.is_empty());
    /// ```
    pub fn new() -> Self {
        SequentialSearchST { head: None, n: 0 }
    }

    /// Returns the number of key-value pairs in the symbol table.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::SequentialSearchST;
    ///
    /// let mut st = SequentialSearchST::new();
    /// assert_eq!(st.size(), 0);
    ///
    /// st.put("key", 42);
    /// assert_eq!(st.size(), 1);
    /// ```
    pub fn size(&self) -> usize {
        self.n
    }

    /// Returns `true` if the symbol table is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::SequentialSearchST;
    ///
    /// let mut st = SequentialSearchST::new();
    /// assert!(st.is_empty());
    ///
    /// st.put("key", 42);
    /// assert!(!st.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Returns `true` if the symbol table contains the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::SequentialSearchST;
    ///
    /// let mut st = SequentialSearchST::new();
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
    /// # Arguments
    ///
    /// * `key` - The key to search for
    ///
    /// # Returns
    ///
    /// * `Some(&V)` - A reference to the value if the key exists
    /// * `None` - If the key is not in the table
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::SequentialSearchST;
    ///
    /// let mut st = SequentialSearchST::new();
    /// st.put("apple", 1);
    /// st.put("banana", 2);
    ///
    /// assert_eq!(st.get(&"apple"), Some(&1));
    /// assert_eq!(st.get(&"cherry"), None);
    /// ```
    pub fn get(&self, key: &K) -> Option<&V> {
        let mut current = &self.head;
        while let Some(node) = current {
            if &node.key == key {
                return Some(&node.val);
            }
            current = &node.next;
        }
        None
    }

    /// Returns a mutable reference to the value associated with the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::SequentialSearchST;
    ///
    /// let mut st = SequentialSearchST::new();
    /// st.put("apple", 1);
    ///
    /// if let Some(val) = st.get_mut(&"apple") {
    ///     *val = 10;
    /// }
    ///
    /// assert_eq!(st.get(&"apple"), Some(&10));
    /// ```
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let mut current = &mut self.head;
        while let Some(node) = current {
            if &node.key == key {
                return Some(&mut node.val);
            }
            current = &mut node.next;
        }
        None
    }

    /// Inserts the specified key-value pair into the symbol table.
    ///
    /// If the key already exists, its value is updated. If the key is new,
    /// it is added at the beginning of the linked list.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to insert
    /// * `val` - The value to associate with the key
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::SequentialSearchST;
    ///
    /// let mut st = SequentialSearchST::new();
    /// st.put("apple", 1);
    /// st.put("banana", 2);
    ///
    /// assert_eq!(st.get(&"apple"), Some(&1));
    ///
    /// st.put("apple", 10);  // Update existing key
    /// assert_eq!(st.get(&"apple"), Some(&10));
    /// ```
    pub fn put(&mut self, key: K, val: V) {
        // Search for existing key
        let mut current = &mut self.head;
        while let Some(node) = current {
            if node.key == key {
                node.val = val;
                return;
            }
            current = &mut node.next;
        }

        // Key not found, add new node at beginning
        let new_node = Box::new(Node {
            key,
            val,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.n += 1;
    }

    /// Removes the specified key and its associated value from the symbol table.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to remove
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::SequentialSearchST;
    ///
    /// let mut st = SequentialSearchST::new();
    /// st.put("apple", 1);
    /// st.put("banana", 2);
    ///
    /// st.delete(&"apple");
    /// assert_eq!(st.get(&"apple"), None);
    /// assert_eq!(st.size(), 1);
    /// ```
    pub fn delete(&mut self, key: &K) {
        self.head = Self::delete_helper(self.head.take(), key, &mut self.n);
    }

    fn delete_helper(
        node: Option<Box<Node<K, V>>>,
        key: &K,
        size: &mut usize,
    ) -> Option<Box<Node<K, V>>> {
        if let Some(mut n) = node {
            if &n.key == key {
                *size -= 1;
                return n.next;
            }
            n.next = Self::delete_helper(n.next, key, size);
            Some(n)
        } else {
            None
        }
    }

    /// Returns an iterator over all keys in the symbol table.
    ///
    /// The keys are returned in no particular order (unordered).
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::SequentialSearchST;
    ///
    /// let mut st = SequentialSearchST::new();
    /// st.put("apple", 1);
    /// st.put("banana", 2);
    /// st.put("cherry", 3);
    ///
    /// let keys: Vec<_> = st.keys().collect();
    /// assert_eq!(keys.len(), 3);
    /// ```
    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys {
            current: &self.head,
        }
    }
}

impl<K, V> Default for SequentialSearchST<K, V>
where
    K: PartialEq,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> fmt::Display for SequentialSearchST<K, V>
where
    K: fmt::Display,
    V: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ ")?;
        let mut current = &self.head;
        let mut first = true;
        while let Some(node) = current {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", node.key, node.val)?;
            current = &node.next;
            first = false;
        }
        write!(f, " }}")
    }
}

/// An iterator over the keys of a `SequentialSearchST`.
#[derive(Debug)]
pub struct Keys<'a, K, V> {
    current: &'a Option<Box<Node<K, V>>>,
}

impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.current {
            self.current = &node.next;
            Some(&node.key)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let st: SequentialSearchST<String, i32> = SequentialSearchST::new();
        assert!(st.is_empty());
        assert_eq!(st.size(), 0);
    }

    #[test]
    fn test_put_and_get() {
        let mut st = SequentialSearchST::new();
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
        let mut st = SequentialSearchST::new();
        st.put("apple", 1);
        assert_eq!(st.get(&"apple"), Some(&1));
        assert_eq!(st.size(), 1);

        st.put("apple", 10);
        assert_eq!(st.get(&"apple"), Some(&10));
        assert_eq!(st.size(), 1); // Size should not change
    }

    #[test]
    fn test_delete() {
        let mut st = SequentialSearchST::new();
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
        let mut st = SequentialSearchST::new();
        st.put("apple", 1);

        st.delete(&"banana");
        assert_eq!(st.size(), 1);
        assert_eq!(st.get(&"apple"), Some(&1));
    }

    #[test]
    fn test_delete_all() {
        let mut st = SequentialSearchST::new();
        st.put("apple", 1);
        st.put("banana", 2);

        st.delete(&"apple");
        st.delete(&"banana");
        assert!(st.is_empty());
        assert_eq!(st.size(), 0);
    }

    #[test]
    fn test_contains() {
        let mut st = SequentialSearchST::new();
        st.put("apple", 1);

        assert!(st.contains(&"apple"));
        assert!(!st.contains(&"banana"));
    }

    #[test]
    fn test_get_mut() {
        let mut st = SequentialSearchST::new();
        st.put("apple", 1);

        if let Some(val) = st.get_mut(&"apple") {
            *val = 100;
        }

        assert_eq!(st.get(&"apple"), Some(&100));
    }

    #[test]
    fn test_keys() {
        let mut st = SequentialSearchST::new();
        st.put("apple", 1);
        st.put("banana", 2);
        st.put("cherry", 3);

        let keys: Vec<_> = st.keys().copied().collect();
        assert_eq!(keys.len(), 3);

        // Note: order is not guaranteed (depends on insertion order and updates)
        assert!(keys.contains(&"apple"));
        assert!(keys.contains(&"banana"));
        assert!(keys.contains(&"cherry"));
    }

    #[test]
    fn test_display() {
        let mut st = SequentialSearchST::new();
        st.put("apple", 1);
        st.put("banana", 2);

        let display = format!("{}", st);
        assert!(display.contains("apple"));
        assert!(display.contains("banana"));
    }

    #[test]
    fn test_with_integers() {
        let mut st = SequentialSearchST::new();
        st.put(1, "one");
        st.put(2, "two");
        st.put(3, "three");

        assert_eq!(st.get(&2), Some(&"two"));
        assert_eq!(st.size(), 3);
    }

    #[test]
    fn test_empty_operations() {
        let st: SequentialSearchST<&str, i32> = SequentialSearchST::new();

        assert_eq!(st.get(&"key"), None);
        assert!(!st.contains(&"key"));
        assert_eq!(st.keys().count(), 0);
    }
}
