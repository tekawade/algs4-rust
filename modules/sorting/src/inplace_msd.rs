//! In-place MSD radix sort implementation.
//!
//! This is a memory-efficient variant of MSD radix sort that uses 3-way partitioning
//! instead of key-indexed counting, eliminating the need for an auxiliary array.
//! The algorithm partitions the array in-place around the character at each position,
//! similar to 3-way quicksort but examining characters from left to right.
//!
//! This implementation is particularly useful when memory is limited, as it only
//! requires O(W) space for the recursion stack instead of O(n + WR) for standard MSD.
//!
//! # Examples
//!
//! ```
//! use algs4_sorting::inplace_msd;
//!
//! let mut data = vec![
//!     "she".to_string(),
//!     "sells".to_string(),
//!     "seashells".to_string(),
//!     "by".to_string(),
//!     "the".to_string(),
//!     "sea".to_string(),
//! ];
//! inplace_msd::sort(&mut data);
//! assert_eq!(data, vec!["by", "sea", "seashells", "sells", "she", "the"]);
//! ```
//!
//! # Performance
//!
//! * Time Complexity:
//!   - Best case: O(n) - when strings share long common prefixes
//!   - Average case: O(nW) where W = average string length
//!   - Worst case: O(nW) - similar to standard MSD
//! * Space Complexity: O(W) - only recursion stack, no auxiliary array
//! * Not stable: in-place partitioning does not preserve relative order
//! * Handles variable-length strings naturally
//! * More cache-friendly than standard MSD due to in-place operations
//!
//! **Reference:** <https://algs4.cs.princeton.edu/51radix>

/// Sorts an array of variable-length strings using in-place MSD radix sort.
///
/// This variant of MSD radix sort uses 3-way partitioning to sort strings in-place,
/// avoiding the auxiliary array required by standard MSD. It processes strings from
/// left to right (most significant to least significant digit) and recursively sorts
/// subarrays that share the same character at each position.
///
/// # Algorithm
///
/// For each character position d:
/// 1. Use 3-way partitioning to divide array into three groups:
///    - Strings where char_at(d) < pivot character
///    - Strings where char_at(d) == pivot character
///    - Strings where char_at(d) > pivot character
/// 2. Recursively sort the "less than" group at position d
/// 3. Recursively sort the "equal to" group at position d+1
/// 4. Recursively sort the "greater than" group at position d
///
/// # Arguments
///
/// * `arr` - A mutable slice of strings to be sorted
///
/// # Examples
///
/// ```
/// use algs4_sorting::inplace_msd;
///
/// let mut strings = vec![
///     "she".to_string(),
///     "sells".to_string(),
///     "seashells".to_string(),
///     "by".to_string(),
///     "the".to_string(),
///     "sea".to_string(),
///     "shore".to_string(),
/// ];
/// inplace_msd::sort(&mut strings);
///
/// // Verify sorted order
/// for i in 0..strings.len() - 1 {
///     assert!(strings[i] <= strings[i + 1]);
/// }
/// ```
///
/// # Performance
///
/// - Time: O(nW) average case
/// - Space: O(W) for recursion stack only (no auxiliary array)
/// - Not stable: equal strings may be rearranged
/// - More memory-efficient than standard MSD
///
/// # Notes
///
/// In-place MSD advantages:
/// - Minimal space usage (only recursion stack)
/// - No need for auxiliary array
/// - Better cache locality due to in-place operations
/// - Handles variable-length strings naturally
///
/// In-place MSD limitations:
/// - Not stable (doesn't preserve relative order)
/// - More swaps than standard MSD
/// - Can be slower than standard MSD for small alphabets
/// - Recursive implementation requires stack space
pub fn sort(arr: &mut [String]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    sort_recursive(arr, 0, n - 1, 0);
}

