//! MSD (Most Significant Digit) radix sort implementation.
//!
//! MSD radix sort is a recursive sorting algorithm that sorts variable-length strings
//! by processing characters from left to right (most significant to least significant).
//! Unlike LSD, it can handle strings of different lengths and uses recursion to sort
//! subarrays that share the same prefix.
//!
//! The algorithm uses key-indexed counting as a subroutine and includes an optimization
//! to switch to insertion sort for small subarrays, which improves performance by
//! avoiding the overhead of key-indexed counting for tiny subarrays.
//!
//! # Examples
//!
//! ```
//! use algs4_sorting::msd;
//!
//! let mut data = vec![
//!     "she".to_string(),
//!     "sells".to_string(),
//!     "seashells".to_string(),
//!     "by".to_string(),
//!     "the".to_string(),
//!     "sea".to_string(),
//!     "shore".to_string(),
//! ];
//! msd::sort(&mut data);
//! assert_eq!(data, vec!["by", "sea", "seashells", "sells", "she", "shore", "the"]);
//! ```
//!
//! # Performance
//!
//! * Time Complexity:
//!   - Best case: O(n) - when strings share long common prefixes
//!   - Average case: O(n + WR) where W = average string length, R = radix
//!   - Worst case: O(nW) - when all strings are distinct
//! * Space Complexity: O(n + WR) - for auxiliary arrays and recursion stack
//! * Stable: maintains relative order of equal elements
//! * Not in-place: requires auxiliary array
//! * Handles variable-length strings naturally
//!
//! **Reference:** <https://algs4.cs.princeton.edu/51radix>

const RADIX: usize = 256; // Extended ASCII alphabet size
const CUTOFF: usize = 15; // Cutoff to insertion sort for small subarrays

/// Sorts an array of variable-length strings using MSD radix sort.
///
/// MSD (Most Significant Digit) radix sort processes strings from left to right,
/// recursively sorting subarrays of strings that share the same character at each position.
/// This allows it to handle variable-length strings efficiently.
///
/// # Algorithm
///
/// 1. For current character position d:
///    - Use key-indexed counting to partition by character at position d
///    - Treat end-of-string as less than any character (-1)
/// 2. Recursively sort each partition (strings with same prefix)
/// 3. Optimization: use insertion sort for small subarrays (< CUTOFF)
///
/// The algorithm naturally handles variable-length strings by treating the end of a
/// string as a special character that sorts before all actual characters.
///
/// # Arguments
///
/// * `arr` - A mutable slice of strings to be sorted
///
/// # Examples
///
/// ```
/// use algs4_sorting::msd;
///
/// let mut strings = vec![
///     "she".to_string(),
///     "sells".to_string(),
///     "seashells".to_string(),
///     "by".to_string(),
///     "the".to_string(),
///     "sea".to_string(),
///     "shore".to_string(),
///     "the".to_string(),
///     "seashells".to_string(),
/// ];
/// msd::sort(&mut strings);
/// assert_eq!(strings[0], "by");
/// assert_eq!(strings[8], "the");
/// ```
///
/// # Performance
///
/// - Time: O(n) best case, O(n + WR) average, O(nW) worst case
/// - Space: O(n + WR) for auxiliary arrays and recursion
/// - Stable: equal strings maintain relative order
/// - Adaptive: faster when strings share common prefixes
///
/// # Notes
///
/// MSD radix sort advantages:
/// - Handles variable-length strings naturally
/// - Sublinear time when strings share long prefixes
/// - Examines only as many characters as needed
/// - No comparisons between strings
///
/// MSD radix sort limitations:
/// - Requires significant extra space
/// - Overhead of recursive calls and auxiliary arrays
/// - Can be slower than quicksort for random strings
/// - Many small subarrays can degrade performance
pub fn sort(arr: &mut [String]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    let mut aux = vec![String::new(); n];
    sort_recursive(arr, &mut aux, 0, n - 1, 0);
}

