/// Lookup Index
///
/// A bidirectional index that maps keys to values and values back to keys.
/// Useful for maintaining relationships where both forward and reverse lookups are needed.
///
/// # Examples
///
/// ```
/// use algs4_searching::lookup_index::LookupIndex;
///
/// let mut index = LookupIndex::new();
/// index.add("movies.txt", "Toy Story");
/// index.add("movies.txt", "Finding Nemo");
/// index.add("actors.txt", "Tom Hanks");
///
/// let keys = index.get_keys(&"Toy Story");
/// assert!(keys.contains(&&"movies.txt"));
/// ```
use std::collections::{HashMap, HashSet};

/// Bidirectional index for key-value lookups in both directions.
#[derive(Debug, Clone)]
pub struct LookupIndex<K, V> {
    forward: HashMap<K, HashSet<V>>, // key -> values
    reverse: HashMap<V, HashSet<K>>, // value -> keys
}

impl<K, V> LookupIndex<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Eq + std::hash::Hash + Clone,
{
    /// Creates a new empty bidirectional lookup index.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::lookup_index::LookupIndex;
    ///
    /// let index: LookupIndex<String, String> = LookupIndex::new();
    /// assert!(index.is_empty());
    /// ```
    pub fn new() -> Self {
        LookupIndex {
            forward: HashMap::new(),
            reverse: HashMap::new(),
        }
    }

    /// Adds a key-value association to the index.
    ///
    /// # Arguments
    ///
    /// * `key` - The key
    /// * `value` - The value to associate with the key
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::lookup_index::LookupIndex;
    ///
    /// let mut index = LookupIndex::new();
    /// index.add("key1", "value1");
    /// assert!(index.contains_key(&"key1"));
    /// assert!(index.contains_value(&"value1"));
    /// ```
    pub fn add(&mut self, key: K, value: V) {
        self.forward
            .entry(key.clone())
            .or_default()
            .insert(value.clone());

        self.reverse.entry(value).or_default().insert(key);
    }

    /// Returns all values associated with the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up
    ///
    /// # Returns
    ///
    /// A Vec of all values associated with the key, or empty Vec if not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::lookup_index::LookupIndex;
    ///
    /// let mut index = LookupIndex::new();
    /// index.add("key1", "value1");
    /// index.add("key1", "value2");
    ///
    /// let values = index.get_values(&"key1");
    /// assert_eq!(values.len(), 2);
    /// ```
    pub fn get_values(&self, key: &K) -> Vec<&V> {
        self.forward
            .get(key)
            .map(|set| set.iter().collect())
            .unwrap_or_default()
    }

    /// Returns all keys associated with the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to look up
    ///
    /// # Returns
    ///
    /// A Vec of all keys associated with the value, or empty Vec if not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::lookup_index::LookupIndex;
    ///
    /// let mut index = LookupIndex::new();
    /// index.add("key1", "value1");
    /// index.add("key2", "value1");
    ///
    /// let keys = index.get_keys(&"value1");
    /// assert_eq!(keys.len(), 2);
    /// ```
    pub fn get_keys(&self, value: &V) -> Vec<&K> {
        self.reverse
            .get(value)
            .map(|set| set.iter().collect())
            .unwrap_or_default()
    }

    /// Returns true if the index contains the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::lookup_index::LookupIndex;
    ///
    /// let mut index = LookupIndex::new();
    /// index.add("exists", "value");
    ///
    /// assert!(index.contains_key(&"exists"));
    /// assert!(!index.contains_key(&"missing"));
    /// ```
    pub fn contains_key(&self, key: &K) -> bool {
        self.forward.contains_key(key)
    }

    /// Returns true if the index contains the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::lookup_index::LookupIndex;
    ///
    /// let mut index = LookupIndex::new();
    /// index.add("key", "exists");
    ///
    /// assert!(index.contains_value(&"exists"));
    /// assert!(!index.contains_value(&"missing"));
    /// ```
    pub fn contains_value(&self, value: &V) -> bool {
        self.reverse.contains_key(value)
    }

