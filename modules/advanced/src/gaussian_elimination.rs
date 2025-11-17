//! Gaussian elimination with partial pivoting for solving linear systems.
//!
//! This module solves a system of linear equations Ax = b using Gaussian
//! elimination with partial pivoting. The algorithm reduces the augmented
//! matrix to upper triangular form, then uses back substitution to find
//! the solution.
//!
//! # Examples
//!
//! ```
//! use algs4_advanced::GaussianElimination;
//!
//! // Solve: 2x + y = 5
//! //        x + 3y = 7
//! let a = vec![
//!     vec![2.0, 1.0],
//!     vec![1.0, 3.0],
//! ];
//! let b = vec![5.0, 7.0];
//!
//! let ge = GaussianElimination::new(a, b);
//! if let Some(x) = ge.primal() {
//!     println!("Solution: x = {:?}", x);
//!     // x ≈ [1.6, 1.8]
//! }
//! ```

use std::fmt;

const EPSILON: f64 = 1.0e-8;

/// Solves a linear system Ax = b using Gaussian elimination with partial pivoting.
///
/// The algorithm performs forward elimination to create an upper triangular matrix,
/// then uses back substitution to solve for x.
#[derive(Debug, Clone)]
pub struct GaussianElimination {
    /// Number of rows
    m: usize,
    /// Number of columns (excluding augmented column)
    n: usize,
    /// Augmented matrix [A|b]
    a: Vec<Vec<f64>>,
}

impl GaussianElimination {
    /// Creates a new Gaussian elimination solver for the system Ax = b.
    ///
    /// # Arguments
    ///
    /// * `A` - The coefficient matrix (m × n)
    /// * `b` - The right-hand side vector (length m)
    ///
    /// # Panics
    ///
    /// Panics if the dimensions are inconsistent.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::GaussianElimination;
    ///
    /// let a = vec![
    ///     vec![2.0, 1.0],
    ///     vec![1.0, 3.0],
    /// ];
    /// let b = vec![5.0, 7.0];
    ///
    /// let ge = GaussianElimination::new(a, b);
    /// assert!(ge.is_feasible());
    /// ```
    #[allow(non_snake_case)]
    pub fn new(A: Vec<Vec<f64>>, b: Vec<f64>) -> Self {
        let m = b.len();
        assert!(!A.is_empty(), "Matrix A cannot be empty");
        let n = A[0].len();
        assert_eq!(A.len(), m, "Dimensions mismatch");

        // Create augmented matrix [A|b]
        let mut a = vec![vec![0.0; n + 1]; m];
        for i in 0..m {
            assert_eq!(A[i].len(), n, "All rows must have same length");
            for j in 0..n {
                a[i][j] = A[i][j];
            }
            a[i][n] = b[i];
        }

        let mut ge = GaussianElimination { m, n, a };
        ge.forward_elimination();
        ge
    }

    /// Performs forward elimination with partial pivoting.
    fn forward_elimination(&mut self) {
        for p in 0..self.n.min(self.m) {
            // Find pivot row
            let mut max = p;
            for i in (p + 1)..self.m {
                if self.a[i][p].abs() > self.a[max][p].abs() {
                    max = i;
                }
            }

            // Swap rows p and max
            self.a.swap(p, max);

            // Skip if pivot is too small
            if self.a[p][p].abs() <= EPSILON {
                continue;
            }

            // Eliminate column p below diagonal
            for i in (p + 1)..self.m {
                let alpha = self.a[i][p] / self.a[p][p];
                for j in p..=self.n {
                    self.a[i][j] -= alpha * self.a[p][j];
                }
            }
        }
    }

    /// Returns the solution to Ax = b, or None if no unique solution exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::GaussianElimination;
    ///
    /// let a = vec![
    ///     vec![2.0, 1.0],
    ///     vec![1.0, 3.0],
    /// ];
    /// let b = vec![5.0, 7.0];
    ///
    /// let ge = GaussianElimination::new(a, b);
    /// if let Some(x) = ge.primal() {
    ///     assert!((x[0] - 1.6).abs() < 1e-6);
    ///     assert!((x[1] - 1.8).abs() < 1e-6);
    /// }
    /// ```
    pub fn primal(&self) -> Option<Vec<f64>> {
        // Check if system has a unique solution
        if !self.is_feasible() {
            return None;
        }

        // Back substitution
        let mut x = vec![0.0; self.n];
        for i in (0..self.n.min(self.m)).rev() {
            let mut sum = 0.0;
            #[allow(clippy::needless_range_loop)]
            for j in (i + 1)..self.n {
                sum += self.a[i][j] * x[j];
            }

            if self.a[i][i].abs() <= EPSILON {
                return None;
            }

            x[i] = (self.a[i][self.n] - sum) / self.a[i][i];
        }

        Some(x)
    }

