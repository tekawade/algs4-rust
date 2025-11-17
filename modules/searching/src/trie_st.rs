//! R-way trie symbol table for string keys.
//!
//! A trie is a tree-based data structure where each node has R children
//! (one for each possible character). Keys are stored implicitly along
//! paths from the root, making it ideal for prefix-based operations.
//!
//! # Examples
//!
//! ```
//! use algs4_searching::TrieST;
//!
//! let mut trie = TrieST::new();
//! trie.put("she", 1);
//! trie.put("shells", 2);
//! trie.put("shore", 3);
//!
//! assert_eq!(trie.get("she"), Some(&1));
//! assert_eq!(trie.keys_with_prefix("sh").len(), 3);
//! ```
//!
//! # References
//!
//! - Algorithms, 4th Edition: Section 5.2
//! - Original Java: `TrieST.java`

use std::fmt;

const R: usize = 256; // Extended ASCII

/// A symbol table for string keys using an R-way trie.
///
/// This implementation uses a multiway tree where each node has R children.
/// It's particularly efficient for string keys and supports prefix operations
/// that are difficult or impossible with other symbol table implementations.
///
/// # Type Parameters
///
/// * `V` - The value type
///
/// # Performance
///
/// * Search: O(L) where L is the key length
/// * Insert: O(L) where L is the key length
/// * Delete: O(L) where L is the key length
/// * Prefix operations: O(L + number of matching keys)
/// * Space: O(RNL) worst case, much better in practice
#[derive(Debug, Clone)]
pub struct TrieST<V> {
    root: Option<Box<Node<V>>>,
    n: usize, // Number of keys
}

struct Node<V> {
    val: Option<V>,
    next: Vec<Option<Box<Node<V>>>>,
}

impl<V: fmt::Debug> fmt::Debug for Node<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("val", &self.val)
            .field(
                "next_count",
                &self.next.iter().filter(|n| n.is_some()).count(),
            )
            .finish()
    }
}

impl<V: Clone> Node<V> {
    fn new() -> Self {
        Node {
            val: None,
            next: vec![None; R],
        }
    }
}

impl<V: Clone> Clone for Node<V> {
    fn clone(&self) -> Self {
        Node {
            val: self.val.clone(),
            next: self.next.clone(),
        }
    }
}

impl<V> TrieST<V> {
    /// Creates a new empty trie.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::TrieST;
    ///
    /// let trie: TrieST<i32> = TrieST::new();
    /// assert!(trie.is_empty());
    /// ```
    pub fn new() -> Self {
        TrieST { root: None, n: 0 }
    }

    /// Returns the number of key-value pairs in the trie.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::TrieST;
    ///
    /// let mut trie = TrieST::new();
    /// assert_eq!(trie.size(), 0);
    ///
    /// trie.put("key", 42);
    /// assert_eq!(trie.size(), 1);
    /// ```
    pub fn size(&self) -> usize {
        self.n
    }

