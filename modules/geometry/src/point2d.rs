//! 2D point data type for geometric algorithms
//!
//! This module provides the `Point2D` type representing an immutable point
//! in the Euclidean plane with various geometric operations.
//!
//! # Examples
//!
//! ```
//! use algs4_geometry::Point2D;
//!
//! let p = Point2D::new(3.0, 4.0);
//! let q = Point2D::new(0.0, 0.0);
//!
//! assert_eq!(p.distance_to(&q), 5.0);
//! assert_eq!(p.r(), 5.0);
//! ```

use std::cmp::Ordering;
use std::fmt;

/// Represents an immutable 2D point in the Euclidean plane.
///
/// This type provides various geometric operations including distance
/// calculations, angle measurements, and counter-clockwise orientation tests.
///
/// # Examples
///
/// ```
/// use algs4_geometry::Point2D;
///
/// let p = Point2D::new(1.0, 2.0);
/// let q = Point2D::new(4.0, 6.0);
///
/// let distance = p.distance_to(&q);
/// println!("Distance: {}", distance);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    x: f64,
    y: f64,
}

impl Point2D {
    /// Creates a new point with the given x and y coordinates.
    ///
    /// # Panics
    ///
    /// Panics if either coordinate is NaN or infinite.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::Point2D;
    ///
    /// let p = Point2D::new(3.0, 4.0);
    /// ```
    pub fn new(x: f64, y: f64) -> Self {
        if x.is_nan() || x.is_infinite() {
            panic!("x-coordinate must be finite");
        }
        if y.is_nan() || y.is_infinite() {
            panic!("y-coordinate must be finite");
        }

        // Convert negative zero to positive zero
        let x = if x == 0.0 { 0.0 } else { x };
        let y = if y == 0.0 { 0.0 } else { y };

        Point2D { x, y }
    }

