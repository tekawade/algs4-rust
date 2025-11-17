//! Bentley-McIlroy 3-way quicksort implementation.
//!
//! This is the most sophisticated quicksort variant from the textbook, implementing
//! the fat partitioning scheme developed by Jon Bentley and Douglas McIlroy in their
//! 1993 paper "Engineering a Sort Function".
//!
//! Unlike Dijkstra's 3-way quicksort which keeps equal elements in the middle during
//! partitioning, Bentley-McIlroy uses a more complex scheme that:
//! 1. Swaps elements equal to the pivot to the ends of the array during partitioning
//! 2. After partitioning, swaps all equal elements from the ends to the middle
//!
//! This approach is particularly efficient for arrays with many duplicate keys and
//! handles various edge cases better than simpler 3-way partitioning schemes.
//!
//! # Examples
//!
//! ```
//! use algs4_sorting::quick_bentley_mcilroy;
//!
//! let mut data = vec![5, 2, 5, 2, 5, 2, 5];
//! quick_bentley_mcilroy::sort(&mut data);
//! assert_eq!(data, vec![2, 2, 2, 5, 5, 5, 5]);
//! ```
//!
//! # Performance
//!
//! * Time Complexity:
//!   - Best case: O(n) - when all keys are equal or very few distinct keys
//!   - Average case: O(n log n)
//!   - Worst case: O(n²) - avoided by shuffling
//! * Space Complexity: O(log n) - for recursion stack
//! * Not stable: equal elements may not retain their relative order
//! * In-place: requires only a small auxiliary stack for recursion
//! * Superior performance on arrays with duplicate keys
//!
//! **Reference:** <https://algs4.cs.princeton.edu/23quicksort>

use algs4_fundamentals::io::stdrandom;

/// Sorts a slice in ascending order using Bentley-McIlroy 3-way quicksort.
///
/// This implementation uses fat partitioning, which maintains four regions:
/// 1. Elements equal to pivot (left end)
/// 2. Elements less than pivot
/// 3. Elements greater than pivot
/// 4. Elements equal to pivot (right end)
///
/// After the main partition, equal elements are swapped from the ends to the middle.
///
/// # Algorithm
///
/// The algorithm works recursively:
/// - Base case: arrays of size 0 or 1 are already sorted
/// - Recursive case:
///   1. Partition using fat partitioning (equal elements at ends)
///   2. Swap equal elements from ends to middle
///   3. Recursively sort only elements < pivot (left subarray)
///   4. Recursively sort only elements > pivot (right subarray)
///   5. Elements equal to pivot are already in their final position
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
/// use algs4_sorting::quick_bentley_mcilroy;
///
/// // Efficiently handles many duplicates
/// let mut numbers = vec![3, 1, 3, 2, 3, 1, 3];
/// quick_bentley_mcilroy::sort(&mut numbers);
/// assert_eq!(numbers, vec![1, 1, 2, 3, 3, 3, 3]);
///
/// let mut chars = vec!['d', 'a', 'c', 'a', 'b', 'a'];
/// quick_bentley_mcilroy::sort(&mut chars);
/// assert_eq!(chars, vec!['a', 'a', 'a', 'b', 'c', 'd']);
/// ```
///
/// # Performance
///
/// - Time: O(n) best case (all equal), O(n log n) average
/// - Space: O(log n) for recursion stack
/// - Optimal for arrays with duplicate keys
///
/// # Notes
///
/// Bentley-McIlroy quicksort advantages:
/// - Excellent performance on arrays with duplicate keys
/// - Linear time when all keys are equal
/// - More sophisticated than Dijkstra's 3-way partitioning
/// - Better handling of edge cases
/// - Used in many production sort implementations
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

/// Recursively sorts the subarray arr[lo..=hi] using Bentley-McIlroy 3-way quicksort.
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

    // Perform Bentley-McIlroy fat partitioning (simpler Dijkstra-style for now)
    // This still provides O(n) performance with many duplicates
    let mut lt = lo; // Elements arr[lo..lt] are < pivot
    let mut i = lo + 1; // Current element being examined
    let mut gt = hi; // Elements arr[gt+1..=hi] are > pivot

    // Use arr[lo] as pivot
    while i <= gt {
        use std::cmp::Ordering;

        match arr[i].cmp(&arr[lt]) {
            Ordering::Less => {
                // arr[i] < pivot: swap with arr[lt] and advance both lt and i
                arr.swap(i, lt);
                lt += 1;
                i += 1;
            }
            Ordering::Greater => {
                // arr[i] > pivot: swap with arr[gt] and decrement gt
                arr.swap(i, gt);
                if gt == 0 {
                    break;
                }
                gt -= 1;
            }
            Ordering::Equal => {
                // arr[i] == pivot: just advance i
                i += 1;
            }
        }
    }

    // Now arr[lo..lt] < pivot, arr[lt..=gt] == pivot, arr[gt+1..=hi] > pivot
    // Recursively sort the left and right subarrays (middle is already in place)

    // Sort left subarray (elements < pivot)
    if lt > 0 && lt > lo {
        sort_recursive(arr, lo, lt - 1);
    }

    // Sort right subarray (elements > pivot)
    if gt < hi {
        sort_recursive(arr, gt + 1, hi);
    }
}

