//! Selection sort implementation.
//!
//! Selection sort is one of the simplest sorting algorithms. It works by repeatedly
//! finding the minimum element from the unsorted portion and placing it at the beginning.
//!
//! The algorithm maintains two subarrays:
//! - The sorted subarray (at the beginning)
//! - The remaining unsorted subarray
//!
//! In each iteration, the minimum element from the unsorted subarray is selected and
//! moved to the sorted subarray.
//!
//! # Examples
//!
//! ```
//! use algs4_sorting::selection;
//!
//! let mut data = vec![64, 25, 12, 22, 11];
//! selection::sort(&mut data);
//! assert_eq!(data, vec![11, 12, 22, 25, 64]);
//! ```
//!
//! # Performance
//!
//! * Time Complexity: O(n²) in all cases (best, average, and worst)
//! * Space Complexity: O(1) - in-place sorting
//! * Not stable: equal elements may not retain their relative order
//! * Number of comparisons: ~n²/2
//! * Number of exchanges: n (one per position)
//!
//! **Reference:** <https://algs4.cs.princeton.edu/21elementary>

/// Sorts a slice in ascending order using selection sort.
///
/// Selection sort works by finding the minimum element in the unsorted portion
/// of the array and swapping it with the element at the current position.
/// This process continues until the entire array is sorted.
///
/// # Algorithm
///
/// For each position i from 0 to n-1:
/// 1. Find the index of the minimum element in the range [i, n)
/// 2. Swap the element at position i with the minimum element
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
/// use algs4_sorting::selection;
///
/// let mut numbers = vec![5, 2, 8, 1, 9];
/// selection::sort(&mut numbers);
/// assert_eq!(numbers, vec![1, 2, 5, 8, 9]);
///
/// let mut chars = vec!['d', 'a', 'c', 'b'];
/// selection::sort(&mut chars);
/// assert_eq!(chars, vec!['a', 'b', 'c', 'd']);
/// ```
///
/// # Performance
///
/// - Time: O(n²) - performs ~n²/2 comparisons regardless of input
/// - Space: O(1) - sorts in place with only a constant amount of extra space
/// - Comparisons: ~n²/2 (exactly (n-1) + (n-2) + ... + 1 + 0 = n(n-1)/2)
/// - Exchanges: n (at most n exchanges, one per iteration)
///
/// # Notes
///
/// Selection sort is not adaptive - it performs the same number of comparisons
/// even if the input is already sorted. However, it makes the minimum number
/// of swaps (n) compared to other O(n²) algorithms, which can be useful when
/// swapping is expensive.
pub fn sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();

    // For each position in the array
    for i in 0..n {
        // Find the index of the minimum element in the unsorted portion
        let min_idx = find_min_index(&arr[i..]) + i;

        // Swap the minimum element with the current position
        arr.swap(i, min_idx);
    }
}

/// Finds the index of the minimum element in a slice.
///
/// # Arguments
///
/// * `arr` - A slice to search
///
/// # Returns
///
/// The index of the minimum element in the slice.
///
/// # Panics
///
/// Panics if the slice is empty (returns 0 which would be invalid).
fn find_min_index<T: Ord>(arr: &[T]) -> usize {
    let mut min_idx = 0;

    for (idx, item) in arr.iter().enumerate().skip(1) {
        if item < &arr[min_idx] {
            min_idx = idx;
        }
    }

    min_idx
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
    fn test_two_elements() {
        let mut arr = vec![2, 1];
        sort(&mut arr);
        assert_eq!(arr, vec![1, 2]);

        let mut arr = vec![1, 2];
        sort(&mut arr);
        assert_eq!(arr, vec![1, 2]);
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
    fn test_large_array() {
        let mut arr: Vec<i32> = (0..100).rev().collect();
        let expected: Vec<i32> = (0..100).collect();
        sort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_chars() {
        let mut arr = vec!['z', 'a', 'm', 'b', 'x'];
        sort(&mut arr);
        assert_eq!(arr, vec!['a', 'b', 'm', 'x', 'z']);
    }
}
