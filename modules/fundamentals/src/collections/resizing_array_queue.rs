//! Array-based implementation of a FIFO queue with automatic resizing.
//!
//! This implementation uses a circular buffer with automatic resizing.
//! The array doubles in size when full and shrinks to half when one-quarter full.
//! This provides amortized constant time for enqueue and dequeue operations.
//!
//! # Examples
//!
//! ```
//! use algs4_fundamentals::collections::ResizingArrayQueue;
//!
//! let mut queue = ResizingArrayQueue::new();
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

/// A FIFO queue implemented using a resizing circular array.
///
/// This implementation supports `enqueue`, `dequeue`, `peek`, `size`, and `is_empty` operations
/// with amortized constant time. It also provides iteration in FIFO order.
///
/// The array capacity doubles when it becomes full and shrinks to half when it becomes
/// one-quarter full, ensuring efficient memory usage.
///
/// # Type Parameters
///
/// * `T` - the type of items in the queue
pub struct ResizingArrayQueue<T> {
    items: Vec<Option<T>>,
    first: usize, // index of first element
    last: usize,  // index of next available slot
    size: usize,
}

impl<T: fmt::Debug> fmt::Debug for ResizingArrayQueue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ResizingArrayQueue")
            .field("size", &self.size)
            .field("capacity", &self.items.len())
            .field("first", &self.first)
            .field("last", &self.last)
            .finish()
    }
}

impl<T> ResizingArrayQueue<T> {
    /// Initializes an empty queue with initial capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::ResizingArrayQueue;
    ///
    /// let queue: ResizingArrayQueue<i32> = ResizingArrayQueue::new();
    /// assert!(queue.is_empty());
    /// ```
    pub fn new() -> Self {
        Self::with_capacity(8)
    }

    /// Initializes an empty queue with the specified capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - the initial capacity
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::ResizingArrayQueue;
    ///
    /// let queue: ResizingArrayQueue<i32> = ResizingArrayQueue::with_capacity(16);
    /// assert!(queue.is_empty());
    /// assert_eq!(queue.capacity(), 16);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        let mut items = Vec::with_capacity(capacity);
        items.resize_with(capacity, || None);
        Self {
            items,
            first: 0,
            last: 0,
            size: 0,
        }
    }

    /// Returns true if this queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::ResizingArrayQueue;
    ///
    /// let mut queue = ResizingArrayQueue::new();
    /// assert!(queue.is_empty());
    ///
    /// queue.enqueue(1);
    /// assert!(!queue.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Returns the number of items in this queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::ResizingArrayQueue;
    ///
    /// let mut queue = ResizingArrayQueue::new();
    /// assert_eq!(queue.size(), 0);
    ///
    /// queue.enqueue(1);
    /// queue.enqueue(2);
    /// assert_eq!(queue.size(), 2);
    /// ```
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns the current capacity of the underlying array.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::ResizingArrayQueue;
    ///
    /// let mut queue = ResizingArrayQueue::new();
    /// let initial_capacity = queue.capacity();
    ///
    /// for i in 0..10 {
    ///     queue.enqueue(i);
    /// }
    ///
    /// // Capacity should have grown
    /// assert!(queue.capacity() >= initial_capacity);
    /// ```
    pub fn capacity(&self) -> usize {
        self.items.len()
    }

    /// Adds the item to this queue.
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
    /// use algs4_fundamentals::collections::ResizingArrayQueue;
    ///
    /// let mut queue = ResizingArrayQueue::new();
    /// queue.enqueue("Hello");
    /// queue.enqueue("World");
    /// assert_eq!(queue.size(), 2);
    /// ```
    pub fn enqueue(&mut self, item: T) {
        // Resize if at capacity
        if self.size == self.items.len() {
            self.resize(2 * self.items.len());
        }

        self.items[self.last] = Some(item);
        self.last = (self.last + 1) % self.items.len();
        self.size += 1;
    }

    /// Removes and returns the item least recently added to this queue.
    ///
    /// Returns `None` if the queue is empty.
    /// Halves the underlying array size if the queue is one-quarter full.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::ResizingArrayQueue;
    ///
    /// let mut queue = ResizingArrayQueue::new();
    /// queue.enqueue(1);
    /// queue.enqueue(2);
    ///
    /// assert_eq!(queue.dequeue(), Some(1));
    /// assert_eq!(queue.dequeue(), Some(2));
    /// assert_eq!(queue.dequeue(), None);
    /// ```
    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let item = self.items[self.first].take();
        self.first = (self.first + 1) % self.items.len();
        self.size -= 1;

        // Shrink if one-quarter full
        if self.size > 0 && self.size <= self.items.len() / 4 {
            self.resize(self.items.len() / 2);
        }

        item
    }

    /// Returns (but does not remove) the item least recently added to this queue.
    ///
    /// Returns `None` if the queue is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::ResizingArrayQueue;
    ///
    /// let mut queue = ResizingArrayQueue::new();
    /// queue.enqueue(1);
    /// queue.enqueue(2);
    ///
    /// assert_eq!(queue.peek(), Some(&1));
    /// assert_eq!(queue.size(), 2); // peek doesn't remove
    /// ```
    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.items[self.first].as_ref()
        }
    }

    /// Returns an iterator over the items in FIFO order.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::ResizingArrayQueue;
    ///
    /// let mut queue = ResizingArrayQueue::new();
    /// queue.enqueue(1);
    /// queue.enqueue(2);
    /// queue.enqueue(3);
    ///
    /// let items: Vec<_> = queue.iter().copied().collect();
    /// assert_eq!(items, vec![1, 2, 3]);
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            items: &self.items,
            first: self.first,
            size: self.size,
            index: 0,
        }
    }

    /// Resizes the underlying array to the specified capacity.
    /// Copies elements in FIFO order starting from index 0.
    fn resize(&mut self, capacity: usize) {
        let mut new_items = Vec::with_capacity(capacity);
        new_items.resize_with(capacity, || None);

        // Copy items in order from first to last
        for (i, new_item) in new_items.iter_mut().enumerate().take(self.size) {
            let src_index = (self.first + i) % self.items.len();
            *new_item = self.items[src_index].take();
        }

        self.items = new_items;
        self.first = 0;
        self.last = self.size;
    }
}

