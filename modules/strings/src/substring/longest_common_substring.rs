///! Longest common substring using suffix arrays.
///!
///! Computes the longest substring that appears in two given strings
///! using suffix array data structure.
use crate::suffix_array::suffix_array::SuffixArray;

/// Finds the longest common substring between two strings.
///
/// Uses suffix arrays to efficiently find the longest substring that
/// appears in both input strings.
///
/// # Time Complexity
///
/// O(n log n + m log m) where n and m are the lengths of the two strings.
///
/// # Examples
///
/// ```
/// use algs4_strings::substring::lcs;
///
/// let s = "abcdefabcxyz";
/// let t = "xyzabcuvwabc";
/// let result = lcs(s, t);
/// assert_eq!(result, "abc");
/// ```
pub fn lcs(s: &str, t: &str) -> String {
    let suffix_s = SuffixArray::new(s);
    let suffix_t = SuffixArray::new(t);

    let mut longest = String::new();
    let mut i = 0;
    let mut j = 0;

    while i < s.len() && j < t.len() {
        let p = suffix_s.index(i);
        let q = suffix_t.index(j);

        // Compute longest common prefix of suffix s[p..] and t[q..]
        let common = lcp_suffix(s, p, t, q);

        if common.len() > longest.len() {
            longest = common;
        }

        // Advance the smaller suffix
        if compare_suffix(s, p, t, q).is_lt() {
            i += 1;
        } else {
            j += 1;
        }
    }

    longest
}

/// Computes the longest common prefix of suffix s[p..] and suffix t[q..].
fn lcp_suffix(s: &str, p: usize, t: &str, q: usize) -> String {
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();

    let n = std::cmp::min(s_chars.len() - p, t_chars.len() - q);

    for i in 0..n {
        if s_chars[p + i] != t_chars[q + i] {
            return s_chars[p..p + i].iter().collect();
        }
    }

    s_chars[p..p + n].iter().collect()
}

/// Compares suffix s[p..] with suffix t[q..].
fn compare_suffix(s: &str, p: usize, t: &str, q: usize) -> std::cmp::Ordering {
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();

    let n = std::cmp::min(s_chars.len() - p, t_chars.len() - q);

    for i in 0..n {
        match s_chars[p + i].cmp(&t_chars[q + i]) {
            std::cmp::Ordering::Equal => continue,
            ord => return ord,
        }
    }

    (s_chars.len() - p).cmp(&(t_chars.len() - q))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let s = "abcdefabcxyz";
        let t = "xyzabcuvwabc";
        let result = lcs(s, t);
        assert_eq!(result, "abc");
    }

    #[test]
    fn test_identical() {
        let s = "hello";
        let t = "hello";
        let result = lcs(s, t);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_no_common() {
        let s = "abc";
        let t = "xyz";
        let result = lcs(s, t);
        assert_eq!(result, "");
    }

    #[test]
    fn test_one_char() {
        let s = "abcdef";
        let t = "xyzc";
        let result = lcs(s, t);
        assert_eq!(result, "c");
    }

    #[test]
    fn test_longer_substring() {
        let s = "the quick brown fox";
        let t = "quick brown is best";
        let result = lcs(s, t);
        assert_eq!(result, "quick brown ");
    }

    #[test]
    fn test_empty_strings() {
        let s = "";
        let t = "anything";
        let result = lcs(s, t);
        assert_eq!(result, "");

        let s = "anything";
        let t = "";
        let result = lcs(s, t);
        assert_eq!(result, "");
    }

    #[test]
    fn test_overlapping() {
        let s = "aaaa";
        let t = "aaa";
        let result = lcs(s, t);
        assert_eq!(result, "aaa");
    }
}
