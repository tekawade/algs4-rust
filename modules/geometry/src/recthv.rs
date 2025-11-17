//! Axis-aligned rectangle data type for geometric algorithms
//!
//! This module provides the `RectHV` type representing an immutable axis-aligned
//! rectangle used in 2D range search and KD-tree algorithms.
//!
//! # Examples
//!
//! ```
//! use algs4_geometry::{RectHV, Point2D};
//!
//! let rect = RectHV::new(0.0, 0.0, 5.0, 3.0);
//!
//! assert_eq!(rect.width(), 5.0);
//! assert_eq!(rect.height(), 3.0);
//!
//! let point = Point2D::new(2.0, 1.0);
//! assert!(rect.contains(&point));
//! assert_eq!(rect.distance_to(&point), 0.0);
//! ```

use crate::Point2D;
use std::fmt;

/// Represents an immutable axis-aligned rectangle.
///
/// This type is optimized for use in 2D range search algorithms and KD-trees.
/// It provides efficient containment checking, intersection detection, and
/// distance calculations.
///
/// # Examples
///
/// ```
/// use algs4_geometry::{RectHV, Point2D};
///
/// let rect = RectHV::new(1.0, 2.0, 4.0, 5.0);
/// let point = Point2D::new(2.0, 3.0);
///
/// assert!(rect.contains(&point));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct RectHV {
    xmin: f64,
    ymin: f64,
    xmax: f64,
    ymax: f64,
}

impl RectHV {
    /// Creates a new axis-aligned rectangle.
    ///
    /// # Panics
    ///
    /// Panics if any coordinate is NaN or if max < min.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::RectHV;
    ///
    /// let rect = RectHV::new(0.0, 0.0, 5.0, 3.0);
    /// ```
    pub fn new(xmin: f64, ymin: f64, xmax: f64, ymax: f64) -> Self {
        if xmin.is_nan() || xmax.is_nan() || ymin.is_nan() || ymax.is_nan() {
            panic!("coordinates must not be NaN");
        }
        if xmin > xmax {
            panic!("xmin must be less than or equal to xmax");
        }
        if ymin > ymax {
            panic!("ymin must be less than or equal to ymax");
        }

        RectHV {
            xmin,
            ymin,
            xmax,
            ymax,
        }
    }

    /// Returns the minimum x-coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::RectHV;
    ///
    /// let rect = RectHV::new(1.0, 2.0, 4.0, 5.0);
    /// assert_eq!(rect.xmin(), 1.0);
    /// ```
    #[inline]
    pub fn xmin(&self) -> f64 {
        self.xmin
    }

    /// Returns the maximum x-coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::RectHV;
    ///
    /// let rect = RectHV::new(1.0, 2.0, 4.0, 5.0);
    /// assert_eq!(rect.xmax(), 4.0);
    /// ```
    #[inline]
    pub fn xmax(&self) -> f64 {
        self.xmax
    }

    /// Returns the minimum y-coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::RectHV;
    ///
    /// let rect = RectHV::new(1.0, 2.0, 4.0, 5.0);
    /// assert_eq!(rect.ymin(), 2.0);
    /// ```
    #[inline]
    pub fn ymin(&self) -> f64 {
        self.ymin
    }

    /// Returns the maximum y-coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::RectHV;
    ///
    /// let rect = RectHV::new(1.0, 2.0, 4.0, 5.0);
    /// assert_eq!(rect.ymax(), 5.0);
    /// ```
    #[inline]
    pub fn ymax(&self) -> f64 {
        self.ymax
    }

    /// Returns the width of the rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::RectHV;
    ///
    /// let rect = RectHV::new(0.0, 0.0, 5.0, 3.0);
    /// assert_eq!(rect.width(), 5.0);
    /// ```
    #[inline]
    pub fn width(&self) -> f64 {
        self.xmax - self.xmin
    }

    /// Returns the height of the rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::RectHV;
    ///
    /// let rect = RectHV::new(0.0, 0.0, 5.0, 3.0);
    /// assert_eq!(rect.height(), 3.0);
    /// ```
    #[inline]
    pub fn height(&self) -> f64 {
        self.ymax - self.ymin
    }