    /// Returns the x-coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::Point2D;
    ///
    /// let p = Point2D::new(3.0, 4.0);
    /// assert_eq!(p.x(), 3.0);
    /// ```
    #[inline]
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Returns the y-coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::Point2D;
    ///
    /// let p = Point2D::new(3.0, 4.0);
    /// assert_eq!(p.y(), 4.0);
    /// ```
    #[inline]
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Returns the polar radius (Euclidean distance from origin).
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::Point2D;
    ///
    /// let p = Point2D::new(3.0, 4.0);
    /// assert_eq!(p.r(), 5.0);
    /// ```
    pub fn r(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Returns the polar angle in radians, between -π and π.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::Point2D;
    ///
    /// let p = Point2D::new(1.0, 0.0);
    /// assert_eq!(p.theta(), 0.0);
    ///
    /// let p = Point2D::new(0.0, 1.0);
    /// assert!((p.theta() - std::f64::consts::FRAC_PI_2).abs() < 1e-10);
    /// ```
    pub fn theta(&self) -> f64 {
        self.y.atan2(self.x)
    }

    /// Returns the angle to the other point in radians, between -π and π.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::Point2D;
    ///
    /// let p = Point2D::new(0.0, 0.0);
    /// let q = Point2D::new(1.0, 0.0);
    /// assert_eq!(p.angle_to(&q), 0.0);
    /// ```
    pub fn angle_to(&self, that: &Point2D) -> f64 {
        let dx = that.x - self.x;
        let dy = that.y - self.y;
        dy.atan2(dx)
    }

    /// Returns the Euclidean distance between this point and that point.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::Point2D;
    ///
    /// let p = Point2D::new(0.0, 0.0);
    /// let q = Point2D::new(3.0, 4.0);
    /// assert_eq!(p.distance_to(&q), 5.0);
    /// ```
    pub fn distance_to(&self, that: &Point2D) -> f64 {
        let dx = self.x - that.x;
        let dy = self.y - that.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Returns the square of the Euclidean distance between this point and that point.
    ///
    /// This is more efficient than `distance_to` when you only need to compare distances.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::Point2D;
    ///
    /// let p = Point2D::new(0.0, 0.0);
    /// let q = Point2D::new(3.0, 4.0);
    /// assert_eq!(p.distance_squared_to(&q), 25.0);
    /// ```
    pub fn distance_squared_to(&self, that: &Point2D) -> f64 {
        let dx = self.x - that.x;
        let dy = self.y - that.y;
        dx * dx + dy * dy
    }

    /// Returns the CCW (counter-clockwise) orientation of the three points a, b, c.
    ///
    /// Returns:
    /// * `1` if a -> b -> c is a counter-clockwise turn
    /// * `-1` if a -> b -> c is a clockwise turn
    /// * `0` if a -> b -> c are collinear
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::Point2D;
    ///
    /// let a = Point2D::new(0.0, 0.0);
    /// let b = Point2D::new(1.0, 0.0);
    /// let c = Point2D::new(0.0, 1.0);
    ///
    /// assert_eq!(Point2D::ccw(&a, &b, &c), 1);  // counter-clockwise
    /// ```
    pub fn ccw(a: &Point2D, b: &Point2D, c: &Point2D) -> i32 {
        let area2 = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);

        if area2 < 0.0 {
            -1
        } else if area2 > 0.0 {
            1
        } else {
            0
        }
    }

    /// Returns twice the signed area of the triangle a-b-c.
    ///
    /// This is useful for determining orientation and calculating areas.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::Point2D;
    ///
    /// let a = Point2D::new(0.0, 0.0);
    /// let b = Point2D::new(1.0, 0.0);
    /// let c = Point2D::new(0.0, 1.0);
    ///
    /// assert_eq!(Point2D::area2(&a, &b, &c), 1.0);
    /// ```
    pub fn area2(a: &Point2D, b: &Point2D, c: &Point2D) -> f64 {
        (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
    }
}

// Implement PartialEq for Point2D
impl PartialEq for Point2D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point2D {}

// Implement Ord and PartialOrd for Point2D
// Compare by y-coordinate first, then by x-coordinate
impl Ord for Point2D {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.y.partial_cmp(&other.y) {
            Some(Ordering::Equal) => self.x.partial_cmp(&other.x).unwrap_or(Ordering::Equal),
            Some(ord) => ord,
            None => Ordering::Equal,
        }
    }
}

impl PartialOrd for Point2D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Display implementation
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Comparator functions
impl Point2D {
    /// Compares two points by x-coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::Point2D;
    ///
    /// let p = Point2D::new(1.0, 5.0);
    /// let q = Point2D::new(2.0, 3.0);
    /// assert_eq!(Point2D::x_order(&p, &q), std::cmp::Ordering::Less);
    /// ```
    pub fn x_order(p: &Point2D, q: &Point2D) -> Ordering {
        p.x.partial_cmp(&q.x).unwrap_or(Ordering::Equal)
    }

    /// Compares two points by y-coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::Point2D;
    ///
    /// let p = Point2D::new(5.0, 1.0);
    /// let q = Point2D::new(3.0, 2.0);
    /// assert_eq!(Point2D::y_order(&p, &q), std::cmp::Ordering::Less);
    /// ```
    pub fn y_order(p: &Point2D, q: &Point2D) -> Ordering {
        p.y.partial_cmp(&q.y).unwrap_or(Ordering::Equal)
    }

    /// Compares two points by polar radius.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::Point2D;
    ///
    /// let p = Point2D::new(3.0, 0.0);
    /// let q = Point2D::new(0.0, 5.0);
    /// assert_eq!(Point2D::r_order(&p, &q), std::cmp::Ordering::Less);
    /// ```
    pub fn r_order(p: &Point2D, q: &Point2D) -> Ordering {
        p.r().partial_cmp(&q.r()).unwrap_or(Ordering::Equal)
    }

