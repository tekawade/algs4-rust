// Copyright (C) 2025 algs4-rust contributors
// SPDX-License-Identifier: GPL-3.0-or-later
//
// This file is part of algs4-rust (Rust implementation of Algorithms, 4th Edition).
// Adapted from the original Java implementation by Robert Sedgewick and Kevin Wayne.

//! Running statistics for a stream of real numbers.
//!
//! The `Accumulator` data type computes the mean, sample variance,
//! and sample standard deviation of a stream of real numbers in one pass.
//! It uses Welford's algorithm for numerical stability.

use std::fmt;

/// An accumulator for computing running statistics.
///
/// The `Accumulator` type computes statistics for a stream of real numbers
/// using Welford's online algorithm, which is numerically stable and uses
/// constant memory.
///
/// # Examples
///
/// ```
/// use algs4_fundamentals::util::Accumulator;
///
/// let mut stats = Accumulator::new();
/// stats.add_data_value(1.0);
/// stats.add_data_value(2.0);
/// stats.add_data_value(3.0);
/// stats.add_data_value(4.0);
/// stats.add_data_value(5.0);
///
/// assert_eq!(stats.count(), 5);
/// assert!((stats.mean() - 3.0).abs() < 1e-10);
/// assert!((stats.stddev() - 1.5811388300841898).abs() < 1e-10);
/// ```
#[derive(Debug, Clone)]
pub struct Accumulator {
    n: usize,        // number of data values
    sum: f64,        // sample variance * (n-1)
    mu: f64,         // sample mean
}

impl Accumulator {
    /// Creates a new, empty accumulator.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::util::Accumulator;
    ///
    /// let stats = Accumulator::new();
    /// assert_eq!(stats.count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            n: 0,
            sum: 0.0,
            mu: 0.0,
        }
    }

    /// Adds a data value to the accumulator.
    ///
    /// Uses Welford's online algorithm to update the mean and variance
    /// incrementally in a numerically stable way.
    ///
    /// # Arguments
    ///
    /// * `x` - The data value to add
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::util::Accumulator;
    ///
    /// let mut stats = Accumulator::new();
    /// stats.add_data_value(1.0);
    /// assert_eq!(stats.count(), 1);
    /// assert_eq!(stats.mean(), 1.0);
    /// ```
    pub fn add_data_value(&mut self, x: f64) {
        self.n += 1;
        let delta = x - self.mu;
        self.mu += delta / (self.n as f64);
        self.sum += delta * (x - self.mu);
    }

    /// Returns the mean of the data values.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::util::Accumulator;
    ///
    /// let mut stats = Accumulator::new();
    /// stats.add_data_value(1.0);
    /// stats.add_data_value(3.0);
    /// assert_eq!(stats.mean(), 2.0);
    /// ```
    #[inline]
    pub fn mean(&self) -> f64 {
        self.mu
    }

    /// Returns the sample variance of the data values.
    ///
    /// Returns `NaN` if the count is 1 or less.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::util::Accumulator;
    ///
    /// let mut stats = Accumulator::new();
    /// stats.add_data_value(1.0);
    /// stats.add_data_value(2.0);
    /// stats.add_data_value(3.0);
    ///
    /// assert!((stats.var() - 1.0).abs() < 1e-10);
    /// ```
    #[inline]
    pub fn var(&self) -> f64 {
        if self.n <= 1 {
            f64::NAN
        } else {
            self.sum / ((self.n - 1) as f64)
        }
    }

    /// Returns the sample standard deviation of the data values.
    ///
    /// Returns `NaN` if the count is 1 or less.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::util::Accumulator;
    ///
    /// let mut stats = Accumulator::new();
    /// stats.add_data_value(1.0);
    /// stats.add_data_value(2.0);
    /// stats.add_data_value(3.0);
    ///
    /// assert!((stats.stddev() - 1.0).abs() < 1e-10);
    /// ```
    #[inline]
    pub fn stddev(&self) -> f64 {
        self.var().sqrt()
    }

    /// Returns the number of data values.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::util::Accumulator;
    ///
    /// let mut stats = Accumulator::new();
    /// assert_eq!(stats.count(), 0);
    /// stats.add_data_value(1.0);
    /// assert_eq!(stats.count(), 1);
    /// ```
    #[inline]
    pub fn count(&self) -> usize {
        self.n
    }
}

impl Default for Accumulator {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Accumulator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "n = {}, mean = {:.5}, stddev = {:.5}",
               self.n, self.mean(), self.stddev())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accumulator_new() {
        let stats = Accumulator::new();
        assert_eq!(stats.count(), 0);
        assert_eq!(stats.mean(), 0.0);
        assert!(stats.var().is_nan());
        assert!(stats.stddev().is_nan());
    }

    #[test]
    fn test_accumulator_single_value() {
        let mut stats = Accumulator::new();
        stats.add_data_value(5.0);
        assert_eq!(stats.count(), 1);
        assert_eq!(stats.mean(), 5.0);
        assert!(stats.var().is_nan());
        assert!(stats.stddev().is_nan());
    }

    #[test]
    fn test_accumulator_mean() {
        let mut stats = Accumulator::new();
        stats.add_data_value(1.0);
        stats.add_data_value(2.0);
        stats.add_data_value(3.0);
        stats.add_data_value(4.0);
        stats.add_data_value(5.0);

        assert_eq!(stats.count(), 5);
        assert!((stats.mean() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_accumulator_variance() {
        let mut stats = Accumulator::new();
        stats.add_data_value(1.0);
        stats.add_data_value(2.0);
        stats.add_data_value(3.0);

        // Variance of [1, 2, 3] is 1.0
        assert!((stats.var() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_accumulator_stddev() {
        let mut stats = Accumulator::new();
        stats.add_data_value(1.0);
        stats.add_data_value(2.0);
        stats.add_data_value(3.0);

        // Stddev of [1, 2, 3] is 1.0
        assert!((stats.stddev() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_accumulator_display() {
        let mut stats = Accumulator::new();
        stats.add_data_value(1.0);
        stats.add_data_value(2.0);
        stats.add_data_value(3.0);

        let display = format!("{}", stats);
        assert!(display.contains("n = 3"));
        assert!(display.contains("mean = 2.00000"));
    }

    #[test]
    fn test_accumulator_default() {
        let stats = Accumulator::default();
        assert_eq!(stats.count(), 0);
    }

    #[test]
    fn test_accumulator_welford_algorithm() {
        // Test numerical stability with values that might cause issues
        // with naive variance calculation
        let mut stats = Accumulator::new();
        let base = 1e9;
        stats.add_data_value(base + 1.0);
        stats.add_data_value(base + 2.0);
        stats.add_data_value(base + 3.0);

        assert!((stats.mean() - (base + 2.0)).abs() < 1e-5);
        assert!((stats.var() - 1.0).abs() < 1e-5);
    }
}
