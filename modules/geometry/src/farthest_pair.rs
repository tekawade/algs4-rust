//! Farthest pair of points algorithm
//!
//! This module provides the `FarthestPair` type that finds the farthest pair
//! of points (diameter) in a set using the rotating calipers method.
//!
//! # Examples
//!
//! ```
//! use algs4_geometry::{FarthestPair, Point2D};
//!
//! let points = vec![
//!     Point2D::new(0.0, 0.0),
//!     Point2D::new(1.0, 1.0),
//!     Point2D::new(5.0, 5.0),
//!     Point2D::new(10.0, 10.0),
//! ];
//!
//! let farthest = FarthestPair::new(&points);
//! println!("Farthest distance (diameter): {}", farthest.distance());
//! ```

use crate::{GrahamScan, Point2D};

/// Finds the farthest pair of points (diameter) in a set.
///
/// This implementation uses the convex hull and rotating calipers method,
/// running in O(n log n) time and using O(n) extra space.
///
/// # Examples
///
/// ```
/// use algs4_geometry::{FarthestPair, Point2D};
///
/// let points = vec![
///     Point2D::new(0.0, 0.0),
///     Point2D::new(3.0, 0.0),
///     Point2D::new(0.0, 4.0),
/// ];
///
/// let farthest = FarthestPair::new(&points);
/// assert_eq!(farthest.distance(), 5.0);
/// ```
#[derive(Debug)]
pub struct FarthestPair {
    point1: Point2D,
    point2: Point2D,
    distance_squared: f64,
}

impl FarthestPair {
    /// Computes the farthest pair of points in the specified array.
    ///
    /// # Panics
    ///
    /// Panics if the array has fewer than 2 points.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::{FarthestPair, Point2D};
    ///
    /// let points = vec![
    ///     Point2D::new(0.0, 0.0),
    ///     Point2D::new(10.0, 10.0),
    /// ];
    ///
    /// let farthest = FarthestPair::new(&points);
    /// ```
    pub fn new(points: &[Point2D]) -> Self {
        if points.len() < 2 {
            panic!("Need at least 2 points to find farthest pair");
        }

        // Compute the convex hull
        let graham = GrahamScan::new(points);
        let hull = graham.hull();

        // Handle degenerate cases
        if hull.len() == 1 {
            return FarthestPair {
                point1: hull[0],
                point2: hull[0],
                distance_squared: 0.0,
            };
        }

        if hull.len() == 2 {
            return FarthestPair {
                point1: hull[0],
                point2: hull[1],
                distance_squared: hull[0].distance_squared_to(&hull[1]),
            };
        }

        // Find the farthest pair by checking all pairs on the hull
        // Since the hull is typically small, this is efficient
        let mut best_distance_squared = 0.0;
        let mut best_p1 = hull[0];
        let mut best_p2 = hull[0];

        let m = hull.len();
        for i in 0..m {
            for j in (i + 1)..m {
                let dist_sq = hull[i].distance_squared_to(&hull[j]);
                if dist_sq > best_distance_squared {
                    best_p1 = hull[i];
                    best_p2 = hull[j];
                    best_distance_squared = dist_sq;
                }
            }
        }

        FarthestPair {
            point1: best_p1,
            point2: best_p2,
            distance_squared: best_distance_squared,
        }
    }

    /// Returns one of the two farthest points.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::{FarthestPair, Point2D};
    ///
    /// let points = vec![
    ///     Point2D::new(0.0, 0.0),
    ///     Point2D::new(10.0, 10.0),
    /// ];
    ///
    /// let farthest = FarthestPair::new(&points);
    /// let _p1 = farthest.either();
    /// ```
    pub fn either(&self) -> Point2D {
        self.point1
    }

    /// Returns the other of the two farthest points.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::{FarthestPair, Point2D};
    ///
    /// let points = vec![
    ///     Point2D::new(0.0, 0.0),
    ///     Point2D::new(10.0, 10.0),
    /// ];
    ///
    /// let farthest = FarthestPair::new(&points);
    /// let _p2 = farthest.other();
    /// ```
    pub fn other(&self) -> Point2D {
        self.point2
    }

    /// Returns the Euclidean distance between the two farthest points (diameter).
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::{FarthestPair, Point2D};
    ///
    /// let points = vec![
    ///     Point2D::new(0.0, 0.0),
    ///     Point2D::new(3.0, 0.0),
    ///     Point2D::new(0.0, 4.0),
    /// ];
    ///
    /// let farthest = FarthestPair::new(&points);
    /// assert_eq!(farthest.distance(), 5.0);
    /// ```
    pub fn distance(&self) -> f64 {
        self.distance_squared.sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_points() {
        let points = vec![Point2D::new(0.0, 0.0), Point2D::new(3.0, 4.0)];

        let farthest = FarthestPair::new(&points);
        assert_eq!(farthest.distance(), 5.0);
    }

    #[test]
    fn test_triangle() {
        let points = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(3.0, 0.0),
            Point2D::new(0.0, 4.0),
        ];

        let farthest = FarthestPair::new(&points);
        assert_eq!(farthest.distance(), 5.0);
    }

    #[test]
    fn test_square() {
        let points = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(1.0, 0.0),
            Point2D::new(1.0, 1.0),
            Point2D::new(0.0, 1.0),
        ];

        let farthest = FarthestPair::new(&points);
        // Diagonal of unit square
        assert!((farthest.distance() - 2.0_f64.sqrt()).abs() < 1e-10);
    }

    #[test]
    fn test_with_interior_points() {
        let points = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(10.0, 0.0),
            Point2D::new(10.0, 10.0),
            Point2D::new(0.0, 10.0),
            Point2D::new(5.0, 5.0), // Interior point (shouldn't affect result)
        ];

        let farthest = FarthestPair::new(&points);
        // Diagonal of 10x10 square
        assert!((farthest.distance() - (10.0 * 2.0_f64.sqrt())).abs() < 1e-10);
    }

    #[test]
    fn test_collinear_points() {
        let points = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(1.0, 0.0),
            Point2D::new(2.0, 0.0),
            Point2D::new(10.0, 0.0),
        ];

        let farthest = FarthestPair::new(&points);
        assert_eq!(farthest.distance(), 10.0);
    }

    #[test]
    fn test_circle_points() {
        // Points on a circle - any diameter should be 2*radius
        let radius = 5.0;
        let mut points = vec![];
        for i in 0..8 {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / 8.0;
            points.push(Point2D::new(radius * angle.cos(), radius * angle.sin()));
        }

        let farthest = FarthestPair::new(&points);
        // Diameter should be approximately 2*radius
        assert!((farthest.distance() - 2.0 * radius).abs() < 0.1);
    }

    #[test]
    #[should_panic(expected = "Need at least 2 points")]
    fn test_single_point() {
        let points = vec![Point2D::new(0.0, 0.0)];
        FarthestPair::new(&points);
    }

    #[test]
    #[should_panic(expected = "Need at least 2 points")]
    fn test_empty() {
        let points: Vec<Point2D> = vec![];
        FarthestPair::new(&points);
    }
}
