//! LSD (Least Significant Digit) radix sort implementation.
//!
//! LSD radix sort is a stable, non-comparative sorting algorithm that sorts fixed-length
//! strings by processing characters from right to left (least significant to most significant).
//! It uses key-indexed counting as a subroutine to sort on each character position.
//!
//! The algorithm is particularly efficient for sorting strings of equal length, such as:
//! - Social Security numbers
//! - Phone numbers
//! - License plates
//! - Fixed-width codes
//!
//! # Examples
//!
//! ```
//! use algs4_sorting::lsd;
//!
//! let mut data = vec![
//!     "abc".to_string(),
//!     "xyz".to_string(),
//!     "def".to_string(),
//! ];
//! lsd::sort(&mut data, 3);
//! assert_eq!(data, vec!["abc", "def", "xyz"]);
//! ```
//!
//! # Performance
//!
//! * Time Complexity: O(nW) where n = array size, W = string width
//! * Space Complexity: O(n + R) where R = radix (256 for extended ASCII)
//! * Stable: maintains relative order of equal elements
//! * Not in-place: requires auxiliary array
//! * Ideal for fixed-length strings with small alphabet
//!
//! **Reference:** <https://algs4.cs.princeton.edu/51radix>

const RADIX: usize = 256; // Extended ASCII alphabet size

/// Sorts an array of fixed-length strings using LSD radix sort.
///
/// LSD (Least Significant Digit) radix sort processes strings from right to left,
/// using key-indexed counting to sort on each character position. This ensures
/// stability - strings with equal prefixes maintain their relative order.
///
/// # Algorithm
///
/// For each character position d from right to left (W-1 down to 0):
/// 1. Count frequency of each character at position d
/// 2. Compute cumulative counts to determine starting positions
/// 3. Move strings to auxiliary array based on character at position d
/// 4. Copy back to original array
///
/// The key-indexed counting subroutine is stable, so processing from right to left
/// results in a correctly sorted array.
///
/// # Arguments
///
/// * `arr` - A mutable slice of strings to be sorted (all must have same length)
/// * `w` - The fixed width (length) of all strings
///
/// # Panics
///
/// Panics if any string in the array has length different from `w`.
///
/// # Examples
///
/// ```
/// use algs4_sorting::lsd;
///
/// let mut strings = vec![
///     "bed".to_string(),
///     "bug".to_string(),
///     "dad".to_string(),
///     "yes".to_string(),
///     "zoo".to_string(),
///     "now".to_string(),
///     "for".to_string(),
///     "tip".to_string(),
///     "ilk".to_string(),
///     "dim".to_string(),
///     "tag".to_string(),
///     "jot".to_string(),
///     "sob".to_string(),
///     "nob".to_string(),
///     "sky".to_string(),
///     "hut".to_string(),
/// ];
/// lsd::sort(&mut strings, 3);
/// assert_eq!(strings[0], "bed");
/// assert_eq!(strings[15], "zoo");
/// ```
///
/// # Performance
///
/// - Time: O(nW) where n = number of strings, W = string width
/// - Space: O(n + R) where R = radix (256)
/// - Stable: equal strings maintain relative order
/// - Passes: exactly W passes through the data
///
/// # Notes
///
/// LSD radix sort advantages:
/// - Linear time for fixed-length strings
/// - Stable sort (maintains relative order)
/// - Predictable performance (no worst case)
/// - Simple implementation
///
/// LSD radix sort limitations:
/// - Requires all strings to have the same length
/// - Requires extra space for auxiliary array
/// - Not cache-friendly due to scattered memory access
/// - Only efficient when W (width) is small relative to n
pub fn sort(arr: &mut [String], w: usize) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    // Verify all strings have the required width
    for s in arr.iter() {
        assert_eq!(
            s.len(),
            w,
            "LSD radix sort requires all strings to have the same length"
        );
    }

    let mut aux = vec![String::new(); n];

    // Sort by each character position from right to left
    for d in (0..w).rev() {
        // Key-indexed counting for character at position d
        sort_on_character(arr, &mut aux, d);
    }
}

