//! 1D interval data type for geometric algorithms
//!
//! This module provides the `Interval1D` type representing a closed interval
//! on the real line with various operations.
//!
//! # Examples
//!
//! ```
//! use algs4_geometry::Interval1D;
//!
//! let interval1 = Interval1D::new(1.0, 5.0);
//! let interval2 = Interval1D::new(3.0, 7.0);
//!
//! assert!(interval1.intersects(&interval2));
//! assert_eq!(interval1.length(), 4.0);
//! ```

use std::cmp::Ordering;
use std::fmt;

/// Represents an immutable closed interval [min, max] on the real line.
///
/// This type provides various operations including containment checking,
/// intersection detection, and comparison methods.
///
/// # Examples
///
/// ```
/// use algs4_geometry::Interval1D;
///
/// let interval = Interval1D::new(2.0, 5.0);
/// assert!(interval.contains(3.0));
/// assert_eq!(interval.length(), 3.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Interval1D {
    min: f64,
    max: f64,
}

impl Interval1D {
    /// Creates a new interval [min, max].
    ///
    /// # Panics
    ///
    /// Panics if either endpoint is NaN, infinite, or if min > max.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::Interval1D;
    ///
    /// let interval = Interval1D::new(1.0, 5.0);
    /// ```
    pub fn new(min: f64, max: f64) -> Self {
        if min.is_nan() || min.is_infinite() {
            panic!("min endpoint must be finite");
        }
        if max.is_nan() || max.is_infinite() {
            panic!("max endpoint must be finite");
        }
        if min > max {
            panic!("min must be less than or equal to max");
        }

        Interval1D { min, max }
    }

    /// Returns the minimum endpoint.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::Interval1D;
    ///
    /// let interval = Interval1D::new(2.0, 5.0);
    /// assert_eq!(interval.min(), 2.0);
    /// ```
    #[inline]
    pub fn min(&self) -> f64 {
        self.min
    }

    /// Returns the maximum endpoint.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::Interval1D;
    ///
    /// let interval = Interval1D::new(2.0, 5.0);
    /// assert_eq!(interval.max(), 5.0);
    /// ```
    #[inline]
    pub fn max(&self) -> f64 {
        self.max
    }

    /// Returns the length of the interval.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::Interval1D;
    ///
    /// let interval = Interval1D::new(2.0, 5.0);
    /// assert_eq!(interval.length(), 3.0);
    /// ```
    #[inline]
    pub fn length(&self) -> f64 {
        self.max - self.min
    }

    /// Returns true if the interval contains the value x.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::Interval1D;
    ///
    /// let interval = Interval1D::new(2.0, 5.0);
    /// assert!(interval.contains(3.0));
    /// assert!(!interval.contains(6.0));
    /// ```
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    /// Returns true if this interval contains the entire other interval.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::Interval1D;
    ///
    /// let interval1 = Interval1D::new(1.0, 10.0);
    /// let interval2 = Interval1D::new(3.0, 5.0);
    /// assert!(interval1.contains_interval(&interval2));
    /// assert!(!interval2.contains_interval(&interval1));
    /// ```
    pub fn contains_interval(&self, that: &Interval1D) -> bool {
        self.min <= that.min && that.max <= self.max
    }

    /// Returns true if this interval intersects the other interval.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::Interval1D;
    ///
    /// let interval1 = Interval1D::new(1.0, 5.0);
    /// let interval2 = Interval1D::new(3.0, 7.0);
    /// assert!(interval1.intersects(&interval2));
    ///
    /// let interval3 = Interval1D::new(8.0, 10.0);
    /// assert!(!interval1.intersects(&interval3));
    /// ```
    pub fn intersects(&self, that: &Interval1D) -> bool {
        if self.max < that.min {
            return false;
        }
        if that.max < self.min {
            return false;
        }
        true
    }
}

// Implement PartialEq for Interval1D
impl PartialEq for Interval1D {
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min && self.max == other.max
    }
}

impl Eq for Interval1D {}

// Display implementation
impl fmt::Display for Interval1D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.min, self.max)
    }
}

// Comparator functions
impl Interval1D {
    /// Compares two intervals by minimum endpoint, breaking ties by maximum endpoint.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::Interval1D;
    ///
    /// let i1 = Interval1D::new(1.0, 5.0);
    /// let i2 = Interval1D::new(2.0, 6.0);
    /// assert_eq!(Interval1D::min_endpoint_order(&i1, &i2), std::cmp::Ordering::Less);
    /// ```
    pub fn min_endpoint_order(a: &Interval1D, b: &Interval1D) -> Ordering {
        match a.min.partial_cmp(&b.min) {
            Some(Ordering::Equal) => a.max.partial_cmp(&b.max).unwrap_or(Ordering::Equal),
            Some(ord) => ord,
            None => Ordering::Equal,
        }
    }

