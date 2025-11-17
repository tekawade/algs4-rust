/// File Index
///
/// A utility for indexing files and maintaining an inverted index mapping
/// keys to the files they appear in. Useful for building search indices.
///
/// # Examples
///
/// ```
/// use searching::file_index::FileIndex;
///
/// let mut index = FileIndex::new();
/// index.add_entry("apple", "file1.txt");
/// index.add_entry("banana", "file2.txt");
/// index.add_entry("apple", "file3.txt");
///
/// let files = index.get("apple");
/// assert_eq!(files.len(), 2);
/// ```
use std::collections::{HashMap, HashSet};

/// File Index data structure for maintaining an inverted index.
#[derive(Debug, Clone)]
pub struct FileIndex<K, F> {
    index: HashMap<K, HashSet<F>>,
}

impl<K: Eq + std::hash::Hash + Clone, F: Eq + std::hash::Hash + Clone> FileIndex<K, F> {
    /// Creates a new empty file index.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::file_index::FileIndex;
    ///
    /// let index: FileIndex<String, String> = FileIndex::new();
    /// assert!(index.is_empty());
    /// ```
    pub fn new() -> Self {
        FileIndex {
            index: HashMap::new(),
        }
    }

    /// Adds an entry mapping a key to a file.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to index
    /// * `file` - The file containing the key
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::file_index::FileIndex;
    ///
    /// let mut index = FileIndex::new();
    /// index.add_entry("term", "document1.txt");
    /// assert!(index.contains("term"));
    /// ```
    pub fn add_entry(&mut self, key: K, file: F) {
        self.index.entry(key).or_default().insert(file);
    }

    /// Returns all files containing the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up
    ///
    /// # Returns
    ///
    /// A Vec of files containing the key, or an empty Vec if not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::file_index::FileIndex;
    ///
    /// let mut index = FileIndex::new();
    /// index.add_entry("apple", "file1.txt");
    /// index.add_entry("apple", "file2.txt");
    ///
    /// let files = index.get("apple");
    /// assert_eq!(files.len(), 2);
    /// ```
    pub fn get(&self, key: &K) -> Vec<&F> {
        self.index
            .get(key)
            .map(|set| set.iter().collect())
            .unwrap_or_default()
    }

    /// Returns true if the index contains the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::file_index::FileIndex;
    ///
    /// let mut index = FileIndex::new();
    /// index.add_entry("exists", "file.txt");
    ///
    /// assert!(index.contains("exists"));
    /// assert!(!index.contains("missing"));
    /// ```
    pub fn contains(&self, key: &K) -> bool {
        self.index.contains_key(key)
    }

    /// Returns the number of unique keys in the index.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::file_index::FileIndex;
    ///
    /// let mut index = FileIndex::new();
    /// index.add_entry("key1", "file1.txt");
    /// index.add_entry("key2", "file2.txt");
    ///
    /// assert_eq!(index.size(), 2);
    /// ```
    pub fn size(&self) -> usize {
        self.index.len()
    }

    /// Returns true if the index is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::file_index::FileIndex;
    ///
    /// let index: FileIndex<String, String> = FileIndex::new();
    /// assert!(index.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }

    /// Returns all keys in the index.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::file_index::FileIndex;
    ///
    /// let mut index = FileIndex::new();
    /// index.add_entry("apple", "file1.txt");
    /// index.add_entry("banana", "file2.txt");
    ///
    /// let keys = index.keys();
    /// assert_eq!(keys.len(), 2);
    /// ```
    pub fn keys(&self) -> Vec<&K> {
        self.index.keys().collect()
    }

    /// Returns the number of files containing the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::file_index::FileIndex;
    ///
    /// let mut index = FileIndex::new();
    /// index.add_entry("apple", "file1.txt");
    /// index.add_entry("apple", "file2.txt");
    /// index.add_entry("apple", "file3.txt");
    ///
    /// assert_eq!(index.file_count("apple"), 3);
    /// ```
    pub fn file_count(&self, key: &K) -> usize {
        self.index.get(key).map(|set| set.len()).unwrap_or(0)
    }
}

impl<K: Eq + std::hash::Hash + Clone, F: Eq + std::hash::Hash + Clone> Default for FileIndex<K, F> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let index: FileIndex<String, String> = FileIndex::new();
        assert!(index.is_empty());
        assert_eq!(index.size(), 0);
    }

    #[test]
    fn test_add_and_get() {
        let mut index = FileIndex::new();
        index.add_entry("apple", "file1.txt");
        index.add_entry("banana", "file2.txt");
        index.add_entry("apple", "file3.txt");

        let apple_files = index.get(&"apple");
        assert_eq!(apple_files.len(), 2);
        assert!(apple_files.contains(&&"file1.txt"));
        assert!(apple_files.contains(&&"file3.txt"));

        let banana_files = index.get(&"banana");
        assert_eq!(banana_files.len(), 1);
        assert!(banana_files.contains(&&"file2.txt"));
    }

    #[test]
    fn test_contains() {
        let mut index = FileIndex::new();
        index.add_entry("exists", "file.txt");

        assert!(index.contains(&"exists"));
        assert!(!index.contains(&"missing"));
    }

    #[test]
    fn test_size() {
        let mut index = FileIndex::new();
        assert_eq!(index.size(), 0);

        index.add_entry("key1", "file1.txt");
        assert_eq!(index.size(), 1);

        index.add_entry("key2", "file2.txt");
        assert_eq!(index.size(), 2);

        // Adding same key to different file doesn't increase size
        index.add_entry("key1", "file3.txt");
        assert_eq!(index.size(), 2);
    }

    #[test]
    fn test_keys() {
        let mut index = FileIndex::new();
        index.add_entry("apple", "file1.txt");
        index.add_entry("banana", "file2.txt");
        index.add_entry("cherry", "file3.txt");

        let keys = index.keys();
        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&&"apple"));
        assert!(keys.contains(&&"banana"));
        assert!(keys.contains(&&"cherry"));
    }

    #[test]
    fn test_file_count() {
        let mut index = FileIndex::new();
        index.add_entry("apple", "file1.txt");
        index.add_entry("apple", "file2.txt");
        index.add_entry("apple", "file3.txt");
        index.add_entry("banana", "file1.txt");

        assert_eq!(index.file_count(&"apple"), 3);
        assert_eq!(index.file_count(&"banana"), 1);
        assert_eq!(index.file_count(&"missing"), 0);
    }

    #[test]
    fn test_duplicate_file() {
        let mut index = FileIndex::new();
        index.add_entry("apple", "file1.txt");
        index.add_entry("apple", "file1.txt"); // Duplicate

        let files = index.get(&"apple");
        assert_eq!(files.len(), 1); // Should only have one entry
    }

    #[test]
    fn test_integer_keys() {
        let mut index = FileIndex::new();
        index.add_entry(1, "file1.txt");
        index.add_entry(2, "file2.txt");
        index.add_entry(1, "file3.txt");

        assert_eq!(index.get(&1).len(), 2);
        assert_eq!(index.get(&2).len(), 1);
    }
}