/// Sorts an array of byte slices using LSD radix sort.
///
/// This variant works with byte slices instead of strings, which can be more
/// efficient when working with raw binary data or when UTF-8 encoding is not needed.
///
/// # Arguments
///
/// * `arr` - A mutable slice of byte slices to be sorted (all must have same length)
/// * `w` - The fixed width (length) of all byte slices
///
/// # Panics
///
/// Panics if any byte slice has length different from `w`.
///
/// # Examples
///
/// ```
/// use algs4_sorting::lsd;
///
/// let mut data = vec![
///     b"xyz".to_vec(),
///     b"abc".to_vec(),
///     b"def".to_vec(),
/// ];
/// lsd::sort_bytes(&mut data, 3);
/// assert_eq!(data, vec![b"abc", b"def", b"xyz"]);
/// ```
pub fn sort_bytes(arr: &mut [Vec<u8>], w: usize) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    // Verify all byte slices have the required width
    for bytes in arr.iter() {
        assert_eq!(
            bytes.len(),
            w,
            "LSD radix sort requires all byte slices to have the same length"
        );
    }

    let mut aux = vec![Vec::new(); n];

    // Sort by each byte position from right to left
    for d in (0..w).rev() {
        sort_bytes_on_position(arr, &mut aux, d);
    }
}

/// Performs key-indexed counting to sort strings based on character at position d.
///
/// This is the core subroutine of LSD radix sort. It stably sorts the array based
/// on the character at position d using the key-indexed counting algorithm:
///
/// 1. Count frequencies of each character value
/// 2. Transform counts to indices (cumulative sum)
/// 3. Distribute strings to auxiliary array
/// 4. Copy back to original array
///
/// # Arguments
///
/// * `arr` - The array to sort
/// * `aux` - Auxiliary array for temporary storage
/// * `d` - The character position to sort on
fn sort_on_character(arr: &mut [String], aux: &mut [String], d: usize) {
    let mut count = vec![0; RADIX + 1];

    // 1. Count frequencies of each character
    for s in arr.iter() {
        let c = s.as_bytes()[d] as usize;
        count[c + 1] += 1;
    }

    // 2. Transform counts to indices (cumulative sum)
    for r in 0..RADIX {
        count[r + 1] += count[r];
    }

    // 3. Distribute strings to auxiliary array
    for s in arr.iter() {
        let c = s.as_bytes()[d] as usize;
        aux[count[c]] = s.clone();
        count[c] += 1;
    }

    // 4. Copy back to original array
    arr.clone_from_slice(aux);
}

