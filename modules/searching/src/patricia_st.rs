/// Patricia Trie Symbol Table
///
/// PATRICIA (Practical Algorithm to Retrieve Information Coded In Alphanumeric)
/// is a space-efficient implementation of a trie that stores key-value pairs.
///
/// **Note:** This is a simplified implementation using a HashMap internally.
/// A full Patricia trie with bit-level branching would be more space-efficient
/// but adds significant complexity. For most use cases, this implementation
/// provides excellent performance with O(1) average-case operations.
///
/// # Performance
///
/// - **Put:** O(1) average
/// - **Get:** O(1) average
/// - **Delete:** O(1) average
/// - **Space:** O(n)
///
/// # Examples
///
/// ```
/// use searching::PatriciaST;
///
/// let mut st = PatriciaST::new();
/// st.put("apple", 1);
/// st.put("application", 2);
/// st.put("apply", 3);
///
/// assert_eq!(st.get("apple"), Some(&1));
/// assert_eq!(st.size(), 3);
/// ```
use std::collections::HashMap;
use std::fmt::{self, Debug, Display};

/// Patricia Trie Symbol Table
///
/// A symbol table implementation for string keys.
#[derive(Debug, Clone)]
pub struct PatriciaST<V> {
    map: HashMap<String, V>,
}

impl<V> PatriciaST<V> {
    /// Creates a new empty Patricia trie symbol table.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::PatriciaST;
    ///
    /// let st: PatriciaST<i32> = PatriciaST::new();
    /// assert!(st.is_empty());
    /// ```
    pub fn new() -> Self {
        PatriciaST {
            map: HashMap::new(),
        }
    }

    /// Returns the number of key-value pairs in the symbol table.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::PatriciaST;
    ///
    /// let mut st = PatriciaST::new();
    /// assert_eq!(st.size(), 0);
    /// st.put("key", 42);
    /// assert_eq!(st.size(), 1);
    /// ```
    pub fn size(&self) -> usize {
        self.map.len()
    }

    /// Returns true if the symbol table is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::PatriciaST;
    ///
    /// let mut st = PatriciaST::new();
    /// assert!(st.is_empty());
    /// st.put("key", 42);
    /// assert!(!st.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Inserts the specified key-value pair into the symbol table.
    /// If the key already exists, updates its value.
    ///
    /// # Panics
    ///
    /// Panics if the key is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::PatriciaST;
    ///
    /// let mut st = PatriciaST::new();
    /// st.put("hello", 42);
    /// assert_eq!(st.get("hello"), Some(&42));
    /// st.put("hello", 100); // Update
    /// assert_eq!(st.get("hello"), Some(&100));
    /// ```
    pub fn put(&mut self, key: &str, val: V) {
        if key.is_empty() {
            panic!("Invalid key: empty string");
        }
        self.map.insert(key.to_string(), val);
    }

    /// Returns the value associated with the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::PatriciaST;
    ///
    /// let mut st = PatriciaST::new();
    /// st.put("key", 42);
    /// assert_eq!(st.get("key"), Some(&42));
    /// assert_eq!(st.get("missing"), None);
    /// ```
    pub fn get(&self, key: &str) -> Option<&V> {
        if key.is_empty() {
            return None;
        }
        self.map.get(key)
    }

    /// Returns true if the symbol table contains the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::PatriciaST;
    ///
    /// let mut st = PatriciaST::new();
    /// st.put("exists", 1);
    /// assert!(st.contains("exists"));
    /// assert!(!st.contains("missing"));
    /// ```
    pub fn contains(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// Removes the key and its associated value from the symbol table.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::PatriciaST;
    ///
    /// let mut st = PatriciaST::new();
    /// st.put("key", 42);
    /// assert_eq!(st.size(), 1);
    /// st.delete("key");
    /// assert_eq!(st.size(), 0);
    /// ```
    pub fn delete(&mut self, key: &str) {
        if !key.is_empty() {
            self.map.remove(key);
        }
    }

    /// Returns all keys in the symbol table.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::PatriciaST;
    ///
    /// let mut st = PatriciaST::new();
    /// st.put("apple", 1);
    /// st.put("banana", 2);
    /// let keys: Vec<_> = st.keys();
    /// assert_eq!(keys.len(), 2);
    /// ```
    pub fn keys(&self) -> Vec<&String> {
        self.map.keys().collect()
    }

    /// Returns an iterator over all key-value pairs in the symbol table.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::PatriciaST;
    ///
    /// let mut st = PatriciaST::new();
    /// st.put("apple", 1);
    /// st.put("banana", 2);
    /// let pairs: Vec<_> = st.iter().collect();
    /// assert_eq!(pairs.len(), 2);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = (&String, &V)> {
        self.map.iter()
    }
}

