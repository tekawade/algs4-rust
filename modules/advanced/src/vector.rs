//! Immutable d-dimensional Euclidean vector implementation.
//!
//! This module provides vector operations including:
//! - Vector arithmetic (addition, subtraction, scalar multiplication)
//! - Dot product
//! - Magnitude (L2 norm)
//! - Distance calculations
//! - Unit vector (direction)
//!
//! # Examples
//!
//! ```
//! use algs4_advanced::Vector;
//!
//! let x = Vector::from_slice(&[1.0, 2.0, 3.0, 4.0]);
//! let y = Vector::from_slice(&[5.0, 2.0, 4.0, 1.0]);
//!
//! let sum = x.plus(&y);     // Vector addition
//! let dot = x.dot(&y);      // Dot product
//! let mag = x.magnitude();  // Euclidean norm
//! let dist = x.distance_to(&y); // Euclidean distance
//!
//! println!("x + y = {}", sum);
//! println!("x · y = {}", dot);
//! println!("|x| = {}", mag);
//! ```

use std::fmt;

/// Represents an immutable d-dimensional Euclidean vector.
///
/// A vector is represented as an array of cartesian coordinates.
#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    /// Cartesian coordinates
    coords: Vec<f64>,
}

impl Vector {
    /// Creates a zero vector of the specified dimension.
    ///
    /// # Arguments
    ///
    /// * `d` - The dimension of the vector
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Vector;
    ///
    /// let v = Vector::new(5);
    /// assert_eq!(v.dimension(), 5);
    /// assert_eq!(v.cartesian(0), 0.0);
    /// ```
    pub fn new(d: usize) -> Self {
        Vector {
            coords: vec![0.0; d],
        }
    }

    /// Creates a vector from the specified array.
    ///
    /// # Arguments
    ///
    /// * `coords` - The array of coordinates (defensive copy made)
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Vector;
    ///
    /// let v = Vector::from_slice(&[1.0, 2.0, 3.0]);
    /// assert_eq!(v.dimension(), 3);
    /// assert_eq!(v.cartesian(0), 1.0);
    /// assert_eq!(v.cartesian(1), 2.0);
    /// assert_eq!(v.cartesian(2), 3.0);
    /// ```
    pub fn from_slice(coords: &[f64]) -> Self {
        Vector {
            coords: coords.to_vec(),
        }
    }

    /// Returns the dimension of this vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Vector;
    ///
    /// let v = Vector::from_slice(&[1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!(v.dimension(), 4);
    /// ```
    pub fn dimension(&self) -> usize {
        self.coords.len()
    }

    /// Returns the ith cartesian coordinate.
    ///
    /// # Arguments
    ///
    /// * `i` - The coordinate index (0-indexed)
    ///
    /// # Panics
    ///
    /// Panics if `i` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Vector;
    ///
    /// let v = Vector::from_slice(&[1.0, 2.0, 3.0]);
    /// assert_eq!(v.cartesian(1), 2.0);
    /// ```
    pub fn cartesian(&self, i: usize) -> f64 {
        self.coords[i]
    }

    /// Returns the sum of this vector and the specified vector.
    ///
    /// # Arguments
    ///
    /// * `that` - The vector to add
    ///
    /// # Panics
    ///
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Vector;
    ///
    /// let x = Vector::from_slice(&[1.0, 2.0, 3.0]);
    /// let y = Vector::from_slice(&[4.0, 5.0, 6.0]);
    /// let sum = x.plus(&y);
    /// assert_eq!(sum.cartesian(0), 5.0);
    /// assert_eq!(sum.cartesian(1), 7.0);
    /// assert_eq!(sum.cartesian(2), 9.0);
    /// ```
    pub fn plus(&self, that: &Vector) -> Vector {
        assert_eq!(
            self.dimension(),
            that.dimension(),
            "Vectors must have the same dimension"
        );
        let coords: Vec<f64> = (0..self.dimension())
            .map(|i| self.coords[i] + that.coords[i])
            .collect();
        Vector { coords }
    }