impl<T> Default for ResizingArrayQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: fmt::Display> fmt::Display for ResizingArrayQueue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let items: Vec<String> = self.iter().map(|item| format!("{}", item)).collect();
        write!(f, "{}", items.join(" "))
    }
}

/// An iterator over references to items in a `ResizingArrayQueue` in FIFO order.
#[derive(Debug)]
pub struct Iter<'a, T> {
    items: &'a [Option<T>],
    first: usize,
    size: usize,
    index: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.size {
            let actual_index = (self.first + self.index) % self.items.len();
            self.index += 1;
            self.items[actual_index].as_ref()
        } else {
            None
        }
    }
}

/// An iterator that moves out of a `ResizingArrayQueue`.
#[derive(Debug)]
pub struct IntoIter<T> {
    queue: ResizingArrayQueue<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.dequeue()
    }
}

impl<T> IntoIterator for ResizingArrayQueue<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { queue: self }
    }
}

impl<'a, T> IntoIterator for &'a ResizingArrayQueue<T> {
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
        let queue: ResizingArrayQueue<i32> = ResizingArrayQueue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.size(), 0);
        assert!(queue.capacity() >= 8);
    }

    #[test]
    fn test_with_capacity() {
        let queue: ResizingArrayQueue<i32> = ResizingArrayQueue::with_capacity(16);
        assert!(queue.is_empty());
        assert_eq!(queue.capacity(), 16);
    }

    #[test]
    fn test_enqueue_dequeue() {
        let mut queue = ResizingArrayQueue::new();
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
        let mut queue = ResizingArrayQueue::new();
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
        let mut queue = ResizingArrayQueue::new();
        assert!(queue.is_empty());

        queue.enqueue(1);
        assert!(!queue.is_empty());

        queue.dequeue();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_size() {
        let mut queue = ResizingArrayQueue::new();
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
    fn test_resizing() {
        let mut queue = ResizingArrayQueue::with_capacity(2);
        assert_eq!(queue.capacity(), 2);

        // Fill beyond initial capacity
        for i in 0..10 {
            queue.enqueue(i);
        }

        // Should have resized up
        assert!(queue.capacity() >= 10);
        assert_eq!(queue.size(), 10);

        // Remove most items
        for _ in 0..8 {
            queue.dequeue();
        }

        // Should have resized down
        assert_eq!(queue.size(), 2);
    }

    #[test]
    fn test_circular_wraparound() {
        let mut queue = ResizingArrayQueue::with_capacity(4);

        // Enqueue and dequeue to move indices
        for i in 0..3 {
            queue.enqueue(i);
        }
        queue.dequeue(); // Remove 0
        queue.dequeue(); // Remove 1

        // Now enqueue more to wrap around
        queue.enqueue(3);
        queue.enqueue(4);
        queue.enqueue(5);

        // Should maintain FIFO order
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), Some(4));
        assert_eq!(queue.dequeue(), Some(5));
    }

    #[test]
    fn test_iter() {
        let mut queue = ResizingArrayQueue::new();
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
        let mut queue = ResizingArrayQueue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        let items: Vec<_> = queue.into_iter().collect();
        assert_eq!(items, vec![1, 2, 3]);
    }

    #[test]
    fn test_display() {
        let mut queue = ResizingArrayQueue::new();
        queue.enqueue("to");
        queue.enqueue("be");
        queue.enqueue("or");

        assert_eq!(queue.to_string(), "to be or");
    }

    #[test]
    fn test_fifo_order() {
        let mut queue = ResizingArrayQueue::new();
        for i in 0..100 {
            queue.enqueue(i);
        }

        for i in 0..100 {
            assert_eq!(queue.dequeue(), Some(i));
        }
    }

    #[test]
    fn test_interleaved_operations() {
        let mut queue = ResizingArrayQueue::new();

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
        let queue: ResizingArrayQueue<i32> = ResizingArrayQueue::default();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_large_queue() {
        let mut queue = ResizingArrayQueue::new();

        // Enqueue many items
        for i in 0..1000 {
            queue.enqueue(i);
        }

        assert_eq!(queue.size(), 1000);

        // Dequeue many items
        for i in 0..1000 {
            assert_eq!(queue.dequeue(), Some(i));
        }

        assert!(queue.is_empty());
    }
}