    /// Returns a comparator that compares two points by polar angle with respect to this point.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::Point2D;
    /// use std::cmp::Ordering;
    ///
    /// let origin = Point2D::new(0.0, 0.0);
    /// let p = Point2D::new(1.0, 0.0);  // 0 radians
    /// let q = Point2D::new(0.0, 1.0);  // π/2 radians
    ///
    /// assert_eq!(origin.polar_order(&p, &q), Ordering::Less);
    /// ```
    pub fn polar_order(&self, p: &Point2D, q: &Point2D) -> Ordering {
        let dx1 = p.x - self.x;
        let dy1 = p.y - self.y;
        let dx2 = q.x - self.x;
        let dy2 = q.y - self.y;

        if dy1 >= 0.0 && dy2 < 0.0 {
            return Ordering::Less; // p above; q below
        } else if dy2 >= 0.0 && dy1 < 0.0 {
            return Ordering::Greater; // p below; q above
        } else if dy1 == 0.0 && dy2 == 0.0 {
            // Both on x-axis
            if dx1 >= 0.0 && dx2 < 0.0 {
                return Ordering::Less;
            } else if dx2 >= 0.0 && dx1 < 0.0 {
                return Ordering::Greater;
            } else {
                return Ordering::Equal;
            }
        } else {
            // Both above or both below
            // Use CCW to determine order (negate because we want increasing angle)
            match Point2D::ccw(self, p, q) {
                -1 => Ordering::Greater,
                1 => Ordering::Less,
                _ => Ordering::Equal,
            }
        }
    }

    /// Returns a comparator that compares two points by atan2 angle with respect to this point.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::Point2D;
    /// use std::cmp::Ordering;
    ///
    /// let origin = Point2D::new(0.0, 0.0);
    /// let p = Point2D::new(1.0, 0.0);
    /// let q = Point2D::new(0.0, 1.0);
    ///
    /// assert_eq!(origin.atan2_order(&p, &q), Ordering::Less);
    /// ```
    pub fn atan2_order(&self, p: &Point2D, q: &Point2D) -> Ordering {
        let angle1 = self.angle_to(p);
        let angle2 = self.angle_to(q);
        angle1.partial_cmp(&angle2).unwrap_or(Ordering::Equal)
    }

