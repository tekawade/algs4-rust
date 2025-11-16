//! Binary Search Tree (BST) implementation.
//!
//! A binary search tree is a binary tree in symmetric order where each node
//! has a key greater than all keys in its left subtree and less than all keys
//! in its right subtree.
//!
//! # Examples
//!
//! ```
//! use algs4_searching::BST;
//!
//! let mut bst = BST::new();
//! bst.put("S", 1);
//! bst.put("E", 2);
//! bst.put("A", 3);
//! bst.put("R", 4);
//! bst.put("C", 5);
//! bst.put("H", 6);
//!
//! assert_eq!(bst.get(&"E"), Some(&2));
//! assert_eq!(bst.size(), 6);
//! assert_eq!(bst.min(), Some(&"A"));
//! assert_eq!(bst.max(), Some(&"S"));
//! ```
//!
//! # References
//!
//! - Algorithms, 4th Edition: Section 3.2
//! - Original Java: `BST.java`

use std::fmt;

/// A symbol table implemented with a binary search tree.
///
/// This implementation uses a recursive approach for all operations.
/// The tree is not balanced, so performance degrades to O(n) in the worst case
/// (when keys are inserted in sorted order).
///
/// # Type Parameters
///
/// * `K` - The key type, must implement `Ord` for ordering
/// * `V` - The value type
///
/// # Performance
///
/// * Search: O(log n) average, O(n) worst case
/// * Insert: O(log n) average, O(n) worst case
/// * Delete: O(log n) average, O(n) worst case
/// * Min/Max: O(h) where h is height
/// * Space: O(n)
#[derive(Debug, Clone)]
pub struct BST<K, V> {
    root: Option<Box<Node<K, V>>>,
}

#[derive(Debug, Clone)]
struct Node<K, V> {
    key: K,
    val: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
    n: usize, // Number of nodes in subtree
}

impl<K, V> Node<K, V> {
    fn new(key: K, val: V, n: usize) -> Self {
        Node {
            key,
            val,
            left: None,
            right: None,
            n,
        }
    }

    fn size(node: &Option<Box<Node<K, V>>>) -> usize {
        node.as_ref().map_or(0, |n| n.n)
    }
}

