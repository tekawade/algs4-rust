//! Fast Fourier Transform using radix-2 Cooley-Tukey algorithm.
//!
//! Provides O(n log n) computation of the discrete Fourier transform
//! and its inverse. Also supports circular and linear convolution.
//!
//! # Examples
//!
//! ```
//! use algs4_advanced::{Complex, fft};
//!
//! let mut x = vec![
//!     Complex::new(1.0, 0.0),
//!     Complex::new(2.0, 0.0),
//!     Complex::new(3.0, 0.0),
//!     Complex::new(4.0, 0.0),
//! ];
//!
//! let y = fft::fft(&x);
//! let z = fft::ifft(&y);
//!
//! // z should be approximately equal to x
//! for i in 0..x.len() {
//!     assert!((z[i].re() - x[i].re()).abs() < 1e-10);
//!     assert!((z[i].im() - x[i].im()).abs() < 1e-10);
//! }
//! ```

use crate::Complex;
use std::f64::consts::PI;

/// Computes the FFT of the specified complex array.
///
/// # Arguments
///
/// * `x` - The complex array (length must be a power of 2)
///
/// # Panics
///
/// Panics if the length is not a power of 2.
///
/// # Examples
///
/// ```
/// use algs4_advanced::{Complex, fft};
///
/// let x = vec![
///     Complex::new(1.0, 0.0),
///     Complex::new(0.0, 0.0),
///     Complex::new(0.0, 0.0),
///     Complex::new(0.0, 0.0),
/// ];
/// let y = fft::fft(&x);
/// // FFT of [1, 0, 0, 0] = [1, 1, 1, 1]
/// ```
pub fn fft(x: &[Complex]) -> Vec<Complex> {
    let n = x.len();

    // Base case
    if n == 1 {
        return vec![x[0]];
    }

    // Check that length is a power of 2
    assert!(
        n.is_power_of_two(),
        "Length must be a power of 2, got {}",
        n
    );

    // FFT of even terms
    let mut even = Vec::with_capacity(n / 2);
    for i in (0..n).step_by(2) {
        even.push(x[i]);
    }
    let q = fft(&even);

    // FFT of odd terms
    let mut odd = Vec::with_capacity(n / 2);
    for i in (1..n).step_by(2) {
        odd.push(x[i]);
    }
    let r = fft(&odd);

    // Combine
    let mut y = vec![Complex::new(0.0, 0.0); n];
    for k in 0..n / 2 {
        let kth = -2.0 * k as f64 * PI / n as f64;
        let wk = Complex::new(kth.cos(), kth.sin());
        let term = wk.times(&r[k]);
        y[k] = q[k].plus(&term);
        y[k + n / 2] = q[k].minus(&term);
    }

    y
}

/// Computes the inverse FFT of the specified complex array.
///
/// # Arguments
///
/// * `x` - The complex array (length must be a power of 2)
///
/// # Panics
///
/// Panics if the length is not a power of 2.
///
/// # Examples
///
/// ```
/// use algs4_advanced::{Complex, fft};
///
/// let x = vec![
///     Complex::new(1.0, 0.0),
///     Complex::new(2.0, 0.0),
///     Complex::new(3.0, 0.0),
///     Complex::new(4.0, 0.0),
/// ];
/// let y = fft::fft(&x);
/// let z = fft::ifft(&y);
/// // z should equal x
/// for i in 0..x.len() {
///     assert!((z[i].re() - x[i].re()).abs() < 1e-10);
/// }
/// ```
pub fn ifft(x: &[Complex]) -> Vec<Complex> {
    let n = x.len();

    // Take conjugate
    let mut y: Vec<Complex> = x.iter().map(|c| c.conjugate()).collect();

    // Compute forward FFT
    y = fft(&y);

    // Take conjugate again and scale
    y.iter()
        .map(|c| c.conjugate().scale(1.0 / n as f64))
        .collect()
}

