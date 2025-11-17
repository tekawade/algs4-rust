//! Complex number implementation with comprehensive arithmetic operations.
//!
//! This module provides an immutable complex number type with support for:
//! - Basic arithmetic (addition, subtraction, multiplication, division)
//! - Transcendental functions (exp, sin, cos, tan)
//! - Complex operations (conjugate, reciprocal, phase, magnitude)
//!
//! # Examples
//!
//! ```
//! use algs4_advanced::Complex;
//!
//! let a = Complex::new(5.0, 6.0);
//! let b = Complex::new(-3.0, 4.0);
//!
//! let sum = a.plus(&b);      // (5-3) + i(6+4) = 2 + 10i
//! let product = a.times(&b); // (5-3i)(−3+4i) = -39 + 2i
//!
//! println!("a + b = {}", sum);
//! println!("a * b = {}", product);
//! println!("|a| = {}", a.abs());
//! println!("phase(a) = {}", a.phase());
//! ```

use std::fmt;

/// Represents an immutable complex number.
///
/// A complex number is of the form a + bi where a is the real part
/// and b is the imaginary part, and i is the imaginary unit (i² = -1).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    /// Real part
    re: f64,
    /// Imaginary part
    im: f64,
}

impl Complex {
    /// Creates a new complex number with the given real and imaginary parts.
    ///
    /// # Arguments
    ///
    /// * `re` - The real part
    /// * `im` - The imaginary part
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Complex;
    ///
    /// let z = Complex::new(3.0, 4.0); // 3 + 4i
    /// assert_eq!(z.re(), 3.0);
    /// assert_eq!(z.im(), 4.0);
    /// ```
    pub fn new(re: f64, im: f64) -> Self {
        Complex { re, im }
    }

    /// Returns the real part of this complex number.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Complex;
    ///
    /// let z = Complex::new(3.0, 4.0);
    /// assert_eq!(z.re(), 3.0);
    /// ```
    pub fn re(&self) -> f64 {
        self.re
    }

    /// Returns the imaginary part of this complex number.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Complex;
    ///
    /// let z = Complex::new(3.0, 4.0);
    /// assert_eq!(z.im(), 4.0);
    /// ```
    pub fn im(&self) -> f64 {
        self.im
    }

    /// Returns the sum of this complex number and the specified complex number.
    ///
    /// # Arguments
    ///
    /// * `that` - The complex number to add
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Complex;
    ///
    /// let a = Complex::new(5.0, 6.0);
    /// let b = Complex::new(-3.0, 4.0);
    /// let sum = a.plus(&b);
    /// assert_eq!(sum.re(), 2.0);
    /// assert_eq!(sum.im(), 10.0);
    /// ```
    pub fn plus(&self, that: &Complex) -> Complex {
        Complex::new(self.re + that.re, self.im + that.im)
    }

    /// Returns the result of subtracting the specified complex number from this complex number.
    ///
    /// # Arguments
    ///
    /// * `that` - The complex number to subtract
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Complex;
    ///
    /// let a = Complex::new(5.0, 6.0);
    /// let b = Complex::new(-3.0, 4.0);
    /// let diff = a.minus(&b);
    /// assert_eq!(diff.re(), 8.0);
    /// assert_eq!(diff.im(), 2.0);
    /// ```
    pub fn minus(&self, that: &Complex) -> Complex {
        Complex::new(self.re - that.re, self.im - that.im)
    }

    /// Returns the product of this complex number and the specified complex number.
    ///
    /// # Arguments
    ///
    /// * `that` - The complex number to multiply by
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Complex;
    ///
    /// let a = Complex::new(5.0, 6.0);
    /// let b = Complex::new(-3.0, 4.0);
    /// let product = a.times(&b);
    /// assert_eq!(product.re(), -39.0);
    /// assert_eq!(product.im(), 2.0);
    /// ```
    pub fn times(&self, that: &Complex) -> Complex {
        let real = self.re * that.re - self.im * that.im;
        let imag = self.re * that.im + self.im * that.re;
        Complex::new(real, imag)
    }

    /// Returns the product of this complex number and the specified scalar.
    ///
    /// # Arguments
    ///
    /// * `alpha` - The scalar to multiply by
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Complex;
    ///
    /// let z = Complex::new(5.0, 6.0);
    /// let scaled = z.scale(2.0);
    /// assert_eq!(scaled.re(), 10.0);
    /// assert_eq!(scaled.im(), 12.0);
    /// ```
    pub fn scale(&self, alpha: f64) -> Complex {
        Complex::new(alpha * self.re, alpha * self.im)
    }

