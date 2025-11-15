//! Array-based implementation of a bag (multiset) with automatic resizing.
//!
//! A bag is an unordered collection that allows duplicates.
//! This implementation uses a resizing array to store items.
//! The `add` operation takes amortized constant time, and iteration
//! takes time proportional to the number of items.
//!
//! # Examples
//!
//! ```
//! use algs4_fundamentals::collections::ResizingArrayBag;
//!
//! let mut bag = ResizingArrayBag::new();
//! bag.add(1);
//! bag.add(2);
//! bag.add(1); // duplicates allowed
//!
//! assert_eq!(bag.size(), 3);
//! ```
//!
//! **Reference:** <https://algs4.cs.princeton.edu/13stacks>

use std::fmt;

/// A bag (multiset) implemented using a resizing array.
///
/// A bag is an unordered collection that allows duplicates.
/// This implementation supports `add`, `size`, and `is_empty` operations
/// with constant or amortized constant time, and provides iteration through all items.
///
/// The array capacity doubles when it becomes full.
///
/// # Type Parameters
///
/// * `T` - the type of items in the bag
#[derive(Debug)]
pub struct ResizingArrayBag<T> {
    items: Vec<T>,
    size: usize,
}

impl<T> ResizingArrayBag<T> {
    /// Initializes an empty bag with initial capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::ResizingArrayBag;
    ///
    /// let bag: ResizingArrayBag<i32> = ResizingArrayBag::new();
    /// assert!(bag.is_empty());
    /// ```
    pub fn new() -> Self {
        Self::with_capacity(8)
    }

    /// Initializes an empty bag with the specified capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - the initial capacity
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::ResizingArrayBag;
    ///
    /// let bag: ResizingArrayBag<i32> = ResizingArrayBag::with_capacity(16);
    /// assert!(bag.is_empty());
    /// assert_eq!(bag.capacity(), 16);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
            size: 0,
        }
    }

    /// Returns true if this bag is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::ResizingArrayBag;
    ///
    /// let mut bag = ResizingArrayBag::new();
    /// assert!(bag.is_empty());
    ///
    /// bag.add(1);
    /// assert!(!bag.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Returns the number of items in this bag.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::ResizingArrayBag;
    ///
    /// let mut bag = ResizingArrayBag::new();
    /// assert_eq!(bag.size(), 0);
    ///
    /// bag.add(1);
    /// bag.add(2);
    /// assert_eq!(bag.size(), 2);
    /// ```
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns the current capacity of the underlying array.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::ResizingArrayBag;
    ///
    /// let mut bag = ResizingArrayBag::new();
    /// let initial_capacity = bag.capacity();
    ///
    /// for i in 0..10 {
    ///     bag.add(i);
    /// }
    ///
    /// // Capacity should have grown
    /// assert!(bag.capacity() >= initial_capacity);
    /// ```
    pub fn capacity(&self) -> usize {
        self.items.capacity()
    }

    /// Adds the item to this bag.
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
    /// use algs4_fundamentals::collections::ResizingArrayBag;
    ///
    /// let mut bag = ResizingArrayBag::new();
    /// bag.add("Hello");
    /// bag.add("World");
    /// bag.add("Hello"); // duplicates allowed
    /// assert_eq!(bag.size(), 3);
    /// ```
    pub fn add(&mut self, item: T) {
        // Resize if at capacity
        if self.size == self.items.capacity() {
            self.resize(2 * self.items.capacity().max(1));
        }

        self.items.push(item);
        self.size += 1;
    }

    /// Returns an iterator over the items in this bag.
    ///
    /// The order of iteration is arbitrary.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::collections::ResizingArrayBag;
    ///
    /// let mut bag = ResizingArrayBag::new();
    /// bag.add(1);
    /// bag.add(2);
    /// bag.add(3);
    ///
    /// let count = bag.iter().count();
    /// assert_eq!(count, 3);
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            items: &self.items[0..self.size],
            index: 0,
        }
    }

    /// Resizes the underlying array to the specified capacity.
    fn resize(&mut self, capacity: usize) {
        let mut new_vec = Vec::with_capacity(capacity);
        new_vec.append(&mut self.items);
        self.items = new_vec;
    }
}

impl<T> Default for ResizingArrayBag<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: fmt::Display> fmt::Display for ResizingArrayBag<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let items: Vec<String> = self.iter().map(|item| format!("{}", item)).collect();
        write!(f, "{}", items.join(" "))
    }
}

/// An iterator over references to items in a `ResizingArrayBag`.
#[derive(Debug)]
pub struct Iter<'a, T> {
    items: &'a [T],
    index: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.items.len() {
            let item = &self.items[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

/// An iterator that moves out of a `ResizingArrayBag`.
#[derive(Debug)]
pub struct IntoIter<T> {
    iter: std::vec::IntoIter<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<T> IntoIterator for ResizingArrayBag<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            iter: self.items.into_iter(),
        }
    }
}

impl<'a, T> IntoIterator for &'a ResizingArrayBag<T> {
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
        let bag: ResizingArrayBag<i32> = ResizingArrayBag::new();
        assert!(bag.is_empty());
        assert_eq!(bag.size(), 0);
        assert!(bag.capacity() >= 8);
    }

    #[test]
    fn test_with_capacity() {
        let bag: ResizingArrayBag<i32> = ResizingArrayBag::with_capacity(16);
        assert!(bag.is_empty());
        assert_eq!(bag.capacity(), 16);
    }

    #[test]
    fn test_add() {
        let mut bag = ResizingArrayBag::new();
        bag.add(1);
        bag.add(2);
        bag.add(3);

        assert_eq!(bag.size(), 3);
        assert!(!bag.is_empty());
    }

    #[test]
    fn test_is_empty() {
        let mut bag = ResizingArrayBag::new();
        assert!(bag.is_empty());

        bag.add(1);
        assert!(!bag.is_empty());
    }

    #[test]
    fn test_size() {
        let mut bag = ResizingArrayBag::new();
        assert_eq!(bag.size(), 0);

        for i in 0..10 {
            bag.add(i);
            assert_eq!(bag.size(), i + 1);
        }
    }

    #[test]
    fn test_duplicates() {
        let mut bag = ResizingArrayBag::new();
        bag.add(1);
        bag.add(1);
        bag.add(1);

        assert_eq!(bag.size(), 3);
        let count = bag.iter().filter(|&&x| x == 1).count();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_resizing() {
        let mut bag = ResizingArrayBag::with_capacity(2);
        assert_eq!(bag.capacity(), 2);

        // Fill beyond initial capacity
        for i in 0..10 {
            bag.add(i);
        }

        // Should have resized up
        assert!(bag.capacity() >= 10);
        assert_eq!(bag.size(), 10);
    }

    #[test]
    fn test_iter() {
        let mut bag = ResizingArrayBag::new();
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
        let mut bag = ResizingArrayBag::new();
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
        let mut bag = ResizingArrayBag::new();
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
        let bag: ResizingArrayBag<i32> = ResizingArrayBag::default();
        assert!(bag.is_empty());
    }

    #[test]
    fn test_large_bag() {
        let mut bag = ResizingArrayBag::new();
        for i in 0..1000 {
            bag.add(i);
        }

        assert_eq!(bag.size(), 1000);
        let count = bag.iter().count();
        assert_eq!(count, 1000);
    }

    #[test]
    fn test_strings() {
        let mut bag = ResizingArrayBag::new();
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
