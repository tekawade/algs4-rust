//! Array-based implementation of a LIFO stack with automatic resizing.
//!
//! This implementation uses a resizing array to store items.
//! The array doubles in size when full and shrinks to half when one-quarter full.
//! This provides amortized constant time for push and pop operations.
//!
//! # Examples
//!
//! ```
//! use algs4_fundamentals::collections::ResizingArrayStack;
//!
//! let mut stack = ResizingArrayStack::new();
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

/// A LIFO stack implemented using a resizing array.
///
/// This implementation supports `push`, `pop`, `peek`, `size`, and `is_empty` operations
/// with amortized constant time. It also provides iteration in LIFO order.
///
/// The array capacity doubles when it becomes full and shrinks to half when it becomes
/// one-quarter full, ensuring efficient memory usage.
///
/// # Type Parameters
///
/// * `T` - the type of items in the stack
#[derive(Debug)]
pub struct ResizingArrayStack<T> {
    items: Vec<T>,
    size: usize,
}

impl<T> ResizingArrayStack<T> {
    /// Initializes an empty stack with initial capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::ResizingArrayStack;
    ///
    /// let stack: ResizingArrayStack<i32> = ResizingArrayStack::new();
    /// assert!(stack.is_empty());
    /// ```
    pub fn new() -> Self {
        Self::with_capacity(8)
    }

    /// Initializes an empty stack with the specified capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - the initial capacity
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::ResizingArrayStack;
    ///
    /// let stack: ResizingArrayStack<i32> = ResizingArrayStack::with_capacity(16);
    /// assert!(stack.is_empty());
    /// assert_eq!(stack.capacity(), 16);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
            size: 0,
        }
    }

    /// Returns true if this stack is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::ResizingArrayStack;
    ///
    /// let mut stack = ResizingArrayStack::new();
    /// assert!(stack.is_empty());
    ///
    /// stack.push(1);
    /// assert!(!stack.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Returns the number of items in this stack.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::ResizingArrayStack;
    ///
    /// let mut stack = ResizingArrayStack::new();
    /// assert_eq!(stack.size(), 0);
    ///
    /// stack.push(1);
    /// stack.push(2);
    /// assert_eq!(stack.size(), 2);
    /// ```
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns the current capacity of the underlying array.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::ResizingArrayStack;
    ///
    /// let mut stack = ResizingArrayStack::new();
    /// let initial_capacity = stack.capacity();
    ///
    /// for i in 0..10 {
    ///     stack.push(i);
    /// }
    ///
    /// // Capacity should have grown
    /// assert!(stack.capacity() >= initial_capacity);
    /// ```
    pub fn capacity(&self) -> usize {
        self.items.capacity()
    }

    /// Adds the item to this stack.
    ///
    /// Doubles the underlying array size if necessary.
    ///
    /// # Arguments
    ///
    /// * `item` - the item to add
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::ResizingArrayStack;
    ///
    /// let mut stack = ResizingArrayStack::new();
    /// stack.push("Hello");
    /// stack.push("World");
    /// assert_eq!(stack.size(), 2);
    /// ```
    pub fn push(&mut self, item: T) {
        // Resize if at capacity
        if self.size == self.items.capacity() {
            self.resize(2 * self.items.capacity().max(1));
        }

        self.items.push(item);
        self.size += 1;
    }

    /// Removes and returns the item most recently added to this stack.
    ///
    /// Returns `None` if the stack is empty.
    /// Halves the underlying array size if the stack is one-quarter full.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::ResizingArrayStack;
    ///
    /// let mut stack = ResizingArrayStack::new();
    /// stack.push(1);
    /// stack.push(2);
    ///
    /// assert_eq!(stack.pop(), Some(2));
    /// assert_eq!(stack.pop(), Some(1));
    /// assert_eq!(stack.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let item = self.items.pop();
        self.size -= 1;

        // Shrink if one-quarter full
        let capacity = self.items.capacity();
        if self.size > 0 && self.size <= capacity / 4 {
            self.resize(capacity / 2);
        }

        item
    }

    /// Returns (but does not remove) the item most recently added to this stack.
    ///
    /// Returns `None` if the stack is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::ResizingArrayStack;
    ///
    /// let mut stack = ResizingArrayStack::new();
    /// stack.push(1);
    /// stack.push(2);
    ///
    /// assert_eq!(stack.peek(), Some(&2));
    /// assert_eq!(stack.size(), 2); // peek doesn't remove
    /// ```
    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.items.get(self.size - 1)
        }
    }

    /// Returns an iterator over the items in LIFO order.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::ResizingArrayStack;
    ///
    /// let mut stack = ResizingArrayStack::new();
    /// stack.push(1);
    /// stack.push(2);
    /// stack.push(3);
    ///
    /// let items: Vec<_> = stack.iter().copied().collect();
    /// assert_eq!(items, vec![3, 2, 1]);
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            items: &self.items[0..self.size],
            index: self.size,
        }
    }

    /// Resizes the underlying array to the specified capacity.
    fn resize(&mut self, capacity: usize) {
        let mut new_vec = Vec::with_capacity(capacity);
        new_vec.append(&mut self.items);
        self.items = new_vec;
    }
}