    /// Returns the complex conjugate of this complex number.
    ///
    /// The conjugate of a + bi is a - bi.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Complex;
    ///
    /// let z = Complex::new(5.0, 6.0);
    /// let conj = z.conjugate();
    /// assert_eq!(conj.re(), 5.0);
    /// assert_eq!(conj.im(), -6.0);
    /// ```
    pub fn conjugate(&self) -> Complex {
        Complex::new(self.re, -self.im)
    }

    /// Returns the reciprocal of this complex number.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Complex;
    ///
    /// let z = Complex::new(5.0, 6.0);
    /// let recip = z.reciprocal();
    /// let product = z.times(&recip);
    /// assert!((product.re() - 1.0).abs() < 1e-10);
    /// assert!(product.im().abs() < 1e-10);
    /// ```
    pub fn reciprocal(&self) -> Complex {
        let scale = self.re * self.re + self.im * self.im;
        Complex::new(self.re / scale, -self.im / scale)
    }

    /// Returns the quotient of this complex number and the specified complex number.
    ///
    /// # Arguments
    ///
    /// * `that` - The divisor
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Complex;
    ///
    /// let a = Complex::new(5.0, 6.0);
    /// let b = Complex::new(-3.0, 4.0);
    /// let quotient = a.divides(&b);
    /// // Verify: quotient * b ≈ a
    /// let check = quotient.times(&b);
    /// assert!((check.re() - a.re()).abs() < 1e-10);
    /// assert!((check.im() - a.im()).abs() < 1e-10);
    /// ```
    pub fn divides(&self, that: &Complex) -> Complex {
        self.times(&that.reciprocal())
    }