    /// Returns true if the system has a solution.
    ///
    /// Checks if any row has all zeros in the coefficient columns
    /// but a non-zero in the augmented column (indicating inconsistency).
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::GaussianElimination;
    ///
    /// let a = vec![
    ///     vec![1.0, 0.0],
    ///     vec![0.0, 1.0],
    /// ];
    /// let b = vec![1.0, 2.0];
    ///
    /// let ge = GaussianElimination::new(a, b);
    /// assert!(ge.is_feasible());
    /// ```
    pub fn is_feasible(&self) -> bool {
        // Check for inconsistent rows (0 = non-zero)
        for i in 0..self.m {
            let mut all_zero = true;
            for j in 0..self.n {
                if self.a[i][j].abs() > EPSILON {
                    all_zero = false;
                    break;
                }
            }
            // If all coefficients are zero but RHS is non-zero, inconsistent
            if all_zero && self.a[i][self.n].abs() > EPSILON {
                return false;
            }
        }
        true
    }
}

impl fmt::Display for GaussianElimination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.m {
            for j in 0..self.n {
                write!(f, "{:9.4} ", self.a[i][j])?;
            }
            writeln!(f, "| {:9.4}", self.a[i][self.n])?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_EPSILON: f64 = 1e-6;

    #[test]
    fn test_simple_2x2_system() {
        // 2x + y = 5
        // x + 3y = 7
        // Solution: x = 1.6, y = 1.8
        let a = vec![vec![2.0, 1.0], vec![1.0, 3.0]];
        let b = vec![5.0, 7.0];

        let ge = GaussianElimination::new(a, b);
        assert!(ge.is_feasible());

        if let Some(x) = ge.primal() {
            assert_eq!(x.len(), 2);
            assert!((x[0] - 1.6).abs() < TEST_EPSILON);
            assert!((x[1] - 1.8).abs() < TEST_EPSILON);
        } else {
            panic!("Expected a solution");
        }
    }

    #[test]
    fn test_3x3_system() {
        // x + 2y + 3z = 14
        // 2x + 5y + 2z = 18
        // 3x + y + 5z = 20
        // Solution: x = 1, y = 2, z = 3
        let a = vec![
            vec![1.0, 2.0, 3.0],
            vec![2.0, 5.0, 2.0],
            vec![3.0, 1.0, 5.0],
        ];
        let b = vec![14.0, 18.0, 20.0];

        let ge = GaussianElimination::new(a, b);
        assert!(ge.is_feasible());

        if let Some(x) = ge.primal() {
            assert_eq!(x.len(), 3);
            assert!((x[0] - 1.0).abs() < TEST_EPSILON);
            assert!((x[1] - 2.0).abs() < TEST_EPSILON);
            assert!((x[2] - 3.0).abs() < TEST_EPSILON);
        } else {
            panic!("Expected a solution");
        }
    }

    #[test]
    fn test_identity_matrix() {
        let a = vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0],
        ];
        let b = vec![3.0, 2.0, 1.0];

        let ge = GaussianElimination::new(a, b);
        assert!(ge.is_feasible());

        if let Some(x) = ge.primal() {
            assert_eq!(x.len(), 3);
            assert!((x[0] - 3.0).abs() < TEST_EPSILON);
            assert!((x[1] - 2.0).abs() < TEST_EPSILON);
            assert!((x[2] - 1.0).abs() < TEST_EPSILON);
        } else {
            panic!("Expected a solution");
        }
    }

    #[test]
    fn test_partial_pivoting() {
        // Test that requires pivoting
        let a = vec![vec![0.0, 2.0], vec![1.0, 1.0]];
        let b = vec![4.0, 3.0];

        let ge = GaussianElimination::new(a, b);
        assert!(ge.is_feasible());

        if let Some(x) = ge.primal() {
            assert_eq!(x.len(), 2);
            assert!((x[0] - 1.0).abs() < TEST_EPSILON);
            assert!((x[1] - 2.0).abs() < TEST_EPSILON);
        } else {
            panic!("Expected a solution");
        }
    }

    #[test]
    fn test_inconsistent_system() {
        // x + y = 1
        // x + y = 2
        // Inconsistent system
        let a = vec![vec![1.0, 1.0], vec![1.0, 1.0]];
        let b = vec![1.0, 2.0];

        let ge = GaussianElimination::new(a, b);
        assert!(!ge.is_feasible());
    }

    #[test]
    fn test_singular_matrix() {
        // x + y = 1
        // 2x + 2y = 2
        // Infinite solutions (dependent equations)
        let a = vec![vec![1.0, 1.0], vec![2.0, 2.0]];
        let b = vec![1.0, 2.0];

        let ge = GaussianElimination::new(a, b);
        // This is actually feasible (infinite solutions)
        assert!(ge.is_feasible());
    }
}
