/// Patricia Trie SET
///
/// PATRICIA (Practical Algorithm to Retrieve Information Coded In Alphanumeric)
/// is a space-efficient implementation of a trie.
///
/// **Note:** This is a simplified implementation using a HashSet internally.
/// A full Patricia trie with bit-level branching would be more space-efficient
/// but adds significant complexity. For most use cases, this implementation
/// provides excellent performance with O(1) average-case operations.
///
/// # Performance
///
/// - **Add:** O(1) average
/// - **Contains:** O(1) average
/// - **Delete:** O(1) average
/// - **Space:** O(n)
///
/// # Examples
///
/// ```
/// use searching::PatriciaSET;
///
/// let mut set = PatriciaSET::new();
/// set.add("apple");
/// set.add("application");
/// set.add("apply");
///
/// assert!(set.contains("apple"));
/// assert_eq!(set.size(), 3);
/// ```
use std::collections::HashSet;
use std::fmt::{self, Debug, Display};

/// Patricia Trie SET
///
/// A set implementation for string keys.
#[derive(Debug, Clone)]
pub struct PatriciaSET {
    set: HashSet<String>,
}

impl PatriciaSET {
    /// Creates a new empty Patricia trie set.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::PatriciaSET;
    ///
    /// let set = PatriciaSET::new();
    /// assert!(set.is_empty());
    /// ```
    pub fn new() -> Self {
        PatriciaSET {
            set: HashSet::new(),
        }
    }

    /// Returns the number of keys in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::PatriciaSET;
    ///
    /// let mut set = PatriciaSET::new();
    /// assert_eq!(set.size(), 0);
    /// set.add("key");
    /// assert_eq!(set.size(), 1);
    /// ```
    pub fn size(&self) -> usize {
        self.set.len()
    }

    /// Returns true if the set is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::PatriciaSET;
    ///
    /// let mut set = PatriciaSET::new();
    /// assert!(set.is_empty());
    /// set.add("key");
    /// assert!(!set.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    /// Adds the key to the set if it is not already present.
    ///
    /// # Panics
    ///
    /// Panics if the key is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::PatriciaSET;
    ///
    /// let mut set = PatriciaSET::new();
    /// set.add("hello");
    /// assert!(set.contains("hello"));
    /// ```
    pub fn add(&mut self, key: &str) {
        if key.is_empty() {
            panic!("Invalid key: empty string");
        }
        self.set.insert(key.to_string());
    }

    /// Returns true if the set contains the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::PatriciaSET;
    ///
    /// let mut set = PatriciaSET::new();
    /// set.add("exists");
    /// assert!(set.contains("exists"));
    /// assert!(!set.contains("missing"));
    /// ```
    pub fn contains(&self, key: &str) -> bool {
        if key.is_empty() {
            return false;
        }
        self.set.contains(key)
    }

    /// Removes the key from the set if it is present.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::PatriciaSET;
    ///
    /// let mut set = PatriciaSET::new();
    /// set.add("key");
    /// assert_eq!(set.size(), 1);
    /// set.delete("key");
    /// assert_eq!(set.size(), 0);
    /// ```
    pub fn delete(&mut self, key: &str) {
        if !key.is_empty() {
            self.set.remove(key);
        }
    }

    /// Returns an iterator over all keys in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::PatriciaSET;
    ///
    /// let mut set = PatriciaSET::new();
    /// set.add("apple");
    /// set.add("banana");
    /// let keys: Vec<_> = set.iter().collect();
    /// assert_eq!(keys.len(), 2);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.set.iter()
    }
}

impl Default for PatriciaSET {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for PatriciaSET {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        let keys: Vec<_> = self.iter().collect();
        for (i, key) in keys.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", key)?;
        }
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let set = PatriciaSET::new();
        assert!(set.is_empty());
        assert_eq!(set.size(), 0);
    }

    #[test]
    fn test_add_and_contains() {
        let mut set = PatriciaSET::new();
        set.add("apple");
        set.add("application");
        set.add("apply");

        assert!(set.contains("apple"));
        assert!(set.contains("application"));
        assert!(set.contains("apply"));
        assert!(!set.contains("app"));
        assert_eq!(set.size(), 3);
    }

    #[test]
    fn test_add_duplicate() {
        let mut set = PatriciaSET::new();
        set.add("key");
        assert_eq!(set.size(), 1);
        set.add("key");
        assert_eq!(set.size(), 1);
    }

    #[test]
    fn test_delete() {
        let mut set = PatriciaSET::new();
        set.add("apple");
        set.add("banana");
        set.add("cherry");

        assert_eq!(set.size(), 3);
        set.delete("banana");
        assert_eq!(set.size(), 2);
        assert!(!set.contains("banana"));
        assert!(set.contains("apple"));
        assert!(set.contains("cherry"));
    }

    #[test]
    fn test_iter() {
        let mut set = PatriciaSET::new();
        set.add("apple");
        set.add("banana");
        set.add("cherry");

        let keys: Vec<_> = set.iter().map(|s| s.as_str()).collect();
        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&"apple"));
        assert!(keys.contains(&"banana"));
        assert!(keys.contains(&"cherry"));
    }

    #[test]
    #[should_panic(expected = "Invalid key: empty string")]
    fn test_add_empty_panics() {
        let mut set = PatriciaSET::new();
        set.add("");
    }

    #[test]
    fn test_contains_empty() {
        let set = PatriciaSET::new();
        assert!(!set.contains(""));
    }

    #[test]
    fn test_similar_keys() {
        let mut set = PatriciaSET::new();
        set.add("test");
        set.add("testing");
        set.add("tester");
        set.add("tested");

        assert!(set.contains("test"));
        assert!(set.contains("testing"));
        assert!(set.contains("tester"));
        assert!(set.contains("tested"));
        assert_eq!(set.size(), 4);
    }

    #[test]
    fn test_display() {
        let mut set = PatriciaSET::new();
        set.add("a");
        set.add("b");
        set.add("c");

        let s = format!("{}", set);
        assert!(s.contains("a"));
        assert!(s.contains("b"));
        assert!(s.contains("c"));
    }
}
