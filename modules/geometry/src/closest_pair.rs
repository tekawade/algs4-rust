//! Closest pair of points algorithm
//!
//! This module provides the `ClosestPair` type that finds the closest pair
//! of points in a set using a divide-and-conquer algorithm.
//!
//! # Examples
//!
//! ```
//! use algs4_geometry::{ClosestPair, Point2D};
//!
//! let points = vec![
//!     Point2D::new(0.0, 0.0),
//!     Point2D::new(1.0, 1.0),
//!     Point2D::new(5.0, 5.0),
//!     Point2D::new(10.0, 10.0),
//! ];
//!
//! let closest = ClosestPair::new(&points);
//! println!("Closest distance: {}", closest.distance());
//! ```

use crate::Point2D;

/// Finds the closest pair of points in a set.
///
/// This implementation uses a divide-and-conquer algorithm that runs in
/// O(n log n) time and uses O(n) extra space.
///
/// # Examples
///
/// ```
/// use algs4_geometry::{ClosestPair, Point2D};
///
/// let points = vec![
///     Point2D::new(0.0, 0.0),
///     Point2D::new(3.0, 4.0),
///     Point2D::new(6.0, 8.0),
/// ];
///
/// let closest = ClosestPair::new(&points);
/// assert_eq!(closest.distance(), 5.0); // Distance from (0,0) to (3,4)
/// ```
#[derive(Debug)]
pub struct ClosestPair {
    point1: Point2D,
    point2: Point2D,
    distance: f64,
}

impl ClosestPair {
    /// Computes the closest pair of points in the specified array.
    ///
    /// # Panics
    ///
    /// Panics if the array has fewer than 2 points.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::{ClosestPair, Point2D};
    ///
    /// let points = vec![
    ///     Point2D::new(0.0, 0.0),
    ///     Point2D::new(1.0, 1.0),
    /// ];
    ///
    /// let closest = ClosestPair::new(&points);
    /// ```
    pub fn new(points: &[Point2D]) -> Self {
        if points.len() < 2 {
            panic!("Need at least 2 points to find closest pair");
        }

        let n = points.len();

        // Sort by x-coordinate (breaking ties by y-coordinate)
        let mut points_by_x = points.to_vec();
        points_by_x.sort_by(|a, b| Point2D::x_order(a, b));

        // Check for coincident points
        for i in 0..n - 1 {
            if points_by_x[i] == points_by_x[i + 1] {
                return ClosestPair {
                    point1: points_by_x[i],
                    point2: points_by_x[i + 1],
                    distance: 0.0,
                };
            }
        }

        // Sort by y-coordinate (breaking ties by x-coordinate)
        let mut points_by_y = points_by_x.clone();
        points_by_y.sort_by(|a, b| Point2D::y_order(a, b));

        // Auxiliary array for merging
        let mut aux = vec![Point2D::new(0.0, 0.0); n];

        let (p1, p2, dist) = Self::closest(&points_by_x, &mut points_by_y, &mut aux, 0, n - 1);

        ClosestPair {
            point1: p1,
            point2: p2,
            distance: dist,
        }
    }

    /// Recursive divide-and-conquer algorithm
    fn closest(
        points_by_x: &[Point2D],
        points_by_y: &mut [Point2D],
        aux: &mut [Point2D],
        lo: usize,
        hi: usize,
    ) -> (Point2D, Point2D, f64) {
        if hi <= lo {
            return (
                Point2D::new(0.0, 0.0),
                Point2D::new(0.0, 0.0),
                f64::INFINITY,
            );
        }

        let mid = lo + (hi - lo) / 2;
        let median = points_by_x[mid];

        // Recursively find closest pair in left and right halves
        let (p1_left, p2_left, delta_left) = Self::closest(points_by_x, points_by_y, aux, lo, mid);
        let (p1_right, p2_right, delta_right) =
            Self::closest(points_by_x, points_by_y, aux, mid + 1, hi);

        // Find minimum of left and right
        let (mut best_p1, mut best_p2, mut delta) = if delta_left < delta_right {
            (p1_left, p2_left, delta_left)
        } else {
            (p1_right, p2_right, delta_right)
        };

        // Merge points_by_y sorted arrays
        Self::merge(points_by_y, aux, lo, mid, hi);

        // Find closest pair with one point in each half (within delta of median)
        let mut m = 0;
        for i in lo..=hi {
            if (points_by_y[i].x() - median.x()).abs() < delta {
                aux[m] = points_by_y[i];
                m += 1;
            }
        }

        // Check distances in the strip
        for i in 0..m {
            // A geometric packing argument shows that this loop iterates at most 7 times
            let mut j = i + 1;
            while j < m && (aux[j].y() - aux[i].y()) < delta {
                let dist = aux[i].distance_to(&aux[j]);
                if dist < delta {
                    delta = dist;
                    best_p1 = aux[i];
                    best_p2 = aux[j];
                }
                j += 1;
            }
        }

        (best_p1, best_p2, delta)
    }

