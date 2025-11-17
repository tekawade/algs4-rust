// Copyright (C) 2025 algs4-rust contributors
// SPDX-License-Identifier: GPL-3.0-or-later
//
// This file is part of algs4-rust (Rust implementation of Algorithms, 4th Edition).
// Adapted from the original Java implementation by Robert Sedgewick and Kevin Wayne.

//! Standard random number generation.
//!
//! This module provides static methods for generating pseudo-random numbers
//! from various distributions (uniform, Gaussian, discrete, etc.).

use rand::distributions::{Distribution, Uniform};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rand_distr::{Exp, Normal, Pareto, Poisson};
use std::cell::RefCell;

thread_local! {
    static RNG: RefCell<StdRng> = RefCell::new(StdRng::from_entropy());
}

/// Sets the seed of the pseudo-random number generator.
///
/// # Examples
///
/// ```
/// use algs4_fundamentals::io::stdrandom;
///
/// stdrandom::set_seed(12345);
/// let x = stdrandom::uniform_f64();
/// stdrandom::set_seed(12345);
/// let y = stdrandom::uniform_f64();
/// assert_eq!(x, y); // Same seed produces same sequence
/// ```
pub fn set_seed(seed: u64) {
    RNG.with(|rng| {
        *rng.borrow_mut() = StdRng::seed_from_u64(seed);
    });
}

/// Returns a random real number uniformly in [0, 1).
///
/// # Examples
///
/// ```
/// use algs4_fundamentals::io::stdrandom;
///
/// let x = stdrandom::uniform_f64();
/// assert!(x >= 0.0 && x < 1.0);
/// ```
#[inline]
pub fn uniform_f64() -> f64 {
    RNG.with(|rng| rng.borrow_mut().gen())
}

/// Returns a random real number uniformly in [a, b).
///
/// # Arguments
///
/// * `a` - the left endpoint
/// * `b` - the right endpoint
///
/// # Panics
///
/// Panics if `b <= a` or if `b - a >= f64::MAX`.
///
/// # Examples
///
/// ```
/// use algs4_fundamentals::io::stdrandom;
///
/// let x = stdrandom::uniform_range_f64(10.0, 20.0);
/// assert!(x >= 10.0 && x < 20.0);
/// ```
pub fn uniform_range_f64(a: f64, b: f64) -> f64 {
    assert!(b > a, "invalid range: [{}, {})", a, b);
    assert!((b - a).is_finite(), "range too large");
    RNG.with(|rng| {
        let dist = Uniform::new(a, b);
        dist.sample(&mut *rng.borrow_mut())
    })
}

/// Returns a random integer uniformly in [0, n).
///
/// # Arguments
///
/// * `n` - the upper bound (exclusive)
///
/// # Panics
///
/// Panics if `n <= 0`.
///
/// # Examples
///
/// ```
/// use algs4_fundamentals::io::stdrandom;
///
/// let x = stdrandom::uniform_usize(100);
/// assert!(x < 100);
/// ```
#[inline]
pub fn uniform_usize(n: usize) -> usize {
    assert!(n > 0, "n must be positive");
    RNG.with(|rng| rng.borrow_mut().gen_range(0..n))
}

/// Returns a random integer uniformly in [0, n).
///
/// # Arguments
///
/// * `n` - the upper bound (exclusive)
///
/// # Panics
///
/// Panics if `n <= 0`.
///
/// # Examples
///
/// ```
/// use algs4_fundamentals::io::stdrandom;
///
/// let x = stdrandom::uniform_i32(100);
/// assert!(x >= 0 && x < 100);
/// ```
pub fn uniform_i32(n: i32) -> i32 {
    assert!(n > 0, "n must be positive");
    RNG.with(|rng| rng.borrow_mut().gen_range(0..n))
}