/// Recursively sorts the subarray arr[lo..=hi] starting at character position d.
///
/// Uses 3-way partitioning to divide the array based on the character at position d,
/// then recursively sorts each partition. This is the in-place equivalent of the
/// standard MSD algorithm.
///
/// # Arguments
///
/// * `arr` - The array to be sorted
/// * `lo` - Low index of the subarray (inclusive)
/// * `hi` - High index of the subarray (inclusive)
/// * `d` - Current character position being examined
fn sort_recursive(arr: &mut [String], lo: usize, hi: usize, d: usize) {
    // Base case: single element or empty subarray
    if lo >= hi {
        return;
    }

    // 3-way partitioning on character at position d
    let mut lt = lo; // arr[lo..lt] have char_at(d) < pivot
    let mut gt = hi; // arr[gt+1..=hi] have char_at(d) > pivot
    let mut i = lo + 1; // Current position

    let pivot = char_at(&arr[lo], d);

    while i <= gt {
        let c = char_at(&arr[i], d);

        use std::cmp::Ordering;
        match c.cmp(&pivot) {
            Ordering::Less => {
                // arr[i] has smaller character at position d
                arr.swap(lt, i);
                lt += 1;
                i += 1;
            }
            Ordering::Greater => {
                // arr[i] has larger character at position d
                arr.swap(i, gt);
                if gt == 0 {
                    break;
                }
                gt -= 1;
            }
            Ordering::Equal => {
                // arr[i] has same character at position d
                i += 1;
            }
        }
    }

    // Now:
    // arr[lo..lt] have char_at(d) < pivot
    // arr[lt..=gt] have char_at(d) == pivot
    // arr[gt+1..=hi] have char_at(d) > pivot

    // Recursively sort the three partitions
    if lt > 0 && lt > lo {
        sort_recursive(arr, lo, lt - 1, d);
    }

    // For equal partition, move to next character position
    // But only if pivot is not -1 (end-of-string)
    if pivot >= 0 && lt <= gt {
        sort_recursive(arr, lt, gt, d + 1);
    }

    if gt < hi {
        sort_recursive(arr, gt + 1, hi, d);
    }
}

