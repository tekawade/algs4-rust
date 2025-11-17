//! 3-way quicksort implementation (Dijkstra's algorithm).
//!
//! 3-way quicksort is a variant of quicksort that handles duplicate keys much more efficiently
//! than standard quicksort. It was popularized by Edsger Dijkstra and is also known as
//! Dutch National Flag partitioning.
//!
//! Instead of partitioning into two groups (less than and greater than or equal to pivot),
//! 3-way quicksort partitions into three groups:
//! - Elements less than the pivot
//! - Elements equal to the pivot
//! - Elements greater than the pivot
//!
//! This is particularly efficient when there are many duplicate keys, reducing time complexity
//! from O(n log n) to O(n) in the best case (when all keys are equal).
//!
//! # Examples
//!
//! ```
//! use algs4_sorting::quick_3way;
//!
//! let mut data = vec![5, 2, 5, 2, 5, 2, 5];
//! quick_3way::sort(&mut data);
//! assert_eq!(data, vec![2, 2, 2, 5, 5, 5, 5]);
//! ```
//!
//! # Performance
//!
//! * Time Complexity:
//!   - Best case: O(n) - when all keys are equal or very few distinct keys
//!   - Average case: O(n log n) - similar to standard quicksort
//!   - Worst case: O(n²) - when partitions are unbalanced (avoided by shuffling)
//! * Space Complexity: O(log n) - for recursion stack
//! * Not stable: equal elements may not retain their relative order
//! * In-place: requires only a small auxiliary stack for recursion
//! * Superior to standard quicksort when there are duplicate keys
//!
//! **Reference:** <https://algs4.cs.princeton.edu/23quicksort>

use algs4_fundamentals::io::stdrandom;

/// Sorts a slice in ascending order using 3-way quicksort.
///
/// 3-way quicksort partitions the array into three groups:
/// 1. Elements less than the pivot
/// 2. Elements equal to the pivot (these are in their final position)
/// 3. Elements greater than the pivot
///
/// This approach is much more efficient than standard quicksort when there are
/// many duplicate keys, because equal elements don't need to be recursively sorted.
///
/// # Algorithm
///
/// The algorithm works recursively:
/// - Base case: arrays of size 0 or 1 are already sorted
/// - Recursive case:
///   1. Partition the array into three groups around a pivot
///   2. Recursively sort only the left group (elements < pivot)
///   3. Recursively sort only the right group (elements > pivot)
///   4. Elements equal to pivot are already in their final position
///
/// The 3-way partitioning uses three pointers:
/// - lt: boundary between elements < pivot and elements = pivot
/// - i: current scanning position
/// - gt: boundary between elements = pivot and elements > pivot
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
/// use algs4_sorting::quick_3way;
///
/// // Efficiently handles many duplicates
/// let mut numbers = vec![3, 1, 3, 2, 3, 1, 3];
/// quick_3way::sort(&mut numbers);
/// assert_eq!(numbers, vec![1, 1, 2, 3, 3, 3, 3]);
///
/// let mut chars = vec!['d', 'a', 'c', 'a', 'b', 'a'];
/// quick_3way::sort(&mut chars);
/// assert_eq!(chars, vec!['a', 'a', 'a', 'b', 'c', 'd']);
/// ```
///
/// # Performance
///
/// - Time: O(n) best case (all equal), O(n log n) average, O(n²) worst case (avoided by shuffling)
/// - Space: O(log n) for recursion stack
/// - Dramatically faster than standard quicksort when duplicate keys are present
///
/// # Notes
///
/// 3-way quicksort advantages:
/// - Optimal for arrays with duplicate keys
/// - Linear time when all keys are equal
/// - Reduces recursive calls by excluding equal elements
/// - Same space complexity as standard quicksort
///
/// 3-way quicksort is recommended over standard quicksort when:
/// - The input may contain many duplicate keys
/// - The key space is small relative to array size
/// - Sorting arrays with limited distinct values
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

/// Recursively sorts the subarray arr[lo..=hi] using 3-way quicksort.
///
/// This function performs 3-way partitioning inline, dividing the array into:
/// - arr[lo..lt]: elements less than pivot
/// - arr[lt..=gt]: elements equal to pivot (in final position)
/// - arr[gt+1..=hi]: elements greater than pivot
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

    // 3-way partitioning
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
        // Best case for 3-way quicksort: O(n) time
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
        // 3-way quicksort excels with many duplicates
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
}
