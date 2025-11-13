// Copyright (C) 2025 algs4-rust contributors
// SPDX-License-Identifier: GPL-3.0-or-later
//
// This file is part of algs4-rust (Rust implementation of Algorithms, 4th Edition).
// Adapted from the original Java implementation by Robert Sedgewick and Kevin Wayne.

//! A simple counter data type.
//!
//! The `Counter` data type is a mutable data type to encapsulate a counter.

use std::cmp::Ordering;
use std::fmt;

/// A simple counter with an associated name.
///
/// The `Counter` type represents a counter with a name identifier. It supports
/// incrementing the counter and comparing counters by their count values.
///
/// # Examples
///
/// ```
/// use algs4_fundamentals::util::Counter;
///
/// let mut heads = Counter::new("heads");
/// let mut tails = Counter::new("tails");
///
/// heads.increment();
/// heads.increment();
/// tails.increment();
///
/// assert_eq!(heads.tally(), 2);
/// assert_eq!(tails.tally(), 1);
/// assert!(heads > tails);
/// ```
#[derive(Debug, Clone, Eq)]
pub struct Counter {
    name: String,
    count: usize,
}

impl Counter {
    /// Creates a new counter with the given name, initialized to 0.
    ///
    /// # Arguments
    ///
    /// * `name` - The name for this counter
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::util::Counter;
    ///
    /// let counter = Counter::new("heads");
    /// assert_eq!(counter.tally(), 0);
    /// ```
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            count: 0,
        }
    }

    /// Increments the counter by 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::util::Counter;
    ///
    /// let mut counter = Counter::new("count");
    /// counter.increment();
    /// assert_eq!(counter.tally(), 1);
    /// ```
    #[inline]
    pub fn increment(&mut self) {
        self.count += 1;
    }

    /// Returns the current count.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::util::Counter;
    ///
    /// let mut counter = Counter::new("count");
    /// assert_eq!(counter.tally(), 0);
    /// counter.increment();
    /// assert_eq!(counter.tally(), 1);
    /// ```
    #[inline]
    pub fn tally(&self) -> usize {
        self.count
    }

    /// Returns the name of this counter.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::util::Counter;
    ///
    /// let counter = Counter::new("heads");
    /// assert_eq!(counter.name(), "heads");
    /// ```
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Counter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.count, self.name)
    }
}

impl PartialEq for Counter {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
    }
}

impl PartialOrd for Counter {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Counter {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count.cmp(&other.count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_new() {
        let counter = Counter::new("test");
        assert_eq!(counter.tally(), 0);
        assert_eq!(counter.name(), "test");
    }

    #[test]
    fn test_counter_increment() {
        let mut counter = Counter::new("test");
        counter.increment();
        assert_eq!(counter.tally(), 1);
        counter.increment();
        counter.increment();
        assert_eq!(counter.tally(), 3);
    }

    #[test]
    fn test_counter_display() {
        let mut counter = Counter::new("heads");
        counter.increment();
        counter.increment();
        assert_eq!(format!("{}", counter), "2 heads");
    }

    #[test]
    fn test_counter_comparison() {
        let mut c1 = Counter::new("first");
        let mut c2 = Counter::new("second");

        c1.increment();
        c1.increment();
        c2.increment();

        assert!(c1 > c2);
        assert!(c2 < c1);
        assert_eq!(c1, c1.clone());
    }

    #[test]
    fn test_counter_equality() {
        let mut c1 = Counter::new("first");
        let mut c2 = Counter::new("second");

        c1.increment();
        c2.increment();

        assert_eq!(c1, c2); // Equal by count, not by name
    }
}
