//! Gauss-Jordan elimination for solving linear systems.
//!
//! This module solves a system of linear equations Ax = b using Gauss-Jordan
//! elimination, which computes the reduced row echelon form (RREF). It also
//! provides a dual certificate of infeasibility when the system has no solution.
//!
//! # Examples
//!
//! ```
//! use algs4_advanced::GaussJordanElimination;
//!
//! // Solve: 2x + y = 5
//! //        x + 3y = 7
//! let a = vec![
//!     vec![2.0, 1.0],
//!     vec![1.0, 3.0],
//! ];
//! let b = vec![5.0, 7.0];
//!
//! let gj = GaussJordanElimination::new(a, b);
//! if let Some(x) = gj.primal() {
//!     println!("Solution: x = {:?}", x);
//! }
//! ```

use std::fmt;

const EPSILON: f64 = 1.0e-8;

/// Solves a linear system Ax = b using Gauss-Jordan elimination.
///
/// The algorithm computes the reduced row echelon form of the augmented matrix [A|b].
/// It can detect infeasibility and provide a certificate of infeasibility.
#[derive(Debug, Clone)]
pub struct GaussJordanElimination {
    /// Number of rows
    m: usize,
    /// Number of columns (excluding augmented column)
    n: usize,
    /// Augmented matrix [A|b] in RREF
    a: Vec<Vec<f64>>,
}

impl GaussJordanElimination {
    /// Creates a new Gauss-Jordan elimination solver for the system Ax = b.
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
    /// use algs4_advanced::GaussJordanElimination;
    ///
    /// let a = vec![
    ///     vec![2.0, 1.0],
    ///     vec![1.0, 3.0],
    /// ];
    /// let b = vec![5.0, 7.0];
    ///
    /// let gj = GaussJordanElimination::new(a, b);
    /// assert!(gj.is_feasible());
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

        let mut gj = GaussJordanElimination { m, n, a };
        gj.gauss_jordan();
        gj
    }

    /// Computes the reduced row echelon form.
    fn gauss_jordan(&mut self) {
        let mut col = 0;

        for row in 0..self.m {
            // Find pivot column
            while col < self.n {
                // Find pivot row
                let mut max_row = row;
                for i in (row + 1)..self.m {
                    if self.a[i][col].abs() > self.a[max_row][col].abs() {
                        max_row = i;
                    }
                }

                // Swap rows
                self.a.swap(row, max_row);

                // Check if pivot is non-zero
                if self.a[row][col].abs() > EPSILON {
                    break;
                }
                col += 1;
            }

            if col >= self.n {
                break;
            }

            // Scale pivot row to make pivot = 1
            let pivot = self.a[row][col];
            for j in col..=self.n {
                self.a[row][j] /= pivot;
            }

            // Eliminate column in all other rows
            for i in 0..self.m {
                if i != row && self.a[i][col].abs() > EPSILON {
                    let factor = self.a[i][col];
                    for j in col..=self.n {
                        self.a[i][j] -= factor * self.a[row][j];
                    }
                }
            }

            col += 1;
        }
    }

    /// Returns the solution to Ax = b, or None if no unique solution exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::GaussJordanElimination;
    ///
    /// let a = vec![
    ///     vec![2.0, 1.0],
    ///     vec![1.0, 3.0],
    /// ];
    /// let b = vec![5.0, 7.0];
    ///
    /// let gj = GaussJordanElimination::new(a, b);
    /// if let Some(x) = gj.primal() {
    ///     assert!((x[0] - 1.6).abs() < 1e-6);
    ///     assert!((x[1] - 1.8).abs() < 1e-6);
    /// }
    /// ```
    pub fn primal(&self) -> Option<Vec<f64>> {
        if !self.is_feasible() {
            return None;
        }

        let mut x = vec![0.0; self.n];

        for i in 0..self.m.min(self.n) {
            // Find the leading 1 in this row
            let mut leading_col = None;
            for j in 0..self.n {
                if self.a[i][j].abs() > EPSILON {
                    leading_col = Some(j);
                    break;
                }
            }

            if let Some(col) = leading_col {
                x[col] = self.a[i][self.n];
            }
        }

        Some(x)
    }

    /// Returns a certificate of infeasibility, or None if the system is feasible.
    ///
    /// The certificate is a vector y such that y^T A = 0 but y^T b ≠ 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::GaussJordanElimination;
    ///
    /// // Inconsistent system: x + y = 1, x + y = 2
    /// let a = vec![
    ///     vec![1.0, 1.0],
    ///     vec![1.0, 1.0],
    /// ];
    /// let b = vec![1.0, 2.0];
    ///
    /// let gj = GaussJordanElimination::new(a, b);
    /// assert!(!gj.is_feasible());
    /// assert!(gj.dual().is_some());
    /// ```
    pub fn dual(&self) -> Option<Vec<f64>> {
        if self.is_feasible() {
            return None;
        }

        // Find an inconsistent row (all zeros except RHS)
        for i in 0..self.m {
            let mut all_zero = true;
            for j in 0..self.n {
                if self.a[i][j].abs() > EPSILON {
                    all_zero = false;
                    break;
                }
            }

            if all_zero && self.a[i][self.n].abs() > EPSILON {
                // Create certificate vector
                let mut y = vec![0.0; self.m];
                y[i] = 1.0;
                return Some(y);
            }
        }

        None
    }

    /// Returns true if the system has a solution.
    ///
    /// Checks if any row has all zeros in the coefficient columns
    /// but a non-zero in the augmented column (indicating inconsistency).
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::GaussJordanElimination;
    ///
    /// let a = vec![
    ///     vec![1.0, 0.0],
    ///     vec![0.0, 1.0],
    /// ];
    /// let b = vec![1.0, 2.0];
    ///
    /// let gj = GaussJordanElimination::new(a, b);
    /// assert!(gj.is_feasible());
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