/// Returns a random integer uniformly in [a, b).
///
/// # Arguments
///
/// * `a` - the left endpoint
/// * `b` - the right endpoint
///
/// # Panics
///
/// Panics if `b <= a`.
///
/// # Examples
///
/// ```
/// use algs4_fundamentals::io::stdrandom;
///
/// let x = stdrandom::uniform_range_i32(10, 20);
/// assert!(x >= 10 && x < 20);
/// ```
pub fn uniform_range_i32(a: i32, b: i32) -> i32 {
    assert!(b > a, "invalid range: [{}, {})", a, b);
    RNG.with(|rng| rng.borrow_mut().gen_range(a..b))
}

/// Returns a random boolean from a Bernoulli distribution with success
/// probability p.
///
/// # Arguments
///
/// * `p` - the probability of returning `true`
///
/// # Panics
///
/// Panics if `p` is not between 0.0 and 1.0.
///
/// # Examples
///
/// ```
/// use algs4_fundamentals::io::stdrandom;
///
/// let coin_flip = stdrandom::bernoulli(0.5);
/// // Returns true or false with equal probability
/// ```
pub fn bernoulli(p: f64) -> bool {
    assert!(
        (0.0..=1.0).contains(&p),
        "probability must be between 0.0 and 1.0"
    );
    uniform_f64() < p
}

/// Returns a random boolean with probability 0.5.
///
/// # Examples
///
/// ```
/// use algs4_fundamentals::io::stdrandom;
///
/// let coin_flip = stdrandom::bernoulli_default();
/// // Returns true or false with equal probability
/// ```
#[inline]
pub fn bernoulli_default() -> bool {
    bernoulli(0.5)
}

/// Returns a random real number from a standard Gaussian distribution.
///
/// Uses the Box-Muller transform.
///
/// # Examples
///
/// ```
/// use algs4_fundamentals::io::stdrandom;
///
/// let x = stdrandom::gaussian();
/// // Returns a value from N(0, 1)
/// ```
pub fn gaussian() -> f64 {
    RNG.with(|rng| {
        let normal = Normal::new(0.0, 1.0).unwrap();
        normal.sample(&mut *rng.borrow_mut())
    })
}

/// Returns a random real number from a Gaussian distribution with mean μ
/// and standard deviation σ.
///
/// # Arguments
///
/// * `mu` - the mean
/// * `sigma` - the standard deviation
///
/// # Panics
///
/// Panics if `sigma < 0.0`.
///
/// # Examples
///
/// ```
/// use algs4_fundamentals::io::stdrandom;
///
/// let x = stdrandom::gaussian_with_params(10.0, 2.0);
/// // Returns a value from N(10, 2²)
/// ```
pub fn gaussian_with_params(mu: f64, sigma: f64) -> f64 {
    assert!(sigma >= 0.0, "standard deviation must be non-negative");
    mu + sigma * gaussian()
}

/// Returns a random integer from a geometric distribution with success
/// probability p.
///
/// # Arguments
///
/// * `p` - the probability of success
///
/// # Panics
///
/// Panics if `p` is not between 0.0 and 1.0.
///
/// # Examples
///
/// ```
/// use algs4_fundamentals::io::stdrandom;
///
/// let trials = stdrandom::geometric(0.5);
/// // Number of trials until first success
/// ```
pub fn geometric(p: f64) -> usize {
    assert!(
        p > 0.0 && p <= 1.0,
        "probability must be between 0.0 and 1.0"
    );
    ((uniform_f64().ln() / (1.0 - p).ln()).ceil() as usize).saturating_sub(1)
}

/// Returns a random integer from a Poisson distribution with mean λ.
///
/// # Arguments
///
/// * `lambda` - the mean
///
/// # Panics
///
/// Panics if `lambda <= 0.0` or `lambda` is infinite.
///
/// # Examples
///
/// ```
/// use algs4_fundamentals::io::stdrandom;
///
/// let x = stdrandom::poisson(5.0);
/// // Returns a value from Poisson(5.0)
/// ```
pub fn poisson(lambda: f64) -> usize {
    assert!(
        lambda > 0.0 && lambda.is_finite(),
        "lambda must be positive and finite"
    );
    RNG.with(|rng| {
        let poisson = Poisson::new(lambda).unwrap();
        poisson.sample(&mut *rng.borrow_mut()) as usize
    })
}