    /// Returns the number of unique keys in the index.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::lookup_index::LookupIndex;
    ///
    /// let mut index = LookupIndex::new();
    /// index.add("key1", "value1");
    /// index.add("key2", "value2");
    ///
    /// assert_eq!(index.key_count(), 2);
    /// ```
    pub fn key_count(&self) -> usize {
        self.forward.len()
    }

    /// Returns the number of unique values in the index.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::lookup_index::LookupIndex;
    ///
    /// let mut index = LookupIndex::new();
    /// index.add("key1", "value1");
    /// index.add("key2", "value2");
    ///
    /// assert_eq!(index.value_count(), 2);
    /// ```
    pub fn value_count(&self) -> usize {
        self.reverse.len()
    }

    /// Returns true if the index is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::lookup_index::LookupIndex;
    ///
    /// let index: LookupIndex<String, String> = LookupIndex::new();
    /// assert!(index.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.forward.is_empty()
    }

    /// Returns all keys in the index.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::lookup_index::LookupIndex;
    ///
    /// let mut index = LookupIndex::new();
    /// index.add("key1", "value1");
    /// index.add("key2", "value2");
    ///
    /// let keys = index.keys();
    /// assert_eq!(keys.len(), 2);
    /// ```
    pub fn keys(&self) -> Vec<&K> {
        self.forward.keys().collect()
    }

    /// Returns all values in the index.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::lookup_index::LookupIndex;
    ///
    /// let mut index = LookupIndex::new();
    /// index.add("key1", "value1");
    /// index.add("key2", "value2");
    ///
    /// let values = index.values();
    /// assert_eq!(values.len(), 2);
    /// ```
    pub fn values(&self) -> Vec<&V> {
        self.reverse.keys().collect()
    }
}

impl<K, V> Default for LookupIndex<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Eq + std::hash::Hash + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let index: LookupIndex<String, String> = LookupIndex::new();
        assert!(index.is_empty());
        assert_eq!(index.key_count(), 0);
        assert_eq!(index.value_count(), 0);
    }

    #[test]
    fn test_add_and_get() {
        let mut index = LookupIndex::new();
        index.add("key1", "value1");
        index.add("key1", "value2");
        index.add("key2", "value1");

        let values = index.get_values(&"key1");
        assert_eq!(values.len(), 2);
        assert!(values.contains(&&"value1"));
        assert!(values.contains(&&"value2"));

        let keys = index.get_keys(&"value1");
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&&"key1"));
        assert!(keys.contains(&&"key2"));
    }

    #[test]
    fn test_contains() {
        let mut index = LookupIndex::new();
        index.add("key1", "value1");

        assert!(index.contains_key(&"key1"));
        assert!(!index.contains_key(&"key2"));

        assert!(index.contains_value(&"value1"));
        assert!(!index.contains_value(&"value2"));
    }

    #[test]
    fn test_counts() {
        let mut index = LookupIndex::new();
        index.add("key1", "value1");
        index.add("key1", "value2");
        index.add("key2", "value1");

        assert_eq!(index.key_count(), 2);
        assert_eq!(index.value_count(), 2);
    }

    #[test]
    fn test_keys_and_values() {
        let mut index = LookupIndex::new();
        index.add("key1", "value1");
        index.add("key2", "value2");

        let keys = index.keys();
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&&"key1"));
        assert!(keys.contains(&&"key2"));

        let values = index.values();
        assert_eq!(values.len(), 2);
        assert!(values.contains(&&"value1"));
        assert!(values.contains(&&"value2"));
    }

    #[test]
    fn test_bidirectional() {
        let mut index = LookupIndex::new();
        index.add("movies.txt", "Toy Story");
        index.add("movies.txt", "Finding Nemo");
        index.add("actors.txt", "Tom Hanks");

        // Forward lookup: file -> movies
        let movies = index.get_values(&"movies.txt");
        assert_eq!(movies.len(), 2);

        // Reverse lookup: movie -> files
        let files = index.get_keys(&"Toy Story");
        assert!(files.contains(&&"movies.txt"));
    }

    #[test]
    fn test_duplicate_add() {
        let mut index = LookupIndex::new();
        index.add("key", "value");
        index.add("key", "value"); // Duplicate

        let values = index.get_values(&"key");
        assert_eq!(values.len(), 1); // Should only have one entry
    }
}
