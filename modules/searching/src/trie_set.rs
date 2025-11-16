//! R-way trie set for string keys (no values).
//!
//! A trie set is similar to a trie symbol table but stores only keys without
//! associated values. It's useful when you only need to track membership
//! and perform prefix operations.
//!
//! # Examples
//!
//! ```
//! use algs4_searching::TrieSET;
//!
//! let mut set = TrieSET::new();
//! set.add("she");
//! set.add("shells");
//! set.add("shore");
//!
//! assert!(set.contains("she"));
//! assert_eq!(set.keys_with_prefix("sh").len(), 3);
//! ```
//!
//! # References
//!
//! - Algorithms, 4th Edition: Section 5.2
//! - Original Java: `TrieSET.java`

use crate::TrieST;
use std::fmt;

/// A set of strings using an R-way trie.
///
/// This implementation wraps a TrieST with unit values to provide a
/// set interface. It's particularly efficient for string membership
/// testing and prefix operations.
///
/// # Performance
///
/// * Add: O(L) where L is the key length
/// * Contains: O(L) where L is the key length
/// * Delete: Not supported in this implementation
/// * Prefix operations: O(L + number of matching keys)
/// * Space: O(RNL) worst case
#[derive(Debug, Clone)]
pub struct TrieSET {
    trie: TrieST<()>,
}

impl TrieSET {
    /// Creates a new empty trie set.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::TrieSET;
    ///
    /// let set = TrieSET::new();
    /// assert!(set.is_empty());
    /// ```
    pub fn new() -> Self {
        TrieSET {
            trie: TrieST::new(),
        }
    }

    /// Returns the number of strings in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::TrieSET;
    ///
    /// let mut set = TrieSET::new();
    /// assert_eq!(set.size(), 0);
    ///
    /// set.add("key");
    /// assert_eq!(set.size(), 1);
    /// ```
    pub fn size(&self) -> usize {
        self.trie.size()
    }

    /// Returns `true` if the set is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::TrieSET;
    ///
    /// let mut set = TrieSET::new();
    /// assert!(set.is_empty());
    ///
    /// set.add("key");
    /// assert!(!set.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.trie.is_empty()
    }

    /// Returns `true` if the set contains the specified string.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::TrieSET;
    ///
    /// let mut set = TrieSET::new();
    /// set.add("she");
    ///
    /// assert!(set.contains("she"));
    /// assert!(!set.contains("shells"));
    /// ```
    pub fn contains(&self, key: &str) -> bool {
        self.trie.contains(key)
    }

    /// Adds the specified string to the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::TrieSET;
    ///
    /// let mut set = TrieSET::new();
    /// set.add("she");
    /// set.add("shells");
    ///
    /// assert!(set.contains("she"));
    /// assert_eq!(set.size(), 2);
    /// ```
    pub fn add(&mut self, key: &str) {
        self.trie.put(key, ());
    }

    /// Returns all strings in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::TrieSET;
    ///
    /// let mut set = TrieSET::new();
    /// set.add("she");
    /// set.add("sells");
    /// set.add("sea");
    ///
    /// let keys = set.keys();
    /// assert_eq!(keys.len(), 3);
    /// ```
    pub fn keys(&self) -> Vec<String> {
        self.trie.keys()
    }

    /// Returns all strings that start with the given prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::TrieSET;
    ///
    /// let mut set = TrieSET::new();
    /// set.add("she");
    /// set.add("shells");
    /// set.add("shore");
    /// set.add("the");
    ///
    /// let keys = set.keys_with_prefix("sh");
    /// assert_eq!(keys.len(), 3);
    /// assert!(keys.contains(&"she".to_string()));
    /// assert!(keys.contains(&"shells".to_string()));
    /// assert!(keys.contains(&"shore".to_string()));
    /// ```
    pub fn keys_with_prefix(&self, prefix: &str) -> Vec<String> {
        self.trie.keys_with_prefix(prefix)
    }

    /// Returns all strings that match the given pattern.
    ///
    /// The pattern can contain:
    /// - Regular characters that must match exactly
    /// - '.' which matches any single character
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::TrieSET;
    ///
    /// let mut set = TrieSET::new();
    /// set.add("she");
    /// set.add("see");
    /// set.add("sea");
    ///
    /// let keys = set.keys_that_match("se.");
    /// assert_eq!(keys.len(), 2);
    /// assert!(keys.contains(&"see".to_string()));
    /// assert!(keys.contains(&"sea".to_string()));
    /// ```
    pub fn keys_that_match(&self, pattern: &str) -> Vec<String> {
        self.trie.keys_that_match(pattern)
    }

    /// Returns the longest string in the set that is a prefix of the given string.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::TrieSET;
    ///
    /// let mut set = TrieSET::new();
    /// set.add("she");
    /// set.add("shells");
    ///
    /// assert_eq!(set.longest_prefix_of("shellsort"), Some("shells".to_string()));
    /// assert_eq!(set.longest_prefix_of("shell"), Some("she".to_string()));
    /// ```
    pub fn longest_prefix_of(&self, query: &str) -> Option<String> {
        self.trie.longest_prefix_of(query)
    }
}