    /// Returns the difference of this vector and the specified vector.
    ///
    /// # Arguments
    ///
    /// * `that` - The vector to subtract
    ///
    /// # Panics
    ///
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Vector;
    ///
    /// let x = Vector::from_slice(&[5.0, 7.0, 9.0]);
    /// let y = Vector::from_slice(&[1.0, 2.0, 3.0]);
    /// let diff = x.minus(&y);
    /// assert_eq!(diff.cartesian(0), 4.0);
    /// assert_eq!(diff.cartesian(1), 5.0);
    /// assert_eq!(diff.cartesian(2), 6.0);
    /// ```
    pub fn minus(&self, that: &Vector) -> Vector {
        assert_eq!(
            self.dimension(),
            that.dimension(),
            "Vectors must have the same dimension"
        );
        let coords: Vec<f64> = (0..self.dimension())
            .map(|i| self.coords[i] - that.coords[i])
            .collect();
        Vector { coords }
    }

    /// Returns the dot product of this vector and the specified vector.
    ///
    /// # Arguments
    ///
    /// * `that` - The vector to compute the dot product with
    ///
    /// # Panics
    ///
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Vector;
    ///
    /// let x = Vector::from_slice(&[1.0, 2.0, 3.0]);
    /// let y = Vector::from_slice(&[4.0, 5.0, 6.0]);
    /// let dot = x.dot(&y);
    /// assert_eq!(dot, 32.0); // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
    /// ```
    pub fn dot(&self, that: &Vector) -> f64 {
        assert_eq!(
            self.dimension(),
            that.dimension(),
            "Vectors must have the same dimension"
        );
        (0..self.dimension())
            .map(|i| self.coords[i] * that.coords[i])
            .sum()
    }

    /// Returns the product of this vector and the specified scalar.
    ///
    /// # Arguments
    ///
    /// * `alpha` - The scalar to multiply by
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Vector;
    ///
    /// let v = Vector::from_slice(&[1.0, 2.0, 3.0]);
    /// let scaled = v.scale(2.0);
    /// assert_eq!(scaled.cartesian(0), 2.0);
    /// assert_eq!(scaled.cartesian(1), 4.0);
    /// assert_eq!(scaled.cartesian(2), 6.0);
    /// ```
    pub fn scale(&self, alpha: f64) -> Vector {
        let coords: Vec<f64> = self.coords.iter().map(|&x| alpha * x).collect();
        Vector { coords }
    }

    /// Returns the magnitude (L2 norm) of this vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Vector;
    ///
    /// let v = Vector::from_slice(&[3.0, 4.0]);
    /// assert_eq!(v.magnitude(), 5.0);
    /// ```
    pub fn magnitude(&self) -> f64 {
        self.coords.iter().map(|&x| x * x).sum::<f64>().sqrt()
    }

    /// Returns the Euclidean distance between this vector and the specified vector.
    ///
    /// # Arguments
    ///
    /// * `that` - The vector to compute the distance to
    ///
    /// # Panics
    ///
    /// Panics if the vectors have different dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Vector;
    ///
    /// let x = Vector::from_slice(&[0.0, 0.0]);
    /// let y = Vector::from_slice(&[3.0, 4.0]);
    /// assert_eq!(x.distance_to(&y), 5.0);
    /// ```
    pub fn distance_to(&self, that: &Vector) -> f64 {
        self.minus(that).magnitude()
    }

    /// Returns a unit vector in the direction of this vector.
    ///
    /// # Panics
    ///
    /// Panics if this is the zero vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Vector;
    ///
    /// let v = Vector::from_slice(&[3.0, 4.0]);
    /// let dir = v.direction();
    /// assert_eq!(dir.cartesian(0), 0.6);
    /// assert_eq!(dir.cartesian(1), 0.8);
    /// assert!((dir.magnitude() - 1.0).abs() < 1e-10);
    /// ```
    pub fn direction(&self) -> Vector {
        let mag = self.magnitude();
        assert!(mag > 0.0, "Cannot compute direction of zero vector");
        self.scale(1.0 / mag)
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        for (i, &coord) in self.coords.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", coord)?;
        }
        write!(f, ")")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_new() {
        let v = Vector::new(5);
        assert_eq!(v.dimension(), 5);
        for i in 0..5 {
            assert_eq!(v.cartesian(i), 0.0);
        }
    }

