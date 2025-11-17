//! Left-leaning Red-Black Binary Search Tree.
//!
//! A red-black BST is a BST where each node has a color (red or black) and satisfies
//! the following properties:
//! - Red links lean left
//! - No node has two red links connected to it
//! - Perfect black balance: every path from root to null has the same number of black links
//!
//! This implementation guarantees O(log n) performance for all operations.
//!
//! # Examples
//!
//! ```
//! use algs4_searching::RedBlackBST;
//!
//! let mut bst = RedBlackBST::new();
//! bst.put("S", 1);
//! bst.put("E", 2);
//! bst.put("A", 3);
//! bst.put("R", 4);
//! bst.put("C", 5);
//! bst.put("H", 6);
//!
//! assert_eq!(bst.get(&"E"), Some(&2));
//! assert_eq!(bst.size(), 6);
//! assert!(bst.height() <= 3); // Balanced tree
//! ```
//!
//! # References
//!
//! - Algorithms, 4th Edition: Section 3.3
//! - Original Java: `RedBlackBST.java`

use std::fmt;

const RED: bool = true;
const BLACK: bool = false;

/// A symbol table implemented with a left-leaning red-black BST.
///
/// This implementation uses a left-leaning red-black BST which guarantees
/// logarithmic performance for all operations through self-balancing.
///
/// # Type Parameters
///
/// * `K` - The key type, must implement `Ord` for ordering
/// * `V` - The value type
///
/// # Performance
///
/// * Search: O(log n) worst case
/// * Insert: O(log n) worst case
/// * Delete: O(log n) worst case
/// * Min/Max: O(log n) worst case
/// * Space: O(n)
#[derive(Debug, Clone)]
pub struct RedBlackBST<K, V> {
    root: Option<Box<Node<K, V>>>,
}

#[derive(Debug, Clone)]
struct Node<K, V> {
    key: K,
    val: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
    n: usize,    // Subtree size
    color: bool, // Color of parent link (RED or BLACK)
}

impl<K, V> Node<K, V> {
    fn new(key: K, val: V, n: usize, color: bool) -> Self {
        Node {
            key,
            val,
            left: None,
            right: None,
            n,
            color,
        }
    }

    fn size(node: &Option<Box<Node<K, V>>>) -> usize {
        node.as_ref().map_or(0, |n| n.n)
    }

    fn is_red(node: &Option<Box<Node<K, V>>>) -> bool {
        node.as_ref().is_some_and(|n| n.color == RED)
    }
}

