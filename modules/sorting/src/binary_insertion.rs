//! Binary insertion sort implementation.
//!
//! Binary insertion sort is a variation of insertion sort that uses binary search
//! to find the insertion position. This reduces the number of comparisons from O(n²)
//! to O(n log n), but the overall time complexity remains O(n²) because shifting
//! elements still requires O(n²) operations.
//!
//! This algorithm is useful when comparisons are expensive but data movement is cheap.
//! For example, when sorting objects with complex comparison operations.
//!
//! # Examples
//!
//! ```
//! use algs4_sorting::binary_insertion;
//!
//! let mut data = vec![64, 25, 12, 22, 11];
//! binary_insertion::sort(&mut data);
//! assert_eq!(data, vec![11, 12, 22, 25, 64]);
//! ```
//!
//! # Performance
//!
//! * Time Complexity:
//!   - Comparisons: O(n log n) - binary search for insertion position
//!   - Data movement: O(n²) - shifting elements for insertion
//!   - Overall: O(n²) due to element shifting
//! * Space Complexity: O(1) - in-place sorting
//! * Stable: equal elements retain their relative order
//! * Adaptive: performs well on partially sorted arrays
//!
//! # Comparison with Standard Insertion Sort
//!
//! - Standard insertion sort: O(n²) comparisons, O(n²) data movement
//! - Binary insertion sort: O(n log n) comparisons, O(n²) data movement
//!
//! Binary insertion sort is advantageous when:
//! - Comparisons are significantly more expensive than data movement
//! - The type being sorted has a complex comparison operation
//!
//! **Reference:** <https://algs4.cs.princeton.edu/21elementary>

/// Sorts a slice in ascending order using binary insertion sort.
///
/// Binary insertion sort improves upon standard insertion sort by using binary search
/// to find the correct insertion position. This reduces the number of comparisons but
/// does not improve the asymptotic time complexity because shifting elements still
/// requires O(n²) operations.
///
/// # Algorithm
///
/// For each position i from 1 to n-1:
/// 1. Use binary search to find the correct insertion position in arr[0..i]
/// 2. Shift all elements from the insertion position to i-1 one position to the right
/// 3. Insert the element at the found position
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
/// use algs4_sorting::binary_insertion;
///
/// let mut numbers = vec![5, 2, 8, 1, 9];
/// binary_insertion::sort(&mut numbers);
/// assert_eq!(numbers, vec![1, 2, 5, 8, 9]);
///
/// let mut chars = vec!['d', 'a', 'c', 'b'];
/// binary_insertion::sort(&mut chars);
/// assert_eq!(chars, vec!['a', 'b', 'c', 'd']);
/// ```
///
/// # Performance
///
/// - Comparisons: O(n log n) - each insertion requires at most log i comparisons
/// - Data movement: O(n²) - shifting elements in worst case
/// - Space: O(1) - sorts in place
///
/// # Notes
///
/// This algorithm is particularly useful when the comparison operation is expensive
/// relative to data movement. For types with simple comparisons (like integers),
/// standard insertion sort or the optimized insertion_x may be faster due to better
/// cache locality and fewer overhead.
pub fn sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();

    // For each element starting from the second one
    for i in 1..n {
        // Use binary search to find the insertion position
        let insert_pos = binary_search_insertion_point(arr, i);

        // Rotate the element at position i to the insertion position
        // This effectively shifts all elements from insert_pos to i-1 one position right
        // and places arr[i] at insert_pos
        if insert_pos < i {
            arr[insert_pos..=i].rotate_right(1);
        }
    }
}

/// Finds the insertion point for arr[end] in the sorted subarray arr[0..end].
///
/// Uses binary search to find the leftmost position where arr[end] should be inserted
/// to maintain sorted order. This ensures the sort is stable (equal elements maintain
/// their relative order).
///
/// # Arguments
///
/// * `arr` - The array containing a sorted prefix arr[0..end] and the element to insert at arr[end]
/// * `end` - The index of the element to be inserted
///
/// # Returns
///
/// The index where arr[end] should be inserted to maintain sorted order.
///
/// # Examples
///
/// For array [1, 3, 5, 2] with end=3, returns 1 (insert position for 2)
/// For array [1, 3, 5, 7] with end=3, returns 3 (already in correct position)
fn binary_search_insertion_point<T: Ord>(arr: &[T], end: usize) -> usize {
    let key = &arr[end];
    let mut left = 0;
    let mut right = end;

    // Binary search for the leftmost position where key should be inserted
    while left < right {
        let mid = left + (right - left) / 2;

        if arr[mid] > *key {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left
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

    #[test]
    fn test_stability() {
        // Test that equal elements maintain their relative order
        #[derive(Debug, PartialEq, Eq)]
        struct Item {
            key: i32,
            index: usize,
        }

        impl Ord for Item {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.key.cmp(&other.key)
            }
        }

        impl PartialOrd for Item {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        let mut arr = vec![
            Item { key: 3, index: 0 },
            Item { key: 1, index: 1 },
            Item { key: 3, index: 2 },
            Item { key: 2, index: 3 },
            Item { key: 3, index: 4 },
        ];

        sort(&mut arr);

        // Check that items with key=3 maintain their relative order (0, 2, 4)
        assert_eq!(arr[2].index, 0);
        assert_eq!(arr[3].index, 2);
        assert_eq!(arr[4].index, 4);
    }

    #[test]
    fn test_binary_search_insertion_point() {
        let arr = vec![1, 3, 5, 7, 2];
        assert_eq!(binary_search_insertion_point(&arr, 4), 1);

        let arr = vec![1, 3, 5, 7, 9];
        assert_eq!(binary_search_insertion_point(&arr, 4), 4);

        let arr = vec![2, 4, 6, 8, 1];
        assert_eq!(binary_search_insertion_point(&arr, 4), 0);
    }
}
