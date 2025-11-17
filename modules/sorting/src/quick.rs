//! Standard quicksort implementation.
//!
//! Quicksort is one of the most important and widely-used sorting algorithms, invented by
//! Tony Hoare in 1960. It is a divide-and-conquer algorithm that works by selecting a pivot
//! element and partitioning the array around the pivot, such that elements less than the
//! pivot come before it and elements greater than the pivot come after it.
//!
//! This implementation uses randomized shuffling before sorting to avoid the O(n²) worst case
//! that can occur with sorted or nearly-sorted inputs.
//!
//! # Examples
//!
//! ```
//! use algs4_sorting::quick;
//!
//! let mut data = vec![64, 25, 12, 22, 11];
//! quick::sort(&mut data);
//! assert_eq!(data, vec![11, 12, 22, 25, 64]);
//! ```
//!
//! # Performance
//!
//! * Time Complexity:
//!   - Best case: O(n log n) - when partitions are balanced
//!   - Average case: O(n log n) - with random shuffling
//!   - Worst case: O(n²) - when partitions are unbalanced (avoided by shuffling)
//! * Space Complexity: O(log n) - for recursion stack
//! * Not stable: equal elements may not retain their relative order
//! * In-place: requires only a small auxiliary stack for recursion
//! * Comparisons: ~2n ln n on average
//! * Exchanges: ~⅓n ln n on average
//!
//! **Reference:** <https://algs4.cs.princeton.edu/23quicksort>

use algs4_fundamentals::io::stdrandom;

/// Sorts a slice in ascending order using quicksort.
///
/// Quicksort is a divide-and-conquer algorithm that:
/// 1. Shuffles the array to avoid worst-case performance
/// 2. Selects a pivot element
/// 3. Partitions the array so elements less than the pivot are on the left,
///    and elements greater than the pivot are on the right
/// 4. Recursively sorts the left and right subarrays
///
/// # Algorithm
///
/// The algorithm works recursively:
/// - Base case: arrays of size 0 or 1 are already sorted
/// - Recursive case:
///   1. Partition the array around a pivot element
///   2. Recursively sort the left subarray (elements < pivot)
///   3. Recursively sort the right subarray (elements > pivot)
///
/// The partitioning operation rearranges the array in-place using two pointers that
/// scan from both ends toward the middle, swapping elements that are out of place.
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
/// use algs4_sorting::quick;
///
/// let mut numbers = vec![5, 2, 8, 1, 9];
/// quick::sort(&mut numbers);
/// assert_eq!(numbers, vec![1, 2, 5, 8, 9]);
///
/// let mut chars = vec!['d', 'a', 'c', 'b'];
/// quick::sort(&mut chars);
/// assert_eq!(chars, vec!['a', 'b', 'c', 'd']);
/// ```
///
/// # Performance
///
/// - Time: O(n log n) average, O(n²) worst case (avoided by shuffling)
/// - Space: O(log n) for recursion stack
/// - Comparisons: ~2n ln n on average
/// - Exchanges: ~⅓n ln n on average
///
/// # Notes
///
/// Quicksort advantages:
/// - In-place sorting with minimal extra space (only recursion stack)
/// - Very fast in practice, especially for random data
/// - Cache-friendly due to in-place partitioning
/// - Average case is O(n log n) with good constants
///
/// Quicksort disadvantages:
/// - Not stable - equal elements may be rearranged
/// - Worst case O(n²) can occur without randomization
/// - Recursive implementation requires stack space
/// - Performance degrades on arrays with many duplicates (use 3-way quicksort instead)
pub fn sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    // Shuffle to avoid worst-case O(n²) performance on sorted/nearly-sorted inputs
    stdrandom::shuffle(arr);

    // Sort the entire array
    sort_recursive(arr, 0, n - 1);
}

