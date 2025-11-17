//! Shellsort implementation.
//!
//! Shellsort is an in-place comparison sort that generalizes insertion sort to allow
//! the exchange of items that are far apart. The algorithm performs a sequence of
//! interleaved insertion sorts based on a gap sequence. By starting with large gaps
//! and progressively reducing them, shellsort can move elements closer to their final
//! positions more efficiently than insertion sort.
//!
//! This implementation uses the 3x+1 increment sequence (Knuth's sequence):
//! 1, 4, 13, 40, 121, 364, 1093, ...
//! where each term is computed as h = 3*h + 1
//!
//! # Examples
//!
//! ```
//! use algs4_sorting::shell;
//!
//! let mut data = vec![64, 25, 12, 22, 11];
//! shell::sort(&mut data);
//! assert_eq!(data, vec![11, 12, 22, 25, 64]);
//! ```
//!
//! # Performance
//!
//! * Time Complexity:
//!   - Best case: O(n log n)
//!   - Average case: depends on gap sequence; O(n^(3/2)) for 3x+1 sequence
//!   - Worst case: O(n^(3/2)) for 3x+1 sequence
//! * Space Complexity: O(1) - in-place sorting
//! * Not stable: equal elements may not retain their relative order
//! * More efficient than insertion sort for medium to large arrays
//!
//! # Gap Sequence
//!
//! The choice of gap sequence significantly affects performance. The 3x+1 sequence
//! (Knuth's sequence) is one of the best-known sequences and provides good practical
//! performance with a proven worst-case bound of O(n^(3/2)).
//!
//! Other gap sequences exist (Sedgewick, Tokuda, etc.) but the 3x+1 sequence is
//! simple, efficient, and well-studied.
//!
//! **Reference:** <https://algs4.cs.princeton.edu/21elementary>

/// Sorts a slice in ascending order using shellsort with the 3x+1 increment sequence.
///
/// Shellsort improves upon insertion sort by comparing and swapping elements that are
/// far apart, then progressively reducing the gap between compared elements. This allows
/// elements to reach their final positions more quickly than with simple adjacent swaps.
///
/// # Algorithm
///
/// 1. Compute the gap sequence: start with the largest gap h such that h < n/3
/// 2. For each gap h (in decreasing order: ..., 121, 40, 13, 4, 1):
///    - Perform insertion sort on elements h positions apart (h-sort)
///    - This creates h interleaved sorted subsequences
/// 3. When h = 1, the final pass is a standard insertion sort
///    - However, the array is now mostly sorted, making this pass very efficient
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
/// use algs4_sorting::shell;
///
/// let mut numbers = vec![5, 2, 8, 1, 9];
/// shell::sort(&mut numbers);
/// assert_eq!(numbers, vec![1, 2, 5, 8, 9]);
///
/// let mut chars = vec!['d', 'a', 'c', 'b'];
/// shell::sort(&mut chars);
/// assert_eq!(chars, vec!['a', 'b', 'c', 'd']);
///
/// // Shellsort is efficient for larger arrays
/// let mut large = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
/// shell::sort(&mut large);
/// assert_eq!(large, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
/// ```
///
/// # Performance
///
/// - Time: O(n^(3/2)) for the 3x+1 increment sequence
/// - Space: O(1) - sorts in place
/// - Much faster than insertion sort for larger arrays
/// - The exact number of comparisons depends on the input and gap sequence
///
/// # Notes
///
/// Shellsort is particularly useful for:
/// - Medium-sized arrays (hundreds to thousands of elements)
/// - Embedded systems where O(n log n) algorithms use too much memory
/// - Situations where a simple, efficient algorithm is needed
///
/// It's faster than insertion sort and selection sort, but typically slower than
/// quicksort, mergesort, or heapsort for large arrays. However, its simplicity and
/// in-place nature make it attractive for certain use cases.
pub fn sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();

    // Handle trivial cases
    if n <= 1 {
        return;
    }

    // Compute the starting gap using the 3x+1 sequence
    // Find the largest h such that h < n/3
    let mut h = 1;
    while h < n / 3 {
        h = 3 * h + 1; // 1, 4, 13, 40, 121, 364, 1093, ...
    }

    // h-sort the array for decreasing values of h
    while h >= 1 {
        // Perform insertion sort on elements h positions apart
        h_sort(arr, h);

        // Move to the next smaller gap in the sequence
        h /= 3;
    }
}

/// Performs insertion sort on elements that are h positions apart.
///
/// This is the core operation of shellsort. It performs an insertion sort on
/// every h-th element, creating h interleaved sorted subsequences.
///
/// # Arguments
///
/// * `arr` - The array to h-sort
/// * `h` - The gap between elements to compare
///
/// # Algorithm
///
/// For each element starting at position h:
/// - Insert arr[i] into the sorted sequence arr[i-h], arr[i-2h], arr[i-3h], ...
/// - This is like insertion sort, but with gap h instead of 1
fn h_sort<T: Ord>(arr: &mut [T], h: usize) {
    let n = arr.len();

    // Start from h and move right, inserting each element into its h-sorted position
    for i in h..n {
        let mut j = i;

        // Move arr[j] to its correct position in the h-sorted sequence
        // by swapping with elements h positions apart
        while j >= h && arr[j] < arr[j - h] {
            arr.swap(j, j - h);
            j -= h;
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
    fn test_very_large_array() {
        // Test with a larger array to show shellsort's efficiency
        let mut arr: Vec<i32> = (0..1000).rev().collect();
        let expected: Vec<i32> = (0..1000).collect();
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
    fn test_three_elements() {
        let mut arr = vec![3, 1, 2];
        sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3]);

        let mut arr = vec![1, 3, 2];
        sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3]);
    }

    #[test]
    fn test_medium_array() {
        // Test with medium-sized array where shellsort shows its advantages
        let mut arr: Vec<i32> = (0..500).rev().collect();
        let expected: Vec<i32> = (0..500).collect();
        sort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_random_medium() {
        let mut arr = vec![
            15, 3, 9, 8, 5, 2, 7, 1, 6, 4, 12, 10, 11, 14, 13, 20, 18, 19, 16, 17,
        ];
        let mut expected = arr.clone();
        expected.sort();
        sort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_h_sort_basic() {
        let mut arr = vec![5, 2, 8, 1, 9, 3, 7, 4];
        h_sort(&mut arr, 4);
        // After 4-sorting: arr[0,4], arr[1,5], arr[2,6], arr[3,7] are each sorted
        assert!(arr[0] <= arr[4]);
        assert!(arr[1] <= arr[5]);
        assert!(arr[2] <= arr[6]);
        assert!(arr[3] <= arr[7]);
    }
}
