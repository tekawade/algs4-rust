/// AVL Tree Symbol Table
///
/// An AVL tree is a self-balancing binary search tree where the heights of the
/// two child subtrees of any node differ by at most one. This ensures O(log n)
/// worst-case time complexity for search, insert, and delete operations.
///
/// # Performance
///
/// - **Search:** O(log n)
/// - **Insert:** O(log n)
/// - **Delete:** O(log n)
/// - **Min/Max:** O(log n)
/// - **Floor/Ceiling:** O(log n)
/// - **Rank/Select:** O(log n)
///
/// # Examples
///
/// ```
/// use searching::AVLTreeST;
///
/// let mut st = AVLTreeST::new();
/// st.put("apple", 1);
/// st.put("banana", 2);
/// st.put("cherry", 3);
///
/// assert_eq!(st.get(&"banana"), Some(&2));
/// assert_eq!(st.size(), 3);
/// assert_eq!(st.height(), 1); // Balanced tree
/// ```
use std::cmp::Ordering;
use std::fmt::{self, Debug, Display};

/// AVL Tree Symbol Table
///
/// A symbol table implementation using an AVL tree (self-balancing BST).
#[derive(Debug, Clone)]
pub struct AVLTreeST<K, V> {
    root: Option<Box<Node<K, V>>>,
}

#[derive(Debug, Clone)]
struct Node<K, V> {
    key: K,
    val: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
    size: usize,
    height: i32,
}

impl<K: Ord, V> Node<K, V> {
    fn new(key: K, val: V) -> Self {
        Node {
            key,
            val,
            left: None,
            right: None,
            size: 1,
            height: 0,
        }
    }

    fn update_size(&mut self) {
        self.size = 1 + Self::size_of(&self.left) + Self::size_of(&self.right);
    }

    fn update_height(&mut self) {
        self.height = 1 + Self::height_of(&self.left).max(Self::height_of(&self.right));
    }

    fn size_of(node: &Option<Box<Node<K, V>>>) -> usize {
        node.as_ref().map_or(0, |n| n.size)
    }

    fn height_of(node: &Option<Box<Node<K, V>>>) -> i32 {
        node.as_ref().map_or(-1, |n| n.height)
    }

    fn balance_factor(&self) -> i32 {
        Self::height_of(&self.left) - Self::height_of(&self.right)
    }
}

impl<K: Ord + Clone, V: Clone> AVLTreeST<K, V> {
    /// Creates a new empty AVL tree symbol table.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::AVLTreeST;
    ///
    /// let st: AVLTreeST<String, i32> = AVLTreeST::new();
    /// assert!(st.is_empty());
    /// ```
    pub fn new() -> Self {
        AVLTreeST { root: None }
    }

    /// Returns the number of key-value pairs in the symbol table.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::AVLTreeST;
    ///
    /// let mut st = AVLTreeST::new();
    /// assert_eq!(st.size(), 0);
    /// st.put("A", 1);
    /// assert_eq!(st.size(), 1);
    /// ```
    pub fn size(&self) -> usize {
        Node::size_of(&self.root)
    }

    /// Returns the height of the AVL tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::AVLTreeST;
    ///
    /// let mut st = AVLTreeST::new();
    /// assert_eq!(st.height(), -1);
    /// st.put(1, "one");
    /// assert_eq!(st.height(), 0);
    /// ```
    pub fn height(&self) -> i32 {
        Node::height_of(&self.root)
    }

    /// Returns true if the symbol table is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::AVLTreeST;
    ///
    /// let mut st = AVLTreeST::new();
    /// assert!(st.is_empty());
    /// st.put(1, "one");
    /// assert!(!st.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// Returns the value associated with the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::AVLTreeST;
    ///
    /// let mut st = AVLTreeST::new();
    /// st.put("key", 42);
    /// assert_eq!(st.get(&"key"), Some(&42));
    /// assert_eq!(st.get(&"missing"), None);
    /// ```
    pub fn get(&self, key: &K) -> Option<&V> {
        Self::get_helper(&self.root, key)
    }

