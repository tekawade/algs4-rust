// Copyright (C) 2025 algs4-rust contributors
// SPDX-License-Identifier: GPL-3.0-or-later
//
// This file is part of algs4-rust (Rust implementation of Algorithms, 4th Edition).
// Adapted from the original Java implementation by Robert Sedgewick and Kevin Wayne.

//! Stopwatch for measuring elapsed time.
//!
//! The `Stopwatch` data type is for measuring the time that elapses between
//! the start and end of a programming task (wall-clock time).

use std::time::Instant;

/// A `Stopwatch` measures elapsed time since it was created.
///
/// # Examples
///
/// ```
/// use algs4_fundamentals::util::Stopwatch;
/// use std::thread;
/// use std::time::Duration;
///
/// let timer = Stopwatch::new();
/// // ... do some work ...
/// thread::sleep(Duration::from_millis(10));
/// let elapsed = timer.elapsed();
/// assert!(elapsed >= 0.010);
/// ```
#[derive(Debug)]
pub struct Stopwatch {
    start: Instant,
}

impl Stopwatch {
    /// Creates a new stopwatch and starts timing.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::util::Stopwatch;
    ///
    /// let timer = Stopwatch::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    /// Returns the elapsed time (in seconds) since this stopwatch was created.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::util::Stopwatch;
    ///
    /// let timer = Stopwatch::new();
    /// // ... do some work ...
    /// println!("Elapsed time: {:.2} seconds", timer.elapsed());
    /// ```
    #[inline]
    pub fn elapsed(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_stopwatch_new() {
        let timer = Stopwatch::new();
        assert!(timer.elapsed() >= 0.0);
    }

    #[test]
    fn test_stopwatch_elapsed() {
        let timer = Stopwatch::new();
        thread::sleep(Duration::from_millis(10));
        let elapsed = timer.elapsed();
        assert!(elapsed >= 0.010);
        assert!(elapsed < 0.100); // Should be less than 100ms
    }

    #[test]
    fn test_stopwatch_multiple_calls() {
        let timer = Stopwatch::new();
        thread::sleep(Duration::from_millis(10));
        let elapsed1 = timer.elapsed();
        thread::sleep(Duration::from_millis(10));
        let elapsed2 = timer.elapsed();
        assert!(elapsed2 > elapsed1);
    }

    #[test]
    fn test_stopwatch_default() {
        let timer = Stopwatch::default();
        assert!(timer.elapsed() >= 0.0);
    }
}
