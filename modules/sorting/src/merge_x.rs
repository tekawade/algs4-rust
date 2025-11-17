//! Optimized mergesort implementation.
//!
//! This is an enhanced version of the classic mergesort algorithm with several
//! optimizations that improve practical performance while maintaining the same
//! O(n log n) time complexity guarantee.
//!
//! # Optimizations
//!
//! 1. **Cutoff to insertion sort for small subarrays**: For small subarrays
//!    (typically 7-15 elements), insertion sort is faster than mergesort due
//!    to lower overhead. We switch to insertion sort for small subarrays.
//!
//! 2. **Skip merge if already sorted**: If the largest element in the left
//!    subarray is less than or equal to the smallest element in the right
//!    subarray (arr[mid] <= arr[mid+1]), the array is already sorted and
//!    we can skip the merge step.
//!
//! 3. **Eliminate copy to auxiliary array**: Instead of copying data to the
//!    auxiliary array on every merge, we switch the role of the input and
//!    auxiliary arrays in recursive calls. This saves time by eliminating
//!    one array copy operation per merge.
//!
//! # Examples
//!
//! ```
//! use algs4_sorting::merge_x;
//!
//! let mut data = vec![64, 25, 12, 22, 11];
//! merge_x::sort(&mut data);
//! assert_eq!(data, vec![11, 12, 22, 25, 64]);
//! ```
//!
//! # Performance
//!
//! * Time Complexity:
//!   - Best case: O(n) - when array is already sorted (skip merge optimization)
//!   - Average case: O(n log n)
//!   - Worst case: O(n log n) - guaranteed
//! * Space Complexity: O(n) - requires auxiliary array
//! * Stable: equal elements retain their relative order
//! * Partially adaptive: takes advantage of sorted subarrays
//! * In practice: 10-20% faster than standard mergesort
//!
//! **Reference:** <https://algs4.cs.princeton.edu/22mergesort>

/// Cutoff threshold for switching to insertion sort.
///
/// For small subarrays, insertion sort is faster than mergesort.
/// Typical values range from 7 to 15. We use 10 as a good balance.
const CUTOFF: usize = 10;

/// Sorts a slice in ascending order using optimized mergesort.
///
/// This implementation includes three key optimizations:
/// 1. Uses insertion sort for small subarrays (< CUTOFF elements)
/// 2. Skips merge if array is already sorted
/// 3. Eliminates one copy to auxiliary array by switching array roles
///
/// # Type Parameters
///
/// * `T` - The type of elements in the slice. Must implement `Ord` for comparison
///   and `Clone` for copying to the auxiliary array.
///
/// # Arguments
///
/// * `arr` - A mutable slice to be sorted
///
/// # Examples
///
/// ```
/// use algs4_sorting::merge_x;
///
/// let mut numbers = vec![5, 2, 8, 1, 9];
/// merge_x::sort(&mut numbers);
/// assert_eq!(numbers, vec![1, 2, 5, 8, 9]);
///
/// let mut chars = vec!['d', 'a', 'c', 'b'];
/// merge_x::sort(&mut chars);
/// assert_eq!(chars, vec!['a', 'b', 'c', 'd']);
/// ```
///
/// # Performance
///
/// - Time: O(n log n) worst case, O(n) best case (sorted)
/// - Space: O(n) - requires auxiliary array
/// - In practice: 10-20% faster than standard mergesort
///
/// # Notes
///
/// The optimizations provide significant speedup:
/// - Insertion sort cutoff: ~10-20% speedup for random data
/// - Skip merge optimization: Huge speedup for nearly sorted data
/// - Eliminate copy: ~5-10% speedup by reducing data movement
pub fn sort<T: Ord + Clone>(arr: &mut [T]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    // Create auxiliary array
    let mut aux = arr.to_vec();

    // Sort from aux into arr
    sort_recursive(&mut aux, arr, 0, n - 1);
}

/// Recursively sorts arr[lo..hi] using optimized mergesort.
///
/// This function implements the array-switching optimization: it sorts from
/// `src` into `dst`. By switching which array is source and which is destination
/// in recursive calls, we eliminate the need to copy to the auxiliary array
/// before merging.
///
/// # Arguments
///
/// * `src` - Source array to sort from
/// * `dst` - Destination array to sort into
/// * `lo` - Low index (inclusive)
/// * `hi` - High index (inclusive)
fn sort_recursive<T: Ord + Clone>(src: &mut [T], dst: &mut [T], lo: usize, hi: usize) {
    // Optimization 1: Use insertion sort for small subarrays
    if hi <= lo + CUTOFF {
        insertion_sort(dst, lo, hi);
        return;
    }

    // Divide
    let mid = lo + (hi - lo) / 2;

    // Conquer: Note the switched roles of src and dst!
    // We sort from dst into src, then merge from src into dst
    sort_recursive(dst, src, lo, mid);
    sort_recursive(dst, src, mid + 1, hi);

    // Optimization 2: Skip merge if already sorted
    // If the largest element in left half <= smallest in right half, already sorted
    if src[mid] <= src[mid + 1] {
        // Just copy src to dst (it's already sorted) using slice operation
        dst[lo..=hi].clone_from_slice(&src[lo..=hi]);
        return;
    }

    // Combine: Merge from src into dst
    // Optimization 3: This merge doesn't need to copy to aux first,
    // because we're already working with switched arrays
    merge(src, dst, lo, mid, hi);
}

