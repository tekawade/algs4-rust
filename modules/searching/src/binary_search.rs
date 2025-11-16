//! Binary search algorithm for sorted arrays.
//!
//! The binary search algorithm finds an element in a sorted array in O(log n) time.
//! It works by repeatedly dividing the search interval in half.
//!
//! # Examples
//!
//! ```
//! use algs4_searching::binary_search;
//!
//! let arr = [1, 3, 5, 7, 9, 11, 13];
//! assert_eq!(binary_search(&arr, &7), Some(3));
//! assert_eq!(binary_search(&arr, &6), None);
//! ```
//!
//! # References
//!
//! - Algorithms, 4th Edition: Section 1.1 and 3.1
//! - Original Java: `BinarySearch.java`

/// Returns the index of the specified key in the sorted array.
///
/// This function performs binary search on a sorted slice and returns
/// the index of the key if found, or `None` if not found.
///
/// # Arguments
///
/// * `arr` - A sorted slice to search
/// * `key` - The key to search for
///
/// # Returns
///
/// * `Some(index)` - The index where the key is found
/// * `None` - If the key is not in the array
///
/// # Time Complexity
///
/// O(log n) where n is the length of the array
///
/// # Examples
///
/// ```
/// use algs4_searching::binary_search;
///
/// let arr = [2, 4, 6, 8, 10];
/// assert_eq!(binary_search(&arr, &6), Some(2));
/// assert_eq!(binary_search(&arr, &5), None);
/// assert_eq!(binary_search(&arr, &2), Some(0));
/// assert_eq!(binary_search(&arr, &10), Some(4));
/// ```
pub fn binary_search<T: Ord>(arr: &[T], key: &T) -> Option<usize> {
    let mut lo = 0;
    let mut hi = arr.len();

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match key.cmp(&arr[mid]) {
            std::cmp::Ordering::Less => hi = mid,
            std::cmp::Ordering::Greater => lo = mid + 1,
            std::cmp::Ordering::Equal => return Some(mid),
        }
    }
    None
}

/// Returns the number of elements in the sorted array that are strictly less than the key.
///
/// This function is also known as the "rank" of the key. It can be used to
/// determine where to insert a key to maintain sorted order.
///
/// # Arguments
///
/// * `arr` - A sorted slice
/// * `key` - The key to rank
///
/// # Returns
///
/// The number of elements less than the key (the insertion point)
///
/// # Time Complexity
///
/// O(log n) where n is the length of the array
///
/// # Examples
///
/// ```
/// use algs4_searching::rank;
///
/// let arr = [2, 4, 6, 8, 10];
/// assert_eq!(rank(&arr, &5), 2);  // 2 and 4 are less than 5
/// assert_eq!(rank(&arr, &6), 2);  // 2 and 4 are less than 6
/// assert_eq!(rank(&arr, &1), 0);  // no elements less than 1
/// assert_eq!(rank(&arr, &15), 5); // all elements less than 15
/// ```
pub fn rank<T: Ord>(arr: &[T], key: &T) -> usize {
    let mut lo = 0;
    let mut hi = arr.len();

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match key.cmp(&arr[mid]) {
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => hi = mid,
            std::cmp::Ordering::Greater => lo = mid + 1,
        }
    }
    lo
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search_empty() {
        let arr: &[i32] = &[];
        assert_eq!(binary_search(arr, &5), None);
    }

    #[test]
    fn test_binary_search_single_found() {
        let arr = [5];
        assert_eq!(binary_search(&arr, &5), Some(0));
    }

    #[test]
    fn test_binary_search_single_not_found() {
        let arr = [5];
        assert_eq!(binary_search(&arr, &3), None);
    }

    #[test]
    fn test_binary_search_first() {
        let arr = [1, 3, 5, 7, 9];
        assert_eq!(binary_search(&arr, &1), Some(0));
    }

    #[test]
    fn test_binary_search_last() {
        let arr = [1, 3, 5, 7, 9];
        assert_eq!(binary_search(&arr, &9), Some(4));
    }

    #[test]
    fn test_binary_search_middle() {
        let arr = [1, 3, 5, 7, 9];
        assert_eq!(binary_search(&arr, &5), Some(2));
    }

    #[test]
    fn test_binary_search_not_found() {
        let arr = [1, 3, 5, 7, 9];
        assert_eq!(binary_search(&arr, &6), None);
        assert_eq!(binary_search(&arr, &0), None);
        assert_eq!(binary_search(&arr, &10), None);
    }

    #[test]
    fn test_binary_search_strings() {
        let arr = ["apple", "banana", "cherry", "date", "fig"];
        assert_eq!(binary_search(&arr, &"cherry"), Some(2));
        assert_eq!(binary_search(&arr, &"grape"), None);
    }

    #[test]
    fn test_rank_empty() {
        let arr: &[i32] = &[];
        assert_eq!(rank(arr, &5), 0);
    }

    #[test]
    fn test_rank_single() {
        let arr = [5];
        assert_eq!(rank(&arr, &3), 0);
        assert_eq!(rank(&arr, &5), 0);
        assert_eq!(rank(&arr, &7), 1);
    }

    #[test]
    fn test_rank_multiple() {
        let arr = [2, 4, 6, 8, 10];
        assert_eq!(rank(&arr, &1), 0); // Before all
        assert_eq!(rank(&arr, &2), 0); // Equal to first
        assert_eq!(rank(&arr, &3), 1); // Between 2 and 4
        assert_eq!(rank(&arr, &5), 2); // Between 4 and 6
        assert_eq!(rank(&arr, &10), 4); // Equal to last
        assert_eq!(rank(&arr, &15), 5); // After all
    }

    #[test]
    fn test_rank_duplicates() {
        let arr = [1, 3, 3, 3, 5, 7];
        assert_eq!(rank(&arr, &3), 1); // Points to first occurrence
        assert_eq!(rank(&arr, &4), 4); // Points after last 3
    }

    #[test]
    fn test_binary_search_large() {
        let arr: Vec<i32> = (0..1000).map(|x| x * 2).collect();
        let arr_slice: &[i32] = &arr;

        assert_eq!(binary_search(arr_slice, &500), Some(250));
        assert_eq!(binary_search(arr_slice, &501), None);
    }

    #[test]
    fn test_rank_large() {
        let arr: Vec<i32> = (0..1000).map(|x| x * 2).collect();
        let arr_slice: &[i32] = &arr;

        assert_eq!(rank(arr_slice, &500), 250);
        assert_eq!(rank(arr_slice, &501), 251);
    }
}