/// Recursively sorts the subarray arr[lo..=hi] using quicksort.
///
/// # Arguments
///
/// * `arr` - The array to be sorted
/// * `lo` - Low index of the subarray (inclusive)
/// * `hi` - High index of the subarray (inclusive)
fn sort_recursive<T: Ord>(arr: &mut [T], lo: usize, hi: usize) {
    // Base case: single element or empty subarray
    if lo >= hi {
        return;
    }

    // Partition the array and get the partition index
    let j = partition(arr, lo, hi);

    // Recursively sort the left partition (elements < pivot)
    if j > 0 {
        sort_recursive(arr, lo, j.saturating_sub(1));
    }

    // Recursively sort the right partition (elements > pivot)
    if j < hi {
        sort_recursive(arr, j + 1, hi);
    }
}

/// Partitions the subarray arr[lo..=hi] around a pivot element.
///
/// This is the key operation of quicksort. After partitioning:
/// - arr[lo..j] contains elements ≤ pivot
/// - arr[j] contains the pivot element in its final sorted position
/// - arr[j+1..=hi] contains elements ≥ pivot
///
/// # Algorithm
///
/// 1. Select arr[lo] as the pivot element
/// 2. Scan from left (i) to find an element ≥ pivot
/// 3. Scan from right (j) to find an element ≤ pivot
/// 4. Swap arr[i] and arr[j]
/// 5. Repeat until the pointers cross
/// 6. Swap pivot with arr[j] to place it in its final position
///
/// # Arguments
///
/// * `arr` - The array being sorted
/// * `lo` - Low index (inclusive)
/// * `hi` - High index (inclusive)
///
/// # Returns
///
/// The index j where the pivot element is placed in its final sorted position.
fn partition<T: Ord>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    let mut i = lo + 1; // Left scan pointer
    let mut j = hi; // Right scan pointer

    loop {
        // Scan from left to find element >= pivot
        while i <= hi && arr[i] < arr[lo] {
            i += 1;
        }

        // Scan from right to find element <= pivot
        while j > lo && arr[j] > arr[lo] {
            j -= 1;
        }

        // Check if pointers cross
        if i >= j {
            break;
        }

        // Swap out-of-place elements
        arr.swap(i, j);
        i += 1;
        j -= 1;
    }

    // Place pivot (arr[lo]) in its final position (arr[j])
    arr.swap(lo, j);

    j
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
        let mut arr: Vec<i32> = (0..1000).rev().collect();
        let expected: Vec<i32> = (0..1000).collect();
        sort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_random_order() {
        let mut arr = vec![64, 25, 12, 22, 11, 90, 88, 45, 50, 23];
        sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 23, 25, 45, 50, 64, 88, 90]);
    }

    #[test]
    fn test_three_elements() {
        let mut arr = vec![3, 1, 2];
        sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3]);

        let mut arr = vec![1, 3, 2];
        sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3]);
    }

    #[test]
    #[allow(clippy::needless_range_loop)]
    fn test_partition_correctness() {
        // After partition, all elements to the left of pivot should be <= pivot
        // and all elements to the right should be >= pivot
        let mut arr = vec![5, 2, 8, 1, 9, 3, 7, 4, 6];
        let len = arr.len();
        let j = partition(&mut arr, 0, len - 1);

        let pivot = arr[j];
        for i in 0..j {
            assert!(arr[i] <= pivot);
        }
        for i in (j + 1)..arr.len() {
            assert!(arr[i] >= pivot);
        }
    }

    #[test]
    fn test_many_duplicates() {
        let mut arr = vec![5, 2, 5, 2, 5, 2, 5, 2, 5];
        sort(&mut arr);
        assert_eq!(arr, vec![2, 2, 2, 2, 5, 5, 5, 5, 5]);
    }

    #[test]
    fn test_power_of_two_size() {
        let mut arr: Vec<i32> = (0..256).rev().collect();
        let expected: Vec<i32> = (0..256).collect();
        sort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_alternating() {
        let mut arr = vec![1, 10, 2, 9, 3, 8, 4, 7, 5, 6];
        sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