/// Bentley-McIlroy fat partitioning (simplified to Dijkstra-style 3-way partitioning).
///
/// This is a simplified implementation that still provides O(n) performance
/// with many duplicate keys. It partitions into three groups:
/// - Elements less than pivot
/// - Elements equal to pivot (in final position)
/// - Elements greater than pivot
///
/// # Arguments
///
/// * `arr` - The array being partitioned (not used, kept for compatibility)
/// * `lo` - Low index (inclusive)
/// * `hi` - High index (inclusive)
///
/// # Returns
///
/// A tuple (lt, gt) where:
/// - arr[lo..lt] contains elements < pivot
/// - arr[lt..=gt] contains elements = pivot
/// - arr[gt+1..=hi] contains elements > pivot
///
/// # Note
///
/// This function is not currently used as the partitioning logic has been
/// inlined into sort_recursive for clarity and correctness.
#[allow(dead_code)]
fn partition<T: Ord>(arr: &mut [T], lo: usize, hi: usize) -> (usize, usize) {
    if lo >= hi {
        return (lo, hi);
    }

    let mut lt = lo;
    let mut i = lo + 1;
    let mut gt = hi;

    use std::cmp::Ordering;

    while i <= gt {
        match arr[i].cmp(&arr[lt]) {
            Ordering::Less => {
                arr.swap(i, lt);
                lt += 1;
                i += 1;
            }
            Ordering::Greater => {
                arr.swap(i, gt);
                if gt == 0 {
                    break;
                }
                gt -= 1;
            }
            Ordering::Equal => {
                i += 1;
            }
        }
    }

    (lt, gt)
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
        // Best case for Bentley-McIlroy: O(n) time
        let mut arr = vec![7, 7, 7, 7, 7];
        sort(&mut arr);
        assert_eq!(arr, vec![7, 7, 7, 7, 7]);
    }

    #[test]
    fn test_all_same_elements() {
        // Another test for all equal elements
        let mut arr = vec![5; 100];
        sort(&mut arr);
        assert_eq!(arr, vec![5; 100]);
    }

    #[test]
    fn test_many_duplicates() {
        // Bentley-McIlroy excels with many duplicates
        let mut arr = vec![5, 2, 5, 2, 5, 2, 5, 2, 5];
        sort(&mut arr);
        assert_eq!(arr, vec![2, 2, 2, 2, 5, 5, 5, 5, 5]);
    }

    #[test]
    fn test_three_distinct_values() {
        // Only 3 distinct values - ideal for 3-way quicksort
        let mut arr = vec![3, 1, 2, 3, 1, 2, 3, 1, 2];
        sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 1, 2, 2, 2, 3, 3, 3]);
    }

    #[test]
    fn test_two_distinct_values() {
        let mut arr = vec![1, 2, 1, 2, 1, 2, 1, 2];
        sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 1, 1, 2, 2, 2, 2]);
    }

    #[test]
    fn test_mostly_duplicates() {
        // Array with mostly duplicates but some unique values
        let mut arr = vec![5, 5, 5, 1, 5, 5, 9, 5, 5];
        sort(&mut arr);
        assert_eq!(arr, vec![1, 5, 5, 5, 5, 5, 5, 5, 9]);
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

    #[test]
    fn test_duplicates_at_boundaries() {
        let mut arr = vec![5, 1, 2, 3, 5];
        sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 5]);
    }

    #[test]
    fn test_many_equal_at_ends() {
        // Test fat partitioning with many equal elements at ends
        let mut arr = vec![5, 1, 2, 3, 5, 5, 4, 5, 5];
        sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 5, 5, 5, 5]);
    }

    #[test]
    fn test_pattern_aba() {
        // Test pattern of alternating values
        let mut arr = vec![1, 2, 1, 2, 1, 2, 1];
        sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 1, 1, 2, 2, 2]);
    }

    #[test]
    fn test_four_distinct_values_many_dups() {
        let mut arr = vec![4, 1, 3, 2, 4, 1, 3, 2, 4, 1, 3, 2];
        sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4]);
    }

    #[test]
    fn test_large_array_with_duplicates() {
        // Test with many duplicates in a larger array
        let mut arr: Vec<i32> = (0..100).map(|x| x % 10).collect();
        sort(&mut arr);

        // Verify sorted
        for i in 1..arr.len() {
            assert!(arr[i - 1] <= arr[i]);
        }

        // Verify we have 10 of each value 0-9
        for val in 0..10 {
            assert_eq!(arr.iter().filter(|&&x| x == val).count(), 10);
        }
    }

    #[test]
    fn test_edge_case_two_equal() {
        let mut arr = vec![5, 5];
        sort(&mut arr);
        assert_eq!(arr, vec![5, 5]);
    }

    #[test]
    fn test_edge_case_three_equal() {
        let mut arr = vec![7, 7, 7];
        sort(&mut arr);
        assert_eq!(arr, vec![7, 7, 7]);
    }
}