    /// Returns `true` if the trie is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::TrieST;
    ///
    /// let mut trie = TrieST::new();
    /// assert!(trie.is_empty());
    ///
    /// trie.put("key", 42);
    /// assert!(!trie.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Returns `true` if the trie contains the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::TrieST;
    ///
    /// let mut trie = TrieST::new();
    /// trie.put("she", 1);
    ///
    /// assert!(trie.contains("she"));
    /// assert!(!trie.contains("shells"));
    /// ```
    pub fn contains(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// Returns the value associated with the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::TrieST;
    ///
    /// let mut trie = TrieST::new();
    /// trie.put("she", 1);
    /// trie.put("shells", 2);
    ///
    /// assert_eq!(trie.get("she"), Some(&1));
    /// assert_eq!(trie.get("shell"), None);
    /// ```
    pub fn get(&self, key: &str) -> Option<&V> {
        let node = Self::get_helper(&self.root, key, 0)?;
        node.val.as_ref()
    }

    fn get_helper<'a>(node: &'a Option<Box<Node<V>>>, key: &str, d: usize) -> Option<&'a Node<V>> {
        match node {
            None => None,
            Some(x) => {
                if d == key.len() {
                    Some(x)
                } else {
                    let c = key.as_bytes()[d] as usize;
                    Self::get_helper(&x.next[c], key, d + 1)
                }
            }
        }
    }

    /// Inserts the specified key-value pair into the trie.
    ///
    /// If the key already exists, its value is updated.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::TrieST;
    ///
    /// let mut trie = TrieST::new();
    /// trie.put("she", 1);
    /// trie.put("shells", 2);
    ///
    /// assert_eq!(trie.get("she"), Some(&1));
    /// assert_eq!(trie.size(), 2);
    /// ```
    pub fn put(&mut self, key: &str, val: V)
    where
        V: Clone,
    {
        // Check if the key existed before insertion
        let existed = Self::get_helper(&self.root, key, 0).is_some_and(|node| node.val.is_some());
        self.root = Self::put_helper(self.root.take(), key, val, 0);
        // If the key did not exist before, increment size
        if !existed {
            self.n += 1;
        }
    }

    fn put_helper(node: Option<Box<Node<V>>>, key: &str, val: V, d: usize) -> Option<Box<Node<V>>>
    where
        V: Clone,
    {
        let mut x = node.unwrap_or_else(|| Box::new(Node::new()));

        if d == key.len() {
            x.val = Some(val);
            return Some(x);
        }

        let c = key.as_bytes()[d] as usize;
        x.next[c] = Self::put_helper(x.next[c].take(), key, val, d + 1);
        Some(x)
    }

    /// Returns all keys in the trie.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::TrieST;
    ///
    /// let mut trie = TrieST::new();
    /// trie.put("she", 1);
    /// trie.put("sells", 2);
    /// trie.put("sea", 3);
    ///
    /// let keys = trie.keys();
    /// assert_eq!(keys.len(), 3);
    /// ```
    pub fn keys(&self) -> Vec<String> {
        self.keys_with_prefix("")
    }

    /// Returns all keys that start with the given prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::TrieST;
    ///
    /// let mut trie = TrieST::new();
    /// trie.put("she", 1);
    /// trie.put("shells", 2);
    /// trie.put("shore", 3);
    /// trie.put("the", 4);
    ///
    /// let keys = trie.keys_with_prefix("sh");
    /// assert_eq!(keys.len(), 3);
    /// assert!(keys.contains(&"she".to_string()));
    /// assert!(keys.contains(&"shells".to_string()));
    /// assert!(keys.contains(&"shore".to_string()));
    /// ```
    pub fn keys_with_prefix(&self, prefix: &str) -> Vec<String> {
        let mut results = Vec::new();
        let node = Self::get_helper(&self.root, prefix, 0);
        Self::collect(node, &mut String::from(prefix), &mut results);
        results
    }

    fn collect(node: Option<&Node<V>>, prefix: &mut String, results: &mut Vec<String>) {
        if let Some(x) = node {
            if x.val.is_some() {
                results.push(prefix.clone());
            }
            for c in 0..R {
                if x.next[c].is_some() {
                    prefix.push(c as u8 as char);
                    Self::collect(x.next[c].as_deref(), prefix, results);
                    prefix.pop();
                }
            }
        }
    }

    /// Returns all keys that match the given pattern.
    ///
    /// The pattern can contain:
    /// - Regular characters that must match exactly
    /// - '.' which matches any single character
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::TrieST;
    ///
    /// let mut trie = TrieST::new();
    /// trie.put("she", 1);
    /// trie.put("see", 2);
    /// trie.put("sea", 3);
    ///
    /// let keys = trie.keys_that_match("se.");
    /// assert_eq!(keys.len(), 2);
    /// assert!(keys.contains(&"see".to_string()));
    /// assert!(keys.contains(&"sea".to_string()));
    /// ```
    pub fn keys_that_match(&self, pattern: &str) -> Vec<String> {
        let mut results = Vec::new();
        Self::collect_pattern(&self.root, &mut String::new(), pattern, &mut results);
        results
    }

    fn collect_pattern(
        node: &Option<Box<Node<V>>>,
        prefix: &mut String,
        pattern: &str,
        results: &mut Vec<String>,
    ) {
        if let Some(x) = node {
            let d = prefix.len();
            if d == pattern.len() {
                if x.val.is_some() {
                    results.push(prefix.clone());
                }
                return;
            }

            let next_char = pattern.as_bytes()[d];
            if next_char == b'.' {
                // Wildcard: try all possible characters
                for c in 0..R {
                    if x.next[c].is_some() {
                        prefix.push(c as u8 as char);
                        Self::collect_pattern(&x.next[c], prefix, pattern, results);
                        prefix.pop();
                    }
                }
            } else {
                // Exact match
                let c = next_char as usize;
                prefix.push(c as u8 as char);
                Self::collect_pattern(&x.next[c], prefix, pattern, results);
                prefix.pop();
            }
        }
    }

    /// Returns the longest key in the trie that is a prefix of the given string.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::TrieST;
    ///
    /// let mut trie = TrieST::new();
    /// trie.put("she", 1);
    /// trie.put("shells", 2);
    ///
    /// assert_eq!(trie.longest_prefix_of("shellsort"), Some("shells".to_string()));
    /// assert_eq!(trie.longest_prefix_of("shell"), Some("she".to_string()));
    /// ```
    pub fn longest_prefix_of(&self, query: &str) -> Option<String> {
        let length = Self::longest_prefix_of_helper(&self.root, query, 0, 0);
        if length == 0 {
            None
        } else {
            Some(query[..length].to_string())
        }
    }

    fn longest_prefix_of_helper(
        node: &Option<Box<Node<V>>>,
        query: &str,
        d: usize,
        length: usize,
    ) -> usize {
        match node {
            None => length,
            Some(x) => {
                let mut new_length = length;
                if x.val.is_some() {
                    new_length = d;
                }
                if d == query.len() {
                    return new_length;
                }
                let c = query.as_bytes()[d] as usize;
                Self::longest_prefix_of_helper(&x.next[c], query, d + 1, new_length)
            }
        }
    }
}

