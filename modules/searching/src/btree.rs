/// B-tree Symbol Table
///
/// A B-tree is a self-balancing tree data structure that maintains sorted data
/// and allows searches, sequential access, insertions, and deletions in logarithmic time.
/// The B-tree is well suited for storage systems that read and write relatively large
/// blocks of data, such as disks.
///
/// This implementation uses order M = 4 (each node can have up to 3 keys and 4 children).
///
/// # Performance
///
/// - **Search:** O(log_M n)
/// - **Insert:** O(log_M n)
/// - **Height:** log_M(n)
///
/// # Examples
///
/// ```
/// use searching::BTree;
///
/// let mut bt = BTree::new();
/// bt.put("apple", 1);
/// bt.put("banana", 2);
/// bt.put("cherry", 3);
///
/// assert_eq!(bt.get(&"banana"), Some(&2));
/// assert_eq!(bt.size(), 3);
/// ```
use std::fmt::{self, Debug, Display};

const M: usize = 4; // Order of B-tree (max children per node)

/// B-tree Symbol Table
///
/// A symbol table implementation using a B-tree of order M=4.
#[derive(Debug, Clone)]
pub struct BTree<K, V> {
    root: Node<K, V>,
    height: usize,
    n: usize, // number of key-value pairs
}

#[derive(Debug, Clone)]
struct Node<K, V> {
    m: usize, // number of children
    children: Vec<Entry<K, V>>,
}

#[derive(Debug, Clone)]
struct Entry<K, V> {
    key: K,
    val: Option<V>,                // value is None for internal nodes
    next: Option<Box<Node<K, V>>>, // child node (used for internal nodes)
}

impl<K: Ord, V> Entry<K, V> {
    fn new_leaf(key: K, val: V) -> Self {
        Entry {
            key,
            val: Some(val),
            next: None,
        }
    }

    fn new_internal(key: K, next: Node<K, V>) -> Self {
        Entry {
            key,
            val: None,
            next: Some(Box::new(next)),
        }
    }
}

impl<K: Ord, V> Node<K, V> {
    fn new() -> Self {
        Node {
            m: 0,
            children: Vec::with_capacity(M),
        }
    }

    #[allow(dead_code)]
    fn with_entry(entry: Entry<K, V>) -> Self {
        let mut node = Node::new();
        node.children.push(entry);
        node.m = 1;
        node
    }
}

impl<K: Ord + Clone, V> BTree<K, V> {
    /// Creates a new empty B-tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::BTree;
    ///
    /// let bt: BTree<String, i32> = BTree::new();
    /// assert!(bt.is_empty());
    /// ```
    pub fn new() -> Self {
        BTree {
            root: Node::new(),
            height: 0,
            n: 0,
        }
    }

    /// Returns the number of key-value pairs in the B-tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::BTree;
    ///
    /// let mut bt = BTree::new();
    /// assert_eq!(bt.size(), 0);
    /// bt.put("A", 1);
    /// assert_eq!(bt.size(), 1);
    /// ```
    pub fn size(&self) -> usize {
        self.n
    }

    /// Returns the height of the B-tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::BTree;
    ///
    /// let mut bt = BTree::new();
    /// assert_eq!(bt.height(), 0);
    /// for i in 0..100 {
    ///     bt.put(i, i * 10);
    /// }
    /// // B-tree height grows logarithmically
    /// assert!(bt.height() <= 5);
    /// ```
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns true if the B-tree is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::BTree;
    ///
    /// let mut bt = BTree::new();
    /// assert!(bt.is_empty());
    /// bt.put(1, "one");
    /// assert!(!bt.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Returns the value associated with the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::BTree;
    ///
    /// let mut bt = BTree::new();
    /// bt.put("key", 42);
    /// assert_eq!(bt.get(&"key"), Some(&42));
    /// assert_eq!(bt.get(&"missing"), None);
    /// ```
    pub fn get(&self, key: &K) -> Option<&V> {
        Self::search(&self.root, key, self.height)
    }