    /// Compares two intervals by maximum endpoint, breaking ties by minimum endpoint.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::Interval1D;
    ///
    /// let i1 = Interval1D::new(1.0, 5.0);
    /// let i2 = Interval1D::new(2.0, 6.0);
    /// assert_eq!(Interval1D::max_endpoint_order(&i1, &i2), std::cmp::Ordering::Less);
    /// ```
    pub fn max_endpoint_order(a: &Interval1D, b: &Interval1D) -> Ordering {
        match a.max.partial_cmp(&b.max) {
            Some(Ordering::Equal) => a.min.partial_cmp(&b.min).unwrap_or(Ordering::Equal),
            Some(ord) => ord,
            None => Ordering::Equal,
        }
    }

    /// Compares two intervals by length.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::Interval1D;
    ///
    /// let i1 = Interval1D::new(1.0, 5.0);  // length 4
    /// let i2 = Interval1D::new(2.0, 9.0);  // length 7
    /// assert_eq!(Interval1D::length_order(&i1, &i2), std::cmp::Ordering::Less);
    /// ```
    pub fn length_order(a: &Interval1D, b: &Interval1D) -> Ordering {
        a.length()
            .partial_cmp(&b.length())
            .unwrap_or(Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let interval = Interval1D::new(1.0, 5.0);
        assert_eq!(interval.min(), 1.0);
        assert_eq!(interval.max(), 5.0);
    }

    #[test]
    #[should_panic(expected = "min endpoint must be finite")]
    fn test_nan_min() {
        Interval1D::new(f64::NAN, 5.0);
    }

    #[test]
    #[should_panic(expected = "max endpoint must be finite")]
    fn test_nan_max() {
        Interval1D::new(1.0, f64::NAN);
    }

    #[test]
    #[should_panic(expected = "min endpoint must be finite")]
    fn test_infinite_min() {
        Interval1D::new(f64::INFINITY, 5.0);
    }

    #[test]
    #[should_panic(expected = "min must be less than or equal to max")]
    fn test_invalid_range() {
        Interval1D::new(5.0, 1.0);
    }

    #[test]
    fn test_length() {
        let interval = Interval1D::new(2.0, 5.0);
        assert_eq!(interval.length(), 3.0);

        let point = Interval1D::new(3.0, 3.0);
        assert_eq!(point.length(), 0.0);
    }

    #[test]
    fn test_contains() {
        let interval = Interval1D::new(2.0, 5.0);

        assert!(interval.contains(2.0));
        assert!(interval.contains(3.0));
        assert!(interval.contains(5.0));
        assert!(!interval.contains(1.0));
        assert!(!interval.contains(6.0));
    }

    #[test]
    fn test_contains_interval() {
        let interval1 = Interval1D::new(1.0, 10.0);
        let interval2 = Interval1D::new(3.0, 5.0);
        let interval3 = Interval1D::new(0.0, 11.0);

        assert!(interval1.contains_interval(&interval2));
        assert!(!interval2.contains_interval(&interval1));
        assert!(interval3.contains_interval(&interval1));
    }

    #[test]
    fn test_intersects() {
        let interval1 = Interval1D::new(1.0, 5.0);
        let interval2 = Interval1D::new(3.0, 7.0);
        let interval3 = Interval1D::new(8.0, 10.0);
        let interval4 = Interval1D::new(5.0, 6.0); // Touching endpoint

        assert!(interval1.intersects(&interval2));
        assert!(interval2.intersects(&interval1));
        assert!(!interval1.intersects(&interval3));
        assert!(interval1.intersects(&interval4)); // Touching counts as intersection
    }

    #[test]
    fn test_equality() {
        let i1 = Interval1D::new(1.0, 5.0);
        let i2 = Interval1D::new(1.0, 5.0);
        let i3 = Interval1D::new(2.0, 5.0);

        assert_eq!(i1, i2);
        assert_ne!(i1, i3);
    }

    #[test]
    fn test_min_endpoint_order() {
        let i1 = Interval1D::new(1.0, 5.0);
        let i2 = Interval1D::new(2.0, 6.0);
        let i3 = Interval1D::new(1.0, 4.0);

        assert_eq!(Interval1D::min_endpoint_order(&i1, &i2), Ordering::Less);
        assert_eq!(Interval1D::min_endpoint_order(&i2, &i1), Ordering::Greater);
        // Same min, different max
        assert_eq!(Interval1D::min_endpoint_order(&i1, &i3), Ordering::Greater);
    }

    #[test]
    fn test_max_endpoint_order() {
        let i1 = Interval1D::new(1.0, 5.0);
        let i2 = Interval1D::new(2.0, 6.0);

        assert_eq!(Interval1D::max_endpoint_order(&i1, &i2), Ordering::Less);
        assert_eq!(Interval1D::max_endpoint_order(&i2, &i1), Ordering::Greater);
    }

    #[test]
    fn test_length_order() {
        let i1 = Interval1D::new(1.0, 5.0); // length 4
        let i2 = Interval1D::new(2.0, 9.0); // length 7

        assert_eq!(Interval1D::length_order(&i1, &i2), Ordering::Less);
        assert_eq!(Interval1D::length_order(&i2, &i1), Ordering::Greater);
    }

    #[test]
    fn test_display() {
        let interval = Interval1D::new(1.0, 5.0);
        assert_eq!(format!("{}", interval), "[1, 5]");
    }
}