impl Default for TrieSET {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TrieSET {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TrieSET {{ ")?;
        let keys = self.keys();
        for (i, key) in keys.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", key)?;
        }
        write!(f, " }}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let set = TrieSET::new();
        assert!(set.is_empty());
        assert_eq!(set.size(), 0);
    }

    #[test]
    fn test_add_and_contains() {
        let mut set = TrieSET::new();
        set.add("she");
        set.add("sells");
        set.add("sea");
        set.add("shells");

        assert!(set.contains("she"));
        assert!(set.contains("sells"));
        assert!(set.contains("sea"));
        assert!(set.contains("shells"));
        assert!(!set.contains("shell"));
        assert_eq!(set.size(), 4);
    }

    #[test]
    fn test_keys() {
        let mut set = TrieSET::new();
        set.add("she");
        set.add("sells");
        set.add("sea");

        let keys = set.keys();
        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&"she".to_string()));
        assert!(keys.contains(&"sells".to_string()));
        assert!(keys.contains(&"sea".to_string()));
    }

    #[test]
    fn test_keys_with_prefix() {
        let mut set = TrieSET::new();
        set.add("she");
        set.add("shells");
        set.add("shore");
        set.add("the");

        let keys = set.keys_with_prefix("sh");
        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&"she".to_string()));
        assert!(keys.contains(&"shells".to_string()));
        assert!(keys.contains(&"shore".to_string()));

        let keys = set.keys_with_prefix("t");
        assert_eq!(keys.len(), 1);
        assert!(keys.contains(&"the".to_string()));
    }

    #[test]
    fn test_keys_that_match() {
        let mut set = TrieSET::new();
        set.add("she");
        set.add("see");
        set.add("sea");
        set.add("the");

        let keys = set.keys_that_match("se.");
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&"see".to_string()));
        assert!(keys.contains(&"sea".to_string()));

        let keys = set.keys_that_match("..e");
        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&"she".to_string()));
        assert!(keys.contains(&"see".to_string()));
        assert!(keys.contains(&"the".to_string()));
    }

    #[test]
    fn test_longest_prefix_of() {
        let mut set = TrieSET::new();
        set.add("she");
        set.add("shells");
        set.add("shore");

        assert_eq!(
            set.longest_prefix_of("shellsort"),
            Some("shells".to_string())
        );
        assert_eq!(set.longest_prefix_of("shell"), Some("she".to_string()));
        assert_eq!(set.longest_prefix_of("s"), None);
    }

    #[test]
    fn test_add_duplicate() {
        let mut set = TrieSET::new();
        set.add("she");
        assert_eq!(set.size(), 1);

        set.add("she");
        assert_eq!(set.size(), 1); // Size should not change
    }

    #[test]
    fn test_empty_operations() {
        let set = TrieSET::new();

        assert!(!set.contains("key"));
        assert_eq!(set.keys().len(), 0);
        assert_eq!(set.keys_with_prefix("k").len(), 0);
    }

    #[test]
    fn test_prefix_of_string() {
        let mut set = TrieSET::new();
        set.add("apple");
        set.add("app");

        assert!(set.contains("apple"));
        assert!(set.contains("app"));
        assert_eq!(set.size(), 2);
    }

    #[test]
    fn test_display() {
        let mut set = TrieSET::new();
        set.add("she");
        set.add("sells");

        let display = format!("{}", set);
        assert!(display.contains("she"));
        assert!(display.contains("sells"));
    }

    #[test]
    fn test_large_set() {
        let mut set = TrieSET::new();

        // Add many strings
        for i in 0..100 {
            set.add(&format!("key{}", i));
        }

        assert_eq!(set.size(), 100);

        // Verify all are present
        for i in 0..100 {
            assert!(set.contains(&format!("key{}", i)));
        }

        // Test prefix search
        let keys = set.keys_with_prefix("key1");
        assert!(keys.len() >= 11); // key1, key10-key19
    }
}
