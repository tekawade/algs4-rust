//! Optimized insertion sort with sentinel.
//!
//! This is an optimized version of insertion sort that uses a sentinel value to
//! eliminate the inner loop boundary check. By first moving the smallest element
//! to position 0, we can guarantee that the inner loop will always terminate when
//! it encounters this sentinel, eliminating the need to check `j > 0` in each iteration.
//!
//! The sentinel optimization reduces the number of comparisons and can provide a
//! measurable performance improvement over standard insertion sort, especially for
//! random data or data with many inversions.
//!
//! # Examples
//!
//! ```
//! use algs4_sorting::insertion_x;
//!
//! let mut data = vec![64, 25, 12, 22, 11];
//! insertion_x::sort(&mut data);
//! assert_eq!(data, vec![11, 12, 22, 25, 64]);
//! ```
//!
//! # Performance
//!
//! * Time Complexity:
//!   - Best case: O(n) - when the array is already sorted
//!   - Average case: O(n²)
//!   - Worst case: O(n²) - when the array is reverse sorted
//! * Space Complexity: O(1) - in-place sorting
//! * Stable: equal elements retain their relative order
//! * Adaptive: performs well on partially sorted arrays
//!
//! # Optimization Details
//!
//! The sentinel optimization works as follows:
//! 1. First pass: find the minimum element and move it to position 0
//! 2. Subsequent passes: use standard insertion sort but without the `j > 0` check
//!
//! This eliminates one comparison per inner loop iteration, reducing the constant
//! factor in the O(n²) time complexity. The improvement is most noticeable for
//! arrays with random or reverse-sorted data.
//!
//! **Reference:** <https://algs4.cs.princeton.edu/21elementary>

/// Sorts a slice in ascending order using optimized insertion sort with sentinel.
///
/// This implementation uses a sentinel optimization to eliminate the boundary check
/// in the inner loop. The algorithm first finds the minimum element and places it
/// at position 0, which serves as a sentinel. This guarantees that the inner loop
/// will always terminate without needing to check if `j > 0`.
///
/// # Algorithm
///
/// 1. Find the minimum element in the array
/// 2. Move the minimum element to position 0 (via a series of swaps)
/// 3. For each remaining position i from 2 to n-1:
///    - Insert arr[i] into the sorted portion without boundary checks
///    - The sentinel at position 0 ensures the loop terminates
///
/// # Type Parameters
///
/// * `T` - The type of elements in the slice. Must implement `Ord` for comparison.
///
/// # Arguments
///
/// * `arr` - A mutable slice to be sorted
///
/// # Examples
///
/// ```
/// use algs4_sorting::insertion_x;
///
/// let mut numbers = vec![5, 2, 8, 1, 9];
/// insertion_x::sort(&mut numbers);
/// assert_eq!(numbers, vec![1, 2, 5, 8, 9]);
///
/// let mut chars = vec!['d', 'a', 'c', 'b'];
/// insertion_x::sort(&mut chars);
/// assert_eq!(chars, vec!['a', 'b', 'c', 'd']);
/// ```
///
/// # Performance
///
/// - Time: O(n²) worst case, O(n) best case (already sorted)
/// - Space: O(1) - sorts in place
/// - Fewer comparisons than standard insertion sort due to sentinel optimization
/// - Same number of exchanges as standard insertion sort
///
/// # Notes
///
/// The sentinel optimization provides a constant factor improvement over standard
/// insertion sort by eliminating one comparison per inner loop iteration. This can
/// result in 10-20% performance improvement in practice, though the asymptotic
/// complexity remains O(n²).
pub fn sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();

    // Handle empty or single-element arrays
    if n <= 1 {
        return;
    }

    // First, move the minimum element to position 0 to serve as a sentinel
    // This ensures the inner loop will always terminate
    let mut min_idx = 0;
    for i in 1..n {
        if arr[i] < arr[min_idx] {
            min_idx = i;
        }
    }

    // Move minimum to position 0 via swaps
    for i in (1..=min_idx).rev() {
        arr.swap(i - 1, i);
    }

    // Now perform insertion sort without the boundary check
    // The sentinel at position 0 guarantees the loop terminates
    for i in 2..n {
        let mut j = i;

        // No need to check j > 0 because arr[0] is the sentinel (minimum)
        // The loop will terminate when we find an element <= arr[j]
        while arr[j] < arr[j - 1] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        let mut arr: Vec<i32> = vec![];
        sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single_element() {
        let mut arr = vec![42];
        sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_two_elements() {
        let mut arr = vec![2, 1];
        sort(&mut arr);
        assert_eq!(arr, vec![1, 2]);

        let mut arr = vec![1, 2];
        sort(&mut arr);
        assert_eq!(arr, vec![1, 2]);
    }

    #[test]
    fn test_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_duplicates() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
        sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 3, 4, 5, 5, 6, 9]);
    }

    #[test]
    fn test_all_duplicates() {
        let mut arr = vec![7, 7, 7, 7, 7];
        sort(&mut arr);
        assert_eq!(arr, vec![7, 7, 7, 7, 7]);
    }

    #[test]
    fn test_negative_numbers() {
        let mut arr = vec![-5, 3, -1, 0, -10, 7];
        sort(&mut arr);
        assert_eq!(arr, vec![-10, -5, -1, 0, 3, 7]);
    }

    #[test]
    fn test_strings() {
        let mut arr = vec!["dog", "cat", "zebra", "ant", "bear"];
        sort(&mut arr);
        assert_eq!(arr, vec!["ant", "bear", "cat", "dog", "zebra"]);
    }

    #[test]
    fn test_chars() {
        let mut arr = vec!['z', 'a', 'm', 'b', 'x'];
        sort(&mut arr);
        assert_eq!(arr, vec!['a', 'b', 'm', 'x', 'z']);
    }

    #[test]
    fn test_large_array() {
        let mut arr: Vec<i32> = (0..100).rev().collect();
        let expected: Vec<i32> = (0..100).collect();
        sort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_nearly_sorted() {
        // Insertion sort should be efficient on nearly sorted data
        let mut arr = vec![1, 2, 3, 5, 4, 6, 7, 8, 9, 10];
        sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_random_small() {
        let mut arr = vec![64, 25, 12, 22, 11];
        sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 64]);
    }
}
