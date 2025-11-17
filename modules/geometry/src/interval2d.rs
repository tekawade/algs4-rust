//! 2D interval (rectangle) data type for geometric algorithms
//!
//! This module provides the `Interval2D` type representing an axis-aligned
//! rectangle in the plane using two 1D intervals.
//!
//! # Examples
//!
//! ```
//! use algs4_geometry::{Interval1D, Interval2D, Point2D};
//!
//! let x_interval = Interval1D::new(0.0, 5.0);
//! let y_interval = Interval1D::new(0.0, 3.0);
//! let rect = Interval2D::new(x_interval, y_interval);
//!
//! assert_eq!(rect.area(), 15.0);
//!
//! let point = Point2D::new(2.0, 1.0);
//! assert!(rect.contains_point(&point));
//! ```

use crate::{Interval1D, Point2D};
use std::fmt;

/// Represents an immutable 2D interval (axis-aligned rectangle).
///
/// This type is composed of two 1D intervals representing the x and y extents.
/// It provides various operations including area calculation, containment checking,
/// and intersection detection.
///
/// # Examples
///
/// ```
/// use algs4_geometry::{Interval1D, Interval2D, Point2D};
///
/// let rect = Interval2D::new(
///     Interval1D::new(1.0, 4.0),
///     Interval1D::new(2.0, 5.0)
/// );
///
/// assert_eq!(rect.area(), 9.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Interval2D {
    x: Interval1D,
    y: Interval1D,
}

impl Interval2D {
    /// Creates a new 2D interval from x and y intervals.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::{Interval1D, Interval2D};
    ///
    /// let x_interval = Interval1D::new(0.0, 5.0);
    /// let y_interval = Interval1D::new(0.0, 3.0);
    /// let rect = Interval2D::new(x_interval, y_interval);
    /// ```
    pub fn new(x: Interval1D, y: Interval1D) -> Self {
        Interval2D { x, y }
    }

    /// Returns the x-interval.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::{Interval1D, Interval2D};
    ///
    /// let x_interval = Interval1D::new(1.0, 5.0);
    /// let y_interval = Interval1D::new(2.0, 6.0);
    /// let rect = Interval2D::new(x_interval, y_interval);
    ///
    /// assert_eq!(rect.x().min(), 1.0);
    /// assert_eq!(rect.x().max(), 5.0);
    /// ```
    #[inline]
    pub fn x(&self) -> &Interval1D {
        &self.x
    }

    /// Returns the y-interval.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::{Interval1D, Interval2D};
    ///
    /// let x_interval = Interval1D::new(1.0, 5.0);
    /// let y_interval = Interval1D::new(2.0, 6.0);
    /// let rect = Interval2D::new(x_interval, y_interval);
    ///
    /// assert_eq!(rect.y().min(), 2.0);
    /// assert_eq!(rect.y().max(), 6.0);
    /// ```
    #[inline]
    pub fn y(&self) -> &Interval1D {
        &self.y
    }

    /// Returns the area of the 2D interval.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::{Interval1D, Interval2D};
    ///
    /// let rect = Interval2D::new(
    ///     Interval1D::new(0.0, 5.0),
    ///     Interval1D::new(0.0, 3.0)
    /// );
    ///
    /// assert_eq!(rect.area(), 15.0);
    /// ```
    pub fn area(&self) -> f64 {
        self.x.length() * self.y.length()
    }

    /// Returns true if the 2D interval contains the point.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::{Interval1D, Interval2D, Point2D};
    ///
    /// let rect = Interval2D::new(
    ///     Interval1D::new(0.0, 5.0),
    ///     Interval1D::new(0.0, 3.0)
    /// );
    ///
    /// let p1 = Point2D::new(2.0, 1.0);
    /// let p2 = Point2D::new(6.0, 1.0);
    ///
    /// assert!(rect.contains_point(&p1));
    /// assert!(!rect.contains_point(&p2));
    /// ```
    pub fn contains_point(&self, p: &Point2D) -> bool {
        self.x.contains(p.x()) && self.y.contains(p.y())
    }

    /// Returns true if this 2D interval contains the entire other 2D interval.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::{Interval1D, Interval2D};
    ///
    /// let rect1 = Interval2D::new(
    ///     Interval1D::new(0.0, 10.0),
    ///     Interval1D::new(0.0, 10.0)
    /// );
    ///
    /// let rect2 = Interval2D::new(
    ///     Interval1D::new(2.0, 5.0),
    ///     Interval1D::new(3.0, 6.0)
    /// );
    ///
    /// assert!(rect1.contains_interval(&rect2));
    /// assert!(!rect2.contains_interval(&rect1));
    /// ```
    pub fn contains_interval(&self, that: &Interval2D) -> bool {
        self.x.contains_interval(&that.x) && self.y.contains_interval(&that.y)
    }