/// Returns a random real number from a Pareto distribution with shape parameter α.
///
/// # Arguments
///
/// * `alpha` - the shape parameter
///
/// # Panics
///
/// Panics if `alpha <= 0.0`.
///
/// # Examples
///
/// ```
/// use algs4_fundamentals::io::stdrandom;
///
/// let x = stdrandom::pareto(1.0);
/// ```
pub fn pareto(alpha: f64) -> f64 {
    assert!(alpha > 0.0, "alpha must be positive");
    RNG.with(|rng| {
        let pareto = Pareto::new(1.0, alpha).unwrap();
        pareto.sample(&mut *rng.borrow_mut())
    })
}

/// Returns a random real number from the Cauchy distribution.
///
/// # Examples
///
/// ```
/// use algs4_fundamentals::io::stdrandom;
///
/// let x = stdrandom::cauchy();
/// ```
pub fn cauchy() -> f64 {
    gaussian() / gaussian()
}

/// Returns a random integer from the specified discrete distribution.
///
/// # Arguments
///
/// * `probabilities` - the probability of each integer (must sum to 1.0)
///
/// # Panics
///
/// Panics if probabilities don't sum to approximately 1.0 or contain negative values.
///
/// # Examples
///
/// ```
/// use algs4_fundamentals::io::stdrandom;
///
/// let probs = vec![0.1, 0.3, 0.6];
/// let i = stdrandom::discrete(&probs);
/// assert!(i < 3);
/// ```
pub fn discrete(probabilities: &[f64]) -> usize {
    assert!(!probabilities.is_empty(), "probabilities array is empty");

    // Check for negative probabilities
    for &p in probabilities {
        assert!(p >= 0.0, "probability cannot be negative");
    }

    let sum: f64 = probabilities.iter().sum();
    assert!(
        (sum - 1.0).abs() < 1e-6,
        "probabilities must sum to 1.0, got {}",
        sum
    );

    let r = uniform_f64();
    let mut cumulative = 0.0;
    for (i, &p) in probabilities.iter().enumerate() {
        cumulative += p;
        if r < cumulative {
            return i;
        }
    }
    probabilities.len() - 1
}

/// Returns a random real number from an exponential distribution with rate λ.
///
/// # Arguments
///
/// * `lambda` - the rate parameter
///
/// # Panics
///
/// Panics if `lambda <= 0.0`.
///
/// # Examples
///
/// ```
/// use algs4_fundamentals::io::stdrandom;
///
/// let x = stdrandom::exponential(1.0);
/// ```
pub fn exponential(lambda: f64) -> f64 {
    assert!(lambda > 0.0, "lambda must be positive");
    RNG.with(|rng| {
        let exp = Exp::new(lambda).unwrap();
        exp.sample(&mut *rng.borrow_mut())
    })
}

/// Rearranges the elements of the specified array in uniformly random order.
///
/// # Examples
///
/// ```
/// use algs4_fundamentals::io::stdrandom;
///
/// let mut arr = vec![1, 2, 3, 4, 5];
/// stdrandom::shuffle(&mut arr);
/// // arr is now in random order
/// ```
pub fn shuffle<T>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        let r = uniform_range_i32(i as i32, n as i32) as usize;
        arr.swap(i, r);
    }
}

