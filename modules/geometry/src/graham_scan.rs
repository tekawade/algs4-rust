//! Graham scan algorithm for computing the convex hull
//!
//! This module provides the `GrahamScan` type that computes the convex hull
//! of a set of points in the plane using the Graham scan algorithm.
//!
//! # Examples
//!
//! ```
//! use algs4_geometry::{GrahamScan, Point2D};
//!
//! let points = vec![
//!     Point2D::new(0.0, 0.0),
//!     Point2D::new(1.0, 1.0),
//!     Point2D::new(2.0, 0.0),
//!     Point2D::new(0.0, 2.0),
//!     Point2D::new(1.0, 0.5), // Interior point
//! ];
//!
//! let graham = GrahamScan::new(&points);
//! let hull = graham.hull();
//!
//! println!("Convex hull has {} points", hull.len());
//! ```

use crate::Point2D;

/// Computes the convex hull of a set of points using the Graham scan algorithm.
///
/// The convex hull is the smallest convex polygon that contains all the points.
/// This implementation runs in O(n log n) time.
///
/// # Examples
///
/// ```
/// use algs4_geometry::{GrahamScan, Point2D};
///
/// let points = vec![
///     Point2D::new(0.0, 0.0),
///     Point2D::new(4.0, 0.0),
///     Point2D::new(4.0, 3.0),
///     Point2D::new(0.0, 3.0),
///     Point2D::new(2.0, 1.0), // Interior point
/// ];
///
/// let graham = GrahamScan::new(&points);
/// let hull = graham.hull();
///
/// // Hull should have 4 points (the rectangle vertices)
/// assert_eq!(hull.len(), 4);
/// ```
#[derive(Debug)]
pub struct GrahamScan {
    hull: Vec<Point2D>,
}

impl GrahamScan {
    /// Computes the convex hull of the specified array of points.
    ///
    /// # Panics
    ///
    /// Panics if the points array is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::{GrahamScan, Point2D};
    ///
    /// let points = vec![
    ///     Point2D::new(0.0, 0.0),
    ///     Point2D::new(1.0, 1.0),
    ///     Point2D::new(2.0, 0.0),
    /// ];
    ///
    /// let graham = GrahamScan::new(&points);
    /// ```
    pub fn new(points: &[Point2D]) -> Self {
        if points.is_empty() {
            panic!("Cannot compute convex hull of empty point set");
        }

        let n = points.len();

        // Make a copy and sort by y-coordinate (lowest first)
        let mut sorted_points = points.to_vec();
        sorted_points.sort();

        // Sort by polar angle with respect to the base point (sorted_points[0])
        // Break ties by distance
        let base = sorted_points[0];
        sorted_points[1..].sort_by(|a, b| {
            let order = base.polar_order(a, b);
            if order == std::cmp::Ordering::Equal {
                // Break ties by distance to base point
                base.distance_to_order(a, b)
            } else {
                order
            }
        });

        // Use a vector as a stack to build the convex hull
        let mut stack: Vec<Point2D> = Vec::with_capacity(n);

        // Find the first point not equal to sorted_points[0]
        let mut k1 = 1;
        while k1 < n && sorted_points[k1] == sorted_points[0] {
            k1 += 1;
        }
        if k1 == n {
            // All points are the same
            stack.push(sorted_points[0]);
            return GrahamScan { hull: stack };
        }

        // Find the first point not collinear with sorted_points[0] and sorted_points[k1]
        let mut k2 = k1 + 1;
        while k2 < n && Point2D::ccw(&sorted_points[0], &sorted_points[k1], &sorted_points[k2]) == 0
        {
            k2 += 1;
        }

        // Push base point and the farthest collinear point
        stack.push(sorted_points[0]);

        if k2 == n {
            // All remaining points are collinear with the first two
            // Push the farthest point
            stack.push(sorted_points[n - 1]);
            return GrahamScan { hull: stack };
        }

        stack.push(sorted_points[k2 - 1]); // Push the last collinear point
        stack.push(sorted_points[k2]); // Push the first non-collinear point

        // Graham scan
        for i in (k2 + 1)..n {
            let mut top = stack.pop().unwrap();

            // Pop points while we have a non-left turn
            while !stack.is_empty()
                && Point2D::ccw(stack.last().unwrap(), &top, &sorted_points[i]) <= 0
            {
                top = stack.pop().unwrap();
            }

            stack.push(top);
            stack.push(sorted_points[i]);
        }

        GrahamScan { hull: stack }
    }