    /// Returns a comparator that compares two points by distance to this point.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::Point2D;
    /// use std::cmp::Ordering;
    ///
    /// let origin = Point2D::new(0.0, 0.0);
    /// let p = Point2D::new(3.0, 0.0);
    /// let q = Point2D::new(0.0, 5.0);
    ///
    /// assert_eq!(origin.distance_to_order(&p, &q), Ordering::Less);
    /// ```
    pub fn distance_to_order(&self, p: &Point2D, q: &Point2D) -> Ordering {
        let dist1 = self.distance_squared_to(p);
        let dist2 = self.distance_squared_to(q);
        dist1.partial_cmp(&dist2).unwrap_or(Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let p = Point2D::new(3.0, 4.0);
        assert_eq!(p.x(), 3.0);
        assert_eq!(p.y(), 4.0);
    }

    #[test]
    fn test_negative_zero() {
        let p = Point2D::new(-0.0, -0.0);
        assert_eq!(p.x(), 0.0);
        assert_eq!(p.y(), 0.0);
    }

    #[test]
    #[should_panic(expected = "x-coordinate must be finite")]
    fn test_nan_x() {
        Point2D::new(f64::NAN, 0.0);
    }

    #[test]
    #[should_panic(expected = "y-coordinate must be finite")]
    fn test_nan_y() {
        Point2D::new(0.0, f64::NAN);
    }

    #[test]
    #[should_panic(expected = "x-coordinate must be finite")]
    fn test_infinite_x() {
        Point2D::new(f64::INFINITY, 0.0);
    }

    #[test]
    fn test_r() {
        let p = Point2D::new(3.0, 4.0);
        assert_eq!(p.r(), 5.0);

        let origin = Point2D::new(0.0, 0.0);
        assert_eq!(origin.r(), 0.0);
    }

    #[test]
    fn test_theta() {
        let p = Point2D::new(1.0, 0.0);
        assert_eq!(p.theta(), 0.0);

        let p = Point2D::new(0.0, 1.0);
        assert!((p.theta() - std::f64::consts::FRAC_PI_2).abs() < 1e-10);

        let p = Point2D::new(-1.0, 0.0);
        assert!((p.theta() - std::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn test_distance() {
        let p = Point2D::new(0.0, 0.0);
        let q = Point2D::new(3.0, 4.0);

        assert_eq!(p.distance_to(&q), 5.0);
        assert_eq!(p.distance_squared_to(&q), 25.0);
    }

    #[test]
    fn test_angle_to() {
        let p = Point2D::new(0.0, 0.0);
        let q = Point2D::new(1.0, 0.0);

        assert_eq!(p.angle_to(&q), 0.0);

        let r = Point2D::new(0.0, 1.0);
        assert!((p.angle_to(&r) - std::f64::consts::FRAC_PI_2).abs() < 1e-10);
    }

    #[test]
    fn test_ccw() {
        let a = Point2D::new(0.0, 0.0);
        let b = Point2D::new(1.0, 0.0);
        let c = Point2D::new(0.0, 1.0);

        assert_eq!(Point2D::ccw(&a, &b, &c), 1); // counter-clockwise
        assert_eq!(Point2D::ccw(&a, &c, &b), -1); // clockwise

        let d = Point2D::new(2.0, 0.0);
        assert_eq!(Point2D::ccw(&a, &b, &d), 0); // collinear
    }

    #[test]
    fn test_area2() {
        let a = Point2D::new(0.0, 0.0);
        let b = Point2D::new(1.0, 0.0);
        let c = Point2D::new(0.0, 1.0);

        assert_eq!(Point2D::area2(&a, &b, &c), 1.0);
    }

    #[test]
    fn test_equality() {
        let p1 = Point2D::new(3.0, 4.0);
        let p2 = Point2D::new(3.0, 4.0);
        let p3 = Point2D::new(4.0, 3.0);

        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
    }

    #[test]
    fn test_ordering() {
        let p1 = Point2D::new(1.0, 2.0);
        let p2 = Point2D::new(2.0, 1.0);
        let p3 = Point2D::new(1.0, 3.0);

        assert!(p2 < p1); // Lower y-coordinate
        assert!(p1 < p3); // Same x, lower y
    }

    #[test]
    fn test_x_order() {
        let p = Point2D::new(1.0, 5.0);
        let q = Point2D::new(2.0, 3.0);

        assert_eq!(Point2D::x_order(&p, &q), Ordering::Less);
        assert_eq!(Point2D::x_order(&q, &p), Ordering::Greater);
        assert_eq!(Point2D::x_order(&p, &p), Ordering::Equal);
    }

    #[test]
    fn test_y_order() {
        let p = Point2D::new(5.0, 1.0);
        let q = Point2D::new(3.0, 2.0);

        assert_eq!(Point2D::y_order(&p, &q), Ordering::Less);
        assert_eq!(Point2D::y_order(&q, &p), Ordering::Greater);
        assert_eq!(Point2D::y_order(&p, &p), Ordering::Equal);
    }

    #[test]
    fn test_r_order() {
        let p = Point2D::new(3.0, 0.0);
        let q = Point2D::new(0.0, 5.0);

        assert_eq!(Point2D::r_order(&p, &q), Ordering::Less);
    }

    #[test]
    fn test_polar_order() {
        let origin = Point2D::new(0.0, 0.0);
        let p = Point2D::new(1.0, 0.0); // 0 radians
        let q = Point2D::new(0.0, 1.0); // π/2 radians

        assert_eq!(origin.polar_order(&p, &q), Ordering::Less);
        assert_eq!(origin.polar_order(&q, &p), Ordering::Greater);
    }

    #[test]
    fn test_distance_to_order() {
        let origin = Point2D::new(0.0, 0.0);
        let p = Point2D::new(3.0, 0.0);
        let q = Point2D::new(0.0, 5.0);

        assert_eq!(origin.distance_to_order(&p, &q), Ordering::Less);
        assert_eq!(origin.distance_to_order(&q, &p), Ordering::Greater);
    }

    #[test]
    fn test_display() {
        let p = Point2D::new(3.0, 4.0);
        assert_eq!(format!("{}", p), "(3, 4)");
    }
}