    /// Merge sorted subarrays
    fn merge(a: &mut [Point2D], aux: &mut [Point2D], lo: usize, mid: usize, hi: usize) {
        // Copy to auxiliary array
        for k in lo..=hi {
            aux[k] = a[k];
        }

        // Merge back to a[]
        let mut i = lo;
        let mut j = mid + 1;
        for k in lo..=hi {
            if i > mid {
                a[k] = aux[j];
                j += 1;
            } else if j > hi {
                a[k] = aux[i];
                i += 1;
            } else if Point2D::y_order(&aux[j], &aux[i]) == std::cmp::Ordering::Less {
                a[k] = aux[j];
                j += 1;
            } else {
                a[k] = aux[i];
                i += 1;
            }
        }
    }

    /// Returns one of the two closest points.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::{ClosestPair, Point2D};
    ///
    /// let points = vec![
    ///     Point2D::new(0.0, 0.0),
    ///     Point2D::new(1.0, 1.0),
    /// ];
    ///
    /// let closest = ClosestPair::new(&points);
    /// let _p1 = closest.either();
    /// ```
    pub fn either(&self) -> Point2D {
        self.point1
    }

    /// Returns the other of the two closest points.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::{ClosestPair, Point2D};
    ///
    /// let points = vec![
    ///     Point2D::new(0.0, 0.0),
    ///     Point2D::new(1.0, 1.0),
    /// ];
    ///
    /// let closest = ClosestPair::new(&points);
    /// let _p2 = closest.other();
    /// ```
    pub fn other(&self) -> Point2D {
        self.point2
    }

    /// Returns the Euclidean distance between the two closest points.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_geometry::{ClosestPair, Point2D};
    ///
    /// let points = vec![
    ///     Point2D::new(0.0, 0.0),
    ///     Point2D::new(3.0, 4.0),
    /// ];
    ///
    /// let closest = ClosestPair::new(&points);
    /// assert_eq!(closest.distance(), 5.0);
    /// ```
    pub fn distance(&self) -> f64 {
        self.distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_points() {
        let points = vec![Point2D::new(0.0, 0.0), Point2D::new(3.0, 4.0)];

        let closest = ClosestPair::new(&points);
        assert_eq!(closest.distance(), 5.0);
    }

    #[test]
    fn test_multiple_points() {
        let points = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(1.0, 1.0),
            Point2D::new(5.0, 5.0),
            Point2D::new(10.0, 10.0),
        ];

        let closest = ClosestPair::new(&points);
        // Closest pair should be (0,0) and (1,1) with distance sqrt(2)
        assert!((closest.distance() - 2.0_f64.sqrt()).abs() < 1e-10);
    }

    #[test]
    fn test_coincident_points() {
        let points = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(1.0, 1.0),
            Point2D::new(1.0, 1.0), // Duplicate
        ];

        let closest = ClosestPair::new(&points);
        assert_eq!(closest.distance(), 0.0);
    }

    #[test]
    fn test_collinear_points() {
        let points = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(1.0, 0.0),
            Point2D::new(2.0, 0.0),
            Point2D::new(10.0, 0.0),
        ];

        let closest = ClosestPair::new(&points);
        assert_eq!(closest.distance(), 1.0);
    }

    #[test]
    fn test_vertical_points() {
        let points = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(0.0, 1.0),
            Point2D::new(0.0, 2.0),
            Point2D::new(0.0, 10.0),
        ];

        let closest = ClosestPair::new(&points);
        assert_eq!(closest.distance(), 1.0);
    }

    #[test]
    fn test_grid_points() {
        let mut points = vec![];
        for i in 0..5 {
            for j in 0..5 {
                points.push(Point2D::new(i as f64, j as f64));
            }
        }

        let closest = ClosestPair::new(&points);
        assert_eq!(closest.distance(), 1.0);
    }

    #[test]
    #[should_panic(expected = "Need at least 2 points")]
    fn test_single_point() {
        let points = vec![Point2D::new(0.0, 0.0)];
        ClosestPair::new(&points);
    }

    #[test]
    #[should_panic(expected = "Need at least 2 points")]
    fn test_empty() {
        let points: Vec<Point2D> = vec![];
        ClosestPair::new(&points);
    }
}
