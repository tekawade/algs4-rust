//! Insertion sort implementation.
//!
//! Insertion sort is a simple sorting algorithm that builds the final sorted array
//! one item at a time. It is much more efficient than other O(n²) algorithms like
//! selection sort when dealing with nearly sorted data.
//!
//! The algorithm works similar to how most people sort playing cards in their hands.
//! We start with an empty left hand and the cards face down on the table. We then
//! remove one card at a time from the table and insert it into the correct position
//! in the left hand.
//!
//! # Examples
//!
//! ```
//! use algs4_sorting::insertion;
//!
//! let mut data = vec![64, 25, 12, 22, 11];
//! insertion::sort(&mut data);
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
//! * Number of comparisons: between n-1 (best) and ~n²/2 (worst)
//! * Number of exchanges: between 0 (best) and ~n²/2 (worst)
//!
//! **Reference:** <https://algs4.cs.princeton.edu/21elementary>

/// Sorts a slice in ascending order using insertion sort.
///
/// Insertion sort works by taking elements one at a time and inserting them
/// into their correct position in the already-sorted portion of the array.
/// For each element, it shifts all larger elements to the right to make room.
///
/// # Algorithm
///
/// For each position i from 1 to n-1:
/// 1. Consider element at position i as the key to be inserted
/// 2. Compare key with elements to its left (j = i-1, i-2, ..., 0)
/// 3. Shift all elements greater than key one position to the right
/// 4. Insert key at its correct position
///
/// The algorithm has early termination: when we find an element that is not
/// greater than the key, we know we've found the correct position and can stop.
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
/// use algs4_sorting::insertion;
///
/// let mut numbers = vec![5, 2, 8, 1, 9];
/// insertion::sort(&mut numbers);
/// assert_eq!(numbers, vec![1, 2, 5, 8, 9]);
///
/// let mut chars = vec!['d', 'a', 'c', 'b'];
/// insertion::sort(&mut chars);
/// assert_eq!(chars, vec!['a', 'b', 'c', 'd']);
/// ```
///
/// # Performance
///
/// - Time: O(n²) worst case, O(n) best case (already sorted)
/// - Space: O(1) - sorts in place with only a constant amount of extra space
/// - Comparisons: between n-1 (sorted) and ~n²/2 (reverse sorted)
/// - Exchanges: between 0 (sorted) and ~n²/2 (reverse sorted)
///
/// # Notes
///
/// Insertion sort is particularly efficient for:
/// - Small arrays (often used as a subroutine in more complex algorithms)
/// - Nearly sorted arrays (adaptive - runs in linear time on sorted input)
/// - Online algorithms (can sort a list as it receives it)
///
/// It is stable (preserves the relative order of equal elements) and adaptive
/// (takes advantage of existing order in the data).
pub fn sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();

    // For each element starting from the second one
    for i in 1..n {
        // Insert arr[i] into the sorted portion arr[0..i]
        let mut j = i;

        // Shift elements to the right while they are greater than arr[j]
        // and we haven't reached the beginning of the array
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

/// Sorts a slice in ascending order using insertion sort (alternative implementation).
///
/// This is an alternative implementation that may be more efficient in some cases
/// as it reduces the number of swaps by doing a single insertion instead of
/// multiple swaps. However, it requires the element type to be `Clone`.
///
/// # Type Parameters
///
/// * `T` - The type of elements in the slice. Must implement `Ord` and `Clone`.
///
/// # Arguments
///
/// * `arr` - A mutable slice to be sorted
///
/// # Note
///
/// This function is private and used only for testing alternative implementation strategies.
#[allow(dead_code)]
fn sort_optimized<T: Ord + Clone>(arr: &mut [T]) {
    let n = arr.len();

    for i in 1..n {
        let key = arr[i].clone();
        let mut j = i;

        // Shift elements to the right while they are greater than the key
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1].clone();
            j -= 1;
        }

        // Insert the key at its correct position
        arr[j] = key;
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

    #[test]
    fn test_nearly_sorted() {
        // Insertion sort should be efficient on nearly sorted data
        let mut arr = vec![1, 2, 3, 5, 4, 6, 7, 8, 9, 10];
        sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_optimized_version() {
        let mut arr = vec![5, 2, 8, 1, 9];
        sort_optimized(&mut arr);
        assert_eq!(arr, vec![1, 2, 5, 8, 9]);

        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
        sort_optimized(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 3, 4, 5, 5, 6, 9]);
    }
}
