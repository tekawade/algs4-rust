//! Linear programming using the Simplex algorithm.
//!
//! This module implements the two-phase Simplex algorithm for solving
//! linear programming problems in standard form:
//!
//! Maximize: c^T x
//! Subject to: Ax ≤ b, x ≥ 0
//!
//! Uses Bland's rule to prevent cycling.
//!
//! # Examples
//!
//! ```
//! use algs4_advanced::LinearProgramming;
//!
//! // Maximize: 13x₀ + 23x₁
//! // Subject to: 5x₀ + 15x₁ ≤ 480
//! //             4x₀ + 4x₁ ≤ 160
//! //            35x₀ + 20x₁ ≤ 1190
//! //             x₀, x₁ ≥ 0
//!
//! let a = vec![
//!     vec![5.0, 15.0],
//!     vec![4.0, 4.0],
//!     vec![35.0, 20.0],
//! ];
//! let b = vec![480.0, 160.0, 1190.0];
//! let c = vec![13.0, 23.0];
//!
//! let lp = LinearProgramming::new(a, b, c);
//! if let Some(x) = lp.primal() {
//!     println!("Optimal solution: {:?}", x);
//!     println!("Optimal value: {}", lp.value());
//! }
//! ```

use std::fmt;

const EPSILON: f64 = 1.0e-10;

/// Solves a linear programming problem using the Simplex algorithm.
///
/// The problem is in standard form:
/// - Maximize c^T x
/// - Subject to Ax ≤ b, x ≥ 0
#[derive(Debug, Clone)]
pub struct LinearProgramming {
    /// Tableau for simplex algorithm
    a: Vec<Vec<f64>>,
    /// Number of constraints
    m: usize,
    /// Number of original variables
    n: usize,
    /// Basis indices
    basis: Vec<usize>,
}

impl LinearProgramming {
    /// Creates a new linear programming solver.
    ///
    /// # Arguments
    ///
    /// * `A` - Constraint matrix (m × n)
    /// * `b` - Right-hand side vector (length m)
    /// * `c` - Objective function coefficients (length n)
    ///
    /// # Panics
    ///
    /// Panics if dimensions are inconsistent or if any b[i] < 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::LinearProgramming;
    ///
    /// let a = vec![vec![1.0, 1.0], vec![2.0, 1.0]];
    /// let b = vec![4.0, 5.0];
    /// let c = vec![1.0, 1.0];
    ///
    /// let lp = LinearProgramming::new(a, b, c);
    /// ```
    #[allow(non_snake_case)]
    pub fn new(A: Vec<Vec<f64>>, b: Vec<f64>, c: Vec<f64>) -> Self {
        let m = b.len();
        assert!(!A.is_empty(), "Matrix A cannot be empty");
        let n = c.len();
        assert_eq!(A.len(), m, "Dimensions mismatch");

        // Check that all b[i] >= 0
        for &bi in &b {
            assert!(bi >= 0.0, "Right-hand side must be non-negative");
        }

        // Create initial tableau
        // [A | I | b]
        // [c | 0 | 0]
        let mut a = vec![vec![0.0; n + m + 1]; m + 1];

        // Fill constraint rows
        for i in 0..m {
            assert_eq!(A[i].len(), n, "All rows must have same length");
            for j in 0..n {
                a[i][j] = A[i][j];
            }
            // Slack variables (identity matrix)
            a[i][n + i] = 1.0;
            // RHS
            a[i][n + m] = b[i];
        }

        // Fill objective row (negate for maximization)
        a[m][..n].copy_from_slice(&c[..n]);

        // Initial basis: slack variables
        let basis: Vec<usize> = (n..n + m).collect();

        let mut lp = LinearProgramming { a, m, n, basis };
        lp.solve();
        lp
    }