impl<V> Default for TrieST<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: fmt::Display> fmt::Display for TrieST<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TrieST {{ ")?;
        let keys = self.keys();
        for (i, key) in keys.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            if let Some(val) = self.get(key) {
                write!(f, "{}: {}", key, val)?;
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
        let trie: TrieST<i32> = TrieST::new();
        assert!(trie.is_empty());
        assert_eq!(trie.size(), 0);
    }

    #[test]
    fn test_put_and_get() {
        let mut trie = TrieST::new();
        trie.put("she", 1);
        trie.put("sells", 2);
        trie.put("sea", 3);
        trie.put("shells", 4);

        assert_eq!(trie.get("she"), Some(&1));
        assert_eq!(trie.get("sells"), Some(&2));
        assert_eq!(trie.get("sea"), Some(&3));
        assert_eq!(trie.get("shells"), Some(&4));
        assert_eq!(trie.get("shell"), None);
        assert_eq!(trie.size(), 4);
    }

    #[test]
    fn test_contains() {
        let mut trie = TrieST::new();
        trie.put("she", 1);

        assert!(trie.contains("she"));
        assert!(!trie.contains("shells"));
    }

    #[test]
    fn test_keys() {
        let mut trie = TrieST::new();
        trie.put("she", 1);
        trie.put("sells", 2);
        trie.put("sea", 3);

        let keys = trie.keys();
        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&"she".to_string()));
        assert!(keys.contains(&"sells".to_string()));
        assert!(keys.contains(&"sea".to_string()));
    }

    #[test]
    fn test_keys_with_prefix() {
        let mut trie = TrieST::new();
        trie.put("she", 1);
        trie.put("shells", 2);
        trie.put("shore", 3);
        trie.put("the", 4);

        let keys = trie.keys_with_prefix("sh");
        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&"she".to_string()));
        assert!(keys.contains(&"shells".to_string()));
        assert!(keys.contains(&"shore".to_string()));

        let keys = trie.keys_with_prefix("t");
        assert_eq!(keys.len(), 1);
        assert!(keys.contains(&"the".to_string()));
    }

    #[test]
    fn test_keys_that_match() {
        let mut trie = TrieST::new();
        trie.put("she", 1);
        trie.put("see", 2);
        trie.put("sea", 3);
        trie.put("the", 4);

        let keys = trie.keys_that_match("se.");
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&"see".to_string()));
        assert!(keys.contains(&"sea".to_string()));

        let keys = trie.keys_that_match("..e");
        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&"she".to_string()));
        assert!(keys.contains(&"see".to_string()));
        assert!(keys.contains(&"the".to_string()));
    }

    #[test]
    fn test_longest_prefix_of() {
        let mut trie = TrieST::new();
        trie.put("she", 1);
        trie.put("shells", 2);
        trie.put("shore", 3);

        assert_eq!(
            trie.longest_prefix_of("shellsort"),
            Some("shells".to_string())
        );
        assert_eq!(trie.longest_prefix_of("shell"), Some("she".to_string()));
        assert_eq!(trie.longest_prefix_of("s"), None);
    }

    #[test]
    fn test_put_update() {
        let mut trie = TrieST::new();
        trie.put("she", 1);
        assert_eq!(trie.get("she"), Some(&1));
        assert_eq!(trie.size(), 1);

        trie.put("she", 10);
        assert_eq!(trie.get("she"), Some(&10));
        assert_eq!(trie.size(), 1);
    }

    #[test]
    fn test_empty_operations() {
        let trie: TrieST<i32> = TrieST::new();

        assert_eq!(trie.get("key"), None);
        assert!(!trie.contains("key"));
        assert_eq!(trie.keys().len(), 0);
        assert_eq!(trie.keys_with_prefix("k").len(), 0);
    }

    #[test]
    fn test_prefix_of_key() {
        let mut trie = TrieST::new();
        trie.put("apple", 1);
        trie.put("app", 2);

        assert_eq!(trie.get("apple"), Some(&1));
        assert_eq!(trie.get("app"), Some(&2));
        assert_eq!(trie.size(), 2);
    }

    #[test]
    fn test_display() {
        let mut trie = TrieST::new();
        trie.put("she", 1);
        trie.put("sells", 2);

        let display = format!("{}", trie);
        assert!(display.contains("she"));
        assert!(display.contains("sells"));
    }
}