    /// Returns true if this 2D interval intersects the other 2D interval.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::{Interval1D, Interval2D};
    ///
    /// let rect1 = Interval2D::new(
    ///     Interval1D::new(0.0, 5.0),
    ///     Interval1D::new(0.0, 5.0)
    /// );
    ///
    /// let rect2 = Interval2D::new(
    ///     Interval1D::new(3.0, 8.0),
    ///     Interval1D::new(3.0, 8.0)
    /// );
    ///
    /// let rect3 = Interval2D::new(
    ///     Interval1D::new(10.0, 15.0),
    ///     Interval1D::new(10.0, 15.0)
    /// );
    ///
    /// assert!(rect1.intersects(&rect2));
    /// assert!(!rect1.intersects(&rect3));
    /// ```
    pub fn intersects(&self, that: &Interval2D) -> bool {
        self.x.intersects(&that.x) && self.y.intersects(&that.y)
    }
}

// Implement PartialEq for Interval2D
impl PartialEq for Interval2D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Interval2D {}

// Display implementation
impl fmt::Display for Interval2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} x {}", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let rect = Interval2D::new(Interval1D::new(0.0, 5.0), Interval1D::new(0.0, 3.0));

        assert_eq!(rect.x().min(), 0.0);
        assert_eq!(rect.x().max(), 5.0);
        assert_eq!(rect.y().min(), 0.0);
        assert_eq!(rect.y().max(), 3.0);
    }

    #[test]
    fn test_area() {
        let rect = Interval2D::new(Interval1D::new(0.0, 5.0), Interval1D::new(0.0, 3.0));

        assert_eq!(rect.area(), 15.0);

        let square = Interval2D::new(Interval1D::new(0.0, 4.0), Interval1D::new(0.0, 4.0));

        assert_eq!(square.area(), 16.0);
    }

    #[test]
    fn test_contains_point() {
        let rect = Interval2D::new(Interval1D::new(0.0, 5.0), Interval1D::new(0.0, 3.0));

        let p1 = Point2D::new(2.0, 1.0);
        let p2 = Point2D::new(0.0, 0.0); // Corner
        let p3 = Point2D::new(5.0, 3.0); // Corner
        let p4 = Point2D::new(6.0, 1.0); // Outside x
        let p5 = Point2D::new(2.0, 4.0); // Outside y

        assert!(rect.contains_point(&p1));
        assert!(rect.contains_point(&p2));
        assert!(rect.contains_point(&p3));
        assert!(!rect.contains_point(&p4));
        assert!(!rect.contains_point(&p5));
    }

    #[test]
    fn test_contains_interval() {
        let rect1 = Interval2D::new(Interval1D::new(0.0, 10.0), Interval1D::new(0.0, 10.0));

        let rect2 = Interval2D::new(Interval1D::new(2.0, 5.0), Interval1D::new(3.0, 6.0));

        let rect3 = Interval2D::new(Interval1D::new(5.0, 15.0), Interval1D::new(5.0, 15.0));

        assert!(rect1.contains_interval(&rect2));
        assert!(!rect2.contains_interval(&rect1));
        assert!(!rect1.contains_interval(&rect3));
    }

    #[test]
    fn test_intersects() {
        let rect1 = Interval2D::new(Interval1D::new(0.0, 5.0), Interval1D::new(0.0, 5.0));

        let rect2 = Interval2D::new(Interval1D::new(3.0, 8.0), Interval1D::new(3.0, 8.0));

        let rect3 = Interval2D::new(Interval1D::new(10.0, 15.0), Interval1D::new(10.0, 15.0));

        let rect4 = Interval2D::new(Interval1D::new(3.0, 8.0), Interval1D::new(10.0, 15.0));

        assert!(rect1.intersects(&rect2));
        assert!(rect2.intersects(&rect1));
        assert!(!rect1.intersects(&rect3));
        assert!(!rect1.intersects(&rect4)); // Intersects in x but not y
    }

    #[test]
    fn test_equality() {
        let rect1 = Interval2D::new(Interval1D::new(0.0, 5.0), Interval1D::new(0.0, 3.0));

        let rect2 = Interval2D::new(Interval1D::new(0.0, 5.0), Interval1D::new(0.0, 3.0));

        let rect3 = Interval2D::new(Interval1D::new(0.0, 6.0), Interval1D::new(0.0, 3.0));

        assert_eq!(rect1, rect2);
        assert_ne!(rect1, rect3);
    }

    #[test]
    fn test_display() {
        let rect = Interval2D::new(Interval1D::new(0.0, 5.0), Interval1D::new(2.0, 8.0));

        assert_eq!(format!("{}", rect), "[0, 5] x [2, 8]");
    }
}