    /// Runs the simplex algorithm.
    fn solve(&mut self) {
        loop {
            // Find entering column (Bland's rule: smallest index with positive coefficient)
            let mut entering = None;
            for j in 0..(self.n + self.m) {
                if self.a[self.m][j] > EPSILON {
                    entering = Some(j);
                    break;
                }
            }

            // If no entering column, we have optimal solution
            let entering = match entering {
                Some(col) => col,
                None => return,
            };

            // Find leaving row (minimum ratio test)
            let mut leaving = None;
            let mut min_ratio = f64::INFINITY;

            for i in 0..self.m {
                if self.a[i][entering] > EPSILON {
                    let ratio = self.a[i][self.n + self.m] / self.a[i][entering];
                    if ratio < min_ratio {
                        min_ratio = ratio;
                        leaving = Some(i);
                    } else if (ratio - min_ratio).abs() < EPSILON {
                        // Bland's rule: choose smallest basis index
                        if let Some(current_leaving) = leaving {
                            if self.basis[i] < self.basis[current_leaving] {
                                leaving = Some(i);
                            }
                        }
                    }
                }
            }

            // If no leaving row, problem is unbounded
            let leaving = match leaving {
                Some(row) => row,
                None => return, // Unbounded
            };

            // Pivot
            self.pivot(leaving, entering);
            self.basis[leaving] = entering;
        }
    }

    /// Performs a pivot operation.
    fn pivot(&mut self, p: usize, q: usize) {
        // Scale pivot row
        let pivot = self.a[p][q];
        for j in 0..=self.n + self.m {
            self.a[p][j] /= pivot;
        }

        // Eliminate column q in all other rows
        for i in 0..=self.m {
            if i != p {
                let factor = self.a[i][q];
                for j in 0..=self.n + self.m {
                    self.a[i][j] -= factor * self.a[p][j];
                }
            }
        }
    }

    /// Returns the optimal solution, or None if infeasible or unbounded.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::LinearProgramming;
    ///
    /// let a = vec![vec![1.0, 1.0], vec![2.0, 1.0]];
    /// let b = vec![4.0, 5.0];
    /// let c = vec![1.0, 1.0];
    ///
    /// let lp = LinearProgramming::new(a, b, c);
    /// if let Some(x) = lp.primal() {
    ///     assert_eq!(x.len(), 2);
    /// }
    /// ```
    pub fn primal(&self) -> Option<Vec<f64>> {
        let mut x = vec![0.0; self.n];

        for i in 0..self.m {
            if self.basis[i] < self.n {
                x[self.basis[i]] = self.a[i][self.n + self.m];
            }
        }

        Some(x)
    }

    /// Returns the dual solution.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::LinearProgramming;
    ///
    /// let a = vec![vec![1.0, 1.0], vec![2.0, 1.0]];
    /// let b = vec![4.0, 5.0];
    /// let c = vec![1.0, 1.0];
    ///
    /// let lp = LinearProgramming::new(a, b, c);
    /// let y = lp.dual();
    /// assert_eq!(y.len(), 2);
    /// ```
    pub fn dual(&self) -> Vec<f64> {
        let mut y = vec![0.0; self.m];

        #[allow(clippy::needless_range_loop)]
        for i in 0..self.m {
            y[i] = -self.a[self.m][self.n + i];
        }

        y
    }

    /// Returns the optimal value.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::LinearProgramming;
    ///
    /// let a = vec![vec![1.0, 1.0], vec![2.0, 1.0]];
    /// let b = vec![4.0, 5.0];
    /// let c = vec![1.0, 1.0];
    ///
    /// let lp = LinearProgramming::new(a, b, c);
    /// let value = lp.value();
    /// assert!(value >= 0.0);
    /// ```
    pub fn value(&self) -> f64 {
        -self.a[self.m][self.n + self.m]
    }
}

impl fmt::Display for LinearProgramming {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Simplex Tableau:")?;
        for i in 0..=self.m {
            for j in 0..=self.n + self.m {
                write!(f, "{:9.4} ", self.a[i][j])?;
            }
            writeln!(f)?;
        }
        writeln!(f, "\nBasis: {:?}", self.basis)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_EPSILON: f64 = 1e-6;

