//! Fast algorithm for finding triples that sum to zero.
//!
//! This module provides an optimized O(n² log n) algorithm for counting
//! triples in an array that sum to zero. It uses sorting and binary search
//! to achieve better performance than the brute force O(n³) approach.
//!
//! Note: This implementation assumes no duplicate values in the array.
//!
//! # Examples
//!
//! ```
//! use algs4_advanced::three_sum_fast;
//!
//! let array = vec![-5, -2, 0, 2, 3, 5, 8];
//! let count = three_sum_fast::count(&array);
//! println!("Number of triples that sum to zero: {}", count);
//! ```

/// Performs binary search to find the index of the target value.
///
/// # Arguments
///
/// * `a` - The sorted array to search
/// * `key` - The value to search for
///
/// # Returns
///
/// The index of the key if found, otherwise -1.
fn binary_search(a: &[i32], key: i32) -> isize {
    let mut lo = 0isize;
    let mut hi = a.len() as isize - 1;

    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        let mid_val = a[mid as usize];

        if mid_val < key {
            lo = mid + 1;
        } else if mid_val > key {
            hi = mid - 1;
        } else {
            return mid;
        }
    }

    -1
}

/// Counts the number of triples that sum to zero.
///
/// Uses sorting and binary search for O(n² log n) time complexity.
/// Assumes no duplicate values in the array.
///
/// # Arguments
///
/// * `a` - The input array (must contain no duplicate values)
///
/// # Returns
///
/// The number of triples (i, j, k) where i < j < k and a[i] + a[j] + a[k] = 0.
///
/// # Examples
///
/// ```
/// use algs4_advanced::three_sum_fast;
///
/// let array = vec![-5, -2, 0, 2, 3, 5, 8];
/// let count = three_sum_fast::count(&array);
/// assert!(count >= 0);
/// ```
pub fn count(a: &[i32]) -> usize {
    let n = a.len();
    if n < 3 {
        return 0;
    }

    // Sort the array
    let mut sorted = a.to_vec();
    sorted.sort_unstable();

    let mut count = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            // Look for k such that a[i] + a[j] + a[k] = 0
            // So we need a[k] = -(a[i] + a[j])
            let target = -(sorted[i] + sorted[j]);
            let k = binary_search(&sorted, target);

            if k > j as isize {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let array = vec![-5, -2, 0, 2, 3, 5, 8];
        assert_eq!(binary_search(&array, 0), 2);
        assert_eq!(binary_search(&array, -5), 0);
        assert_eq!(binary_search(&array, 8), 6);
        assert_eq!(binary_search(&array, 4), -1);
    }

    #[test]
    fn test_count_empty() {
        let array: Vec<i32> = vec![];
        assert_eq!(count(&array), 0);
    }

    #[test]
    fn test_count_small() {
        let array = vec![1, 2, 3];
        assert_eq!(count(&array), 0);
    }

    #[test]
    fn test_count_simple() {
        let array = vec![-1, 0, 1];
        assert_eq!(count(&array), 1); // (-1, 0, 1)
    }

    #[test]
    fn test_count_multiple() {
        // -5 + 2 + 3 = 0
        // -3 + 1 + 2 = 0
        // -2 + 0 + 2 = 0
        let array = vec![-5, -3, -2, 0, 1, 2, 3, 5];
        let result = count(&array);
        assert!(result >= 3);
    }

    #[test]
    fn test_count_all_positive() {
        let array = vec![1, 2, 3, 4, 5];
        assert_eq!(count(&array), 0);
    }

    #[test]
    fn test_count_all_negative() {
        let array = vec![-5, -4, -3, -2, -1];
        assert_eq!(count(&array), 0);
    }

    #[test]
    fn test_count_larger_array() {
        let array = vec![-10, -5, -3, -2, 0, 1, 2, 3, 5, 8, 10];
        let result = count(&array);
        // Should find several triples
        assert!(result > 0);
    }

    #[test]
    fn test_count_no_zeros() {
        let array = vec![-4, -2, 1, 3];
        let result = count(&array);
        // -4 + 1 + 3 = 0
        assert_eq!(result, 1);
    }

    #[test]
    fn test_count_with_single_zero() {
        let array = vec![-5, -2, 0, 2, 5];
        let result = count(&array);
        // -5 + 0 + 5 = 0
        // -2 + 0 + 2 = 0
        assert_eq!(result, 2);
    }

    #[test]
    fn test_edge_case_two_elements() {
        let array = vec![1, -1];
        assert_eq!(count(&array), 0); // Need at least 3 elements
    }

    #[test]
    fn test_edge_case_exact_three_elements_match() {
        let array = vec![-5, 2, 3];
        assert_eq!(count(&array), 1); // -5 + 2 + 3 = 0
    }

    #[test]
    fn test_edge_case_exact_three_elements_no_match() {
        let array = vec![1, 2, 3];
        assert_eq!(count(&array), 0);
    }

    #[test]
    fn test_unsorted_input() {
        let array = vec![3, -2, 5, -5, 0, 2, -3];
        let result = count(&array);
        // Should handle unsorted input correctly
        assert!(result > 0);
    }

    #[test]
    fn test_larger_range() {
        let array = vec![-100, -50, -10, 0, 10, 50, 100];
        let result = count(&array);
        // -100 + 0 + 100 = 0
        // -50 + 0 + 50 = 0
        // -10 + 0 + 10 = 0
        assert_eq!(result, 3);
    }

    #[test]
    fn test_many_combinations() {
        let array = vec![-6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6];
        let result = count(&array);
        // Many possible combinations
        assert!(result > 10);
    }
}