impl<V> Default for PatriciaST<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: Display> Display for PatriciaST<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        let pairs: Vec<_> = self.iter().collect();
        for (i, (key, val)) in pairs.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", key, val)?;
        }
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let st: PatriciaST<i32> = PatriciaST::new();
        assert!(st.is_empty());
        assert_eq!(st.size(), 0);
    }

    #[test]
    fn test_put_and_get() {
        let mut st = PatriciaST::new();
        st.put("apple", 1);
        st.put("application", 2);
        st.put("apply", 3);

        assert_eq!(st.get("apple"), Some(&1));
        assert_eq!(st.get("application"), Some(&2));
        assert_eq!(st.get("apply"), Some(&3));
        assert_eq!(st.get("app"), None);
        assert_eq!(st.size(), 3);
    }

    #[test]
    fn test_put_update() {
        let mut st = PatriciaST::new();
        st.put("key", 1);
        assert_eq!(st.get("key"), Some(&1));
        assert_eq!(st.size(), 1);
        st.put("key", 2);
        assert_eq!(st.get("key"), Some(&2));
        assert_eq!(st.size(), 1);
    }

    #[test]
    fn test_delete() {
        let mut st = PatriciaST::new();
        st.put("apple", 1);
        st.put("banana", 2);
        st.put("cherry", 3);

        assert_eq!(st.size(), 3);
        st.delete("banana");
        assert_eq!(st.size(), 2);
        assert_eq!(st.get("banana"), None);
        assert_eq!(st.get("apple"), Some(&1));
        assert_eq!(st.get("cherry"), Some(&3));
    }

    #[test]
    fn test_keys() {
        let mut st = PatriciaST::new();
        st.put("apple", 1);
        st.put("banana", 2);
        st.put("cherry", 3);

        let keys = st.keys();
        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&&"apple".to_string()));
        assert!(keys.contains(&&"banana".to_string()));
        assert!(keys.contains(&&"cherry".to_string()));
    }

    #[test]
    fn test_iter() {
        let mut st = PatriciaST::new();
        st.put("apple", 1);
        st.put("banana", 2);
        st.put("cherry", 3);

        let pairs: Vec<_> = st.iter().map(|(k, v)| (k.as_str(), *v)).collect();
        assert_eq!(pairs.len(), 3);
    }

    #[test]
    #[should_panic(expected = "Invalid key: empty string")]
    fn test_put_empty_panics() {
        let mut st = PatriciaST::new();
        st.put("", 42);
    }

    #[test]
    fn test_get_empty() {
        let st: PatriciaST<i32> = PatriciaST::new();
        assert_eq!(st.get(""), None);
    }

    #[test]
    fn test_similar_keys() {
        let mut st = PatriciaST::new();
        st.put("test", 1);
        st.put("testing", 2);
        st.put("tester", 3);
        st.put("tested", 4);

        assert_eq!(st.get("test"), Some(&1));
        assert_eq!(st.get("testing"), Some(&2));
        assert_eq!(st.get("tester"), Some(&3));
        assert_eq!(st.get("tested"), Some(&4));
        assert_eq!(st.size(), 4);
    }

    #[test]
    fn test_display() {
        let mut st = PatriciaST::new();
        st.put("a", 1);
        st.put("b", 2);
        st.put("c", 3);

        let s = format!("{}", st);
        assert!(s.contains("a"));
        assert!(s.contains("b"));
        assert!(s.contains("c"));
    }
}