    fn get_helper<'a>(node: &'a Option<Box<Node<K, V>>>, key: &K) -> Option<&'a V> {
        node.as_ref().and_then(|n| match key.cmp(&n.key) {
            Ordering::Less => Self::get_helper(&n.left, key),
            Ordering::Greater => Self::get_helper(&n.right, key),
            Ordering::Equal => Some(&n.val),
        })
    }

    /// Returns true if the symbol table contains the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::AVLTreeST;
    ///
    /// let mut st = AVLTreeST::new();
    /// st.put("exists", 1);
    /// assert!(st.contains(&"exists"));
    /// assert!(!st.contains(&"missing"));
    /// ```
    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Inserts the specified key-value pair into the symbol table.
    /// If the key already exists, updates its value.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::AVLTreeST;
    ///
    /// let mut st = AVLTreeST::new();
    /// st.put("A", 1);
    /// st.put("B", 2);
    /// assert_eq!(st.get(&"A"), Some(&1));
    /// st.put("A", 10); // Update
    /// assert_eq!(st.get(&"A"), Some(&10));
    /// ```
    pub fn put(&mut self, key: K, val: V) {
        self.root = Self::put_helper(self.root.take(), key, val);
    }

    fn put_helper(node: Option<Box<Node<K, V>>>, key: K, val: V) -> Option<Box<Node<K, V>>> {
        let mut n = match node {
            None => return Some(Box::new(Node::new(key, val))),
            Some(mut n) => {
                match key.cmp(&n.key) {
                    Ordering::Less => n.left = Self::put_helper(n.left.take(), key, val),
                    Ordering::Greater => n.right = Self::put_helper(n.right.take(), key, val),
                    Ordering::Equal => n.val = val,
                }
                n
            }
        };

        n.update_size();
        n.update_height();
        Some(Self::balance(n))
    }

    /// Removes the specified key and its associated value from the symbol table.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::AVLTreeST;
    ///
    /// let mut st = AVLTreeST::new();
    /// st.put("A", 1);
    /// assert_eq!(st.size(), 1);
    /// st.delete(&"A");
    /// assert_eq!(st.size(), 0);
    /// ```
    pub fn delete(&mut self, key: &K) {
        if !self.contains(key) {
            return;
        }
        self.root = Self::delete_helper(self.root.take(), key);
    }

    fn delete_helper(node: Option<Box<Node<K, V>>>, key: &K) -> Option<Box<Node<K, V>>> {
        let mut n = node?;

        match key.cmp(&n.key) {
            Ordering::Less => n.left = Self::delete_helper(n.left.take(), key),
            Ordering::Greater => n.right = Self::delete_helper(n.right.take(), key),
            Ordering::Equal => {
                if n.left.is_none() {
                    return n.right;
                }
                if n.right.is_none() {
                    return n.left;
                }

                // Node with two children: get the inorder successor (min in right subtree)
                let mut t = n;
                // Find min in right subtree
                let min_node = Self::min_node(t.right.as_ref().unwrap());
                // Clone the min node's key and value before modifying the tree
                let min_key = min_node.key.clone();
                let min_val = min_node.val.clone();

                // Create new node with min's key/value
                n = Box::new(Node::new(min_key, min_val));
                // Delete min from right subtree
                n.right = Self::delete_min_helper(t.right.take());
                n.left = t.left;
            }
        }

        n.update_size();
        n.update_height();
        Some(Self::balance(n))
    }

    /// Removes the smallest key from the symbol table.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::AVLTreeST;
    ///
    /// let mut st = AVLTreeST::new();
    /// st.put(3, "three");
    /// st.put(1, "one");
    /// st.put(2, "two");
    /// st.delete_min();
    /// assert_eq!(st.min(), Some(&2));
    /// ```
    pub fn delete_min(&mut self) {
        if self.is_empty() {
            return;
        }
        self.root = Self::delete_min_helper(self.root.take());
    }

    fn delete_min_helper(node: Option<Box<Node<K, V>>>) -> Option<Box<Node<K, V>>> {
        let mut n = node?;

        if n.left.is_none() {
            return n.right;
        }

        n.left = Self::delete_min_helper(n.left.take());
        n.update_size();
        n.update_height();
        Some(Self::balance(n))
    }

    /// Removes the largest key from the symbol table.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::AVLTreeST;
    ///
    /// let mut st = AVLTreeST::new();
    /// st.put(1, "one");
    /// st.put(2, "two");
    /// st.put(3, "three");
    /// st.delete_max();
    /// assert_eq!(st.max(), Some(&2));
    /// ```
    pub fn delete_max(&mut self) {
        if self.is_empty() {
            return;
        }
        self.root = Self::delete_max_helper(self.root.take());
    }

    fn delete_max_helper(node: Option<Box<Node<K, V>>>) -> Option<Box<Node<K, V>>> {
        let mut n = node?;

        if n.right.is_none() {
            return n.left;
        }

        n.right = Self::delete_max_helper(n.right.take());
        n.update_size();
        n.update_height();
        Some(Self::balance(n))
    }

    /// Returns the smallest key in the symbol table.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::AVLTreeST;
    ///
    /// let mut st = AVLTreeST::new();
    /// st.put(3, "three");
    /// st.put(1, "one");
    /// st.put(2, "two");
    /// assert_eq!(st.min(), Some(&1));
    /// ```
    pub fn min(&self) -> Option<&K> {
        self.root.as_ref().map(|n| &Self::min_node(n).key)
    }

    fn min_node(node: &Node<K, V>) -> &Node<K, V> {
        match &node.left {
            Some(left) => Self::min_node(left),
            None => node,
        }
    }

    /// Returns the largest key in the symbol table.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::AVLTreeST;
    ///
    /// let mut st = AVLTreeST::new();
    /// st.put(1, "one");
    /// st.put(2, "two");
    /// st.put(3, "three");
    /// assert_eq!(st.max(), Some(&3));
    /// ```
    pub fn max(&self) -> Option<&K> {
        self.root.as_ref().map(|n| &Self::max_node(n).key)
    }

    fn max_node(node: &Node<K, V>) -> &Node<K, V> {
        match &node.right {
            Some(right) => Self::max_node(right),
            None => node,
        }
    }

    /// Returns the largest key less than or equal to the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::AVLTreeST;
    ///
    /// let mut st = AVLTreeST::new();
    /// st.put(1, "one");
    /// st.put(3, "three");
    /// st.put(5, "five");
    /// assert_eq!(st.floor(&4), Some(&3));
    /// assert_eq!(st.floor(&3), Some(&3));
    /// assert_eq!(st.floor(&0), None);
    /// ```
    pub fn floor(&self, key: &K) -> Option<&K> {
        Self::floor_helper(&self.root, key)
    }

    fn floor_helper<'a>(node: &'a Option<Box<Node<K, V>>>, key: &K) -> Option<&'a K> {
        node.as_ref().and_then(|n| match key.cmp(&n.key) {
            Ordering::Equal => Some(&n.key),
            Ordering::Less => Self::floor_helper(&n.left, key),
            Ordering::Greater => Self::floor_helper(&n.right, key).or(Some(&n.key)),
        })
    }

    /// Returns the smallest key greater than or equal to the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::AVLTreeST;
    ///
    /// let mut st = AVLTreeST::new();
    /// st.put(1, "one");
    /// st.put(3, "three");
    /// st.put(5, "five");
    /// assert_eq!(st.ceiling(&2), Some(&3));
    /// assert_eq!(st.ceiling(&3), Some(&3));
    /// assert_eq!(st.ceiling(&6), None);
    /// ```
    pub fn ceiling(&self, key: &K) -> Option<&K> {
        Self::ceiling_helper(&self.root, key)
    }

    fn ceiling_helper<'a>(node: &'a Option<Box<Node<K, V>>>, key: &K) -> Option<&'a K> {
        node.as_ref().and_then(|n| match key.cmp(&n.key) {
            Ordering::Equal => Some(&n.key),
            Ordering::Greater => Self::ceiling_helper(&n.right, key),
            Ordering::Less => Self::ceiling_helper(&n.left, key).or(Some(&n.key)),
        })
    }

    /// Returns the kth smallest key in the symbol table (0-indexed).
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::AVLTreeST;
    ///
    /// let mut st = AVLTreeST::new();
    /// st.put(3, "three");
    /// st.put(1, "one");
    /// st.put(2, "two");
    /// assert_eq!(st.select(0), Some(&1));
    /// assert_eq!(st.select(1), Some(&2));
    /// assert_eq!(st.select(2), Some(&3));
    /// ```
    pub fn select(&self, k: usize) -> Option<&K> {
        if k >= self.size() {
            return None;
        }
        Self::select_helper(&self.root, k)
    }

    fn select_helper<'a>(node: &'a Option<Box<Node<K, V>>>, k: usize) -> Option<&'a K> {
        node.as_ref().and_then(|n| {
            let left_size = Node::size_of(&n.left);
            match k.cmp(&left_size) {
                Ordering::Less => Self::select_helper(&n.left, k),
                Ordering::Greater => Self::select_helper(&n.right, k - left_size - 1),
                Ordering::Equal => Some(&n.key),
            }
        })
    }

    /// Returns the number of keys less than the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::AVLTreeST;
    ///
    /// let mut st = AVLTreeST::new();
    /// st.put(1, "one");
    /// st.put(3, "three");
    /// st.put(5, "five");
    /// assert_eq!(st.rank(&3), 1);
    /// assert_eq!(st.rank(&4), 2);
    /// ```
    pub fn rank(&self, key: &K) -> usize {
        Self::rank_helper(&self.root, key)
    }

    fn rank_helper(node: &Option<Box<Node<K, V>>>, key: &K) -> usize {
        node.as_ref().map_or(0, |n| match key.cmp(&n.key) {
            Ordering::Less => Self::rank_helper(&n.left, key),
            Ordering::Greater => 1 + Node::size_of(&n.left) + Self::rank_helper(&n.right, key),
            Ordering::Equal => Node::size_of(&n.left),
        })
    }

    /// Returns all keys in the symbol table in sorted order.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::AVLTreeST;
    ///
    /// let mut st = AVLTreeST::new();
    /// st.put(3, "three");
    /// st.put(1, "one");
    /// st.put(2, "two");
    /// let keys: Vec<_> = st.keys().collect();
    /// assert_eq!(keys, vec![&1, &2, &3]);
    /// ```
    pub fn keys(&self) -> Vec<&K> {
        if self.is_empty() {
            return Vec::new();
        }
        let min = self.min().unwrap();
        let max = self.max().unwrap();
        self.keys_range(min, max)
    }

    /// Returns all keys in the given range in sorted order.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::AVLTreeST;
    ///
    /// let mut st = AVLTreeST::new();
    /// for i in 1..=10 {
    ///     st.put(i, i * 10);
    /// }
    /// let keys: Vec<_> = st.keys_range(&3, &7);
    /// assert_eq!(keys, vec![&3, &4, &5, &6, &7]);
    /// ```
    pub fn keys_range(&self, lo: &K, hi: &K) -> Vec<&K> {
        let mut queue = Vec::new();
        Self::keys_helper(&self.root, &mut queue, lo, hi);
        queue
    }

    fn keys_helper<'a>(node: &'a Option<Box<Node<K, V>>>, queue: &mut Vec<&'a K>, lo: &K, hi: &K) {
        if let Some(n) = node {
            if lo < &n.key {
                Self::keys_helper(&n.left, queue, lo, hi);
            }
            if lo <= &n.key && &n.key <= hi {
                queue.push(&n.key);
            }
            if hi > &n.key {
                Self::keys_helper(&n.right, queue, lo, hi);
            }
        }
    }

    /// Returns the number of keys in the given range.
    ///
    /// # Examples
    ///
    /// ```
    /// use searching::AVLTreeST;
    ///
    /// let mut st = AVLTreeST::new();
    /// for i in 1..=10 {
    ///     st.put(i, i * 10);
    /// }
    /// assert_eq!(st.size_range(&3, &7), 5);
    /// ```
    pub fn size_range(&self, lo: &K, hi: &K) -> usize {
        if lo > hi {
            return 0;
        }
        if self.contains(hi) {
            self.rank(hi) - self.rank(lo) + 1
        } else {
            self.rank(hi) - self.rank(lo)
        }
    }

    // Balancing operations

    fn balance(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let balance_factor = node.balance_factor();

        // Left-heavy
        if balance_factor > 1 {
            // Left-right case: rotate left child left first
            if let Some(ref left) = node.left {
                if left.balance_factor() < 0 {
                    node.left = Some(Self::rotate_left(node.left.take().unwrap()));
                }
            }
            // Left-left case: rotate right
            return Self::rotate_right(node);
        }

        // Right-heavy
        if balance_factor < -1 {
            // Right-left case: rotate right child right first
            if let Some(ref right) = node.right {
                if right.balance_factor() > 0 {
                    node.right = Some(Self::rotate_right(node.right.take().unwrap()));
                }
            }
            // Right-right case: rotate left
            return Self::rotate_left(node);
        }

        node
    }

    fn rotate_left(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut new_root = node.right.take().expect("Right child must exist");
        node.right = new_root.left.take();
        node.update_size();
        node.update_height();
        new_root.left = Some(node);
        new_root.update_size();
        new_root.update_height();
        new_root
    }

    fn rotate_right(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut new_root = node.left.take().expect("Left child must exist");
        node.left = new_root.right.take();
        node.update_size();
        node.update_height();
        new_root.right = Some(node);
        new_root.update_size();
        new_root.update_height();
        new_root
    }
}