/// Returns the character at position d in string s, or -1 if d >= s.len().
///
/// This helper function allows in-place MSD to handle variable-length strings by
/// treating the end of a string as a special character (-1) that sorts before all
/// actual characters.
///
/// # Arguments
///
/// * `s` - The string to examine
/// * `d` - The character position
///
/// # Returns
///
/// The character at position d as an i32, or -1 if d is beyond the string length
fn char_at(s: &str, d: usize) -> i32 {
    if d < s.len() {
        s.as_bytes()[d] as i32
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        let mut arr: Vec<String> = vec![];
        sort(&mut arr);
        assert_eq!(arr, Vec::<String>::new());
    }

    #[test]
    fn test_single_element() {
        let mut arr = vec!["hello".to_string()];
        sort(&mut arr);
        assert_eq!(arr, vec!["hello"]);
    }

    #[test]
    fn test_variable_length_strings() {
        let mut arr = vec![
            "she".to_string(),
            "sells".to_string(),
            "seashells".to_string(),
            "by".to_string(),
            "the".to_string(),
            "sea".to_string(),
            "shore".to_string(),
        ];
        sort(&mut arr);

        // Verify sorted order
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }

    #[test]
    fn test_different_lengths() {
        let mut arr = vec!["a".to_string(), "aaa".to_string(), "aa".to_string()];
        sort(&mut arr);
        assert_eq!(arr, vec!["a", "aa", "aaa"]);
    }

    #[test]
    fn test_common_prefixes() {
        let mut arr = vec![
            "prefix123".to_string(),
            "prefix1".to_string(),
            "prefix12".to_string(),
            "pre".to_string(),
        ];
        sort(&mut arr);
        assert_eq!(arr, vec!["pre", "prefix1", "prefix12", "prefix123"]);
    }

    #[test]
    fn test_already_sorted() {
        let mut arr = vec!["abc".to_string(), "def".to_string(), "ghi".to_string()];
        sort(&mut arr);
        assert_eq!(arr, vec!["abc", "def", "ghi"]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr = vec!["zzz".to_string(), "mmm".to_string(), "aaa".to_string()];
        sort(&mut arr);
        assert_eq!(arr, vec!["aaa", "mmm", "zzz"]);
    }

    #[test]
    fn test_equal_strings() {
        let mut arr = vec!["same".to_string(), "same".to_string(), "same".to_string()];
        sort(&mut arr);
        assert_eq!(arr, vec!["same", "same", "same"]);
    }

    #[test]
    fn test_two_elements() {
        let mut arr = vec!["bb".to_string(), "aa".to_string()];
        sort(&mut arr);
        assert_eq!(arr, vec!["aa", "bb"]);
    }

    #[test]
    fn test_empty_string() {
        let mut arr = vec!["abc".to_string(), "".to_string(), "def".to_string()];
        sort(&mut arr);
        assert_eq!(arr, vec!["", "abc", "def"]);
    }

    #[test]
    fn test_multiple_empty_strings() {
        let mut arr = vec![
            "abc".to_string(),
            "".to_string(),
            "".to_string(),
            "def".to_string(),
        ];
        sort(&mut arr);
        assert_eq!(arr, vec!["", "", "abc", "def"]);
    }

    #[test]
    fn test_single_character() {
        let mut arr = vec!["z".to_string(), "a".to_string(), "m".to_string()];
        sort(&mut arr);
        assert_eq!(arr, vec!["a", "m", "z"]);
    }

    #[test]
    fn test_mixed_lengths() {
        let mut arr = vec![
            "a".to_string(),
            "aa".to_string(),
            "aaa".to_string(),
            "b".to_string(),
            "bb".to_string(),
            "bbb".to_string(),
        ];
        sort(&mut arr);
        assert_eq!(arr, vec!["a", "aa", "aaa", "b", "bb", "bbb"]);
    }

    #[test]
    fn test_duplicates() {
        let mut arr = vec![
            "dog".to_string(),
            "cat".to_string(),
            "dog".to_string(),
            "bird".to_string(),
            "cat".to_string(),
        ];
        sort(&mut arr);
        assert_eq!(arr, vec!["bird", "cat", "cat", "dog", "dog"]);
    }

    #[test]
    fn test_char_at_helper() {
        assert_eq!(char_at("hello", 0), b'h' as i32);
        assert_eq!(char_at("hello", 4), b'o' as i32);
        assert_eq!(char_at("hello", 5), -1);
        assert_eq!(char_at("hello", 100), -1);
        assert_eq!(char_at("", 0), -1);
    }

    #[test]
    fn test_long_strings() {
        let mut arr = vec![
            "verylongstringzzz".to_string(),
            "verylongstringaaa".to_string(),
            "verylongstringmmm".to_string(),
        ];
        sort(&mut arr);
        assert_eq!(
            arr,
            vec![
                "verylongstringaaa",
                "verylongstringmmm",
                "verylongstringzzz"
            ]
        );
    }

    #[test]
    fn test_three_elements() {
        let mut arr = vec!["c".to_string(), "a".to_string(), "b".to_string()];
        sort(&mut arr);
        assert_eq!(arr, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_benchmark_data() {
        let mut arr = vec![
            "she".to_string(),
            "sells".to_string(),
            "seashells".to_string(),
            "by".to_string(),
            "the".to_string(),
            "sea".to_string(),
            "shore".to_string(),
            "the".to_string(),
            "shells".to_string(),
            "she".to_string(),
            "sells".to_string(),
            "are".to_string(),
            "surely".to_string(),
            "seashells".to_string(),
        ];
        sort(&mut arr);

        // Verify sorted
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }

    #[test]
    fn test_many_duplicates() {
        let mut arr = vec![
            "hello".to_string(),
            "hello".to_string(),
            "world".to_string(),
            "hello".to_string(),
            "world".to_string(),
        ];
        sort(&mut arr);
        assert_eq!(arr, vec!["hello", "hello", "hello", "world", "world"]);
    }

    #[test]
    fn test_all_different() {
        let mut arr = vec![
            "zebra".to_string(),
            "apple".to_string(),
            "mango".to_string(),
            "banana".to_string(),
        ];
        sort(&mut arr);
        assert_eq!(arr, vec!["apple", "banana", "mango", "zebra"]);
    }
}