/// Recursively sorts the subarray arr[lo..=hi] starting at character position d.
///
/// This is the main recursive function that implements MSD radix sort.
/// It uses key-indexed counting to partition the array by the character at position d,
/// then recursively sorts each partition.
///
/// # Arguments
///
/// * `arr` - The array to be sorted
/// * `aux` - Auxiliary array for temporary storage
/// * `lo` - Low index of the subarray (inclusive)
/// * `hi` - High index of the subarray (inclusive)
/// * `d` - Current character position being examined
fn sort_recursive(arr: &mut [String], aux: &mut [String], lo: usize, hi: usize, d: usize) {
    // Base case: small subarray or single element
    if hi <= lo {
        return;
    }

    // Optimization: use insertion sort for small subarrays
    if hi - lo < CUTOFF {
        insertion_sort(arr, lo, hi, d);
        return;
    }

    // Key-indexed counting
    let mut count = vec![0; RADIX + 2]; // Extra bucket for end-of-string

    // 1. Count frequencies
    // char_at returns -1 for end-of-string, so we add 2 to map -1 to index 0
    for item in arr.iter().take(hi + 1).skip(lo) {
        let c = char_at(item, d);
        count[(c + 2) as usize] += 1;
    }

    // 2. Transform counts to indices
    for r in 0..=RADIX {
        count[r + 1] += count[r];
    }

    // 3. Distribute to auxiliary array
    for item in arr.iter().take(hi + 1).skip(lo) {
        let c = char_at(item, d);
        let idx = count[(c + 1) as usize];
        aux[idx] = item.clone();
        count[(c + 1) as usize] += 1;
    }

    // 4. Copy back
    arr[lo..=hi].clone_from_slice(&aux[0..=(hi - lo)]);

    // 5. Recursively sort for each character value
    // count[r] now contains the starting index for character r-2
    // We skip r=0 (end-of-string) since those strings are in final position
    for r in 0..RADIX {
        if count[r + 1] > count[r] {
            let subarray_lo = lo + count[r];
            let subarray_hi = lo + count[r + 1] - 1;
            if subarray_hi > subarray_lo {
                sort_recursive(arr, aux, subarray_lo, subarray_hi, d + 1);
            }
        }
    }
}

/// Returns the character at position d in string s, or -1 if d >= s.len().
///
/// This helper function allows MSD to handle variable-length strings by treating
/// the end of a string as a special character (-1) that sorts before all actual characters.
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

/// Insertion sort for strings, starting at character position d.
///
/// This is used as an optimization for small subarrays in MSD radix sort.
/// It only compares characters starting from position d, since characters
/// before d are already known to be equal.
///
/// # Arguments
///
/// * `arr` - The array to sort
/// * `lo` - Low index of the subarray (inclusive)
/// * `hi` - High index of the subarray (inclusive)
/// * `d` - Starting character position for comparisons
fn insertion_sort(arr: &mut [String], lo: usize, hi: usize, d: usize) {
    for i in (lo + 1)..=hi {
        let mut j = i;
        while j > lo && is_less(&arr[j], &arr[j - 1], d) {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

/// Compares two strings starting from character position d.
///
/// Returns true if string v is less than string w when comparing from position d onward.
/// This is used by insertion_sort to avoid redundant comparisons of known-equal prefixes.
///
/// # Arguments
///
/// * `v` - First string
/// * `w` - Second string
/// * `d` - Starting character position
///
/// # Returns
///
/// true if v < w starting from position d, false otherwise
fn is_less(v: &str, w: &str, d: usize) -> bool {
    let v_bytes = v.as_bytes();
    let w_bytes = w.as_bytes();

    let mut i = d;
    while i < v_bytes.len() && i < w_bytes.len() {
        if v_bytes[i] < w_bytes[i] {
            return true;
        }
        if v_bytes[i] > w_bytes[i] {
            return false;
        }
        i += 1;
    }

    // If all compared characters are equal, shorter string is less
    v_bytes.len() < w_bytes.len()
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
        assert_eq!(
            arr,
            vec!["by", "sea", "seashells", "sells", "she", "shore", "the"]
        );
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
    fn test_is_less_helper() {
        assert!(is_less("abc", "abd", 0));
        assert!(!is_less("abd", "abc", 0));
        assert!(is_less("ab", "abc", 0));
        assert!(!is_less("abc", "ab", 0));
        assert!(!is_less("abc", "abc", 0));

        // Test with offset
        assert!(is_less("abc", "abd", 2)); // comparing 'c' vs 'd'
        assert!(!is_less("abd", "abc", 2)); // comparing 'd' vs 'c'
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
    fn test_small_array_uses_insertion_sort() {
        // Array smaller than CUTOFF should use insertion sort
        let mut arr = vec![
            "e".to_string(),
            "d".to_string(),
            "c".to_string(),
            "b".to_string(),
            "a".to_string(),
        ];
        sort(&mut arr);
        assert_eq!(arr, vec!["a", "b", "c", "d", "e"]);
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
}
