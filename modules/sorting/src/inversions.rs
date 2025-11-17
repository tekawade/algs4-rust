//! Count inversions in an array using a modified mergesort algorithm.
//!
//! An inversion is a pair of indices (i, j) where i < j but arr[i] > arr[j].
//! The number of inversions is a measure of how far an array is from being sorted.
//! A sorted array has zero inversions, while a reverse-sorted array has the maximum
//! number of inversions: n(n-1)/2.
//!
//! This implementation uses a divide-and-conquer approach based on mergesort to count
//! inversions in O(n log n) time, which is significantly faster than the naive O(n²)
//! approach of checking all pairs.
//!
//! # Examples
//!
//! ```
//! use algs4_sorting::inversions;
//!
//! let data = vec![2, 4, 1, 3, 5];
//! let count = inversions::count(&data);
//! assert_eq!(count, 3); // Inversions: (2,1), (4,1), (4,3)
//! ```
//!
//! # Performance
//!
//! * Time Complexity: O(n log n) - same as mergesort
//! * Space Complexity: O(n) - requires auxiliary array for merging
//! * Much faster than naive O(n²) approach for large arrays
//!
//! **Reference:** <https://algs4.cs.princeton.edu/22mergesort>

/// Counts the number of inversions in a slice.
///
/// An inversion is a pair of indices (i, j) where i < j but arr[i] > arr[j].
/// This function uses a modified mergesort algorithm to count inversions efficiently.
///
/// # Algorithm
///
/// The algorithm works by dividing the array and counting three types of inversions:
///
/// 1. **Left inversions**: inversions within the left half
/// 2. **Right inversions**: inversions within the right half
/// 3. **Split inversions**: inversions where i is in left half and j is in right half
///
/// The key insight is that split inversions can be counted during the merge operation:
/// - When merging, if we choose an element from the right half, it means it's smaller
///   than all remaining elements in the left half
/// - Each such choice contributes (remaining left elements) inversions
///
/// # Type Parameters
///
/// * `T` - The type of elements in the slice. Must implement `Ord` for comparison
///   and `Clone` for copying to the auxiliary array.
///
/// # Arguments
///
/// * `arr` - A slice to count inversions in (not modified)
///
/// # Returns
///
/// The total number of inversions in the array
///
/// # Examples
///
/// ```
/// use algs4_sorting::inversions;
///
/// // Sorted array - no inversions
/// let sorted = vec![1, 2, 3, 4, 5];
/// assert_eq!(inversions::count(&sorted), 0);
///
/// // Reverse sorted - maximum inversions
/// let reversed = vec![5, 4, 3, 2, 1];
/// assert_eq!(inversions::count(&reversed), 10); // n(n-1)/2 = 5*4/2 = 10
///
/// // Partially sorted
/// let data = vec![2, 4, 1, 3, 5];
/// assert_eq!(inversions::count(&data), 3); // (2,1), (4,1), (4,3)
/// ```
///
/// # Performance
///
/// - Time: O(n log n) - uses divide-and-conquer like mergesort
/// - Space: O(n) - requires auxiliary array
///
/// This is much faster than the naive O(n²) approach of checking all pairs,
/// especially for large arrays.
///
/// # Notes
///
/// - The input array is not modified
/// - Equal elements are not considered inversions (strict inequality)
/// - Works with any type that implements Ord and Clone
pub fn count<T: Ord + Clone>(arr: &[T]) -> usize {
    let n = arr.len();
    if n <= 1 {
        return 0;
    }

    // Create a mutable copy for the mergesort process
    let mut arr_copy = arr.to_vec();
    let mut aux = arr.to_vec();

    count_recursive(&mut arr_copy, &mut aux, 0, n - 1)
}

/// Recursively counts inversions in arr[lo..hi].
///
/// This function follows the mergesort pattern but accumulates inversion counts.
/// It counts inversions in three parts:
/// 1. Inversions in the left half (recursive call)
/// 2. Inversions in the right half (recursive call)
/// 3. Split inversions (counted during merge)
///
/// # Arguments
///
/// * `arr` - The array being processed
/// * `aux` - Auxiliary array for merging
/// * `lo` - Low index of the subarray (inclusive)
/// * `hi` - High index of the subarray (inclusive)
///
/// # Returns
///
/// The number of inversions in arr[lo..hi]
fn count_recursive<T: Ord + Clone>(arr: &mut [T], aux: &mut [T], lo: usize, hi: usize) -> usize {
    // Base case: single element has no inversions
    if lo >= hi {
        return 0;
    }

    // Divide: find the midpoint
    let mid = lo + (hi - lo) / 2;

    // Conquer: count inversions in each half
    let left_inversions = count_recursive(arr, aux, lo, mid);
    let right_inversions = count_recursive(arr, aux, mid + 1, hi);

    // Combine: count split inversions and merge
    let split_inversions = merge_and_count(arr, aux, lo, mid, hi);

    left_inversions + right_inversions + split_inversions
}

