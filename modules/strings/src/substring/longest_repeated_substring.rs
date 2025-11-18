//! Longest repeated substring using suffix arrays.
//!
//! Computes the longest substring that appears at least twice in a given string
//! using the suffix array data structure and LCP (longest common prefix) array.
use crate::suffix_array::suffix_array::SuffixArray;

/// Finds the longest repeated substring.
///
/// Returns the longest substring that appears at least twice in the input string.
/// The repeated substrings may overlap but must be distinct.
///
/// # Time Complexity
///
/// O(n log n) where n is the length of the string.
///
/// # Examples
///
/// ```
/// use algs4_strings::substring::lrs;
///
/// let text = "banana";
/// let result = lrs(text);
/// assert_eq!(result, "ana");
/// ```
///
/// ```
/// use algs4_strings::substring::lrs;
///
/// let text = "aabcaabdaab";
/// let result = lrs(text);
/// assert_eq!(result, "aab");
/// ```
pub fn lrs(text: &str) -> String {
    let n = text.len();
    if n == 0 {
        return String::new();
    }

    let sa = SuffixArray::new(text);
    let mut longest = String::new();

    // Check LCP between consecutive suffixes in sorted order
    for i in 1..n {
        let lcp_len = sa.lcp(i);
        if lcp_len > longest.len() {
            let start_idx = sa.index(i);
            longest = text.chars().skip(start_idx).take(lcp_len).collect();
        }
    }

    longest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_banana() {
        let text = "banana";
        let result = lrs(text);
        assert_eq!(result, "ana");
    }

    #[test]
    fn test_aabcaabdaab() {
        let text = "aabcaabdaab";
        let result = lrs(text);
        assert_eq!(result, "aab");
    }

    #[test]
    fn test_no_repeat() {
        let text = "abcdef";
        let result = lrs(text);
        assert_eq!(result, "");
    }

    #[test]
    fn test_all_same() {
        let text = "aaaa";
        let result = lrs(text);
        assert_eq!(result, "aaa");
    }

    #[test]
    fn test_two_char_repeat() {
        let text = "abab";
        let result = lrs(text);
        assert_eq!(result, "ab");
    }

    #[test]
    fn test_empty() {
        let text = "";
        let result = lrs(text);
        assert_eq!(result, "");
    }

    #[test]
    fn test_single_char() {
        let text = "a";
        let result = lrs(text);
        assert_eq!(result, "");
    }

    #[test]
    fn test_longer_text() {
        let text = "it was the best of times it was the worst of times";
        let result = lrs(text);
        // "it was the " and " of times" are both 11 chars, but " of times" comes first
        assert!(result == "it was the " || result == " of times");
        assert_eq!(result.len(), 11);
    }

    #[test]
    fn test_overlapping_repeats() {
        let text = "abcabcabc";
        let result = lrs(text);
        assert_eq!(result, "abcabc");
    }
}