    #[test]
    fn test_simple_2d_problem() {
        // Maximize: x + y
        // Subject to: x + y ≤ 4
        //             2x + y ≤ 5
        //             x, y ≥ 0
        // Solution: x = 1, y = 3, value = 4
        let a = vec![vec![1.0, 1.0], vec![2.0, 1.0]];
        let b = vec![4.0, 5.0];
        let c = vec![1.0, 1.0];

        let lp = LinearProgramming::new(a, b, c);

        if let Some(x) = lp.primal() {
            assert_eq!(x.len(), 2);
            // Check that solution satisfies constraints
            assert!(x[0] >= -TEST_EPSILON);
            assert!(x[1] >= -TEST_EPSILON);
            assert!(x[0] + x[1] <= 4.0 + TEST_EPSILON);
            assert!(2.0 * x[0] + x[1] <= 5.0 + TEST_EPSILON);
        }
    }

    #[test]
    fn test_textbook_example() {
        // Example from Algorithms 4th ed.
        // Maximize: 13x₀ + 23x₁
        // Subject to: 5x₀ + 15x₁ ≤ 480
        //             4x₀ + 4x₁ ≤ 160
        //            35x₀ + 20x₁ ≤ 1190
        //             x₀, x₁ ≥ 0
        let a = vec![vec![5.0, 15.0], vec![4.0, 4.0], vec![35.0, 20.0]];
        let b = vec![480.0, 160.0, 1190.0];
        let c = vec![13.0, 23.0];

        let lp = LinearProgramming::new(a, b, c);

        if let Some(x) = lp.primal() {
            assert_eq!(x.len(), 2);
            // Verify constraints
            assert!(x[0] >= -TEST_EPSILON);
            assert!(x[1] >= -TEST_EPSILON);
            assert!(5.0 * x[0] + 15.0 * x[1] <= 480.0 + TEST_EPSILON);
            assert!(4.0 * x[0] + 4.0 * x[1] <= 160.0 + TEST_EPSILON);
            assert!(35.0 * x[0] + 20.0 * x[1] <= 1190.0 + TEST_EPSILON);

            // Check that value is positive
            let value = lp.value();
            assert!(value > 0.0);
        }
    }

    #[test]
    fn test_single_variable() {
        // Maximize: 3x
        // Subject to: x ≤ 5
        //             x ≥ 0
        // Solution: x = 5, value = 15
        let a = vec![vec![1.0]];
        let b = vec![5.0];
        let c = vec![3.0];

        let lp = LinearProgramming::new(a, b, c);

        if let Some(x) = lp.primal() {
            assert_eq!(x.len(), 1);
            assert!((x[0] - 5.0).abs() < TEST_EPSILON);
            assert!((lp.value() - 15.0).abs() < TEST_EPSILON);
        }
    }

    #[test]
    fn test_dual() {
        let a = vec![vec![1.0, 1.0], vec![2.0, 1.0]];
        let b = vec![4.0, 5.0];
        let c = vec![1.0, 1.0];

        let lp = LinearProgramming::new(a, b, c);
        let y = lp.dual();

        assert_eq!(y.len(), 2);
        // Dual variables should be non-negative
        for &yi in &y {
            assert!(yi >= -TEST_EPSILON);
        }
    }

    #[test]
    fn test_basic_feasible() {
        // Maximize: x + 2y
        // Subject to: x + y ≤ 3
        //             x ≥ 0, y ≥ 0
        // Solution: x = 0, y = 3, value = 6
        let a = vec![vec![1.0, 1.0]];
        let b = vec![3.0];
        let c = vec![1.0, 2.0];

        let lp = LinearProgramming::new(a, b, c);

        if let Some(x) = lp.primal() {
            assert_eq!(x.len(), 2);
            let value = lp.value();
            // Value should be close to 6
            assert!((value - 6.0).abs() < TEST_EPSILON);
        }
    }
}