impl<K, V> BST<K, V>
where
    K: Ord,
{
    /// Creates a new empty binary search tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BST;
    ///
    /// let bst: BST<String, i32> = BST::new();
    /// assert!(bst.is_empty());
    /// ```
    pub fn new() -> Self {
        BST { root: None }
    }

    /// Returns the number of key-value pairs in the BST.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BST;
    ///
    /// let mut bst = BST::new();
    /// assert_eq!(bst.size(), 0);
    ///
    /// bst.put("key", 42);
    /// assert_eq!(bst.size(), 1);
    /// ```
    pub fn size(&self) -> usize {
        Node::size(&self.root)
    }

    /// Returns `true` if the BST is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BST;
    ///
    /// let mut bst = BST::new();
    /// assert!(bst.is_empty());
    ///
    /// bst.put("key", 42);
    /// assert!(!bst.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// Returns `true` if the BST contains the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BST;
    ///
    /// let mut bst = BST::new();
    /// bst.put("apple", 1);
    ///
    /// assert!(bst.contains(&"apple"));
    /// assert!(!bst.contains(&"banana"));
    /// ```
    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Returns the value associated with the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BST;
    ///
    /// let mut bst = BST::new();
    /// bst.put("apple", 1);
    /// bst.put("banana", 2);
    ///
    /// assert_eq!(bst.get(&"apple"), Some(&1));
    /// assert_eq!(bst.get(&"cherry"), None);
    /// ```
    pub fn get(&self, key: &K) -> Option<&V> {
        Self::get_helper(&self.root, key)
    }

    fn get_helper<'a>(node: &'a Option<Box<Node<K, V>>>, key: &K) -> Option<&'a V> {
        match node {
            None => None,
            Some(n) => match key.cmp(&n.key) {
                std::cmp::Ordering::Less => Self::get_helper(&n.left, key),
                std::cmp::Ordering::Greater => Self::get_helper(&n.right, key),
                std::cmp::Ordering::Equal => Some(&n.val),
            },
        }
    }

    /// Inserts the specified key-value pair into the BST.
    ///
    /// If the key already exists, its value is updated.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BST;
    ///
    /// let mut bst = BST::new();
    /// bst.put("S", 1);
    /// bst.put("E", 2);
    /// bst.put("A", 3);
    ///
    /// assert_eq!(bst.get(&"E"), Some(&2));
    /// assert_eq!(bst.size(), 3);
    /// ```
    pub fn put(&mut self, key: K, val: V) {
        self.root = Self::put_helper(self.root.take(), key, val);
    }

    fn put_helper(node: Option<Box<Node<K, V>>>, key: K, val: V) -> Option<Box<Node<K, V>>> {
        match node {
            None => Some(Box::new(Node::new(key, val, 1))),
            Some(mut n) => {
                match key.cmp(&n.key) {
                    std::cmp::Ordering::Less => {
                        n.left = Self::put_helper(n.left.take(), key, val);
                    }
                    std::cmp::Ordering::Greater => {
                        n.right = Self::put_helper(n.right.take(), key, val);
                    }
                    std::cmp::Ordering::Equal => {
                        n.val = val;
                    }
                }
                n.n = 1 + Node::size(&n.left) + Node::size(&n.right);
                Some(n)
            }
        }
    }

    /// Removes the smallest key and its associated value from the BST.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BST;
    ///
    /// let mut bst = BST::new();
    /// bst.put("S", 1);
    /// bst.put("E", 2);
    /// bst.put("A", 3);
    ///
    /// bst.delete_min();
    /// assert_eq!(bst.min(), Some(&"E"));
    /// ```
    pub fn delete_min(&mut self) {
        if !self.is_empty() {
            self.root = Self::delete_min_helper(self.root.take());
        }
    }

    fn delete_min_helper(node: Option<Box<Node<K, V>>>) -> Option<Box<Node<K, V>>> {
        match node {
            None => None,
            Some(mut n) => {
                if n.left.is_none() {
                    return n.right;
                }
                n.left = Self::delete_min_helper(n.left.take());
                n.n = 1 + Node::size(&n.left) + Node::size(&n.right);
                Some(n)
            }
        }
    }

    /// Removes the largest key and its associated value from the BST.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BST;
    ///
    /// let mut bst = BST::new();
    /// bst.put("S", 1);
    /// bst.put("E", 2);
    /// bst.put("A", 3);
    ///
    /// bst.delete_max();
    /// assert_eq!(bst.max(), Some(&"E"));
    /// ```
    pub fn delete_max(&mut self) {
        if !self.is_empty() {
            self.root = Self::delete_max_helper(self.root.take());
        }
    }

    fn delete_max_helper(node: Option<Box<Node<K, V>>>) -> Option<Box<Node<K, V>>> {
        match node {
            None => None,
            Some(mut n) => {
                if n.right.is_none() {
                    return n.left;
                }
                n.right = Self::delete_max_helper(n.right.take());
                n.n = 1 + Node::size(&n.left) + Node::size(&n.right);
                Some(n)
            }
        }
    }

    /// Removes the specified key and its associated value from the BST.
    ///
    /// Uses Hibbard deletion (replace with successor).
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BST;
    ///
    /// let mut bst = BST::new();
    /// bst.put("S", 1);
    /// bst.put("E", 2);
    /// bst.put("A", 3);
    ///
    /// bst.delete(&"E");
    /// assert_eq!(bst.get(&"E"), None);
    /// assert_eq!(bst.size(), 2);
    /// ```
    pub fn delete(&mut self, key: &K)
    where
        K: Clone,
        V: Clone,
    {
        self.root = Self::delete_helper(self.root.take(), key);
    }

    fn delete_helper(node: Option<Box<Node<K, V>>>, key: &K) -> Option<Box<Node<K, V>>>
    where
        K: Clone,
        V: Clone,
    {
        match node {
            None => None,
            Some(mut n) => {
                match key.cmp(&n.key) {
                    std::cmp::Ordering::Less => {
                        n.left = Self::delete_helper(n.left.take(), key);
                    }
                    std::cmp::Ordering::Greater => {
                        n.right = Self::delete_helper(n.right.take(), key);
                    }
                    std::cmp::Ordering::Equal => {
                        // Node to delete found
                        if n.right.is_none() {
                            return n.left;
                        }
                        if n.left.is_none() {
                            return n.right;
                        }

                        // Node has two children: replace with successor (min of right subtree)
                        let min_node = Self::min_node(n.right.as_deref()).unwrap();
                        let min_key = min_node.key.clone();
                        let min_val = min_node.val.clone();

                        n.key = min_key;
                        n.val = min_val;
                        n.right = Self::delete_min_helper(n.right.take());
                    }
                }
                n.n = 1 + Node::size(&n.left) + Node::size(&n.right);
                Some(n)
            }
        }
    }

    /// Returns the smallest key in the BST.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BST;
    ///
    /// let mut bst = BST::new();
    /// bst.put("S", 1);
    /// bst.put("E", 2);
    /// bst.put("A", 3);
    ///
    /// assert_eq!(bst.min(), Some(&"A"));
    /// ```
    pub fn min(&self) -> Option<&K> {
        Self::min_node(self.root.as_deref()).map(|n| &n.key)
    }

    fn min_node(node: Option<&Node<K, V>>) -> Option<&Node<K, V>> {
        match node {
            None => None,
            Some(n) => {
                if n.left.is_none() {
                    Some(n)
                } else {
                    Self::min_node(n.left.as_deref())
                }
            }
        }
    }

    /// Returns the largest key in the BST.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BST;
    ///
    /// let mut bst = BST::new();
    /// bst.put("S", 1);
    /// bst.put("E", 2);
    /// bst.put("A", 3);
    ///
    /// assert_eq!(bst.max(), Some(&"S"));
    /// ```
    pub fn max(&self) -> Option<&K> {
        Self::max_node(self.root.as_deref()).map(|n| &n.key)
    }

    fn max_node(node: Option<&Node<K, V>>) -> Option<&Node<K, V>> {
        match node {
            None => None,
            Some(n) => {
                if n.right.is_none() {
                    Some(n)
                } else {
                    Self::max_node(n.right.as_deref())
                }
            }
        }
    }

    /// Returns the largest key less than or equal to the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BST;
    ///
    /// let mut bst = BST::new();
    /// bst.put("S", 1);
    /// bst.put("E", 2);
    /// bst.put("A", 3);
    /// bst.put("R", 4);
    ///
    /// assert_eq!(bst.floor(&"Q"), Some(&"E"));
    /// assert_eq!(bst.floor(&"E"), Some(&"E"));
    /// ```
    pub fn floor(&self, key: &K) -> Option<&K> {
        Self::floor_helper(&self.root, key).map(|n| &n.key)
    }

    fn floor_helper<'a>(
        node: &'a Option<Box<Node<K, V>>>,
        key: &K,
    ) -> Option<&'a Node<K, V>> {
        match node {
            None => None,
            Some(n) => match key.cmp(&n.key) {
                std::cmp::Ordering::Equal => Some(n),
                std::cmp::Ordering::Less => Self::floor_helper(&n.left, key),
                std::cmp::Ordering::Greater => {
                    let t = Self::floor_helper(&n.right, key);
                    if t.is_some() {
                        t
                    } else {
                        Some(n)
                    }
                }
            },
        }
    }

    /// Returns the smallest key greater than or equal to the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BST;
    ///
    /// let mut bst = BST::new();
    /// bst.put("S", 1);
    /// bst.put("E", 2);
    /// bst.put("A", 3);
    /// bst.put("R", 4);
    ///
    /// assert_eq!(bst.ceiling(&"Q"), Some(&"R"));
    /// assert_eq!(bst.ceiling(&"E"), Some(&"E"));
    /// ```
    pub fn ceiling(&self, key: &K) -> Option<&K> {
        Self::ceiling_helper(&self.root, key).map(|n| &n.key)
    }

    fn ceiling_helper<'a>(
        node: &'a Option<Box<Node<K, V>>>,
        key: &K,
    ) -> Option<&'a Node<K, V>> {
        match node {
            None => None,
            Some(n) => match key.cmp(&n.key) {
                std::cmp::Ordering::Equal => Some(n),
                std::cmp::Ordering::Greater => Self::ceiling_helper(&n.right, key),
                std::cmp::Ordering::Less => {
                    let t = Self::ceiling_helper(&n.left, key);
                    if t.is_some() {
                        t
                    } else {
                        Some(n)
                    }
                }
            },
        }
    }

    /// Returns the key of rank k (the k-th smallest key).
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BST;
    ///
    /// let mut bst = BST::new();
    /// bst.put("S", 1);
    /// bst.put("E", 2);
    /// bst.put("A", 3);
    /// bst.put("R", 4);
    ///
    /// assert_eq!(bst.select(0), Some(&"A"));
    /// assert_eq!(bst.select(1), Some(&"E"));
    /// assert_eq!(bst.select(2), Some(&"R"));
    /// assert_eq!(bst.select(3), Some(&"S"));
    /// ```
    pub fn select(&self, k: usize) -> Option<&K> {
        Self::select_helper(&self.root, k).map(|n| &n.key)
    }

    fn select_helper(node: &Option<Box<Node<K, V>>>, k: usize) -> Option<&Node<K, V>> {
        match node {
            None => None,
            Some(n) => {
                let t = Node::size(&n.left);
                match k.cmp(&t) {
                    std::cmp::Ordering::Less => Self::select_helper(&n.left, k),
                    std::cmp::Ordering::Greater => Self::select_helper(&n.right, k - t - 1),
                    std::cmp::Ordering::Equal => Some(n),
                }
            }
        }
    }

    /// Returns the number of keys strictly less than the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BST;
    ///
    /// let mut bst = BST::new();
    /// bst.put("S", 1);
    /// bst.put("E", 2);
    /// bst.put("A", 3);
    /// bst.put("R", 4);
    ///
    /// assert_eq!(bst.rank(&"A"), 0);
    /// assert_eq!(bst.rank(&"E"), 1);
    /// assert_eq!(bst.rank(&"Q"), 2);  // Between E and R
    /// ```
    pub fn rank(&self, key: &K) -> usize {
        Self::rank_helper(&self.root, key)
    }

    fn rank_helper(node: &Option<Box<Node<K, V>>>, key: &K) -> usize {
        match node {
            None => 0,
            Some(n) => match key.cmp(&n.key) {
                std::cmp::Ordering::Less => Self::rank_helper(&n.left, key),
                std::cmp::Ordering::Greater => {
                    1 + Node::size(&n.left) + Self::rank_helper(&n.right, key)
                }
                std::cmp::Ordering::Equal => Node::size(&n.left),
            },
        }
    }

    /// Returns the height of the BST.
    ///
    /// An empty tree has height -1, a tree with one node has height 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BST;
    ///
    /// let mut bst = BST::new();
    /// assert_eq!(bst.height(), -1);
    ///
    /// bst.put("S", 1);
    /// assert_eq!(bst.height(), 0);
    ///
    /// bst.put("E", 2);
    /// assert_eq!(bst.height(), 1);
    /// ```
    pub fn height(&self) -> isize {
        Self::height_helper(&self.root)
    }

    fn height_helper(node: &Option<Box<Node<K, V>>>) -> isize {
        match node {
            None => -1,
            Some(n) => {
                1 + Self::height_helper(&n.left).max(Self::height_helper(&n.right))
            }
        }
    }

    /// Returns an iterator over all keys in the BST in sorted order.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::BST;
    ///
    /// let mut bst = BST::new();
    /// bst.put("S", 1);
    /// bst.put("E", 2);
    /// bst.put("A", 3);
    ///
    /// let keys = bst.keys();
    /// assert_eq!(keys, vec![&"A", &"E", &"S"]);
    /// ```
    pub fn keys(&self) -> Vec<&K> {
        let mut keys = Vec::new();
        Self::inorder(&self.root, &mut keys);
        keys
    }

    fn inorder<'a>(node: &'a Option<Box<Node<K, V>>>, keys: &mut Vec<&'a K>) {
        if let Some(n) = node {
            Self::inorder(&n.left, keys);
            keys.push(&n.key);
            Self::inorder(&n.right, keys);
        }
    }
}