/// Merges two sorted subarrays and counts split inversions.
///
/// This is the key operation that counts inversions across the two halves.
/// When we choose an element from the right half during merging, it means
/// that element is smaller than all remaining elements in the left half,
/// creating inversions with each of them.
///
/// # Algorithm
///
/// 1. Copy arr[lo..hi] to aux[lo..hi]
/// 2. Merge back to arr, similar to standard merge
/// 3. When choosing from right half:
///    - Count inversions: all remaining elements in left half are inverted
///    - Add (mid - i + 1) to inversion count
///
/// # Arguments
///
/// * `arr` - The array being sorted/counted
/// * `aux` - Auxiliary array for merging
/// * `lo` - Low index (inclusive)
/// * `mid` - Middle index (end of first sorted subarray)
/// * `hi` - High index (inclusive)
///
/// # Returns
///
/// The number of split inversions (inversions across the two halves)
fn merge_and_count<T: Ord + Clone>(
    arr: &mut [T],
    aux: &mut [T],
    lo: usize,
    mid: usize,
    hi: usize,
) -> usize {
    let mut inversions = 0;

    // Copy to auxiliary array
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
            // Right element is smaller - this creates inversions!
            // All remaining elements in left half (from i to mid) are greater than aux[j]
            arr[k] = aux[j].clone();
            inversions += mid - i + 1;
            j += 1;
        } else {
            // Left element is smaller or equal - no new inversions
            arr[k] = aux[i].clone();
            i += 1;
        }
        k += 1;
    }

    inversions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        let arr: Vec<i32> = vec![];
        assert_eq!(count(&arr), 0);
    }

    #[test]
    fn test_single_element() {
        let arr = vec![42];
        assert_eq!(count(&arr), 0);
    }

    #[test]
    fn test_two_elements_sorted() {
        let arr = vec![1, 2];
        assert_eq!(count(&arr), 0);
    }

    #[test]
    fn test_two_elements_inverted() {
        let arr = vec![2, 1];
        assert_eq!(count(&arr), 1);
    }

    #[test]
    fn test_sorted_array() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(count(&arr), 0);
    }

    #[test]
    fn test_reverse_sorted_array() {
        // Reverse sorted has maximum inversions: n(n-1)/2
        let arr = vec![5, 4, 3, 2, 1];
        assert_eq!(count(&arr), 10); // 5*4/2 = 10
    }

    #[test]
    fn test_reverse_sorted_four() {
        let arr = vec![4, 3, 2, 1];
        assert_eq!(count(&arr), 6); // 4*3/2 = 6
    }

    #[test]
    fn test_known_inversions() {
        // Array: [2, 4, 1, 3, 5]
        // Inversions: (2,1), (4,1), (4,3)
        let arr = vec![2, 4, 1, 3, 5];
        assert_eq!(count(&arr), 3);
    }

    #[test]
    fn test_known_inversions_2() {
        // Array: [5, 3, 2, 4, 1]
        // Inversions:
        // (5,3), (5,2), (5,4), (5,1)
        // (3,2), (3,1)
        // (2,1)
        // (4,1)
        // Total: 8
        let arr = vec![5, 3, 2, 4, 1];
        assert_eq!(count(&arr), 8);
    }

    #[test]
    fn test_duplicates() {
        // Equal elements don't form inversions
        let arr = vec![3, 1, 3, 1];
        // Inversions: (3,1) at positions (0,1), (3,1) at positions (0,3),
        //             (3,1) at positions (2,3)
        // Total: 3
        assert_eq!(count(&arr), 3);
    }

    #[test]
    fn test_all_same() {
        let arr = vec![5, 5, 5, 5, 5];
        assert_eq!(count(&arr), 0);
    }

    #[test]
    fn test_three_elements() {
        let arr = vec![3, 2, 1];
        assert_eq!(count(&arr), 3); // 3*2/2 = 3

        let arr = vec![1, 3, 2];
        assert_eq!(count(&arr), 1); // Only (3,2)

        let arr = vec![2, 1, 3];
        assert_eq!(count(&arr), 1); // Only (2,1)
    }

    #[test]
    fn test_large_sorted_array() {
        let arr: Vec<i32> = (0..1000).collect();
        assert_eq!(count(&arr), 0);
    }

    #[test]
    fn test_large_reverse_sorted_array() {
        let arr: Vec<i32> = (0..100).rev().collect();
        // n(n-1)/2 = 100*99/2 = 4950
        assert_eq!(count(&arr), 4950);
    }

    #[test]
    fn test_partially_sorted() {
        let arr = vec![1, 3, 5, 2, 4, 6];
        // Inversions: (3,2), (5,2), (5,4)
        assert_eq!(count(&arr), 3);
    }

    #[test]
    fn test_strings() {
        let arr = vec!["dog", "cat", "zebra", "ant"];
        // Inversions: (dog,cat), (dog,ant), (cat,ant), (zebra,ant)
        // dog > cat, dog > ant, cat > ant, zebra > ant
        // That's 4 inversions
        assert_eq!(count(&arr), 4);
    }

    #[test]
    fn test_chars() {
        let arr = vec!['d', 'c', 'b', 'a'];
        // Reverse sorted: 4*3/2 = 6
        assert_eq!(count(&arr), 6);
    }

    #[test]
    fn test_negative_numbers() {
        let arr = vec![3, 1, -2, 4, 0];
        // Inversions: (3,1), (3,-2), (3,0), (1,-2), (1,0), (4,0)
        assert_eq!(count(&arr), 6);
    }

    #[test]
    fn test_power_of_two_size() {
        let arr: Vec<i32> = (0..256).rev().collect();
        // n(n-1)/2 = 256*255/2 = 32640
        assert_eq!(count(&arr), 32640);
    }

    #[test]
    fn test_single_inversion_at_start() {
        let arr = vec![2, 1, 3, 4, 5];
        assert_eq!(count(&arr), 1);
    }

    #[test]
    fn test_single_inversion_at_end() {
        let arr = vec![1, 2, 3, 5, 4];
        assert_eq!(count(&arr), 1);
    }

    #[test]
    fn test_alternating() {
        let arr = vec![2, 1, 4, 3, 6, 5];
        // Inversions: (2,1), (4,3), (6,5)
        assert_eq!(count(&arr), 3);
    }

    #[test]
    fn test_nearly_sorted() {
        // Each element is at most 1 position away from correct position
        let arr = vec![1, 3, 2, 4, 6, 5, 7];
        // Inversions: (3,2), (6,5)
        assert_eq!(count(&arr), 2);
    }
}