/// Rearranges the elements in the subarray `arr[lo..hi]` in uniformly random order.
///
/// # Arguments
///
/// * `arr` - the array
/// * `lo` - the left endpoint (inclusive)
/// * `hi` - the right endpoint (exclusive)
///
/// # Panics
///
/// Panics if indices are out of bounds or if `lo > hi`.
///
/// # Examples
///
/// ```
/// use algs4_fundamentals::io::stdrandom;
///
/// let mut arr = vec![1, 2, 3, 4, 5];
/// stdrandom::shuffle_range(&mut arr, 1, 4);
/// // Elements 1, 2, 3 are shuffled, 0 and 4 are unchanged
/// ```
pub fn shuffle_range<T>(arr: &mut [T], lo: usize, hi: usize) {
    assert!(lo <= hi && hi <= arr.len(), "invalid subarray range");
    for i in lo..hi {
        let r = uniform_range_i32(i as i32, hi as i32) as usize;
        arr.swap(i, r);
    }
}

/// Returns a uniformly random permutation of n elements.
///
/// # Examples
///
/// ```
/// use algs4_fundamentals::io::stdrandom;
///
/// let perm = stdrandom::permutation(5);
/// assert_eq!(perm.len(), 5);
/// // perm contains 0, 1, 2, 3, 4 in random order
/// ```
pub fn permutation(n: usize) -> Vec<usize> {
    let mut perm: Vec<usize> = (0..n).collect();
    shuffle(&mut perm);
    perm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_seed_reproducible() {
        set_seed(12345);
        let x = uniform_f64();
        set_seed(12345);
        let y = uniform_f64();
        assert_eq!(x, y);
    }

    #[test]
    fn test_uniform_f64() {
        for _ in 0..100 {
            let x = uniform_f64();
            assert!((0.0..1.0).contains(&x));
        }
    }

    #[test]
    fn test_uniform_range_f64() {
        for _ in 0..100 {
            let x = uniform_range_f64(10.0, 20.0);
            assert!((10.0..20.0).contains(&x));
        }
    }

    #[test]
    fn test_uniform_usize() {
        for _ in 0..100 {
            let x = uniform_usize(10);
            assert!(x < 10);
        }
    }

    #[test]
    fn test_uniform_i32() {
        for _ in 0..100 {
            let x = uniform_i32(10);
            assert!((0..10).contains(&x));
        }
    }

    #[test]
    fn test_uniform_range_i32() {
        for _ in 0..100 {
            let x = uniform_range_i32(10, 20);
            assert!((10..20).contains(&x));
        }
    }

    #[test]
    fn test_bernoulli() {
        // Test that it returns boolean
        let _ = bernoulli(0.5);
        let _ = bernoulli_default();
    }

    #[test]
    fn test_gaussian() {
        // Just test that it doesn't panic
        for _ in 0..100 {
            let _ = gaussian();
            let _ = gaussian_with_params(10.0, 2.0);
        }
    }

    #[test]
    fn test_geometric() {
        for _ in 0..10 {
            let x = geometric(0.5);
            assert!(x < 1000); // Should be small with p=0.5
        }
    }

    #[test]
    fn test_poisson() {
        for _ in 0..10 {
            let x = poisson(5.0);
            assert!(x < 50); // Unlikely to be very large
        }
    }

    #[test]
    fn test_discrete() {
        let probs = vec![0.1, 0.3, 0.6];
        for _ in 0..10 {
            let i = discrete(&probs);
            assert!(i < 3);
        }
    }

    #[test]
    fn test_shuffle() {
        let mut arr = vec![1, 2, 3, 4, 5];
        shuffle(&mut arr);
        assert_eq!(arr.len(), 5);
        // Check all elements still present
        let mut sorted = arr.clone();
        sorted.sort();
        assert_eq!(sorted, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_shuffle_range() {
        let mut arr = vec![1, 2, 3, 4, 5];
        let first = arr[0];
        let last = arr[4];
        shuffle_range(&mut arr, 1, 4);
        assert_eq!(arr[0], first); // First unchanged
        assert_eq!(arr[4], last); // Last unchanged
    }

    #[test]
    fn test_permutation() {
        let perm = permutation(5);
        assert_eq!(perm.len(), 5);
        let mut sorted = perm.clone();
        sorted.sort();
        assert_eq!(sorted, vec![0, 1, 2, 3, 4]);
    }
}