impl<K, V> RedBlackBST<K, V>
where
    K: Ord,
{
    /// Creates a new empty red-black BST.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::RedBlackBST;
    ///
    /// let bst: RedBlackBST<String, i32> = RedBlackBST::new();
    /// assert!(bst.is_empty());
    /// ```
    pub fn new() -> Self {
        RedBlackBST { root: None }
    }

    /// Returns the number of key-value pairs in the BST.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::RedBlackBST;
    ///
    /// let mut bst = RedBlackBST::new();
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
    /// use algs4_searching::RedBlackBST;
    ///
    /// let mut bst = RedBlackBST::new();
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
    /// use algs4_searching::RedBlackBST;
    ///
    /// let mut bst = RedBlackBST::new();
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
    /// use algs4_searching::RedBlackBST;
    ///
    /// let mut bst = RedBlackBST::new();
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
    /// If the key already exists, its value is updated. The tree is rebalanced
    /// after insertion to maintain red-black properties.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::RedBlackBST;
    ///
    /// let mut bst = RedBlackBST::new();
    /// bst.put("S", 1);
    /// bst.put("E", 2);
    /// bst.put("A", 3);
    ///
    /// assert_eq!(bst.get(&"E"), Some(&2));
    /// assert_eq!(bst.size(), 3);
    /// ```
    pub fn put(&mut self, key: K, val: V) {
        self.root = Self::put_helper(self.root.take(), key, val);
        if let Some(ref mut root) = self.root {
            root.color = BLACK;
        }
    }

    fn put_helper(node: Option<Box<Node<K, V>>>, key: K, val: V) -> Option<Box<Node<K, V>>> {
        let mut h = match node {
            None => return Some(Box::new(Node::new(key, val, 1, RED))),
            Some(n) => n,
        };

        match key.cmp(&h.key) {
            std::cmp::Ordering::Less => {
                h.left = Self::put_helper(h.left.take(), key, val);
            }
            std::cmp::Ordering::Greater => {
                h.right = Self::put_helper(h.right.take(), key, val);
            }
            std::cmp::Ordering::Equal => {
                h.val = val;
            }
        }

        // Fix right-leaning red links
        if Node::is_red(&h.right) && !Node::is_red(&h.left) {
            h = Self::rotate_left(h);
        }
        // Balance 4-node
        let left_left_is_red = h.left.as_ref().is_some_and(|n| Node::is_red(&n.left));
        if Node::is_red(&h.left) && left_left_is_red {
            h = Self::rotate_right(h);
        }
        // Split 4-node
        if Node::is_red(&h.left) && Node::is_red(&h.right) {
            Self::flip_colors(&mut h);
        }

        h.n = 1 + Node::size(&h.left) + Node::size(&h.right);
        Some(h)
    }

    // Rotate right
    fn rotate_right(mut h: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut x = h.left.take().unwrap();
        h.left = x.right.take();
        x.color = h.color;
        h.color = RED;
        x.n = h.n;
        h.n = 1 + Node::size(&h.left) + Node::size(&h.right);
        x.right = Some(h);
        x
    }

    // Rotate left
    fn rotate_left(mut h: Box<Node<K, V>>) -> Box<Node<K, V>> {
        let mut x = h.right.take().unwrap();
        h.right = x.left.take();
        x.color = h.color;
        h.color = RED;
        x.n = h.n;
        h.n = 1 + Node::size(&h.left) + Node::size(&h.right);
        x.left = Some(h);
        x
    }

    // Flip colors
    fn flip_colors(h: &mut Node<K, V>) {
        h.color = !h.color;
        if let Some(ref mut left) = h.left {
            left.color = !left.color;
        }
        if let Some(ref mut right) = h.right {
            right.color = !right.color;
        }
    }

    /// Returns the smallest key in the BST.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::RedBlackBST;
    ///
    /// let mut bst = RedBlackBST::new();
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
    /// use algs4_searching::RedBlackBST;
    ///
    /// let mut bst = RedBlackBST::new();
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

    /// Returns the height of the BST.
    ///
    /// An empty tree has height -1, a tree with one node has height 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::RedBlackBST;
    ///
    /// let mut bst = RedBlackBST::new();
    /// assert_eq!(bst.height(), -1);
    ///
    /// bst.put("S", 1);
    /// assert_eq!(bst.height(), 0);
    ///
    /// bst.put("E", 2);
    /// bst.put("A", 3);
    /// assert!(bst.height() <= 2); // Balanced
    /// ```
    pub fn height(&self) -> isize {
        Self::height_helper(&self.root)
    }

    fn height_helper(node: &Option<Box<Node<K, V>>>) -> isize {
        match node {
            None => -1,
            Some(n) => 1 + Self::height_helper(&n.left).max(Self::height_helper(&n.right)),
        }
    }

    /// Returns all keys in the BST in sorted order.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::RedBlackBST;
    ///
    /// let mut bst = RedBlackBST::new();
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

    /// Checks if the tree satisfies the red-black invariants (for testing).
    #[cfg(test)]
    fn is_red_black(&self) -> bool {
        self.is_balanced() && self.is_23()
    }

    #[cfg(test)]
    fn is_balanced(&self) -> bool {
        let mut black = 0;
        let mut node = &self.root;
        while let Some(n) = node {
            if !n.color {
                black += 1;
            }
            node = &n.left;
        }
        self.is_balanced_helper(&self.root, black)
    }

    #[cfg(test)]
    #[allow(clippy::only_used_in_recursion)]
    fn is_balanced_helper(&self, node: &Option<Box<Node<K, V>>>, black: usize) -> bool {
        match node {
            None => black == 0,
            Some(n) => {
                let mut new_black = black;
                if !n.color {
                    new_black -= 1;
                }
                self.is_balanced_helper(&n.left, new_black)
                    && self.is_balanced_helper(&n.right, new_black)
            }
        }
    }

    #[cfg(test)]
    fn is_23(&self) -> bool {
        self.is_23_helper(&self.root)
    }

    #[cfg(test)]
    #[allow(clippy::only_used_in_recursion)]
    fn is_23_helper(&self, node: &Option<Box<Node<K, V>>>) -> bool {
        match node {
            None => true,
            Some(n) => {
                // No right-leaning red links
                if Node::is_red(&n.right) {
                    return false;
                }
                // No two consecutive red links
                if n.color && Node::is_red(&n.left) {
                    return false;
                }
                self.is_23_helper(&n.left) && self.is_23_helper(&n.right)
            }
        }
    }
}

impl<K, V> Default for RedBlackBST<K, V>
where
    K: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> fmt::Display for RedBlackBST<K, V>