    /// Returns true if the rectangle contains the point.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::{RectHV, Point2D};
    ///
    /// let rect = RectHV::new(0.0, 0.0, 5.0, 3.0);
    /// let p1 = Point2D::new(2.0, 1.0);
    /// let p2 = Point2D::new(6.0, 1.0);
    ///
    /// assert!(rect.contains(&p1));
    /// assert!(!rect.contains(&p2));
    /// ```
    pub fn contains(&self, p: &Point2D) -> bool {
        self.xmin <= p.x() && p.x() <= self.xmax && self.ymin <= p.y() && p.y() <= self.ymax
    }

    /// Returns true if this rectangle intersects that rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::RectHV;
    ///
    /// let rect1 = RectHV::new(0.0, 0.0, 5.0, 5.0);
    /// let rect2 = RectHV::new(3.0, 3.0, 8.0, 8.0);
    /// let rect3 = RectHV::new(10.0, 10.0, 15.0, 15.0);
    ///
    /// assert!(rect1.intersects(&rect2));
    /// assert!(!rect1.intersects(&rect3));
    /// ```
    pub fn intersects(&self, that: &RectHV) -> bool {
        self.xmax >= that.xmin
            && that.xmax >= self.xmin
            && self.ymax >= that.ymin
            && that.ymax >= self.ymin
    }

    /// Returns the Euclidean distance from the point to the closest point in the rectangle.
    ///
    /// Returns 0 if the point is inside the rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::{RectHV, Point2D};
    ///
    /// let rect = RectHV::new(0.0, 0.0, 5.0, 3.0);
    ///
    /// // Point inside rectangle
    /// let p1 = Point2D::new(2.0, 1.0);
    /// assert_eq!(rect.distance_to(&p1), 0.0);
    ///
    /// // Point outside rectangle
    /// let p2 = Point2D::new(8.0, 0.0);
    /// assert_eq!(rect.distance_to(&p2), 3.0);
    /// ```
    pub fn distance_to(&self, p: &Point2D) -> f64 {
        self.distance_squared_to(p).sqrt()
    }

    /// Returns the square of the Euclidean distance from the point to the closest point
    /// in the rectangle.
    ///
    /// Returns 0 if the point is inside the rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::{RectHV, Point2D};
    ///
    /// let rect = RectHV::new(0.0, 0.0, 5.0, 3.0);
    ///
    /// // Point inside rectangle
    /// let p1 = Point2D::new(2.0, 1.0);
    /// assert_eq!(rect.distance_squared_to(&p1), 0.0);
    ///
    /// // Point outside rectangle
    /// let p2 = Point2D::new(8.0, 0.0);
    /// assert_eq!(rect.distance_squared_to(&p2), 9.0);
    /// ```
    pub fn distance_squared_to(&self, p: &Point2D) -> f64 {
        let mut dx = 0.0;
        let mut dy = 0.0;

        if p.x() < self.xmin {
            dx = self.xmin - p.x();
        } else if p.x() > self.xmax {
            dx = p.x() - self.xmax;
        }

        if p.y() < self.ymin {
            dy = self.ymin - p.y();
        } else if p.y() > self.ymax {
            dy = p.y() - self.ymax;
        }

        dx * dx + dy * dy
    }
}

// Implement PartialEq for RectHV
impl PartialEq for RectHV {
    fn eq(&self, other: &Self) -> bool {
        self.xmin == other.xmin
            && self.ymin == other.ymin
            && self.xmax == other.xmax
            && self.ymax == other.ymax
    }
}

impl Eq for RectHV {}

