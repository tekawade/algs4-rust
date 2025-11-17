//! # algs4-geometry
//!
//! Geometric algorithms from *Algorithms, 4th Edition*
//! by Robert Sedgewick and Kevin Wayne.
//!
//! This module contains:
//! - Point and interval data types
//! - Convex hull (Graham scan)
//! - Line segment intersection
//! - Closest pair and farthest pair
//!
//! ## Example
//!
//! ```
//! use algs4_geometry::Point2D;
//!
//! let p = Point2D::new(3.0, 4.0);
//! let q = Point2D::new(0.0, 0.0);
//! assert_eq!(p.distance_to(&q), 5.0);
//! ```

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![allow(clippy::needless_return)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::needless_range_loop)]

// Geometric primitives
pub mod interval1d;
pub mod interval2d;
pub mod point2d;
pub mod recthv;

// Geometric algorithms
pub mod closest_pair;
pub mod farthest_pair;
pub mod graham_scan;

// Re-export main types
pub use closest_pair::ClosestPair;
pub use farthest_pair::FarthestPair;
pub use graham_scan::GrahamScan;
pub use interval1d::Interval1D;
pub use interval2d::Interval2D;
pub use point2d::Point2D;
pub use recthv::RectHV;