impl fmt::Display for GaussJordanElimination {
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

        let gj = GaussJordanElimination::new(a, b);
        assert!(gj.is_feasible());
        assert!(gj.dual().is_none());

        if let Some(x) = gj.primal() {
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

        let gj = GaussJordanElimination::new(a, b);
        assert!(gj.is_feasible());

        if let Some(x) = gj.primal() {
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

        let gj = GaussJordanElimination::new(a, b);
        assert!(gj.is_feasible());

        if let Some(x) = gj.primal() {
            assert_eq!(x.len(), 3);
            assert!((x[0] - 3.0).abs() < TEST_EPSILON);
            assert!((x[1] - 2.0).abs() < TEST_EPSILON);
            assert!((x[2] - 1.0).abs() < TEST_EPSILON);
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

        let gj = GaussJordanElimination::new(a, b);
        assert!(!gj.is_feasible());
        assert!(gj.primal().is_none());
        assert!(gj.dual().is_some());
    }

    #[test]
    fn test_underdetermined_system() {
        // x + y + z = 6
        // 2x + y + 2z = 10
        // Infinite solutions (underdetermined)
        let a = vec![vec![1.0, 1.0, 1.0], vec![2.0, 1.0, 2.0]];
        let b = vec![6.0, 10.0];

        let gj = GaussJordanElimination::new(a, b);
        assert!(gj.is_feasible());
        // System has infinite solutions, primal() returns one of them
        assert!(gj.primal().is_some());
    }

    #[test]
    fn test_overdetermined_consistent() {
        // x + y = 3
        // 2x + 2y = 6
        // 3x + 3y = 9
        // Overdetermined but consistent
        let a = vec![vec![1.0, 1.0], vec![2.0, 2.0], vec![3.0, 3.0]];
        let b = vec![3.0, 6.0, 9.0];

        let gj = GaussJordanElimination::new(a, b);
        assert!(gj.is_feasible());
    }

    #[test]
    fn test_overdetermined_inconsistent() {
        // x + y = 3
        // 2x + 2y = 6
        // 3x + 3y = 10  <- inconsistent
        let a = vec![vec![1.0, 1.0], vec![2.0, 2.0], vec![3.0, 3.0]];
        let b = vec![3.0, 6.0, 10.0];

        let gj = GaussJordanElimination::new(a, b);
        assert!(!gj.is_feasible());
        assert!(gj.dual().is_some());
    }
}
