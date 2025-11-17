//! Boyer-Moore substring search algorithm.
//!
//! The Boyer-Moore algorithm finds the first occurrence of a pattern in a text
//! using a "bad character" heuristic to skip sections of the text, achieving
//! sublinear performance in typical cases.

/// Boyer-Moore substring search.
///
/// The Boyer-Moore algorithm preprocesses the pattern to build a "right"
/// array that enables skipping characters when mismatches occur.
///
/// # Examples
///
/// ```
/// use algs4_strings::pattern_matching::BoyerMoore;
///
/// let pattern = "NEEDLE";
/// let bm = BoyerMoore::new(pattern);
///
/// let text = "FINDINANEEDLEINTHEHAYSTACK";
/// assert_eq!(bm.search(text), Some(7));
/// ```
#[derive(Clone, Debug)]
pub struct BoyerMoore {
    pattern: Vec<u8>,
    right: Vec<isize>,
    radix: usize,
}

impl BoyerMoore {
    /// Creates a new Boyer-Moore matcher for the given pattern (as bytes).
    ///
    /// This constructor preprocesses the pattern to build the "right" array
    /// in O(M + R) time, where M is the pattern length and R is the alphabet size.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::pattern_matching::BoyerMoore;
    ///
    /// let bm = BoyerMoore::from_bytes(b"NEEDLE");
    /// assert_eq!(bm.search_bytes(b"FINDINANEEDLEINTHEHAYSTACK"), Some(7));
    /// ```
    pub fn from_bytes(pattern: &[u8]) -> Self {
        let radix = 256; // Extended ASCII
        let m = pattern.len();

        // Build right array: for each character, store the rightmost position
        // in the pattern (or -1 if not in the pattern)
        let mut right = vec![-1isize; radix];
        for (j, &c) in pattern.iter().enumerate() {
            right[c as usize] = j as isize;
        }

        BoyerMoore {
            pattern: pattern.to_vec(),
            right,
            radix,
        }
    }

    /// Creates a new Boyer-Moore matcher for the given pattern string.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::pattern_matching::BoyerMoore;
    ///
    /// let bm = BoyerMoore::new("NEEDLE");
    /// assert_eq!(bm.search("FINDINANEEDLEINTHEHAYSTACK"), Some(7));
    /// ```
    pub fn new(pattern: &str) -> Self {
        Self::from_bytes(pattern.as_bytes())
    }

    /// Searches for the pattern in the given text (as bytes).
    ///
    /// Returns the index of the first occurrence of the pattern in the text,
    /// or None if the pattern is not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::pattern_matching::BoyerMoore;
    ///
    /// let bm = BoyerMoore::from_bytes(b"NEEDLE");
    /// assert_eq!(bm.search_bytes(b"FINDINANEEDLEINTHEHAYSTACK"), Some(7));
    /// assert_eq!(bm.search_bytes(b"HAYSTACK"), None);
    /// ```
    pub fn search_bytes(&self, text: &[u8]) -> Option<usize> {
        let n = text.len();
        let m = self.pattern.len();

        if m > n {
            return None;
        }

        let mut skip;
        let mut i = 0; // Text position
        while i <= n - m {
            skip = 0;
            // Scan from right to left
            for j in (0..m).rev() {
                if self.pattern[j] != text[i + j] {
                    // Mismatch: compute skip
                    let right_pos = self.right[text[i + j] as usize];
                    skip = j as isize - right_pos;
                    if skip < 1 {
                        skip = 1;
                    }
                    break;
                }
            }
            if skip == 0 {
                return Some(i); // Found
            }
            i += skip as usize;
        }
        None // Not found
    }

    /// Searches for the pattern in the given text string.
    ///
    /// Returns the index of the first occurrence of the pattern in the text,
    /// or None if the pattern is not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::pattern_matching::BoyerMoore;
    ///
    /// let bm = BoyerMoore::new("NEEDLE");
    /// assert_eq!(bm.search("FINDINANEEDLEINTHEHAYSTACK"), Some(7));
    /// assert_eq!(bm.search("HAYSTACK"), None);
    /// ```
    pub fn search(&self, text: &str) -> Option<usize> {
        self.search_bytes(text.as_bytes())
    }

    /// Returns the pattern being searched for.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::pattern_matching::BoyerMoore;
    ///
    /// let bm = BoyerMoore::new("NEEDLE");
    /// assert_eq!(bm.pattern(), "NEEDLE");
    /// ```
    pub fn pattern(&self) -> &str {
        std::str::from_utf8(&self.pattern).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boyer_moore_basic() {
        let bm = BoyerMoore::new("NEEDLE");
        assert_eq!(bm.search("FINDINANEEDLEINTHEHAYSTACK"), Some(7));
        assert_eq!(bm.search("HAYSTACK"), None);
    }

    #[test]
    fn test_boyer_moore_pattern_at_start() {
        let bm = BoyerMoore::new("ABC");
        assert_eq!(bm.search("ABCDEF"), Some(0));
    }

    #[test]
    fn test_boyer_moore_pattern_at_end() {
        let bm = BoyerMoore::new("DEF");
        assert_eq!(bm.search("ABCDEF"), Some(3));
    }

    #[test]
    fn test_boyer_moore_single_char() {
        let bm = BoyerMoore::new("A");
        assert_eq!(bm.search("BCDA"), Some(3));
    }

    #[test]
    fn test_boyer_moore_not_found() {
        let bm = BoyerMoore::new("XYZ");
        assert_eq!(bm.search("ABCDEF"), None);
    }

    #[test]
    fn test_boyer_moore_repeated_pattern() {
        let bm = BoyerMoore::new("AAA");
        assert_eq!(bm.search("BAAAAB"), Some(1));
    }

    #[test]
    fn test_boyer_moore_overlapping() {
        let bm = BoyerMoore::new("ABAB");
        assert_eq!(bm.search("ABABAB"), Some(0));
    }

    #[test]
    fn test_boyer_moore_empty_text() {
        let bm = BoyerMoore::new("ABC");
        assert_eq!(bm.search(""), None);
    }

    #[test]
    fn test_boyer_moore_pattern_longer_than_text() {
        let bm = BoyerMoore::new("ABCDEF");
        assert_eq!(bm.search("ABC"), None);
    }

    #[test]
    fn test_boyer_moore_bytes() {
        let bm = BoyerMoore::from_bytes(b"NEEDLE");
        assert_eq!(bm.search_bytes(b"FINDINANEEDLEINTHEHAYSTACK"), Some(7));
    }

    #[test]
    fn test_boyer_moore_textbook_example() {
        let bm = BoyerMoore::new("AACAA");
        let text = "AABRAACADABRAACAADABRA";
        assert_eq!(bm.search(text), Some(12));
    }
}