impl<K, V> Default for BST<K, V>
where
    K: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> fmt::Display for BST<K, V>
where
    K: fmt::Display,
    V: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BST {{ ")?;
        Self::display_helper(&self.root, f, true)?;
        write!(f, " }}")
    }
}

impl<K, V> BST<K, V>
where
    K: fmt::Display,
    V: fmt::Display,
{
    fn display_helper(
        node: &Option<Box<Node<K, V>>>,
        f: &mut fmt::Formatter<'_>,
        first: bool,
    ) -> fmt::Result {
        if let Some(n) = node {
            Self::display_helper(&n.left, f, first)?;
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", n.key, n.val)?;
            Self::display_helper(&n.right, f, false)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bst: BST<String, i32> = BST::new();
        assert!(bst.is_empty());
        assert_eq!(bst.size(), 0);
    }

    #[test]
    fn test_put_and_get() {
        let mut bst = BST::new();
        bst.put("S", 1);
        bst.put("E", 2);
        bst.put("A", 3);
        bst.put("R", 4);
        bst.put("C", 5);
        bst.put("H", 6);

        assert_eq!(bst.get(&"S"), Some(&1));
        assert_eq!(bst.get(&"E"), Some(&2));
        assert_eq!(bst.get(&"A"), Some(&3));
        assert_eq!(bst.get(&"X"), None);
        assert_eq!(bst.size(), 6);
    }

    #[test]
    fn test_ordered_iteration() {
        let mut bst = BST::new();
        bst.put("S", 1);
        bst.put("E", 2);
        bst.put("A", 3);
        bst.put("R", 4);

        let keys = bst.keys();
        let expected = vec![&"A", &"E", &"R", &"S"];
        assert_eq!(keys, expected);
    }

    #[test]
    fn test_min_max() {
        let mut bst = BST::new();
        bst.put("S", 1);
        bst.put("E", 2);
        bst.put("A", 3);
        bst.put("R", 4);

        assert_eq!(bst.min(), Some(&"A"));
        assert_eq!(bst.max(), Some(&"S"));
    }

    #[test]
    fn test_floor_ceiling() {
        let mut bst = BST::new();
        bst.put("S", 1);
        bst.put("E", 2);
        bst.put("A", 3);
        bst.put("R", 4);

        assert_eq!(bst.floor(&"Q"), Some(&"E"));
        assert_eq!(bst.floor(&"E"), Some(&"E"));
        assert_eq!(bst.ceiling(&"Q"), Some(&"R"));
        assert_eq!(bst.ceiling(&"E"), Some(&"E"));
    }

    #[test]
    fn test_select_and_rank() {
        let mut bst = BST::new();
        bst.put("S", 1);
        bst.put("E", 2);
        bst.put("A", 3);
        bst.put("R", 4);

        assert_eq!(bst.select(0), Some(&"A"));
        assert_eq!(bst.select(1), Some(&"E"));
        assert_eq!(bst.select(2), Some(&"R"));
        assert_eq!(bst.select(3), Some(&"S"));

        assert_eq!(bst.rank(&"A"), 0);
        assert_eq!(bst.rank(&"E"), 1);
        assert_eq!(bst.rank(&"R"), 2);
        assert_eq!(bst.rank(&"S"), 3);
    }

    #[test]
    fn test_delete_min_max() {
        let mut bst = BST::new();
        bst.put("S", 1);
        bst.put("E", 2);
        bst.put("A", 3);
        bst.put("R", 4);

        bst.delete_min();
        assert_eq!(bst.min(), Some(&"E"));
        assert_eq!(bst.size(), 3);

        bst.delete_max();
        assert_eq!(bst.max(), Some(&"R"));
        assert_eq!(bst.size(), 2);
    }

    #[test]
    fn test_delete() {
        let mut bst = BST::new();
        bst.put("S", 1);
        bst.put("E", 2);
        bst.put("A", 3);
        bst.put("R", 4);
        bst.put("C", 5);
        bst.put("H", 6);

        bst.delete(&"E");
        assert_eq!(bst.get(&"E"), None);
        assert_eq!(bst.size(), 5);

        // Verify BST property is maintained
        let keys = bst.keys();
        assert_eq!(keys, vec![&"A", &"C", &"H", &"R", &"S"]);
    }

    #[test]
    fn test_height() {
        let mut bst = BST::new();
        assert_eq!(bst.height(), -1);

        bst.put("S", 1);
        assert_eq!(bst.height(), 0);

        bst.put("E", 2);
        assert_eq!(bst.height(), 1);

        bst.put("A", 3);
        assert_eq!(bst.height(), 2);
    }

    #[test]
    fn test_put_update() {
        let mut bst = BST::new();
        bst.put("A", 1);
        assert_eq!(bst.get(&"A"), Some(&1));
        assert_eq!(bst.size(), 1);

        bst.put("A", 10);
        assert_eq!(bst.get(&"A"), Some(&10));
        assert_eq!(bst.size(), 1);
    }

    #[test]
    fn test_with_integers() {
        let mut bst = BST::new();
        bst.put(5, "five");
        bst.put(3, "three");
        bst.put(7, "seven");
        bst.put(1, "one");
        bst.put(9, "nine");

        assert_eq!(bst.min(), Some(&1));
        assert_eq!(bst.max(), Some(&9));
        assert_eq!(bst.select(2), Some(&5));
        assert_eq!(bst.rank(&5), 2);
    }

    #[test]
    fn test_empty_operations() {
        let bst: BST<&str, i32> = BST::new();

        assert_eq!(bst.get(&"key"), None);
        assert!(!bst.contains(&"key"));
        assert_eq!(bst.min(), None);
        assert_eq!(bst.max(), None);
        assert_eq!(bst.keys().len(), 0);
    }
}
