//! Immutable polynomial with integer coefficients.
//!
//! This module provides polynomial operations including:
//! - Polynomial arithmetic (addition, subtraction, multiplication)
//! - Composition (p(q(x)))
//! - Differentiation
//! - Evaluation at a point
//!
//! # Examples
//!
//! ```
//! use algs4_advanced::Polynomial;
//!
//! // Create polynomial 4x³ + 3x² + 2x + 1
//! let p = Polynomial::new(1, 0)
//!     .plus(&Polynomial::new(2, 1))
//!     .plus(&Polynomial::new(3, 2))
//!     .plus(&Polynomial::new(4, 3));
//!
//! println!("p(x) = {}", p);
//! println!("p(2) = {}", p.evaluate(2));
//! println!("degree = {}", p.degree());
//!
//! let q = Polynomial::new(1, 1); // x
//! let pq = p.compose(&q);
//! println!("p(q(x)) = {}", pq);
//! ```

use std::cmp::Ordering;
use std::fmt;

/// Represents an immutable polynomial with integer coefficients.
///
/// The polynomial is represented as a sum of terms, where each term
/// has the form `coefficient * x^exponent`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Polynomial {
    /// Coefficients: coef[i] is coefficient of x^i
    coef: Vec<i32>,
    /// Degree of polynomial (accounting for leading zero coefficients)
    deg: usize,
}

impl Polynomial {
    /// Creates a polynomial term a*x^b.
    ///
    /// # Arguments
    ///
    /// * `a` - The coefficient
    /// * `b` - The exponent
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Polynomial;
    ///
    /// let p = Polynomial::new(5, 3); // 5x³
    /// assert_eq!(p.degree(), 3);
    /// assert_eq!(p.evaluate(2), 40); // 5 * 2³ = 40
    /// ```
    pub fn new(a: i32, b: usize) -> Self {
        let mut coef = vec![0; b + 1];
        coef[b] = a;
        Polynomial { coef, deg: b }
    }

    /// Creates a zero polynomial.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Polynomial;
    ///
    /// let p = Polynomial::zero();
    /// assert_eq!(p.degree(), 0);
    /// assert_eq!(p.evaluate(5), 0);
    /// ```
    pub fn zero() -> Self {
        Polynomial {
            coef: vec![0],
            deg: 0,
        }
    }

    /// Returns the degree of this polynomial.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Polynomial;
    ///
    /// let p = Polynomial::new(1, 0)
    ///     .plus(&Polynomial::new(2, 1))
    ///     .plus(&Polynomial::new(3, 2));
    /// assert_eq!(p.degree(), 2);
    /// ```
    pub fn degree(&self) -> usize {
        self.deg
    }

    /// Returns the sum of this polynomial and the specified polynomial.
    ///
    /// # Arguments
    ///
    /// * `that` - The polynomial to add
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Polynomial;
    ///
    /// let p1 = Polynomial::new(4, 3).plus(&Polynomial::new(3, 2));
    /// let p2 = Polynomial::new(2, 1).plus(&Polynomial::new(1, 0));
    /// let sum = p1.plus(&p2);
    /// // sum = 4x³ + 3x² + 2x + 1
    /// assert_eq!(sum.evaluate(1), 10);
    /// ```
    pub fn plus(&self, that: &Polynomial) -> Polynomial {
        let max_len = self.coef.len().max(that.coef.len());
        let mut coef = vec![0; max_len];

        for i in 0..self.coef.len() {
            coef[i] += self.coef[i];
        }
        for i in 0..that.coef.len() {
            coef[i] += that.coef[i];
        }

        Polynomial::from_coefficients(coef)
    }

    /// Returns the difference of this polynomial and the specified polynomial.
    ///
    /// # Arguments
    ///
    /// * `that` - The polynomial to subtract
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Polynomial;
    ///
    /// let p1 = Polynomial::new(5, 2); // 5x²
    /// let p2 = Polynomial::new(3, 2); // 3x²
    /// let diff = p1.minus(&p2);       // 2x²
    /// assert_eq!(diff.evaluate(3), 18); // 2 * 9 = 18
    /// ```
    pub fn minus(&self, that: &Polynomial) -> Polynomial {
        let max_len = self.coef.len().max(that.coef.len());
        let mut coef = vec![0; max_len];

        for i in 0..self.coef.len() {
            coef[i] += self.coef[i];
        }
        for i in 0..that.coef.len() {
            coef[i] -= that.coef[i];
        }

        Polynomial::from_coefficients(coef)
    }

