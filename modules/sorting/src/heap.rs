//! Heapsort implementation.
//!
//! Heapsort is a comparison-based sorting algorithm that uses a binary heap data structure.
//! It was invented by J. W. J. Williams in 1964. The algorithm divides its input into
//! a sorted and an unsorted region, and iteratively shrinks the unsorted region by extracting
//! the largest element and moving that to the sorted region.
//!
//! Heapsort works in two phases:
//! 1. **Heap construction**: Build a max-heap from the unordered array (bottom-up)
//! 2. **Sortdown**: Repeatedly remove the maximum element and restore heap property
//!
//! # Examples
//!
//! ```
//! use algs4_sorting::heap;
//!
//! let mut data = vec![64, 25, 12, 22, 11];
//! heap::sort(&mut data);
//! assert_eq!(data, vec![11, 12, 22, 25, 64]);
//! ```
//!
//! # Performance
//!
//! * Time Complexity:
//!   - Best case: O(n log n)
//!   - Average case: O(n log n)
//!   - Worst case: O(n log n) - guaranteed
//! * Space Complexity: O(1) - in-place sorting (only constant extra space)
//! * Not stable: equal elements may not retain their relative order
//! * Not adaptive: always performs the same number of operations
//! * Comparisons: at most 2n lg n + 2n
//! * In-place: sorts within the input array without requiring auxiliary space
//!
//! **Reference:** <https://algs4.cs.princeton.edu/24pq>

/// Sorts a slice in ascending order using heapsort.
///
/// Heapsort is a comparison-based algorithm that uses a binary heap data structure.
/// It guarantees O(n log n) performance with only O(1) auxiliary space, making it
/// an excellent choice when memory is limited.
///
/// # Algorithm
///
/// The algorithm works in two phases:
///
/// 1. **Heap construction phase** - Build a max-heap from the unordered array:
///    - Start from the last parent node (at index n/2 - 1)
///    - Work backwards to the root, sinking each node
///    - This builds a valid max-heap in O(n) time
///
/// 2. **Sortdown phase** - Repeatedly extract maximum and shrink heap:
///    - Swap the root (maximum) with the last element in heap
///    - Decrease heap size by 1
///    - Sink the new root to restore heap property
///    - Repeat until heap size is 1
///
/// # Heap Structure (0-based indexing)
///
/// For an element at index k:
/// - Parent: (k - 1) / 2
/// - Left child: 2k + 1
/// - Right child: 2k + 2
///
/// The heap property: parent >= both children (for max-heap)
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
/// use algs4_sorting::heap;
///
/// let mut numbers = vec![5, 2, 8, 1, 9];
/// heap::sort(&mut numbers);
/// assert_eq!(numbers, vec![1, 2, 5, 8, 9]);
///
/// let mut chars = vec!['d', 'a', 'c', 'b'];
/// heap::sort(&mut chars);
/// assert_eq!(chars, vec!['a', 'b', 'c', 'd']);
/// ```
///
/// # Performance
///
/// - Time: O(n log n) guaranteed in all cases
/// - Space: O(1) - in-place sorting
/// - Comparisons: at most 2n lg n + 2n
/// - Exchanges: at most n lg n + n
///
/// # Notes
///
/// Heapsort advantages:
/// - Guaranteed O(n log n) performance (no worst case)
/// - In-place sorting (O(1) extra space)
/// - No need for recursion (unlike quicksort)
///
/// Heapsort disadvantages:
/// - Not stable (equal elements may be reordered)
/// - Not adaptive (doesn't take advantage of existing order)
/// - Poor cache performance compared to quicksort
/// - Typically slower than quicksort in practice due to poor locality
pub fn sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    // Phase 1: Heap construction
    // Build max-heap by sinking each node from n/2-1 down to 0
    // We start at n/2-1 because nodes after that are leaves (no children)
    for k in (0..n / 2).rev() {
        sink(arr, k, n);
    }

    // Phase 2: Sortdown
    // Repeatedly remove maximum (at root) and restore heap property
    let mut heap_size = n;
    while heap_size > 1 {
        heap_size -= 1;
        // Move maximum to its final position
        arr.swap(0, heap_size);
        // Restore heap property for remaining elements
        sink(arr, 0, heap_size);
    }
}

/// Restores the heap property by sinking a node down the heap.
///
/// This is the fundamental operation for maintaining heap order. When a node
/// is smaller than one or both of its children, it must be exchanged with the
/// larger child and the process repeated until the heap property is restored.
///
/// # Algorithm
///
/// Starting at node k:
/// 1. Find the larger of the two children
/// 2. If parent < larger child, exchange them
/// 3. Move down to that child's position
/// 4. Repeat until node is >= both children or reaches a leaf
///
/// # Arguments
///
/// * `arr` - The array representing the heap
/// * `k` - The index of the node to sink (0-based)
/// * `n` - The heap size (number of elements in the heap)
///
/// # Notes
///
/// This function assumes that the subtrees below k are valid heaps,
/// and only the element at k may violate the heap property.
fn sink<T: Ord>(arr: &mut [T], mut k: usize, n: usize) {
    while 2 * k + 1 < n {
        // While k has at least one child
        let mut j = 2 * k + 1; // Left child

        // Find the larger child
        if j + 1 < n && arr[j] < arr[j + 1] {
            j += 1; // Right child is larger
        }

        // If parent is >= larger child, heap property is satisfied
        if arr[k] >= arr[j] {
            break;
        }

        // Exchange parent with larger child
        arr.swap(k, j);

        // Move down to child's position
        k = j;
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
    fn test_power_of_two_size() {
        // Test with array size that's a power of 2 (complete binary tree)
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
    fn test_three_elements() {
        let mut arr = vec![3, 1, 2];
        sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3]);

        let mut arr = vec![1, 3, 2];
        sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3]);
    }

    #[test]
    fn test_heap_property_after_construction() {
        // Verify that heap construction creates a valid max-heap
        let mut arr = vec![1, 2, 3, 4, 5, 6, 7];
        let n = arr.len();

        // Build heap (same as first phase of sort)
        for k in (0..n / 2).rev() {
            sink(&mut arr, k, n);
        }

        // Check heap property: parent >= children
        for i in 0..n / 2 {
            let left = 2 * i + 1;
            let right = 2 * i + 2;

            if left < n {
                assert!(
                    arr[i] >= arr[left],
                    "Parent at {} should be >= left child at {}",
                    i,
                    left
                );
            }
            if right < n {
                assert!(
                    arr[i] >= arr[right],
                    "Parent at {} should be >= right child at {}",
                    i,
                    right
                );
            }
        }
    }

    #[test]
    fn test_very_large_array() {
        // Test with a larger array to ensure efficiency
        let mut arr: Vec<i32> = (0..10000).rev().collect();
        let expected: Vec<i32> = (0..10000).collect();
        sort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_partially_sorted() {
        let mut arr = vec![1, 3, 2, 5, 4, 7, 6, 9, 8];
        sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
