//! Linked-list implementation of a LIFO stack.
//!
//! This implementation uses a singly linked list to store items.
//! All operations take constant time in the worst case.
//!
//! # Examples
//!
//! ```
//! use algs4_fundamentals::collections::LinkedStack;
//!
//! let mut stack = LinkedStack::new();
//! stack.push(1);
//! stack.push(2);
//! stack.push(3);
//!
//! assert_eq!(stack.pop(), Some(3));
//! assert_eq!(stack.pop(), Some(2));
//! assert_eq!(stack.size(), 1);
//! ```
//!
//! **Reference:** <https://algs4.cs.princeton.edu/13stacks>

use std::fmt;

/// A LIFO stack implemented using a singly linked list.
///
/// This implementation supports `push`, `pop`, `peek`, `size`, and `is_empty` operations,
/// each in constant worst-case time. It also provides iteration in LIFO order.
///
/// # Type Parameters
///
/// * `T` - the type of items in the stack
#[derive(Debug)]
pub struct LinkedStack<T> {
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

impl<T> LinkedStack<T> {
    /// Initializes an empty stack.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::LinkedStack;
    ///
    /// let stack: LinkedStack<i32> = LinkedStack::new();
    /// assert!(stack.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            first: None,
            size: 0,
        }
    }

    /// Returns true if this stack is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::LinkedStack;
    ///
    /// let mut stack = LinkedStack::new();
    /// assert!(stack.is_empty());
    ///
    /// stack.push(1);
    /// assert!(!stack.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.first.is_none()
    }

    /// Returns the number of items in this stack.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::LinkedStack;
    ///
    /// let mut stack = LinkedStack::new();
    /// assert_eq!(stack.size(), 0);
    ///
    /// stack.push(1);
    /// stack.push(2);
    /// assert_eq!(stack.size(), 2);
    /// ```
    pub fn size(&self) -> usize {
        self.size
    }

    /// Adds the item to this stack.
    ///
    /// # Arguments
    ///
    /// * `item` - the item to add
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::LinkedStack;
    ///
    /// let mut stack = LinkedStack::new();
    /// stack.push("Hello");
    /// stack.push("World");
    /// assert_eq!(stack.size(), 2);
    /// ```
    pub fn push(&mut self, item: T) {
        let old_first = self.first.take();
        self.first = Some(Box::new(Node {
            item,
            next: old_first,
        }));
        self.size += 1;
    }

    /// Removes and returns the item most recently added to this stack.
    ///
    /// Returns `None` if the stack is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::LinkedStack;
    ///
    /// let mut stack = LinkedStack::new();
    /// stack.push(1);
    /// stack.push(2);
    ///
    /// assert_eq!(stack.pop(), Some(2));
    /// assert_eq!(stack.pop(), Some(1));
    /// assert_eq!(stack.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        self.first.take().map(|node| {
            self.first = node.next;
            self.size -= 1;
            node.item
        })
    }

    /// Returns (but does not remove) the item most recently added to this stack.
    ///
    /// Returns `None` if the stack is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::LinkedStack;
    ///
    /// let mut stack = LinkedStack::new();
    /// stack.push(1);
    /// stack.push(2);
    ///
    /// assert_eq!(stack.peek(), Some(&2));
    /// assert_eq!(stack.size(), 2); // peek doesn't remove
    /// ```
    pub fn peek(&self) -> Option<&T> {
        self.first.as_ref().map(|node| &node.item)
    }

    /// Returns an iterator over the items in LIFO order.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::LinkedStack;
    ///
    /// let mut stack = LinkedStack::new();
    /// stack.push(1);
    /// stack.push(2);
    /// stack.push(3);
    ///
    /// let items: Vec<_> = stack.iter().copied().collect();
    /// assert_eq!(items, vec![3, 2, 1]);
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            current: self.first.as_deref(),
        }
    }
}

impl<T> Default for LinkedStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: fmt::Display> fmt::Display for LinkedStack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let items: Vec<String> = self.iter().map(|item| format!("{}", item)).collect();
        write!(f, "{}", items.join(" "))
    }
}

/// An iterator over references to items in a `LinkedStack` in LIFO order.
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

/// An iterator that moves out of a `LinkedStack`.
#[derive(Debug)]
pub struct IntoIter<T> {
    stack: LinkedStack<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

impl<T> IntoIterator for LinkedStack<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { stack: self }
    }
}

impl<'a, T> IntoIterator for &'a LinkedStack<T> {
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
        let stack: LinkedStack<i32> = LinkedStack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn test_push_pop() {
        let mut stack = LinkedStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.size(), 3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_peek() {
        let mut stack = LinkedStack::new();
        assert_eq!(stack.peek(), None);

        stack.push(1);
        stack.push(2);
        assert_eq!(stack.peek(), Some(&2));
        assert_eq!(stack.size(), 2); // peek doesn't remove

        stack.pop();
        assert_eq!(stack.peek(), Some(&1));
    }

    #[test]
    fn test_is_empty() {
        let mut stack = LinkedStack::new();
        assert!(stack.is_empty());

        stack.push(1);
        assert!(!stack.is_empty());

        stack.pop();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_size() {
        let mut stack = LinkedStack::new();
        assert_eq!(stack.size(), 0);

        for i in 0..10 {
            stack.push(i);
            assert_eq!(stack.size(), i + 1);
        }

        for i in (0..10).rev() {
            assert_eq!(stack.size(), i + 1);
            stack.pop();
        }

        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn test_iter() {
        let mut stack = LinkedStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        let items: Vec<_> = stack.iter().copied().collect();
        assert_eq!(items, vec![3, 2, 1]);

        // Stack should still have items after iteration
        assert_eq!(stack.size(), 3);
    }

    #[test]
    fn test_into_iter() {
        let mut stack = LinkedStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        let items: Vec<_> = stack.into_iter().collect();
        assert_eq!(items, vec![3, 2, 1]);
    }

    #[test]
    fn test_display() {
        let mut stack = LinkedStack::new();
        stack.push("to");
        stack.push("be");
        stack.push("or");

        assert_eq!(stack.to_string(), "or be to");
    }

    #[test]
    fn test_strings() {
        let mut stack = LinkedStack::new();
        let input = "to be or not to - be - - that - - - is";

        for word in input.split_whitespace() {
            if word == "-" {
                stack.pop();
            } else {
                stack.push(word);
            }
        }

        // After processing: stack has "is" on top, then "to"
        assert_eq!(stack.to_string(), "is to");
        assert_eq!(stack.size(), 2);
    }

    #[test]
    fn test_default() {
        let stack: LinkedStack<i32> = LinkedStack::default();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_lifo_order() {
        let mut stack = LinkedStack::new();
        for i in 0..100 {
            stack.push(i);
        }

        for i in (0..100).rev() {
            assert_eq!(stack.pop(), Some(i));
        }
    }
}