    /// Returns the product of this polynomial and the specified polynomial.
    ///
    /// # Arguments
    ///
    /// * `that` - The polynomial to multiply
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Polynomial;
    ///
    /// let p1 = Polynomial::new(1, 1).plus(&Polynomial::new(1, 0)); // x + 1
    /// let p2 = Polynomial::new(1, 1).minus(&Polynomial::new(1, 0)); // x - 1
    /// let product = p1.times(&p2); // x² - 1
    /// assert_eq!(product.evaluate(3), 8); // 9 - 1 = 8
    /// ```
    pub fn times(&self, that: &Polynomial) -> Polynomial {
        let mut coef = vec![0; self.coef.len() + that.coef.len() - 1];

        for i in 0..self.coef.len() {
            for j in 0..that.coef.len() {
                coef[i + j] += self.coef[i] * that.coef[j];
            }
        }

        Polynomial::from_coefficients(coef)
    }

    /// Returns the composition of this polynomial and the specified polynomial.
    ///
    /// Returns p(q(x)) where p is this polynomial and q is the argument.
    ///
    /// # Arguments
    ///
    /// * `that` - The polynomial to compose with
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Polynomial;
    ///
    /// let p = Polynomial::new(1, 2); // x²
    /// let q = Polynomial::new(1, 1).plus(&Polynomial::new(1, 0)); // x + 1
    /// let pq = p.compose(&q); // (x + 1)² = x² + 2x + 1
    /// assert_eq!(pq.evaluate(2), 9); // (2 + 1)² = 9
    /// ```
    pub fn compose(&self, that: &Polynomial) -> Polynomial {
        let mut result = Polynomial::zero();
        for i in (0..=self.deg).rev() {
            let term = Polynomial::new(self.coef[i], 0);
            result = term.plus(&result.times(that));
        }
        result
    }

    /// Returns the derivative of this polynomial.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Polynomial;
    ///
    /// let p = Polynomial::new(3, 2)
    ///     .plus(&Polynomial::new(2, 1))
    ///     .plus(&Polynomial::new(1, 0)); // 3x² + 2x + 1
    /// let dp = p.differentiate(); // 6x + 2
    /// assert_eq!(dp.evaluate(1), 8);
    /// ```
    pub fn differentiate(&self) -> Polynomial {
        if self.deg == 0 {
            return Polynomial::zero();
        }

        let mut coef = vec![0; self.deg];
        for (i, item) in coef.iter_mut().enumerate().take(self.deg) {
            *item = (i + 1) as i32 * self.coef[i + 1];
        }

        Polynomial::from_coefficients(coef)
    }

    /// Evaluates this polynomial at the specified value.
    ///
    /// Uses Horner's method for efficient evaluation.
    ///
    /// # Arguments
    ///
    /// * `x` - The value at which to evaluate the polynomial
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Polynomial;
    ///
    /// let p = Polynomial::new(1, 0)
    ///     .plus(&Polynomial::new(2, 1))
    ///     .plus(&Polynomial::new(3, 2)); // 3x² + 2x + 1
    /// assert_eq!(p.evaluate(2), 17); // 3*4 + 2*2 + 1 = 17
    /// ```
    pub fn evaluate(&self, x: i32) -> i32 {
        let mut result = 0;
        for i in (0..=self.deg).rev() {
            result = result * x + self.coef[i];
        }
        result
    }

    /// Helper function to create polynomial from coefficient array.
    fn from_coefficients(mut coef: Vec<i32>) -> Polynomial {
        // Find actual degree (ignoring leading zeros)
        let mut deg = coef.len() - 1;
        while deg > 0 && coef[deg] == 0 {
            deg -= 1;
        }

        // Trim trailing zeros
        coef.truncate(deg + 1);

        Polynomial { coef, deg }
    }
}

impl PartialOrd for Polynomial {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Polynomial {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare by degree first
        match self.deg.cmp(&other.deg) {
            Ordering::Equal => {
                // If same degree, compare coefficients from highest to lowest
                for i in (0..=self.deg).rev() {
                    match self.coef[i].cmp(&other.coef[i]) {
                        Ordering::Equal => continue,
                        other => return other,
                    }
                }
                Ordering::Equal
            }
            other => other,
        }
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.deg == 0 {
            return write!(f, "{}", self.coef[0]);
        }

