//! Linked-list implementation of a bag (multiset).
//!
//! A bag is an unordered collection that allows duplicates.
//! This implementation uses a singly linked list to store items.
//! The `add` operation takes constant time, and iteration takes time proportional to the number of items.
//!
//! # Examples
//!
//! ```
//! use algs4_fundamentals::collections::LinkedBag;
//!
//! let mut bag = LinkedBag::new();
//! bag.add(1);
//! bag.add(2);
//! bag.add(1); // duplicates allowed
//!
//! assert_eq!(bag.size(), 3);
//! ```
//!
//! **Reference:** <https://algs4.cs.princeton.edu/13stacks>

use std::fmt;

/// A bag (multiset) implemented using a singly linked list.
///
/// A bag is an unordered collection that allows duplicates.
/// This implementation supports `add`, `size`, and `is_empty` operations in constant time,
/// and provides iteration through all items.
///
/// # Type Parameters
///
/// * `T` - the type of items in the bag
#[derive(Debug)]
pub struct LinkedBag<T> {
    first: Option<Box<Node<T>>>,
    size: usize,
}

/// Internal node structure for the linked list.
struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
}

impl<T: fmt::Debug> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("item", &self.item)
            .field("has_next", &self.next.is_some())
            .finish()
    }
}

impl<T> LinkedBag<T> {
    /// Initializes an empty bag.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::LinkedBag;
    ///
    /// let bag: LinkedBag<i32> = LinkedBag::new();
    /// assert!(bag.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            first: None,
            size: 0,
        }
    }

    /// Returns true if this bag is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::LinkedBag;
    ///
    /// let mut bag = LinkedBag::new();
    /// assert!(bag.is_empty());
    ///
    /// bag.add(1);
    /// assert!(!bag.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.first.is_none()
    }

    /// Returns the number of items in this bag.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::LinkedBag;
    ///
    /// let mut bag = LinkedBag::new();
    /// assert_eq!(bag.size(), 0);
    ///
    /// bag.add(1);
    /// bag.add(2);
    /// assert_eq!(bag.size(), 2);
    /// ```
    pub fn size(&self) -> usize {
        self.size
    }

    /// Adds the item to this bag.
    ///
    /// # Arguments
    ///
    /// * `item` - the item to add
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::LinkedBag;
    ///
    /// let mut bag = LinkedBag::new();
    /// bag.add("Hello");
    /// bag.add("World");
    /// bag.add("Hello"); // duplicates allowed
    /// assert_eq!(bag.size(), 3);
    /// ```
    pub fn add(&mut self, item: T) {
        let old_first = self.first.take();
        self.first = Some(Box::new(Node {
            item,
            next: old_first,
        }));
        self.size += 1;
    }

    /// Returns an iterator over the items in this bag.
    ///
    /// The order of iteration is arbitrary (in this implementation, reverse insertion order).
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::LinkedBag;
    ///
    /// let mut bag = LinkedBag::new();
    /// bag.add(1);
    /// bag.add(2);
    /// bag.add(3);
    ///
    /// let count = bag.iter().count();
    /// assert_eq!(count, 3);
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            current: self.first.as_deref(),
        }
    }
}

impl<T> Default for LinkedBag<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: fmt::Display> fmt::Display for LinkedBag<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let items: Vec<String> = self.iter().map(|item| format!("{}", item)).collect();
        write!(f, "{}", items.join(" "))
    }
}

/// An iterator over references to items in a `LinkedBag`.
#[derive(Debug)]
pub struct Iter<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_deref();
            &node.item
        })
    }
}

/// An iterator that moves out of a `LinkedBag`.
#[derive(Debug)]
pub struct IntoIter<T> {
    bag: LinkedBag<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.bag.first.take().map(|node| {
            self.bag.first = node.next;
            self.bag.size -= 1;
            node.item
        })
    }
}

impl<T> IntoIterator for LinkedBag<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { bag: self }
    }
}

impl<'a, T> IntoIterator for &'a LinkedBag<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bag: LinkedBag<i32> = LinkedBag::new();
        assert!(bag.is_empty());
        assert_eq!(bag.size(), 0);
    }

    #[test]
    fn test_add() {
        let mut bag = LinkedBag::new();
        bag.add(1);
        bag.add(2);
        bag.add(3);

        assert_eq!(bag.size(), 3);
        assert!(!bag.is_empty());
    }

    #[test]
    fn test_is_empty() {
        let mut bag = LinkedBag::new();
        assert!(bag.is_empty());

        bag.add(1);
        assert!(!bag.is_empty());
    }

    #[test]
    fn test_size() {
        let mut bag = LinkedBag::new();
        assert_eq!(bag.size(), 0);

        for i in 0..10 {
            bag.add(i);
            assert_eq!(bag.size(), i + 1);
        }
    }

    #[test]
    fn test_duplicates() {
        let mut bag = LinkedBag::new();
        bag.add(1);
        bag.add(1);
        bag.add(1);

        assert_eq!(bag.size(), 3);
        let count = bag.iter().filter(|&&x| x == 1).count();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_iter() {
        let mut bag = LinkedBag::new();
        bag.add(1);
        bag.add(2);
        bag.add(3);

        let items: Vec<_> = bag.iter().copied().collect();
        assert_eq!(items.len(), 3);
        assert!(items.contains(&1));
        assert!(items.contains(&2));
        assert!(items.contains(&3));

        // Bag should still have items after iteration
        assert_eq!(bag.size(), 3);
    }

    #[test]
    fn test_into_iter() {
        let mut bag = LinkedBag::new();
        bag.add(1);
        bag.add(2);
        bag.add(3);

        let items: Vec<_> = bag.into_iter().collect();
        assert_eq!(items.len(), 3);
        assert!(items.contains(&1));
        assert!(items.contains(&2));
        assert!(items.contains(&3));
    }

    #[test]
    fn test_display() {
        let mut bag = LinkedBag::new();
        bag.add("to");
        bag.add("be");
        bag.add("or");

        let display = bag.to_string();
        assert!(display.contains("to"));
        assert!(display.contains("be"));
        assert!(display.contains("or"));
    }

    #[test]
    fn test_default() {
        let bag: LinkedBag<i32> = LinkedBag::default();
        assert!(bag.is_empty());
    }

    #[test]
    fn test_large_bag() {
        let mut bag = LinkedBag::new();
        for i in 0..1000 {
            bag.add(i);
        }

        assert_eq!(bag.size(), 1000);
        let count = bag.iter().count();
        assert_eq!(count, 1000);
    }

    #[test]
    fn test_strings() {
        let mut bag = LinkedBag::new();
        let input = "to be or not to be";

        for word in input.split_whitespace() {
            bag.add(word);
        }

        assert_eq!(bag.size(), 6);

        // Count occurrences of "be"
        let be_count = bag.iter().filter(|&&w| w == "be").count();
        assert_eq!(be_count, 2);

        // Count occurrences of "to"
        let to_count = bag.iter().filter(|&&w| w == "to").count();
        assert_eq!(to_count, 2);
    }
}
