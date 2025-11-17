//! Knuth-Morris-Pratt (KMP) substring search algorithm.
//!
//! The KMP algorithm finds the first occurrence of a pattern in a text string
//! in O(N + M) time, where N is the length of the text and M is the length of
//! the pattern.

/// Knuth-Morris-Pratt substring search.
///
/// The KMP algorithm preprocesses the pattern to build a deterministic finite
/// automaton (DFA) that enables efficient substring search without backup
/// in the text string.
///
/// # Examples
///
/// ```
/// use algs4_strings::pattern_matching::KMP;
///
/// let pattern = "ABABAC";
/// let kmp = KMP::new(pattern);
///
/// let text = "BCBAABACAABABACAA";
/// assert_eq!(kmp.search(text), Some(9));
///
/// let text2 = "BCBAABACAA";
/// assert_eq!(kmp.search(text2), None);
/// ```
#[derive(Clone, Debug)]
pub struct KMP {
    pattern: Vec<u8>,
    dfa: Vec<Vec<usize>>,
}

impl KMP {
    /// Creates a new KMP matcher for the given pattern (as bytes).
    ///
    /// This constructor preprocesses the pattern to build the DFA in O(RM) time,
    /// where R is the alphabet size and M is the pattern length.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::pattern_matching::KMP;
    ///
    /// let kmp = KMP::from_bytes(b"ABABAC");
    /// assert_eq!(kmp.search_bytes(b"BCBAABACAABABACAA"), Some(9));
    /// ```
    pub fn from_bytes(pattern: &[u8]) -> Self {
        let m = pattern.len();
        let radix = 256; // Extended ASCII

        // Build DFA from pattern
        let mut dfa = vec![vec![0; m]; radix];
        dfa[pattern[0] as usize][0] = 1;

        let mut x = 0; // Restart state
        for j in 1..m {
            // Copy mismatch cases
            for item in dfa.iter_mut().take(radix) {
                item[j] = item[x];
            }
            // Set match case
            dfa[pattern[j] as usize][j] = j + 1;
            // Update restart state
            x = dfa[pattern[j] as usize][x];
        }

        KMP {
            pattern: pattern.to_vec(),
            dfa,
        }
    }

    /// Creates a new KMP matcher for the given pattern string.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::pattern_matching::KMP;
    ///
    /// let kmp = KMP::new("ABABAC");
    /// assert_eq!(kmp.search("BCBAABACAABABACAA"), Some(9));
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
    /// use algs4_strings::pattern_matching::KMP;
    ///
    /// let kmp = KMP::from_bytes(b"ABABAC");
    /// assert_eq!(kmp.search_bytes(b"BCBAABACAABABACAA"), Some(9));
    /// assert_eq!(kmp.search_bytes(b"BCBAABACAA"), None);
    /// ```
    pub fn search_bytes(&self, text: &[u8]) -> Option<usize> {
        let m = self.pattern.len();

        let mut j = 0; // Pattern position
        for (i, &c) in text.iter().enumerate() {
            j = self.dfa[c as usize][j];
            if j == m {
                return Some(i + 1 - m); // Found
            }
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
    /// use algs4_strings::pattern_matching::KMP;
    ///
    /// let kmp = KMP::new("ABABAC");
    /// assert_eq!(kmp.search("BCBAABACAABABACAA"), Some(9));
    /// assert_eq!(kmp.search("BCBAABACAA"), None);
    /// ```
    pub fn search(&self, text: &str) -> Option<usize> {
        self.search_bytes(text.as_bytes())
    }

    /// Returns the pattern being searched for.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::pattern_matching::KMP;
    ///
    /// let kmp = KMP::new("ABABAC");
    /// assert_eq!(kmp.pattern(), "ABABAC");
    /// ```
    pub fn pattern(&self) -> &str {
        std::str::from_utf8(&self.pattern).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kmp_basic() {
        let kmp = KMP::new("ABABAC");
        assert_eq!(kmp.search("BCBAABACAABABACAA"), Some(9));
        assert_eq!(kmp.search("BCBAABACAA"), None);
    }

    #[test]
    fn test_kmp_pattern_at_start() {
        let kmp = KMP::new("ABC");
        assert_eq!(kmp.search("ABCDEF"), Some(0));
    }

    #[test]
    fn test_kmp_pattern_at_end() {
        let kmp = KMP::new("DEF");
        assert_eq!(kmp.search("ABCDEF"), Some(3));
    }

    #[test]
    fn test_kmp_single_char() {
        let kmp = KMP::new("A");
        assert_eq!(kmp.search("BCDA"), Some(3));
    }

    #[test]
    fn test_kmp_not_found() {
        let kmp = KMP::new("XYZ");
        assert_eq!(kmp.search("ABCDEF"), None);
    }

    #[test]
    fn test_kmp_repeated_pattern() {
        let kmp = KMP::new("AAA");
        assert_eq!(kmp.search("BAAAAB"), Some(1));
    }

    #[test]
    fn test_kmp_overlapping() {
        let kmp = KMP::new("ABAB");
        assert_eq!(kmp.search("ABABAB"), Some(0));
    }

    #[test]
    fn test_kmp_empty_text() {
        let kmp = KMP::new("ABC");
        assert_eq!(kmp.search(""), None);
    }

    #[test]
    fn test_kmp_pattern_longer_than_text() {
        let kmp = KMP::new("ABCDEF");
        assert_eq!(kmp.search("ABC"), None);
    }

    #[test]
    fn test_kmp_bytes() {
        let kmp = KMP::from_bytes(b"ABABAC");
        assert_eq!(kmp.search_bytes(b"BCBAABACAABABACAA"), Some(9));
    }
}