        let mut first = true;
        for i in (0..=self.deg).rev() {
            if self.coef[i] == 0 {
                continue;
            }

            if !first {
                if self.coef[i] > 0 {
                    write!(f, " + ")?;
                } else {
                    write!(f, " - ")?;
                }
            } else if self.coef[i] < 0 {
                write!(f, "-")?;
            }
            first = false;

            let abs_coef = self.coef[i].abs();

            if i == 0 {
                write!(f, "{}", abs_coef)?;
            } else if abs_coef == 1 {
                if i == 1 {
                    write!(f, "x")?;
                } else {
                    write!(f, "x^{}", i)?;
                }
            } else if i == 1 {
                write!(f, "{}x", abs_coef)?;
            } else {
                write!(f, "{}x^{}", abs_coef, i)?;
            }
        }

        if first {
            write!(f, "0")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let p = Polynomial::new(5, 3);
        assert_eq!(p.degree(), 3);
        assert_eq!(p.evaluate(2), 40); // 5 * 8 = 40
    }

    #[test]
    fn test_zero() {
        let p = Polynomial::zero();
        assert_eq!(p.degree(), 0);
        assert_eq!(p.evaluate(100), 0);
    }

    #[test]
    fn test_addition() {
        let p1 = Polynomial::new(1, 2); // x²
        let p2 = Polynomial::new(2, 1); // 2x
        let p3 = Polynomial::new(3, 0); // 3
        let sum = p1.plus(&p2).plus(&p3); // x² + 2x + 3
        assert_eq!(sum.evaluate(2), 11); // 4 + 4 + 3 = 11
    }

    #[test]
    fn test_subtraction() {
        let p1 = Polynomial::new(5, 2); // 5x²
        let p2 = Polynomial::new(3, 2); // 3x²
        let diff = p1.minus(&p2); // 2x²
        assert_eq!(diff.evaluate(3), 18); // 2 * 9 = 18
    }

    #[test]
    fn test_multiplication() {
        let p1 = Polynomial::new(1, 1).plus(&Polynomial::new(1, 0)); // x + 1
        let p2 = Polynomial::new(1, 1).minus(&Polynomial::new(1, 0)); // x - 1
        let product = p1.times(&p2); // x² - 1
        assert_eq!(product.evaluate(3), 8); // 9 - 1 = 8
    }

    #[test]
    fn test_composition() {
        let p = Polynomial::new(1, 2); // x²
        let q = Polynomial::new(1, 1).plus(&Polynomial::new(1, 0)); // x + 1
        let pq = p.compose(&q); // (x + 1)²
        assert_eq!(pq.evaluate(2), 9); // (2 + 1)² = 9
    }

    #[test]
    fn test_differentiate() {
        let p = Polynomial::new(3, 2)
            .plus(&Polynomial::new(2, 1))
            .plus(&Polynomial::new(1, 0)); // 3x² + 2x + 1
        let dp = p.differentiate(); // 6x + 2
        assert_eq!(dp.evaluate(1), 8); // 6 + 2 = 8
        assert_eq!(dp.evaluate(2), 14); // 12 + 2 = 14
    }

    #[test]
    fn test_evaluate() {
        let p = Polynomial::new(1, 0)
            .plus(&Polynomial::new(2, 1))
            .plus(&Polynomial::new(3, 2)); // 3x² + 2x + 1
        assert_eq!(p.evaluate(0), 1);
        assert_eq!(p.evaluate(1), 6); // 3 + 2 + 1 = 6
        assert_eq!(p.evaluate(2), 17); // 12 + 4 + 1 = 17
    }

    #[test]
    fn test_display() {
        let p = Polynomial::new(4, 3)
            .plus(&Polynomial::new(3, 2))
            .plus(&Polynomial::new(-2, 1))
            .plus(&Polynomial::new(1, 0));
        assert_eq!(format!("{}", p), "4x^3 + 3x^2 - 2x + 1");
    }

    #[test]
    fn test_ordering() {
        let p1 = Polynomial::new(1, 2);
        let p2 = Polynomial::new(1, 3);
        assert!(p1 < p2);
    }
}
