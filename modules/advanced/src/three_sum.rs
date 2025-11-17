//! Brute force algorithm for finding triples that sum to zero.
//!
//! This module provides a brute force O(n³) algorithm for counting and finding
//! triples in an array that sum to zero. For a faster O(n² log n) algorithm,
//! see the `three_sum_fast` module.
//!
//! # Examples
//!
//! ```
//! use algs4_advanced::three_sum;
//!
//! let array = vec![-5, -2, 0, 2, 3, 5, 8];
//! let count = three_sum::count(&array);
//! println!("Number of triples that sum to zero: {}", count);
//!
//! // Print all triples
//! three_sum::print_all(&array);
//! ```

/// Counts the number of triples that sum to zero.
///
/// Uses a brute force algorithm with O(n³) time complexity.
///
/// # Arguments
///
/// * `a` - The input array
///
/// # Returns
///
/// The number of triples (i, j, k) where i < j < k and a[i] + a[j] + a[k] = 0.
///
/// # Examples
///
/// ```
/// use algs4_advanced::three_sum;
///
/// let array = vec![-5, -2, 0, 2, 3, 5, 8];
/// let count = three_sum::count(&array);
/// assert!(count >= 0);
/// ```
pub fn count(a: &[i32]) -> usize {
    let n = a.len();
    let mut count = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                if a[i] + a[j] + a[k] == 0 {
                    count += 1;
                }
            }
        }
    }

    count
}

/// Prints all triples that sum to zero.
///
/// Uses a brute force algorithm with O(n³) time complexity.
///
/// # Arguments
///
/// * `a` - The input array
///
/// # Examples
///
/// ```
/// use algs4_advanced::three_sum;
///
/// let array = vec![-5, -2, 0, 2, 3, 5, 8];
/// three_sum::print_all(&array);
/// ```
pub fn print_all(a: &[i32]) {
    let n = a.len();

    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                if a[i] + a[j] + a[k] == 0 {
                    println!("{} {} {}", a[i], a[j], a[k]);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        // -3 + -2 + 5 = 0
        // -3 + 1 + 2 = 0
        // -2 + 0 + 2 = 0
        let array = vec![-3, -2, 0, 1, 2, 5];
        let result = count(&array);
        assert!(result >= 3);
    }

    #[test]
    fn test_count_duplicates() {
        let array = vec![0, 0, 0];
        assert_eq!(count(&array), 1); // (0, 0, 0)
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
        let array = vec![-5, -3, -2, 0, 1, 2, 3, 5, 8];
        let result = count(&array);
        // Should find several triples like:
        // -5 + 2 + 3 = 0
        // -3 + 1 + 2 = 0
        // -2 + 0 + 2 = 0
        // etc.
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
    fn test_count_with_zeros() {
        let array = vec![0, 0, 0, 0];
        // Can choose any 3 zeros: C(4,3) = 4
        assert_eq!(count(&array), 4);
    }

    #[test]
    fn test_print_all_simple() {
        let array = vec![-1, 0, 1];
        // This should print: -1 0 1
        print_all(&array);
    }

    #[test]
    fn test_print_all_multiple() {
        let array = vec![-3, -2, 0, 1, 2, 5];
        // Should print multiple triples
        print_all(&array);
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
}