impl<K: Ord + Clone, V: Clone> Default for AVLTreeST<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord + Clone + Display, V: Clone + Display> Display for AVLTreeST<K, V> {
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
        let st: AVLTreeST<i32, i32> = AVLTreeST::new();
        assert!(st.is_empty());
        assert_eq!(st.size(), 0);
        assert_eq!(st.height(), -1);
    }

    #[test]
    fn test_put_and_get() {
        let mut st = AVLTreeST::new();
        st.put("A", 1);
        st.put("B", 2);
        st.put("C", 3);

        assert_eq!(st.get(&"A"), Some(&1));
        assert_eq!(st.get(&"B"), Some(&2));
        assert_eq!(st.get(&"C"), Some(&3));
        assert_eq!(st.get(&"D"), None);
    }

    #[test]
    fn test_put_update() {
        let mut st = AVLTreeST::new();
        st.put("key", 1);
        assert_eq!(st.get(&"key"), Some(&1));
        st.put("key", 2);
        assert_eq!(st.get(&"key"), Some(&2));
        assert_eq!(st.size(), 1);
    }

    #[test]
    fn test_size_and_height() {
        let mut st = AVLTreeST::new();
        for i in 1..=7 {
            st.put(i, i * 10);
        }
        assert_eq!(st.size(), 7);
        // AVL tree with 7 nodes should have height 2 (balanced)
        assert!(st.height() <= 3);
    }

    #[test]
    fn test_balanced_tree() {
        let mut st = AVLTreeST::new();
        // Insert in order - should still be balanced
        for i in 1..=15 {
            st.put(i, i);
        }
        // Height should be close to log2(15) ≈ 3.9, so at most 4
        assert!(st.height() <= 4);
    }

    #[test]
    fn test_delete() {
        let mut st = AVLTreeST::new();
        st.put(1, "one");
        st.put(2, "two");
        st.put(3, "three");

        assert_eq!(st.size(), 3);
        st.delete(&2);
        assert_eq!(st.size(), 2);
        assert_eq!(st.get(&2), None);
        assert_eq!(st.get(&1), Some(&"one"));
        assert_eq!(st.get(&3), Some(&"three"));
    }

    #[test]
    fn test_delete_min_max() {
        let mut st = AVLTreeST::new();
        for i in 1..=5 {
            st.put(i, i);
        }

        st.delete_min();
        assert_eq!(st.min(), Some(&2));
        assert_eq!(st.size(), 4);

        st.delete_max();
        assert_eq!(st.max(), Some(&4));
        assert_eq!(st.size(), 3);
    }

    #[test]
    fn test_min_max() {
        let mut st = AVLTreeST::new();
        st.put(3, "three");
        st.put(1, "one");
        st.put(5, "five");
        st.put(2, "two");
        st.put(4, "four");

        assert_eq!(st.min(), Some(&1));
        assert_eq!(st.max(), Some(&5));
    }

    #[test]
    fn test_floor_ceiling() {
        let mut st = AVLTreeST::new();
        st.put(1, "one");
        st.put(3, "three");
        st.put(5, "five");
        st.put(7, "seven");

        assert_eq!(st.floor(&4), Some(&3));
        assert_eq!(st.floor(&5), Some(&5));
        assert_eq!(st.floor(&0), None);

        assert_eq!(st.ceiling(&4), Some(&5));
        assert_eq!(st.ceiling(&5), Some(&5));
        assert_eq!(st.ceiling(&8), None);
    }

    #[test]
    fn test_select_rank() {
        let mut st = AVLTreeST::new();
        for i in [3, 1, 5, 2, 4] {
            st.put(i, i * 10);
        }

        assert_eq!(st.select(0), Some(&1));
        assert_eq!(st.select(1), Some(&2));
        assert_eq!(st.select(2), Some(&3));
        assert_eq!(st.select(3), Some(&4));
        assert_eq!(st.select(4), Some(&5));

        assert_eq!(st.rank(&1), 0);
        assert_eq!(st.rank(&2), 1);
        assert_eq!(st.rank(&3), 2);
        assert_eq!(st.rank(&4), 3);
        assert_eq!(st.rank(&5), 4);
    }

    #[test]
    fn test_keys() {
        let mut st = AVLTreeST::new();
        st.put(3, "three");
        st.put(1, "one");
        st.put(5, "five");
        st.put(2, "two");
        st.put(4, "four");

        let keys: Vec<_> = st.keys();
        assert_eq!(keys, vec![&1, &2, &3, &4, &5]);
    }

    #[test]
    fn test_keys_range() {
        let mut st = AVLTreeST::new();
        for i in 1..=10 {
            st.put(i, i);
        }

        let keys = st.keys_range(&3, &7);
        assert_eq!(keys, vec![&3, &4, &5, &6, &7]);
    }

    #[test]
    fn test_size_range() {
        let mut st = AVLTreeST::new();
        for i in 1..=10 {
            st.put(i, i);
        }

        assert_eq!(st.size_range(&3, &7), 5);
        assert_eq!(st.size_range(&1, &10), 10);
        assert_eq!(st.size_range(&5, &5), 1);
    }

    #[test]
    fn test_display() {
        let mut st = AVLTreeST::new();
        st.put(1, "one");
        st.put(2, "two");
        st.put(3, "three");

        let s = format!("{}", st);
        assert!(s.contains("1: one"));
        assert!(s.contains("2: two"));
        assert!(s.contains("3: three"));
    }
}
