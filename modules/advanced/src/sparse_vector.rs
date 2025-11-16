//! Sparse vector implementation using a hash map.
//!
//! A sparse vector stores only the non-zero coordinates, making it
//! memory-efficient for high-dimensional vectors with few non-zero values.
//!
//! # Examples
//!
//! ```
//! use algs4_advanced::SparseVector;
//!
//! let mut v = SparseVector::new(10000);
//! v.put(5, 0.75);
//! v.put(100, 0.11);
//! v.put(9999, 0.33);
//!
//! println!("dimension = {}", v.dimension());
//! println!("non-zeros = {}", v.nnz());
//! println!("magnitude = {}", v.magnitude());
//! ```

use std::collections::HashMap;
use std::fmt;

/// Represents a d-dimensional sparse vector.
///
/// Only non-zero coordinates are stored to save memory.
#[derive(Debug, Clone)]
pub struct SparseVector {
    /// Dimension of the vector
    d: usize,
    /// Symbol table mapping indices to non-zero values
    st: HashMap<usize, f64>,
}

impl SparseVector {
    /// Creates a d-dimensional zero vector.
    ///
    /// # Arguments
    ///
    /// * `d` - The dimension of the vector
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::SparseVector;
    ///
    /// let v = SparseVector::new(1000);
    /// assert_eq!(v.dimension(), 1000);
    /// assert_eq!(v.nnz(), 0);
    /// ```
    pub fn new(d: usize) -> Self {
        SparseVector {
            d,
            st: HashMap::new(),
        }
    }

    /// Sets the ith coordinate to the specified value.
    ///
    /// # Arguments
    ///
    /// * `i` - The index
    /// * `value` - The value to set
    ///
    /// # Panics
    ///
    /// Panics if index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::SparseVector;
    ///
    /// let mut v = SparseVector::new(10);
    /// v.put(5, 0.75);
    /// assert_eq!(v.get(5), 0.75);
    /// ```
    pub fn put(&mut self, i: usize, value: f64) {
        assert!(i < self.d, "Index out of bounds");
        if value == 0.0 {
            self.st.remove(&i);
        } else {
            self.st.insert(i, value);
        }
    }

    /// Returns the ith coordinate.
    ///
    /// # Arguments
    ///
    /// * `i` - The index
    ///
    /// # Panics
    ///
    /// Panics if index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::SparseVector;
    ///
    /// let mut v = SparseVector::new(10);
    /// v.put(5, 0.75);
    /// assert_eq!(v.get(5), 0.75);
    /// assert_eq!(v.get(6), 0.0);
    /// ```
    pub fn get(&self, i: usize) -> f64 {
        assert!(i < self.d, "Index out of bounds");
        *self.st.get(&i).unwrap_or(&0.0)
    }

    /// Returns the number of non-zero entries.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::SparseVector;
    ///
    /// let mut v = SparseVector::new(1000);
    /// v.put(5, 0.75);
    /// v.put(100, 0.11);
    /// assert_eq!(v.nnz(), 2);
    /// ```
    pub fn nnz(&self) -> usize {
        self.st.len()
    }

    /// Returns the dimension of this vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::SparseVector;
    ///
    /// let v = SparseVector::new(1000);
    /// assert_eq!(v.dimension(), 1000);
    /// ```
    pub fn dimension(&self) -> usize {
        self.d
    }

    /// Returns the dot product of this vector with the specified vector.
    ///
    /// # Arguments
    ///
    /// * `that` - The vector to compute dot product with
    ///
    /// # Panics
    ///
    /// Panics if vectors have different dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::SparseVector;
    ///
    /// let mut a = SparseVector::new(10);
    /// a.put(3, 0.50);
    /// a.put(4, 0.25);
    ///
    /// let mut b = SparseVector::new(10);
    /// b.put(3, 2.0);
    /// b.put(8, 4.0);
    ///
    /// let dot = a.dot(&b);
    /// assert_eq!(dot, 1.0); // 0.50 * 2.0 = 1.0
    /// ```
    pub fn dot(&self, that: &SparseVector) -> f64 {
        assert_eq!(self.d, that.d, "Vectors must have same dimension");

        let mut sum = 0.0;

        // Iterate over the vector with fewer non-zeros
        if self.st.len() <= that.st.len() {
            for (&i, &value) in &self.st {
                if let Some(&that_value) = that.st.get(&i) {
                    sum += value * that_value;
                }
            }
        } else {
            for (&i, &value) in &that.st {
                if let Some(&self_value) = self.st.get(&i) {
                    sum += self_value * value;
                }
            }
        }

        sum
    }

