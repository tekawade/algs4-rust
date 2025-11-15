//! Linked-list implementation of a FIFO queue.
//!
//! This implementation uses a singly linked list with pointers to both
//! the first and last nodes. All operations take constant time in the worst case.
//!
//! # Examples
//!
//! ```
//! use algs4_fundamentals::collections::LinkedQueue;
//!
//! let mut queue = LinkedQueue::new();
//! queue.enqueue(1);
//! queue.enqueue(2);
//! queue.enqueue(3);
//!
//! assert_eq!(queue.dequeue(), Some(1));
//! assert_eq!(queue.dequeue(), Some(2));
//! assert_eq!(queue.size(), 1);
//! ```
//!
//! **Reference:** <https://algs4.cs.princeton.edu/13stacks>

use std::fmt;
use std::ptr::NonNull;

/// A FIFO queue implemented using a singly linked list.
///
/// This implementation supports `enqueue`, `dequeue`, `peek`, `size`, and `is_empty` operations,
/// each in constant worst-case time. It also provides iteration in FIFO order.
///
/// # Type Parameters
///
/// * `T` - the type of items in the queue
#[derive(Debug)]
pub struct LinkedQueue<T> {
    first: Option<Box<Node<T>>>,
    last: Option<NonNull<Node<T>>>,
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

impl<T> LinkedQueue<T> {
    /// Initializes an empty queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::LinkedQueue;
    ///
    /// let queue: LinkedQueue<i32> = LinkedQueue::new();
    /// assert!(queue.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            first: None,
            last: None,
            size: 0,
        }
    }

    /// Returns true if this queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::LinkedQueue;
    ///
    /// let mut queue = LinkedQueue::new();
    /// assert!(queue.is_empty());
    ///
    /// queue.enqueue(1);
    /// assert!(!queue.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.first.is_none()
    }

    /// Returns the number of items in this queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::LinkedQueue;
    ///
    /// let mut queue = LinkedQueue::new();
    /// assert_eq!(queue.size(), 0);
    ///
    /// queue.enqueue(1);
    /// queue.enqueue(2);
    /// assert_eq!(queue.size(), 2);
    /// ```
    pub fn size(&self) -> usize {
        self.size
    }

    /// Adds the item to this queue.
    ///
    /// # Arguments
    ///
    /// * `item` - the item to add
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::LinkedQueue;
    ///
    /// let mut queue = LinkedQueue::new();
    /// queue.enqueue("Hello");
    /// queue.enqueue("World");
    /// assert_eq!(queue.size(), 2);
    /// ```
    pub fn enqueue(&mut self, item: T) {
        let mut new_node = Box::new(Node { item, next: None });
        let new_last = NonNull::from(&mut *new_node);

        match self.last {
            Some(mut old_last) => {
                // Add to end of existing queue
                unsafe {
                    old_last.as_mut().next = Some(new_node);
                }
            }
            None => {
                // Queue was empty
                self.first = Some(new_node);
            }
        }

        self.last = Some(new_last);
        self.size += 1;
    }

    /// Removes and returns the item least recently added to this queue.
    ///
    /// Returns `None` if the queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::LinkedQueue;
    ///
    /// let mut queue = LinkedQueue::new();
    /// queue.enqueue(1);
    /// queue.enqueue(2);
    ///
    /// assert_eq!(queue.dequeue(), Some(1));
    /// assert_eq!(queue.dequeue(), Some(2));
    /// assert_eq!(queue.dequeue(), None);
    /// ```
    pub fn dequeue(&mut self) -> Option<T> {
        self.first.take().map(|node| {
            self.first = node.next;
            self.size -= 1;

            // Clear last pointer if queue is now empty
            if self.first.is_none() {
                self.last = None;
            }

            node.item
        })
    }

    /// Returns (but does not remove) the item least recently added to this queue.
    ///
    /// Returns `None` if the queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::LinkedQueue;
    ///
    /// let mut queue = LinkedQueue::new();
    /// queue.enqueue(1);
    /// queue.enqueue(2);
    ///
    /// assert_eq!(queue.peek(), Some(&1));
    /// assert_eq!(queue.size(), 2); // peek doesn't remove
    /// ```
    pub fn peek(&self) -> Option<&T> {
        self.first.as_ref().map(|node| &node.item)
    }

    /// Returns an iterator over the items in FIFO order.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::LinkedQueue;
    ///
    /// let mut queue = LinkedQueue::new();
    /// queue.enqueue(1);
    /// queue.enqueue(2);
    /// queue.enqueue(3);
    ///
    /// let items: Vec<_> = queue.iter().copied().collect();
    /// assert_eq!(items, vec![1, 2, 3]);
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            current: self.first.as_deref(),
        }
    }
}