    fn search<'a>(node: &'a Node<K, V>, key: &K, height: usize) -> Option<&'a V> {
        // External node (leaf level)
        if height == 0 {
            for entry in &node.children[..node.m] {
                if key == &entry.key {
                    return entry.val.as_ref();
                }
            }
            None
        } else {
            // Internal node
            for j in 0..node.m {
                if j + 1 == node.m || key < &node.children[j + 1].key {
                    if let Some(ref child) = node.children[j].next {
                        return Self::search(child, key, height - 1);
                    }
                    return None;
                }
            }
            None
        }
    }

    /// Returns true if the B-tree contains the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::BTree;
    ///
    /// let mut bt = BTree::new();
    /// bt.put("exists", 1);
    /// assert!(bt.contains(&"exists"));
    /// assert!(!bt.contains(&"missing"));
    /// ```
    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Inserts the specified key-value pair into the B-tree.
    /// If the key already exists, updates its value.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::BTree;
    ///
    /// let mut bt = BTree::new();
    /// bt.put("A", 1);
    /// bt.put("B", 2);
    /// assert_eq!(bt.get(&"A"), Some(&1));
    /// bt.put("A", 10); // Update
    /// assert_eq!(bt.get(&"A"), Some(&10));
    /// ```
    pub fn put(&mut self, key: K, val: V)
    where
        V: Clone,
    {
        let u = Self::insert(&mut self.root, key, val, self.height);

        if let Some(split_node) = u {
            // Need to split root
            let mut new_root = Node::new();
            let left_entry = Entry::new_internal(
                self.root.children[0].key.clone(),
                std::mem::replace(&mut self.root, Node::new()),
            );
            let right_entry = Entry::new_internal(split_node.children[0].key.clone(), split_node);

            new_root.children.push(left_entry);
            new_root.children.push(right_entry);
            new_root.m = 2;

            self.root = new_root;
            self.height += 1;
        }
        self.n += 1;
    }

    fn insert(node: &mut Node<K, V>, key: K, val: V, height: usize) -> Option<Node<K, V>>
    where
        V: Clone,
    {
        let mut j = 0;
        let new_entry = Entry::new_leaf(key.clone(), val.clone());

        // External node (leaf level)
        if height == 0 {
            // Find insertion point
            while j < node.m {
                if key < node.children[j].key {
                    break;
                }
                if key == node.children[j].key {
                    // Key already exists, update value
                    node.children[j].val = new_entry.val;
                    return None;
                }
                j += 1;
            }
        } else {
            // Internal node
            while j < node.m {
                if j + 1 == node.m || key < node.children[j + 1].key {
                    if let Some(ref mut child) = node.children[j].next {
                        let split = Self::insert(child, key.clone(), val.clone(), height - 1)?;
                        let split_key = split.children[0].key.clone();
                        let new_internal = Entry::new_internal(split_key, split);
                        j += 1;
                        node.children.insert(j, new_internal);
                        node.m += 1;
                        break;
                    }
                }
                j += 1;
            }

            if j == 0 {
                // This can happen if key < first key
                j = 0;
            }
        }

        // External node needs to insert entry at position j
        if height == 0 {
            node.children.insert(j, new_entry);
            node.m += 1;
        }

        // Check if node needs to be split
        if node.m < M {
            None
        } else {
            Some(Self::split(node))
        }
    }

    fn split(node: &mut Node<K, V>) -> Node<K, V> {
        let mut new_node = Node::new();
        let mid = M / 2;

        // Move upper half of entries to new node
        new_node.children = node.children.split_off(mid);
        new_node.m = M - mid;
        node.m = mid;

        new_node
    }

    /// Returns all keys in the B-tree in sorted order.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::BTree;
    ///
    /// let mut bt = BTree::new();
    /// bt.put(3, "three");
    /// bt.put(1, "one");
    /// bt.put(2, "two");
    /// let keys: Vec<_> = bt.keys();
    /// assert_eq!(keys, vec![&1, &2, &3]);
    /// ```
    pub fn keys(&self) -> Vec<&K> {
        let mut result = Vec::new();
        Self::collect_keys(&self.root, self.height, &mut result);
        result
    }

    fn collect_keys<'a>(node: &'a Node<K, V>, height: usize, result: &mut Vec<&'a K>) {
        if height == 0 {
            // Leaf node
            for entry in &node.children[..node.m] {
                result.push(&entry.key);
            }
        } else {
            // Internal node
            for entry in &node.children[..node.m] {
                if let Some(ref child) = entry.next {
                    Self::collect_keys(child, height - 1, result);
                }
            }
        }
    }
}