where
    K: fmt::Display,
    V: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RedBlackBST {{ ")?;
        Self::display_helper(&self.root, f, &mut true)?;
        write!(f, " }}")
    }
}

impl<K, V> RedBlackBST<K, V>
where
    K: fmt::Display,
    V: fmt::Display,
{
    fn display_helper(
        node: &Option<Box<Node<K, V>>>,
        f: &mut fmt::Formatter<'_>,
        first: &mut bool,
    ) -> fmt::Result {
        if let Some(n) = node {
            Self::display_helper(&n.left, f, first)?;
            if !*first {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", n.key, n.val)?;
            *first = false;
            Self::display_helper(&n.right, f, first)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bst: RedBlackBST<String, i32> = RedBlackBST::new();
        assert!(bst.is_empty());
        assert_eq!(bst.size(), 0);
    }

    #[test]
    fn test_put_and_get() {
        let mut bst = RedBlackBST::new();
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
        let mut bst = RedBlackBST::new();
        bst.put("S", 1);
        bst.put("E", 2);
        bst.put("A", 3);
        bst.put("R", 4);

        let keys = bst.keys();
        assert_eq!(keys, vec![&"A", &"E", &"R", &"S"]);
    }

    #[test]
    fn test_min_max() {
        let mut bst = RedBlackBST::new();
        bst.put("S", 1);
        bst.put("E", 2);
        bst.put("A", 3);
        bst.put("R", 4);

        assert_eq!(bst.min(), Some(&"A"));
        assert_eq!(bst.max(), Some(&"S"));
    }

    #[test]
    fn test_height_balanced() {
        let mut bst = RedBlackBST::new();

        // Insert many elements
        for i in 0..100 {
            bst.put(i, i * 10);
        }

        // Height should be logarithmic
        let height = bst.height();
        assert!(height < 20); // log2(100) ≈ 6.6, with red nodes ~13
        assert_eq!(bst.size(), 100);
    }

    #[test]
    fn test_red_black_properties() {
        let mut bst = RedBlackBST::new();

        // Insert in sorted order (worst case for regular BST)
        for i in 0..20 {
            bst.put(i, i * 10);
            // Check invariants after each insertion
            assert!(
                bst.is_red_black(),
                "Invariants violated after inserting {}",
                i
            );
        }
    }

    #[test]
    fn test_red_black_properties_reverse() {
        let mut bst = RedBlackBST::new();

        // Insert in reverse sorted order
        for i in (0..20).rev() {
            bst.put(i, i * 10);
            assert!(
                bst.is_red_black(),
                "Invariants violated after inserting {}",
                i
            );
        }
    }

    #[test]
    fn test_put_update() {
        let mut bst = RedBlackBST::new();
        bst.put("A", 1);
        assert_eq!(bst.get(&"A"), Some(&1));
        assert_eq!(bst.size(), 1);

        bst.put("A", 10);
        assert_eq!(bst.get(&"A"), Some(&10));
        assert_eq!(bst.size(), 1);
    }

    #[test]
    fn test_with_integers() {
        let mut bst = RedBlackBST::new();
        bst.put(5, "five");
        bst.put(3, "three");
        bst.put(7, "seven");
        bst.put(1, "one");
        bst.put(9, "nine");

        assert_eq!(bst.min(), Some(&1));
        assert_eq!(bst.max(), Some(&9));
        assert!(bst.is_red_black());
    }

    #[test]
    fn test_empty_operations() {
        let bst: RedBlackBST<&str, i32> = RedBlackBST::new();

        assert_eq!(bst.get(&"key"), None);
        assert!(!bst.contains(&"key"));
        assert_eq!(bst.min(), None);
        assert_eq!(bst.max(), None);
        assert_eq!(bst.keys().len(), 0);
    }

    #[test]
    fn test_display() {
        let mut bst = RedBlackBST::new();
        bst.put("apple", 1);
        bst.put("banana", 2);

        let display = format!("{}", bst);
        assert!(display.contains("apple"));
        assert!(display.contains("banana"));
    }

    #[test]
    fn test_large_tree_balance() {
        let mut bst = RedBlackBST::new();

        // Insert 1000 elements
        for i in 0..1000 {
            bst.put(i, i);
        }

        // Tree should remain balanced
        assert!(bst.is_red_black());
        assert_eq!(bst.size(), 1000);

        // Height should be roughly 2*log2(n)
        let height = bst.height();
        assert!(height < 25); // 2*log2(1000) ≈ 20
    }
}