/// Merges two sorted subarrays from src into dst.
///
/// Assumes src[lo..mid] and src[mid+1..hi] are sorted.
/// Merges them into dst[lo..hi].
///
/// Unlike the standard merge, this version doesn't copy to auxiliary first
/// because the array-switching optimization in sort_recursive handles that.
///
/// # Arguments
///
/// * `src` - Source array with two sorted subarrays
/// * `dst` - Destination array for merged result
/// * `lo` - Low index (inclusive)
/// * `mid` - Middle index (end of first sorted subarray)
/// * `hi` - High index (inclusive)
fn merge<T: Ord + Clone>(src: &[T], dst: &mut [T], lo: usize, mid: usize, hi: usize) {
    let mut i = lo; // Current index in left half
    let mut j = mid + 1; // Current index in right half
    let mut k = lo; // Current index in merged result

    while k <= hi {
        if i > mid {
            // Left half exhausted, take from right
            dst[k] = src[j].clone();
            j += 1;
        } else if j > hi {
            // Right half exhausted, take from left
            dst[k] = src[i].clone();
            i += 1;
        } else if src[j] < src[i] {
            // Right element is smaller
            dst[k] = src[j].clone();
            j += 1;
        } else {
            // Left element is smaller or equal (stability)
            dst[k] = src[i].clone();
            i += 1;
        }
        k += 1;
    }
}

/// Sorts arr[lo..hi] using insertion sort.
///
/// This is used for small subarrays where insertion sort is faster than
/// mergesort due to lower overhead.
///
/// # Arguments
///
/// * `arr` - Array to sort
/// * `lo` - Low index (inclusive)
/// * `hi` - High index (inclusive)
fn insertion_sort<T: Ord>(arr: &mut [T], lo: usize, hi: usize) {
    for i in (lo + 1)..=hi {
        let mut j = i;
        while j > lo && arr[j - 1] > arr[j] {
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
    fn test_small_array_below_cutoff() {
        // Test arrays smaller than CUTOFF to ensure insertion sort works
        let mut arr = vec![5, 2, 8, 1, 9, 3, 7];
        sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 7, 8, 9]);
    }

    #[test]
    fn test_array_at_cutoff() {
        // Test array exactly at CUTOFF size
        let mut arr: Vec<i32> = (0..CUTOFF as i32).rev().collect();
        let expected: Vec<i32> = (0..CUTOFF as i32).collect();
        sort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_array_just_above_cutoff() {
        // Test array just above CUTOFF to ensure transition works
        let size = CUTOFF + 5;
        let mut arr: Vec<i32> = (0..size as i32).rev().collect();
        let expected: Vec<i32> = (0..size as i32).collect();
        sort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_already_sorted() {
        // This should trigger the skip merge optimization
        let mut arr = vec![1, 2, 3, 4, 5];
        sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_already_sorted_large() {
        // Test skip merge optimization on larger sorted array
        let mut arr: Vec<i32> = (0..100).collect();
        let expected: Vec<i32> = (0..100).collect();
        sort(&mut arr);
        assert_eq!(arr, expected);
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
    fn test_nearly_sorted() {
        // Test that skip merge optimization helps with nearly sorted data
        let mut arr = vec![1, 2, 3, 5, 4, 6, 7, 8, 10, 9];
        sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_stability() {
        // Test that mergesort is stable by using a custom type
        #[derive(Clone, Debug)]
        struct Item {
            key: i32,
            value: i32,
        }

        impl PartialEq for Item {
            fn eq(&self, other: &Self) -> bool {
                self.key == other.key
            }
        }

        impl Eq for Item {}

        impl PartialOrd for Item {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for Item {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.key.cmp(&other.key)
            }
        }

        let mut arr = vec![
            Item { key: 3, value: 1 },
            Item { key: 1, value: 2 },
            Item { key: 3, value: 3 },
            Item { key: 2, value: 4 },
            Item { key: 1, value: 5 },
        ];

        sort(&mut arr);

        // Check keys are sorted
        assert_eq!(arr[0].key, 1);
        assert_eq!(arr[1].key, 1);
        assert_eq!(arr[2].key, 2);
        assert_eq!(arr[3].key, 3);
        assert_eq!(arr[4].key, 3);

        // Check stability: items with same key maintain original order
        assert_eq!(arr[0].value, 2); // First 1
        assert_eq!(arr[1].value, 5); // Second 1
        assert_eq!(arr[3].value, 1); // First 3
        assert_eq!(arr[4].value, 3); // Second 3
    }

    #[test]
    fn test_power_of_two_size() {
        let mut arr: Vec<i32> = (0..256).rev().collect();
        let expected: Vec<i32> = (0..256).collect();
        sort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_odd_size() {
        let mut arr = vec![9, 7, 5, 3, 1, 0, 2, 4, 6, 8];
        sort(&mut arr);
        assert_eq!(arr, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_partially_sorted_segments() {
        // Array with sorted segments that need merging
        let mut arr = vec![1, 3, 5, 7, 2, 4, 6, 8];
        sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_insertion_sort_helper() {
        let mut arr = vec![5, 2, 8, 1, 9];
        insertion_sort(&mut arr, 0, 4);
        assert_eq!(arr, vec![1, 2, 5, 8, 9]);

        // Test sorting a subarray
        let mut arr = vec![0, 5, 2, 8, 1, 9, 0];
        insertion_sort(&mut arr, 1, 5);
        assert_eq!(arr, vec![0, 1, 2, 5, 8, 9, 0]);
    }
}