    /// Returns the absolute value (modulus) of this complex number.
    ///
    /// The modulus of a + bi is √(a² + b²).
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Complex;
    ///
    /// let z = Complex::new(3.0, 4.0);
    /// assert_eq!(z.abs(), 5.0);
    /// ```
    pub fn abs(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    /// Returns the phase (argument) of this complex number.
    ///
    /// The phase is the angle θ in polar form r⋅e^(iθ).
    /// Returns a value in the range [-π, π].
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Complex;
    ///
    /// let z = Complex::new(1.0, 1.0);
    /// let phase = z.phase();
    /// assert!((phase - std::f64::consts::PI / 4.0).abs() < 1e-10);
    /// ```
    pub fn phase(&self) -> f64 {
        self.im.atan2(self.re)
    }

    /// Returns the exponential of this complex number.
    ///
    /// Uses Euler's formula: e^(a+bi) = e^a (cos(b) + i⋅sin(b))
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Complex;
    ///
    /// let z = Complex::new(0.0, std::f64::consts::PI);
    /// let result = z.exp();
    /// // e^(iπ) = -1
    /// assert!((result.re() + 1.0).abs() < 1e-10);
    /// assert!(result.im().abs() < 1e-10);
    /// ```
    pub fn exp(&self) -> Complex {
        Complex::new(self.re.exp() * self.im.cos(), self.re.exp() * self.im.sin())
    }

    /// Returns the sine of this complex number.
    ///
    /// sin(z) = (e^(iz) - e^(-iz)) / (2i)
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Complex;
    ///
    /// let z = Complex::new(0.0, 0.0);
    /// let result = z.sin();
    /// assert!(result.re().abs() < 1e-10);
    /// assert!(result.im().abs() < 1e-10);
    /// ```
    pub fn sin(&self) -> Complex {
        Complex::new(
            self.re.sin() * self.im.cosh(),
            self.re.cos() * self.im.sinh(),
        )
    }

    /// Returns the cosine of this complex number.
    ///
    /// cos(z) = (e^(iz) + e^(-iz)) / 2
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Complex;
    ///
    /// let z = Complex::new(0.0, 0.0);
    /// let result = z.cos();
    /// assert!((result.re() - 1.0).abs() < 1e-10);
    /// assert!(result.im().abs() < 1e-10);
    /// ```
    pub fn cos(&self) -> Complex {
        Complex::new(
            self.re.cos() * self.im.cosh(),
            -self.re.sin() * self.im.sinh(),
        )
    }

    /// Returns the tangent of this complex number.
    ///
    /// tan(z) = sin(z) / cos(z)
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::Complex;
    ///
    /// let z = Complex::new(0.0, 0.0);
    /// let result = z.tan();
    /// assert!(result.re().abs() < 1e-10);
    /// assert!(result.im().abs() < 1e-10);
    /// ```
    pub fn tan(&self) -> Complex {
        self.sin().divides(&self.cos())
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.im == 0.0 {
            write!(f, "{}", self.re)
        } else if self.re == 0.0 {
            write!(f, "{}i", self.im)
        } else if self.im < 0.0 {
            write!(f, "{} - {}i", self.re, -self.im)
        } else {
            write!(f, "{} + {}i", self.re, self.im)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_new_and_accessors() {
        let z = Complex::new(5.0, 6.0);
        assert_eq!(z.re(), 5.0);
        assert_eq!(z.im(), 6.0);
    }

    #[test]
    fn test_addition() {
        let a = Complex::new(5.0, 6.0);
        let b = Complex::new(-3.0, 4.0);
        let sum = a.plus(&b);
        assert_eq!(sum.re(), 2.0);
        assert_eq!(sum.im(), 10.0);
    }

    #[test]
    fn test_subtraction() {
        let a = Complex::new(5.0, 6.0);
        let b = Complex::new(-3.0, 4.0);
        let diff = a.minus(&b);
        assert_eq!(diff.re(), 8.0);
        assert_eq!(diff.im(), 2.0);
    }

    #[test]
    fn test_multiplication() {
        let a = Complex::new(5.0, 6.0);
        let b = Complex::new(-3.0, 4.0);
        let product = a.times(&b);
        // (5 + 6i) * (-3 + 4i) = -15 + 20i - 18i + 24i² = -15 + 2i - 24 = -39 + 2i
        assert_eq!(product.re(), -39.0);
        assert_eq!(product.im(), 2.0);
    }

    #[test]
    fn test_scale() {
        let z = Complex::new(5.0, 6.0);
        let scaled = z.scale(2.0);
        assert_eq!(scaled.re(), 10.0);
        assert_eq!(scaled.im(), 12.0);
    }

    #[test]
    fn test_conjugate() {
        let z = Complex::new(5.0, 6.0);
        let conj = z.conjugate();
        assert_eq!(conj.re(), 5.0);
        assert_eq!(conj.im(), -6.0);
    }

    #[test]
    fn test_abs() {
        let z = Complex::new(3.0, 4.0);
        assert_eq!(z.abs(), 5.0);
    }

    #[test]
    fn test_phase() {
        let z = Complex::new(1.0, 1.0);
        let phase = z.phase();
        assert!((phase - std::f64::consts::PI / 4.0).abs() < EPSILON);
    }

    #[test]
    fn test_reciprocal() {
        let z = Complex::new(5.0, 6.0);
        let recip = z.reciprocal();
        let product = z.times(&recip);
        assert!((product.re() - 1.0).abs() < EPSILON);
        assert!(product.im().abs() < EPSILON);
    }

    #[test]
    fn test_division() {
        let a = Complex::new(5.0, 6.0);
        let b = Complex::new(-3.0, 4.0);
        let quotient = a.divides(&b);
        // Verify: quotient * b ≈ a
        let check = quotient.times(&b);
        assert!((check.re() - a.re()).abs() < EPSILON);
        assert!((check.im() - a.im()).abs() < EPSILON);
    }

    #[test]
    fn test_exp() {
        // e^(iπ) = -1 (Euler's identity)
        let z = Complex::new(0.0, std::f64::consts::PI);
        let result = z.exp();
        assert!((result.re() + 1.0).abs() < EPSILON);
        assert!(result.im().abs() < EPSILON);
    }

    #[test]
    fn test_sin() {
        let z = Complex::new(0.0, 0.0);
        let result = z.sin();
        assert!(result.re().abs() < EPSILON);
        assert!(result.im().abs() < EPSILON);
    }

    #[test]
    fn test_cos() {
        let z = Complex::new(0.0, 0.0);
        let result = z.cos();
        assert!((result.re() - 1.0).abs() < EPSILON);
        assert!(result.im().abs() < EPSILON);
    }

    #[test]
    fn test_tan() {
        let z = Complex::new(0.0, 0.0);
        let result = z.tan();
        assert!(result.re().abs() < EPSILON);
        assert!(result.im().abs() < EPSILON);
    }

    #[test]
    fn test_display() {
        let z1 = Complex::new(5.0, 6.0);
        assert_eq!(format!("{}", z1), "5 + 6i");

        let z2 = Complex::new(5.0, -6.0);
        assert_eq!(format!("{}", z2), "5 - 6i");

        let z3 = Complex::new(0.0, 6.0);
        assert_eq!(format!("{}", z3), "6i");

        let z4 = Complex::new(5.0, 0.0);
        assert_eq!(format!("{}", z4), "5");
    }
}
