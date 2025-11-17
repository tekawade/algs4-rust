//! Top-down mergesort implementation.
//!
//! Mergesort is one of the most important sorting algorithms, invented by John von Neumann
//! in 1945. It is a divide-and-conquer algorithm that recursively divides the array into
//! two halves, sorts them, and then merges the two sorted halves.
//!
//! This is the classic recursive (top-down) implementation that guarantees O(n log n)
//! performance regardless of the input.
//!
//! # Examples
//!
//! ```
//! use algs4_sorting::merge;
//!
//! let mut data = vec![64, 25, 12, 22, 11];
//! merge::sort(&mut data);
//! assert_eq!(data, vec![11, 12, 22, 25, 64]);
//! ```
//!
//! # Performance
//!
//! * Time Complexity:
//!   - Best case: O(n log n)
//!   - Average case: O(n log n)
//!   - Worst case: O(n log n) - guaranteed
//! * Space Complexity: O(n) - requires auxiliary array for merging
//! * Stable: equal elements retain their relative order
//! * Not adaptive: always performs the same number of operations
//! * Comparisons: between ~½n lg n and ~n lg n
//! * Array accesses: ~6n lg n (2n for copy, 2n for move back, ~2n for compares)
//!
//! **Reference:** <https://algs4.cs.princeton.edu/22mergesort>

/// Sorts a slice in ascending order using top-down mergesort.
///
/// Mergesort is a divide-and-conquer algorithm that:
/// 1. Divides the array into two halves
/// 2. Recursively sorts each half
/// 3. Merges the two sorted halves
///
/// # Algorithm
///
/// The algorithm works recursively:
/// - Base case: arrays of size 0 or 1 are already sorted
/// - Recursive case:
///   1. Find the midpoint and divide array into two halves
///   2. Recursively sort the left half
///   3. Recursively sort the right half
///   4. Merge the two sorted halves
///
/// The merge operation uses an auxiliary array to combine two sorted subarrays
/// into a single sorted array.
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
/// use algs4_sorting::merge;
///
/// let mut numbers = vec![5, 2, 8, 1, 9];
/// merge::sort(&mut numbers);
/// assert_eq!(numbers, vec![1, 2, 5, 8, 9]);
///
/// let mut chars = vec!['d', 'a', 'c', 'b'];
/// merge::sort(&mut chars);
/// assert_eq!(chars, vec!['a', 'b', 'c', 'd']);
/// ```
///
/// # Performance
///
/// - Time: O(n log n) guaranteed - always divides in half and merges
/// - Space: O(n) - requires auxiliary array for merging
/// - Comparisons: between ~½n lg n and ~n lg n
/// - Array accesses: ~6n lg n
///
/// # Notes
///
/// Mergesort advantages:
/// - Guaranteed O(n log n) performance (no worst case)
/// - Stable sorting (preserves relative order of equal elements)
/// - Predictable performance
/// - Well-suited for linked lists (can sort in O(1) extra space)
///
/// Mergesort disadvantages:
/// - Requires O(n) extra space for the auxiliary array
/// - Not adaptive - doesn't take advantage of existing order
/// - For small arrays, insertion sort is faster due to lower overhead
pub fn sort<T: Ord + Clone>(arr: &mut [T]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    // Create auxiliary array for merging
    let mut aux = arr.to_vec();
    sort_recursive(arr, &mut aux, 0, n - 1);
}

/// Recursively sorts the subarray arr[lo..hi] using mergesort.
///
/// # Arguments
///
/// * `arr` - The array to be sorted
/// * `aux` - Auxiliary array for merging
/// * `lo` - Low index of the subarray (inclusive)
/// * `hi` - High index of the subarray (inclusive)
fn sort_recursive<T: Ord + Clone>(arr: &mut [T], aux: &mut [T], lo: usize, hi: usize) {
    // Base case: single element or empty
    if lo >= hi {
        return;
    }

    // Divide: find the midpoint
    let mid = lo + (hi - lo) / 2;

    // Conquer: recursively sort both halves
    sort_recursive(arr, aux, lo, mid);
    sort_recursive(arr, aux, mid + 1, hi);

    // Combine: merge the sorted halves
    merge(arr, aux, lo, mid, hi);
}

/// Merges two sorted subarrays arr[lo..mid] and arr[mid+1..hi].
///
/// This is the key operation of mergesort. It assumes that arr[lo..mid] and
/// arr[mid+1..hi] are already sorted, and merges them into a single sorted
/// subarray arr[lo..hi].
///
/// # Algorithm
///
/// 1. Copy arr[lo..hi] to aux[lo..hi]
/// 2. Maintain three indices:
///    - i: current position in left half (lo..mid)
///    - j: current position in right half (mid+1..hi)
///    - k: current position in merged result (lo..hi)
/// 3. Compare aux[i] and aux[j], copy smaller to arr[k]
/// 4. Handle remaining elements when one half is exhausted
///
/// # Arguments
///
/// * `arr` - The array being sorted
/// * `aux` - Auxiliary array for merging
/// * `lo` - Low index (inclusive)
/// * `mid` - Middle index (end of first sorted subarray)
/// * `hi` - High index (inclusive)
fn merge<T: Ord + Clone>(arr: &mut [T], aux: &mut [T], lo: usize, mid: usize, hi: usize) {
    // Copy to auxiliary array using slice operation
    aux[lo..=hi].clone_from_slice(&arr[lo..=hi]);

    // Merge back to arr[lo..hi]
    let mut i = lo; // Current index in left half
    let mut j = mid + 1; // Current index in right half
    let mut k = lo; // Current index in merged result

    while k <= hi {
        if i > mid {
            // Left half exhausted, take from right
            arr[k] = aux[j].clone();
            j += 1;
        } else if j > hi {
            // Right half exhausted, take from left
            arr[k] = aux[i].clone();
            i += 1;
        } else if aux[j] < aux[i] {
            // Right element is smaller
            arr[k] = aux[j].clone();
            j += 1;
        } else {
            // Left element is smaller or equal (stability)
            arr[k] = aux[i].clone();
            i += 1;
        }
        k += 1;
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
        // Test with array size that's a power of 2
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
}