/// Computes the circular convolution of x and y.
///
/// # Arguments
///
/// * `x` - The first complex array
/// * `y` - The second complex array (must have same length as x)
///
/// # Panics
///
/// Panics if the arrays have different lengths or length is not a power of 2.
///
/// # Examples
///
/// ```
/// use algs4_advanced::{Complex, fft};
///
/// let x = vec![
///     Complex::new(1.0, 0.0),
///     Complex::new(0.0, 0.0),
///     Complex::new(0.0, 0.0),
///     Complex::new(0.0, 0.0),
/// ];
/// let y = vec![
///     Complex::new(1.0, 0.0),
///     Complex::new(1.0, 0.0),
///     Complex::new(0.0, 0.0),
///     Complex::new(0.0, 0.0),
/// ];
/// let c = fft::cconvolve(&x, &y);
/// ```
pub fn cconvolve(x: &[Complex], y: &[Complex]) -> Vec<Complex> {
    assert_eq!(
        x.len(),
        y.len(),
        "Arrays must have the same length for convolution"
    );

    let n = x.len();

    // Compute FFT of each sequence
    let a = fft(x);
    let b = fft(y);

    // Point-wise multiply
    let mut c = Vec::with_capacity(n);
    for i in 0..n {
        c.push(a[i].times(&b[i]));
    }

    // Compute inverse FFT
    ifft(&c)
}

/// Computes the linear convolution of x and y.
///
/// # Arguments
///
/// * `x` - The first complex array
/// * `y` - The second complex array
///
/// # Panics
///
/// Panics if the combined length (rounded to power of 2) exceeds reasonable limits.
///
/// # Examples
///
/// ```
/// use algs4_advanced::{Complex, fft};
///
/// let x = vec![
///     Complex::new(1.0, 0.0),
///     Complex::new(2.0, 0.0),
/// ];
/// let y = vec![
///     Complex::new(3.0, 0.0),
///     Complex::new(4.0, 0.0),
/// ];
/// let c = fft::convolve(&x, &y);
/// ```
pub fn convolve(x: &[Complex], y: &[Complex]) -> Vec<Complex> {
    // Pad to power of 2
    let n = (x.len() + y.len() - 1).next_power_of_two();

    let mut a = x.to_vec();
    a.resize(n, Complex::new(0.0, 0.0));

    let mut b = y.to_vec();
    b.resize(n, Complex::new(0.0, 0.0));

    cconvolve(&a, &b)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_fft_ifft_roundtrip() {
        let x = vec![
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
            Complex::new(4.0, 0.0),
        ];

        let y = fft(&x);
        let z = ifft(&y);

        for i in 0..x.len() {
            assert!((z[i].re() - x[i].re()).abs() < EPSILON);
            assert!((z[i].im() - x[i].im()).abs() < EPSILON);
        }
    }

    #[test]
    fn test_fft_single_element() {
        let x = vec![Complex::new(5.0, 3.0)];
        let y = fft(&x);
        assert_eq!(y.len(), 1);
        assert!((y[0].re() - 5.0).abs() < EPSILON);
        assert!((y[0].im() - 3.0).abs() < EPSILON);
    }

    #[test]
    fn test_circular_convolution() {
        let x = vec![
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
        ];
        let y = vec![
            Complex::new(1.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
        ];
        let c = cconvolve(&x, &y);
        assert_eq!(c.len(), 4);
    }

    #[test]
    fn test_linear_convolution() {
        let x = vec![Complex::new(1.0, 0.0), Complex::new(2.0, 0.0)];
        let y = vec![Complex::new(3.0, 0.0), Complex::new(4.0, 0.0)];
        let c = convolve(&x, &y);
        // (1 + 2x) * (3 + 4x) = 3 + 4x + 6x + 8x² = 3 + 10x + 8x²
        assert!((c[0].re() - 3.0).abs() < EPSILON); // constant term
        assert!((c[1].re() - 10.0).abs() < EPSILON); // x coefficient
        assert!((c[2].re() - 8.0).abs() < EPSILON); // x² coefficient
    }

    #[test]
    #[should_panic(expected = "Length must be a power of 2")]
    fn test_fft_non_power_of_two() {
        let x = vec![
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
        ];
        fft(&x);
    }
}