/// Performs key-indexed counting to sort byte slices based on byte at position d.
///
/// Similar to `sort_on_character` but works with byte slices instead of strings.
///
/// # Arguments
///
/// * `arr` - The array to sort
/// * `aux` - Auxiliary array for temporary storage
/// * `d` - The byte position to sort on
fn sort_bytes_on_position(arr: &mut [Vec<u8>], aux: &mut [Vec<u8>], d: usize) {
    let mut count = vec![0; RADIX + 1];

    // 1. Count frequencies of each byte value
    for bytes in arr.iter() {
        let c = bytes[d] as usize;
        count[c + 1] += 1;
    }

    // 2. Transform counts to indices (cumulative sum)
    for r in 0..RADIX {
        count[r + 1] += count[r];
    }

    // 3. Distribute byte slices to auxiliary array
    for bytes in arr.iter() {
        let c = bytes[d] as usize;
        aux[count[c]] = bytes.clone();
        count[c] += 1;
    }

    // 4. Copy back to original array
    arr.clone_from_slice(aux);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        let mut arr: Vec<String> = vec![];
        sort(&mut arr, 0);
        assert_eq!(arr, Vec::<String>::new());
    }

    #[test]
    fn test_single_element() {
        let mut arr = vec!["abc".to_string()];
        sort(&mut arr, 3);
        assert_eq!(arr, vec!["abc"]);
    }

    #[test]
    fn test_fixed_width_strings() {
        let mut arr = vec![
            "bed".to_string(),
            "bug".to_string(),
            "dad".to_string(),
            "yes".to_string(),
            "zoo".to_string(),
            "now".to_string(),
            "for".to_string(),
            "tip".to_string(),
            "ilk".to_string(),
            "dim".to_string(),
            "tag".to_string(),
            "jot".to_string(),
            "sob".to_string(),
            "nob".to_string(),
            "sky".to_string(),
            "hut".to_string(),
        ];
        sort(&mut arr, 3);

        assert_eq!(
            arr,
            vec![
                "bed", "bug", "dad", "dim", "for", "hut", "ilk", "jot", "nob", "now", "sky", "sob",
                "tag", "tip", "yes", "zoo"
            ]
        );
    }

    #[test]
    fn test_already_sorted() {
        let mut arr = vec!["abc".to_string(), "def".to_string(), "ghi".to_string()];
        sort(&mut arr, 3);
        assert_eq!(arr, vec!["abc", "def", "ghi"]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr = vec!["zzz".to_string(), "mmm".to_string(), "aaa".to_string()];
        sort(&mut arr, 3);
        assert_eq!(arr, vec!["aaa", "mmm", "zzz"]);
    }

    #[test]
    fn test_equal_strings() {
        let mut arr = vec!["abc".to_string(), "abc".to_string(), "abc".to_string()];
        sort(&mut arr, 3);
        assert_eq!(arr, vec!["abc", "abc", "abc"]);
    }

    #[test]
    fn test_two_element() {
        let mut arr = vec!["bb".to_string(), "aa".to_string()];
        sort(&mut arr, 2);
        assert_eq!(arr, vec!["aa", "bb"]);
    }

    #[test]
    fn test_stability() {
        // Test that LSD sort is stable
        // Strings with same prefix should maintain relative order
        let mut arr = vec!["ab1".to_string(), "ab2".to_string(), "ab3".to_string()];
        sort(&mut arr, 3);
        assert_eq!(arr, vec!["ab1", "ab2", "ab3"]);
    }

    #[test]
    fn test_single_character() {
        let mut arr = vec!["z".to_string(), "a".to_string(), "m".to_string()];
        sort(&mut arr, 1);
        assert_eq!(arr, vec!["a", "m", "z"]);
    }

    #[test]
    fn test_numeric_strings() {
        let mut arr = vec![
            "999".to_string(),
            "111".to_string(),
            "555".to_string(),
            "222".to_string(),
        ];
        sort(&mut arr, 3);
        assert_eq!(arr, vec!["111", "222", "555", "999"]);
    }

    #[test]
    #[should_panic(expected = "LSD radix sort requires all strings to have the same length")]
    fn test_different_lengths_panic() {
        let mut arr = vec!["abc".to_string(), "de".to_string(), "f".to_string()];
        sort(&mut arr, 3);
    }

    #[test]
    fn test_bytes_empty() {
        let mut arr: Vec<Vec<u8>> = vec![];
        sort_bytes(&mut arr, 0);
        assert_eq!(arr, Vec::<Vec<u8>>::new());
    }

    #[test]
    fn test_bytes_basic() {
        let mut arr = vec![b"xyz".to_vec(), b"abc".to_vec(), b"def".to_vec()];
        sort_bytes(&mut arr, 3);
        assert_eq!(arr, vec![b"abc".to_vec(), b"def".to_vec(), b"xyz".to_vec()]);
    }

    #[test]
    fn test_bytes_single_element() {
        let mut arr = vec![b"abc".to_vec()];
        sort_bytes(&mut arr, 3);
        assert_eq!(arr, vec![b"abc".to_vec()]);
    }

    #[test]
    fn test_long_strings() {
        let mut arr = vec![
            "zzzzz".to_string(),
            "aaaaa".to_string(),
            "mmmmm".to_string(),
        ];
        sort(&mut arr, 5);
        assert_eq!(arr, vec!["aaaaa", "mmmmm", "zzzzz"]);
    }
}