impl<T> Default for LinkedQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: fmt::Display> fmt::Display for LinkedQueue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let items: Vec<String> = self.iter().map(|item| format!("{}", item)).collect();
        write!(f, "{}", items.join(" "))
    }
}

/// An iterator over references to items in a `LinkedQueue` in FIFO order.
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

/// An iterator that moves out of a `LinkedQueue`.
#[derive(Debug)]
pub struct IntoIter<T> {
    queue: LinkedQueue<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.dequeue()
    }
}

impl<T> IntoIterator for LinkedQueue<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { queue: self }
    }
}

impl<'a, T> IntoIterator for &'a LinkedQueue<T> {
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
        let queue: LinkedQueue<i32> = LinkedQueue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.size(), 0);
    }

    #[test]
    fn test_enqueue_dequeue() {
        let mut queue = LinkedQueue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.size(), 3);
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);
        assert!(queue.is_empty());
    }

    #[test]
    fn test_peek() {
        let mut queue = LinkedQueue::new();
        assert_eq!(queue.peek(), None);

        queue.enqueue(1);
        queue.enqueue(2);
        assert_eq!(queue.peek(), Some(&1));
        assert_eq!(queue.size(), 2); // peek doesn't remove

        queue.dequeue();
        assert_eq!(queue.peek(), Some(&2));
    }

    #[test]
    fn test_is_empty() {
        let mut queue = LinkedQueue::new();
        assert!(queue.is_empty());

        queue.enqueue(1);
        assert!(!queue.is_empty());

        queue.dequeue();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_size() {
        let mut queue = LinkedQueue::new();
        assert_eq!(queue.size(), 0);

        for i in 0..10 {
            queue.enqueue(i);
            assert_eq!(queue.size(), i + 1);
        }

        for i in (0..10).rev() {
            assert_eq!(queue.size(), i + 1);
            queue.dequeue();
        }

        assert_eq!(queue.size(), 0);
    }

    #[test]
    fn test_iter() {
        let mut queue = LinkedQueue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        let items: Vec<_> = queue.iter().copied().collect();
        assert_eq!(items, vec![1, 2, 3]);

        // Queue should still have items after iteration
        assert_eq!(queue.size(), 3);
    }

    #[test]
    fn test_into_iter() {
        let mut queue = LinkedQueue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        let items: Vec<_> = queue.into_iter().collect();
        assert_eq!(items, vec![1, 2, 3]);
    }

    #[test]
    fn test_display() {
        let mut queue = LinkedQueue::new();
        queue.enqueue("to");
        queue.enqueue("be");
        queue.enqueue("or");

        assert_eq!(queue.to_string(), "to be or");
    }

    #[test]
    fn test_fifo_order() {
        let mut queue = LinkedQueue::new();
        for i in 0..100 {
            queue.enqueue(i);
        }

        for i in 0..100 {
            assert_eq!(queue.dequeue(), Some(i));
        }
    }

    #[test]
    fn test_interleaved_operations() {
        let mut queue = LinkedQueue::new();

        queue.enqueue(1);
        queue.enqueue(2);
        assert_eq!(queue.dequeue(), Some(1));

        queue.enqueue(3);
        queue.enqueue(4);
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));

        queue.enqueue(5);
        assert_eq!(queue.dequeue(), Some(4));
        assert_eq!(queue.dequeue(), Some(5));
        assert!(queue.is_empty());
    }

    #[test]
    fn test_default() {
        let queue: LinkedQueue<i32> = LinkedQueue::default();
        assert!(queue.is_empty());
    }
}