// Display implementation
impl fmt::Display for RectHV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}, {}] x [{}, {}]",
            self.xmin, self.xmax, self.ymin, self.ymax
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let rect = RectHV::new(1.0, 2.0, 4.0, 5.0);
        assert_eq!(rect.xmin(), 1.0);
        assert_eq!(rect.ymin(), 2.0);
        assert_eq!(rect.xmax(), 4.0);
        assert_eq!(rect.ymax(), 5.0);
    }

    #[test]
    #[should_panic(expected = "coordinates must not be NaN")]
    fn test_nan_coordinate() {
        RectHV::new(f64::NAN, 0.0, 1.0, 1.0);
    }

    #[test]
    #[should_panic(expected = "xmin must be less than or equal to xmax")]
    fn test_invalid_x_range() {
        RectHV::new(5.0, 0.0, 1.0, 1.0);
    }

    #[test]
    #[should_panic(expected = "ymin must be less than or equal to ymax")]
    fn test_invalid_y_range() {
        RectHV::new(0.0, 5.0, 1.0, 1.0);
    }

    #[test]
    fn test_dimensions() {
        let rect = RectHV::new(0.0, 0.0, 5.0, 3.0);
        assert_eq!(rect.width(), 5.0);
        assert_eq!(rect.height(), 3.0);
    }

    #[test]
    fn test_contains() {
        let rect = RectHV::new(0.0, 0.0, 5.0, 3.0);

        let p1 = Point2D::new(2.0, 1.0);
        let p2 = Point2D::new(0.0, 0.0); // Corner
        let p3 = Point2D::new(5.0, 3.0); // Corner
        let p4 = Point2D::new(6.0, 1.0); // Outside
        let p5 = Point2D::new(-1.0, 1.0); // Outside

        assert!(rect.contains(&p1));
        assert!(rect.contains(&p2));
        assert!(rect.contains(&p3));
        assert!(!rect.contains(&p4));
        assert!(!rect.contains(&p5));
    }

    #[test]
    fn test_intersects() {
        let rect1 = RectHV::new(0.0, 0.0, 5.0, 5.0);
        let rect2 = RectHV::new(3.0, 3.0, 8.0, 8.0);
        let rect3 = RectHV::new(10.0, 10.0, 15.0, 15.0);
        let rect4 = RectHV::new(5.0, 0.0, 10.0, 5.0); // Touching edge

        assert!(rect1.intersects(&rect2));
        assert!(rect2.intersects(&rect1));
        assert!(!rect1.intersects(&rect3));
        assert!(rect1.intersects(&rect4)); // Touching counts as intersection
    }

    #[test]
    fn test_distance_to_inside() {
        let rect = RectHV::new(0.0, 0.0, 5.0, 3.0);
        let p = Point2D::new(2.0, 1.0);

        assert_eq!(rect.distance_to(&p), 0.0);
        assert_eq!(rect.distance_squared_to(&p), 0.0);
    }

    #[test]
    fn test_distance_to_outside() {
        let rect = RectHV::new(0.0, 0.0, 5.0, 3.0);

        // Point to the right
        let p1 = Point2D::new(8.0, 0.0);
        assert_eq!(rect.distance_to(&p1), 3.0);
        assert_eq!(rect.distance_squared_to(&p1), 9.0);

        // Point above
        let p2 = Point2D::new(0.0, 7.0);
        assert_eq!(rect.distance_to(&p2), 4.0);
        assert_eq!(rect.distance_squared_to(&p2), 16.0);

        // Point in diagonal (outside both x and y)
        let p3 = Point2D::new(8.0, 7.0);
        assert_eq!(rect.distance_squared_to(&p3), 25.0); // 3^2 + 4^2
        assert_eq!(rect.distance_to(&p3), 5.0);
    }

    #[test]
    fn test_distance_to_edge() {
        let rect = RectHV::new(0.0, 0.0, 5.0, 3.0);

        // Point on edge
        let p = Point2D::new(5.0, 1.0);
        assert_eq!(rect.distance_to(&p), 0.0);
    }

    #[test]
    fn test_equality() {
        let rect1 = RectHV::new(0.0, 0.0, 5.0, 3.0);
        let rect2 = RectHV::new(0.0, 0.0, 5.0, 3.0);
        let rect3 = RectHV::new(0.0, 0.0, 6.0, 3.0);

        assert_eq!(rect1, rect2);
        assert_ne!(rect1, rect3);
    }

    #[test]
    fn test_display() {
        let rect = RectHV::new(0.0, 1.0, 5.0, 8.0);
        assert_eq!(format!("{}", rect), "[0, 5] x [1, 8]");
    }
}