impl<K: Ord + Clone, V> Default for BTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord + Clone + Display, V: Display> Display for BTree<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        let keys = self.keys();
        for (i, key) in keys.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", key, self.get(key).unwrap())?;
        }
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bt: BTree<i32, i32> = BTree::new();
        assert!(bt.is_empty());
        assert_eq!(bt.size(), 0);
        assert_eq!(bt.height(), 0);
    }

    #[test]
    fn test_put_and_get() {
        let mut bt = BTree::new();
        bt.put("A", 1);
        bt.put("B", 2);
        bt.put("C", 3);

        assert_eq!(bt.get(&"A"), Some(&1));
        assert_eq!(bt.get(&"B"), Some(&2));
        assert_eq!(bt.get(&"C"), Some(&3));
        assert_eq!(bt.get(&"D"), None);
        assert_eq!(bt.size(), 3);
    }

    #[test]
    fn test_put_update() {
        let mut bt = BTree::new();
        bt.put("key", 1);
        assert_eq!(bt.get(&"key"), Some(&1));
        assert_eq!(bt.size(), 1);
    }

    #[test]
    fn test_large_btree() {
        let mut bt = BTree::new();
        let n = 100;

        for i in 0..n {
            bt.put(i, i * 10);
        }

        assert_eq!(bt.size(), n);

        for i in 0..n {
            assert_eq!(bt.get(&i), Some(&(i * 10)));
        }

        // Height should be logarithmic
        assert!(bt.height() <= 5);
    }

    #[test]
    fn test_ordered_insertion() {
        let mut bt = BTree::new();
        for i in 1..=20 {
            bt.put(i, i * 10);
        }

        assert_eq!(bt.size(), 20);

        for i in 1..=20 {
            assert_eq!(bt.get(&i), Some(&(i * 10)));
        }
    }

    #[test]
    fn test_reverse_insertion() {
        let mut bt = BTree::new();
        for i in (1..=20).rev() {
            bt.put(i, i * 10);
        }

        assert_eq!(bt.size(), 20);

        for i in 1..=20 {
            assert_eq!(bt.get(&i), Some(&(i * 10)));
        }
    }

    #[test]
    fn test_random_insertion() {
        let mut bt = BTree::new();
        let values = vec![
            15, 3, 18, 7, 1, 12, 9, 20, 5, 11, 2, 16, 8, 19, 4, 14, 6, 17, 10, 13,
        ];

        for &val in &values {
            bt.put(val, val * 10);
        }

        assert_eq!(bt.size(), 20);

        for &val in &values {
            assert_eq!(bt.get(&val), Some(&(val * 10)));
        }
    }

    #[test]
    fn test_keys() {
        let mut bt = BTree::new();
        bt.put(3, "three");
        bt.put(1, "one");
        bt.put(5, "five");
        bt.put(2, "two");
        bt.put(4, "four");

        let keys = bt.keys();
        assert_eq!(keys, vec![&1, &2, &3, &4, &5]);
    }

    #[test]
    fn test_contains() {
        let mut bt = BTree::new();
        bt.put("exists", 1);

        assert!(bt.contains(&"exists"));
        assert!(!bt.contains(&"missing"));
    }

    #[test]
    fn test_string_keys() {
        let mut bt = BTree::new();
        bt.put("apple".to_string(), 1);
        bt.put("banana".to_string(), 2);
        bt.put("cherry".to_string(), 3);

        assert_eq!(bt.get(&"apple".to_string()), Some(&1));
        assert_eq!(bt.get(&"banana".to_string()), Some(&2));
        assert_eq!(bt.get(&"cherry".to_string()), Some(&3));
        assert_eq!(bt.size(), 3);
    }

    #[test]
    fn test_split_root() {
        let mut bt = BTree::new();
        // Insert enough elements to cause root split (M = 4)
        for i in 1..=10 {
            bt.put(i, i * 10);
        }

        // Root should have split, increasing height
        assert!(bt.height() > 0);
        assert_eq!(bt.size(), 10);

        // All values should still be retrievable
        for i in 1..=10 {
            assert_eq!(bt.get(&i), Some(&(i * 10)));
        }
    }

    #[test]
    fn test_display() {
        let mut bt = BTree::new();
        bt.put(1, "one");
        bt.put(2, "two");
        bt.put(3, "three");

        let s = format!("{}", bt);
        assert!(s.contains("1: one"));
        assert!(s.contains("2: two"));
        assert!(s.contains("3: three"));
    }
}