impl<T> Default for ResizingArrayStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: fmt::Display> fmt::Display for ResizingArrayStack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let items: Vec<String> = self.iter().map(|item| format!("{}", item)).collect();
        write!(f, "{}", items.join(" "))
    }
}

/// An iterator over references to items in a `ResizingArrayStack` in LIFO order.
#[derive(Debug)]
pub struct Iter<'a, T> {
    items: &'a [T],
    index: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > 0 {
            self.index -= 1;
            Some(&self.items[self.index])
        } else {
            None
        }
    }
}

/// An iterator that moves out of a `ResizingArrayStack`.
#[derive(Debug)]
pub struct IntoIter<T> {
    stack: ResizingArrayStack<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

impl<T> IntoIterator for ResizingArrayStack<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { stack: self }
    }
}

impl<'a, T> IntoIterator for &'a ResizingArrayStack<T> {
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
        let stack: ResizingArrayStack<i32> = ResizingArrayStack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.size(), 0);
        assert!(stack.capacity() >= 8);
    }

    #[test]
    fn test_with_capacity() {
        let stack: ResizingArrayStack<i32> = ResizingArrayStack::with_capacity(16);
        assert!(stack.is_empty());
        assert_eq!(stack.capacity(), 16);
    }

    #[test]
    fn test_push_pop() {
        let mut stack = ResizingArrayStack::new();
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
        let mut stack = ResizingArrayStack::new();
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
        let mut stack = ResizingArrayStack::new();
        assert!(stack.is_empty());

        stack.push(1);
        assert!(!stack.is_empty());

        stack.pop();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_size() {
        let mut stack = ResizingArrayStack::new();
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
    fn test_resizing() {
        let mut stack = ResizingArrayStack::with_capacity(2);
        assert_eq!(stack.capacity(), 2);

        // Fill beyond initial capacity
        for i in 0..10 {
            stack.push(i);
        }

        // Should have resized up
        assert!(stack.capacity() >= 10);
        assert_eq!(stack.size(), 10);

        // Remove most items
        for _ in 0..8 {
            stack.pop();
        }

        // Should have resized down
        assert_eq!(stack.size(), 2);
    }

    #[test]
    fn test_iter() {
        let mut stack = ResizingArrayStack::new();
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
        let mut stack = ResizingArrayStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        let items: Vec<_> = stack.into_iter().collect();
        assert_eq!(items, vec![3, 2, 1]);
    }

    #[test]
    fn test_display() {
        let mut stack = ResizingArrayStack::new();
        stack.push("to");
        stack.push("be");
        stack.push("or");

        assert_eq!(stack.to_string(), "or be to");
    }

    #[test]
    fn test_strings() {
        let mut stack = ResizingArrayStack::new();
        let input = "to be or not to - be - - that - - - is";

        for word in input.split_whitespace() {
            if word == "-" {
                stack.pop();
            } else {
                stack.push(word);
            }
        }

        assert_eq!(stack.to_string(), "is to");
        assert_eq!(stack.size(), 2);
    }

    #[test]
    fn test_default() {
        let stack: ResizingArrayStack<i32> = ResizingArrayStack::default();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_lifo_order() {
        let mut stack = ResizingArrayStack::new();
        for i in 0..100 {
            stack.push(i);
        }

        for i in (0..100).rev() {
            assert_eq!(stack.pop(), Some(i));
        }
    }

    #[test]
    fn test_large_stack() {
        let mut stack = ResizingArrayStack::new();

        // Push many items
        for i in 0..1000 {
            stack.push(i);
        }

        assert_eq!(stack.size(), 1000);

        // Pop many items
        for i in (0..1000).rev() {
            assert_eq!(stack.pop(), Some(i));
        }

        assert!(stack.is_empty());
    }
}