    /// Returns the magnitude (L2 norm) of this vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::SparseVector;
    ///
    /// let mut v = SparseVector::new(10);
    /// v.put(0, 3.0);
    /// v.put(1, 4.0);
    /// assert_eq!(v.magnitude(), 5.0);
    /// ```
    pub fn magnitude(&self) -> f64 {
        self.dot(self).sqrt()
    }

    /// Returns the result of scaling this vector by alpha.
    ///
    /// # Arguments
    ///
    /// * `alpha` - The scaling factor
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::SparseVector;
    ///
    /// let mut v = SparseVector::new(10);
    /// v.put(3, 0.5);
    /// let scaled = v.scale(2.0);
    /// assert_eq!(scaled.get(3), 1.0);
    /// ```
    pub fn scale(&self, alpha: f64) -> SparseVector {
        let mut result = SparseVector::new(self.d);
        for (&i, &value) in &self.st {
            result.put(i, alpha * value);
        }
        result
    }

    /// Returns the sum of this vector and the specified vector.
    ///
    /// # Arguments
    ///
    /// * `that` - The vector to add
    ///
    /// # Panics
    ///
    /// Panics if vectors have different dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::SparseVector;
    ///
    /// let mut a = SparseVector::new(10);
    /// a.put(3, 0.5);
    ///
    /// let mut b = SparseVector::new(10);
    /// b.put(3, 0.3);
    /// b.put(5, 0.7);
    ///
    /// let sum = a.plus(&b);
    /// assert_eq!(sum.get(3), 0.8);
    /// assert_eq!(sum.get(5), 0.7);
    /// ```
    pub fn plus(&self, that: &SparseVector) -> SparseVector {
        assert_eq!(self.d, that.d, "Vectors must have same dimension");

        let mut result = SparseVector::new(self.d);

        for (&i, &value) in &self.st {
            result.put(i, value);
        }

        for (&i, &value) in &that.st {
            result.put(i, result.get(i) + value);
        }

        result
    }
}

impl fmt::Display for SparseVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut indices: Vec<_> = self.st.keys().collect();
        indices.sort();

        for &i in indices.iter() {
            write!(f, "({}, {}) ", i, self.st[i])?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_new() {
        let v = SparseVector::new(1000);
        assert_eq!(v.dimension(), 1000);
        assert_eq!(v.nnz(), 0);
    }

    #[test]
    fn test_put_and_get() {
        let mut v = SparseVector::new(10);
        v.put(5, 0.75);
        assert_eq!(v.get(5), 0.75);
        assert_eq!(v.get(6), 0.0);
        assert_eq!(v.nnz(), 1);
    }

    #[test]
    fn test_put_zero_removes_entry() {
        let mut v = SparseVector::new(10);
        v.put(5, 0.75);
        assert_eq!(v.nnz(), 1);
        v.put(5, 0.0);
        assert_eq!(v.nnz(), 0);
        assert_eq!(v.get(5), 0.0);
    }

    #[test]
    fn test_dot_product() {
        let mut a = SparseVector::new(10);
        a.put(3, 0.50);
        a.put(4, 0.25);

        let mut b = SparseVector::new(10);
        b.put(3, 2.0);
        b.put(8, 4.0);

        let dot = a.dot(&b);
        assert!((dot - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_magnitude() {
        let mut v = SparseVector::new(10);
        v.put(0, 3.0);
        v.put(1, 4.0);
        assert!((v.magnitude() - 5.0).abs() < EPSILON);
    }

    #[test]
    fn test_scale() {
        let mut v = SparseVector::new(10);
        v.put(3, 0.5);
        v.put(5, 0.3);
        let scaled = v.scale(2.0);
        assert!((scaled.get(3) - 1.0).abs() < EPSILON);
        assert!((scaled.get(5) - 0.6).abs() < EPSILON);
    }

    #[test]
    fn test_plus() {
        let mut a = SparseVector::new(10);
        a.put(3, 0.5);

        let mut b = SparseVector::new(10);
        b.put(3, 0.3);
        b.put(5, 0.7);

        let sum = a.plus(&b);
        assert!((sum.get(3) - 0.8).abs() < EPSILON);
        assert!((sum.get(5) - 0.7).abs() < EPSILON);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_put_out_of_bounds() {
        let mut v = SparseVector::new(10);
        v.put(10, 1.0);
    }

    #[test]
    #[should_panic(expected = "Vectors must have same dimension")]
    fn test_dot_different_dimensions() {
        let a = SparseVector::new(10);
        let b = SparseVector::new(20);
        a.dot(&b);
    }
}
