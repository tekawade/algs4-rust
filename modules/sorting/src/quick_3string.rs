//! 3-way string quicksort implementation.
//!
//! 3-way string quicksort is a hybrid algorithm that combines the ideas of quicksort's
//! divide-and-conquer approach with MSD radix sort's character-by-character examination.
//! It uses 3-way partitioning on the character at the current position, making it
//! particularly efficient for strings with long common prefixes or many duplicate values.
//!
//! Unlike standard quicksort which compares entire strings, 3-way string quicksort
//! examines strings character by character, avoiding redundant comparisons of known-equal
//! prefixes. This makes it adaptive to the structure of the input data.
//!
//! # Examples
//!
//! ```
//! use algs4_sorting::quick_3string;
//!
//! let mut data = vec![
//!     "she".to_string(),
//!     "sells".to_string(),
//!     "seashells".to_string(),
//!     "by".to_string(),
//!     "the".to_string(),
//!     "sea".to_string(),
//! ];
//! quick_3string::sort(&mut data);
//! assert_eq!(data, vec!["by", "sea", "seashells", "sells", "she", "the"]);
//! ```
//!
//! # Performance
//!
//! * Time Complexity:
//!   - Best case: O(n) - when all strings are equal
//!   - Average case: O(n log n) - for random strings
//!   - Worst case: O(nW) - when strings are in order or reverse order
//! * Space Complexity: O(W + log n) - for recursion stack
//! * Not stable: in-place partitioning does not preserve relative order
//! * Handles variable-length strings naturally
//! * Very efficient for strings with long common prefixes or many duplicates
//!
//! **Reference:** <https://algs4.cs.princeton.edu/51radix>

/// Sorts an array of variable-length strings using 3-way string quicksort.
///
/// 3-way string quicksort partitions strings based on the character at the current
/// position, dividing the array into three groups:
/// - Strings with smaller character at position d
/// - Strings with equal character at position d
/// - Strings with larger character at position d
///
/// The algorithm then recursively sorts the left and right groups at the same position,
/// and the middle group at the next position (since their characters at d are equal).
///
/// # Algorithm
///
/// For current character position d:
/// 1. Choose pivot character (from first string at position d)
/// 2. Use 3-way partitioning to divide array:
///    - arr[lo..lt]: char_at(d) < pivot
///    - arr[lt..=gt]: char_at(d) == pivot
///    - arr[gt+1..=hi]: char_at(d) > pivot
/// 3. Recursively sort left partition at position d
/// 4. Recursively sort middle partition at position d+1
/// 5. Recursively sort right partition at position d
///
/// # Arguments
///
/// * `arr` - A mutable slice of strings to be sorted
///
/// # Examples
///
/// ```
/// use algs4_sorting::quick_3string;
///
/// let mut strings = vec![
///     "she".to_string(),
///     "sells".to_string(),
///     "seashells".to_string(),
///     "by".to_string(),
///     "the".to_string(),
///     "sea".to_string(),
///     "shore".to_string(),
///     "surely".to_string(),
/// ];
/// quick_3string::sort(&mut strings);
///
/// // Verify sorted order
/// for i in 0..strings.len() - 1 {
///     assert!(strings[i] <= strings[i + 1]);
/// }
/// ```
///
/// # Performance
///
/// - Time: O(n log n) average for random strings
/// - Space: O(W + log n) for recursion stack
/// - Not stable: equal strings may be rearranged
/// - Adaptive: faster for strings with common prefixes or duplicates
///
/// # Notes
///
/// 3-way string quicksort advantages:
/// - Excellent average-case performance
/// - Adaptive to input structure (prefixes, duplicates)
/// - In-place sorting (no auxiliary array)
/// - Handles variable-length strings naturally
/// - Avoids redundant character comparisons
/// - Linear time when all strings are equal
///
/// 3-way string quicksort vs alternatives:
/// - Faster than standard quicksort for string data
/// - More space-efficient than standard MSD radix sort
/// - Generally faster than MSD for random strings
/// - Slower than LSD for fixed-length strings
/// - Better than MSD when strings have few common prefixes
pub fn sort(arr: &mut [String]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    sort_recursive(arr, 0, n - 1, 0);
}

/// Recursively sorts the subarray arr[lo..=hi] starting at character position d.
///
/// Uses 3-way partitioning on the character at position d, then recursively sorts
/// the three resulting partitions appropriately.
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
    // Left partition: strings with smaller character at position d
    if lt > 0 && lt > lo {
        sort_recursive(arr, lo, lt - 1, d);
    }

    // Middle partition: strings with equal character at position d
    // Move to next character position (d+1), but only if pivot is not -1 (end-of-string)
    if pivot >= 0 && lt <= gt {
        sort_recursive(arr, lt, gt, d + 1);
    }

    // Right partition: strings with larger character at position d
    if gt < hi {
        sort_recursive(arr, gt + 1, hi, d);
    }
}

/// Returns the character at position d in string s, or -1 if d >= s.len().
///
/// This helper function allows 3-way string quicksort to handle variable-length strings
/// by treating the end of a string as a special character (-1) that sorts before all
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
    fn test_many_duplicates() {
        // 3-way string quicksort excels with duplicates
        let mut arr = vec![
            "dog".to_string(),
            "cat".to_string(),
            "dog".to_string(),
            "bird".to_string(),
            "cat".to_string(),
            "dog".to_string(),
            "cat".to_string(),
        ];
        sort(&mut arr);
        assert_eq!(arr, vec!["bird", "cat", "cat", "cat", "dog", "dog", "dog"]);
    }

    #[test]
    fn test_all_duplicates() {
        // Best case: linear time
        let mut arr = vec![
            "hello".to_string(),
            "hello".to_string(),
            "hello".to_string(),
            "hello".to_string(),
        ];
        sort(&mut arr);
        assert_eq!(arr, vec!["hello", "hello", "hello", "hello"]);
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
    fn test_prefixes() {
        // Strings that are prefixes of other strings
        let mut arr = vec![
            "testing".to_string(),
            "test".to_string(),
            "tester".to_string(),
            "te".to_string(),
        ];
        sort(&mut arr);
        assert_eq!(arr, vec!["te", "test", "tester", "testing"]);
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
    fn test_many_common_prefixes() {
        // Should be very efficient due to common prefixes
        let mut arr = vec![
            "algorithm".to_string(),
            "algorithmic".to_string(),
            "algorithms".to_string(),
            "algo".to_string(),
            "algebra".to_string(),
        ];
        sort(&mut arr);
        assert_eq!(
            arr,
            vec!["algebra", "algo", "algorithm", "algorithmic", "algorithms"]
        );
    }

    #[test]
    fn test_numeric_like_strings() {
        let mut arr = vec![
            "999".to_string(),
            "111".to_string(),
            "555".to_string(),
            "222".to_string(),
        ];
        sort(&mut arr);
        assert_eq!(arr, vec!["111", "222", "555", "999"]);
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