    /// Returns the extreme points on the convex hull in counterclockwise order.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::{GrahamScan, Point2D};
    ///
    /// let points = vec![
    ///     Point2D::new(0.0, 0.0),
    ///     Point2D::new(1.0, 1.0),
    ///     Point2D::new(2.0, 0.0),
    /// ];
    ///
    /// let graham = GrahamScan::new(&points);
    /// let hull = graham.hull();
    ///
    /// assert_eq!(hull.len(), 3);
    /// ```
    pub fn hull(&self) -> &[Point2D] {
        &self.hull
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle() {
        let points = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(1.0, 1.0),
            Point2D::new(2.0, 0.0),
        ];

        let graham = GrahamScan::new(&points);
        let hull = graham.hull();

        assert_eq!(hull.len(), 3);
    }

    #[test]
    fn test_square_with_interior_point() {
        let points = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(4.0, 0.0),
            Point2D::new(4.0, 4.0),
            Point2D::new(0.0, 4.0),
            Point2D::new(2.0, 2.0), // Interior point
        ];

        let graham = GrahamScan::new(&points);
        let hull = graham.hull();

        // Hull should have 4 points (the square vertices)
        assert_eq!(hull.len(), 4);
    }

    #[test]
    fn test_collinear_points() {
        let points = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(1.0, 0.0),
            Point2D::new(2.0, 0.0),
            Point2D::new(3.0, 0.0),
        ];

        let graham = GrahamScan::new(&points);
        let hull = graham.hull();

        // Hull should have 2 points (the endpoints)
        assert_eq!(hull.len(), 2);
    }

    #[test]
    fn test_single_point() {
        let points = vec![Point2D::new(1.0, 1.0)];

        let graham = GrahamScan::new(&points);
        let hull = graham.hull();

        assert_eq!(hull.len(), 1);
        assert_eq!(hull[0], points[0]);
    }

    #[test]
    fn test_two_points() {
        let points = vec![Point2D::new(0.0, 0.0), Point2D::new(1.0, 1.0)];

        let graham = GrahamScan::new(&points);
        let hull = graham.hull();

        assert_eq!(hull.len(), 2);
    }

    #[test]
    fn test_pentagon() {
        // Regular pentagon-like shape
        let points = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(2.0, -0.5),
            Point2D::new(3.0, 1.5),
            Point2D::new(1.5, 3.0),
            Point2D::new(-0.5, 2.0),
            Point2D::new(1.0, 1.0), // Interior point
        ];

        let graham = GrahamScan::new(&points);
        let hull = graham.hull();

        // Should exclude the interior point
        assert_eq!(hull.len(), 5);
    }

    #[test]
    fn test_duplicate_points() {
        let points = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(0.0, 0.0), // Duplicate
            Point2D::new(1.0, 0.0),
            Point2D::new(1.0, 1.0),
            Point2D::new(0.0, 1.0),
        ];

        let graham = GrahamScan::new(&points);
        let hull = graham.hull();

        // Hull should be a square
        assert!(hull.len() >= 3); // At least 3 distinct points
    }

    #[test]
    #[should_panic(expected = "Cannot compute convex hull of empty point set")]
    fn test_empty_points() {
        let points: Vec<Point2D> = vec![];
        GrahamScan::new(&points);
    }

    #[test]
    fn test_all_same_points() {
        let points = vec![
            Point2D::new(1.0, 1.0),
            Point2D::new(1.0, 1.0),
            Point2D::new(1.0, 1.0),
        ];

        let graham = GrahamScan::new(&points);
        let hull = graham.hull();

        assert_eq!(hull.len(), 1);
    }
}