    #[test]
    fn test_from_slice() {
        let v = Vector::from_slice(&[1.0, 2.0, 3.0, 4.0]);
        assert_eq!(v.dimension(), 4);
        assert_eq!(v.cartesian(0), 1.0);
        assert_eq!(v.cartesian(1), 2.0);
        assert_eq!(v.cartesian(2), 3.0);
        assert_eq!(v.cartesian(3), 4.0);
    }

    #[test]
    fn test_addition() {
        let x = Vector::from_slice(&[1.0, 2.0, 3.0, 4.0]);
        let y = Vector::from_slice(&[5.0, 2.0, 4.0, 1.0]);
        let sum = x.plus(&y);
        assert_eq!(sum.cartesian(0), 6.0);
        assert_eq!(sum.cartesian(1), 4.0);
        assert_eq!(sum.cartesian(2), 7.0);
        assert_eq!(sum.cartesian(3), 5.0);
    }

    #[test]
    fn test_subtraction() {
        let x = Vector::from_slice(&[5.0, 4.0, 7.0, 5.0]);
        let y = Vector::from_slice(&[1.0, 2.0, 3.0, 4.0]);
        let diff = x.minus(&y);
        assert_eq!(diff.cartesian(0), 4.0);
        assert_eq!(diff.cartesian(1), 2.0);
        assert_eq!(diff.cartesian(2), 4.0);
        assert_eq!(diff.cartesian(3), 1.0);
    }

    #[test]
    fn test_dot_product() {
        let x = Vector::from_slice(&[1.0, 2.0, 3.0, 4.0]);
        let y = Vector::from_slice(&[5.0, 2.0, 4.0, 1.0]);
        let dot = x.dot(&y);
        // 1*5 + 2*2 + 3*4 + 4*1 = 5 + 4 + 12 + 4 = 25
        assert_eq!(dot, 25.0);
    }

    #[test]
    fn test_scale() {
        let v = Vector::from_slice(&[1.0, 2.0, 3.0]);
        let scaled = v.scale(2.5);
        assert_eq!(scaled.cartesian(0), 2.5);
        assert_eq!(scaled.cartesian(1), 5.0);
        assert_eq!(scaled.cartesian(2), 7.5);
    }

    #[test]
    fn test_magnitude() {
        let v = Vector::from_slice(&[3.0, 4.0]);
        assert_eq!(v.magnitude(), 5.0);
    }

    #[test]
    fn test_distance() {
        let x = Vector::from_slice(&[0.0, 0.0]);
        let y = Vector::from_slice(&[3.0, 4.0]);
        assert_eq!(x.distance_to(&y), 5.0);
    }

    #[test]
    fn test_direction() {
        let v = Vector::from_slice(&[3.0, 4.0]);
        let dir = v.direction();
        assert!((dir.cartesian(0) - 0.6).abs() < EPSILON);
        assert!((dir.cartesian(1) - 0.8).abs() < EPSILON);
        assert!((dir.magnitude() - 1.0).abs() < EPSILON);
    }

    #[test]
    #[should_panic(expected = "Cannot compute direction of zero vector")]
    fn test_direction_zero_vector() {
        let v = Vector::new(3);
        v.direction();
    }

    #[test]
    #[should_panic(expected = "Vectors must have the same dimension")]
    fn test_addition_different_dimensions() {
        let x = Vector::from_slice(&[1.0, 2.0]);
        let y = Vector::from_slice(&[1.0, 2.0, 3.0]);
        x.plus(&y);
    }

    #[test]
    fn test_display() {
        let v = Vector::from_slice(&[1.0, 2.0, 3.0]);
        assert_eq!(format!("{}", v), "(1, 2, 3)");
    }
}
