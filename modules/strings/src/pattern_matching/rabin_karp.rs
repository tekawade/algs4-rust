//! Rabin-Karp substring search algorithm.
//!
//! The Rabin-Karp algorithm uses hashing to find a pattern in a text.
//! It computes a hash of the pattern and then rolls the hash through
//! the text, checking for matches.

/// Rabin-Karp substring search.
///
/// The Rabin-Karp algorithm uses a rolling hash function to search for
/// a pattern in linear time with high probability. It's particularly
/// useful for multiple pattern search.
///
/// # Examples
///
/// ```
/// use algs4_strings::pattern_matching::RabinKarp;
///
/// let pattern = "AACAA";
/// let rk = RabinKarp::new(pattern);
///
/// let text = "AABRAACADABRAACAADABRA";
/// assert_eq!(rk.search(text), Some(12));
/// ```
#[derive(Clone, Debug)]
pub struct RabinKarp {
    pattern: Vec<u8>,
    pattern_hash: u64,
    m: usize,   // Pattern length
    q: u64,     // Large prime (modulus)
    radix: u64, // Alphabet size
    rm: u64,    // radix^(m-1) % q
}

impl RabinKarp {
    const PRIME: u64 = 1_000_000_007; // Large prime for modulus

    /// Creates a new Rabin-Karp matcher for the given pattern (as bytes).
    ///
    /// This constructor preprocesses the pattern to compute its hash and
    /// precompute radix^(m-1) % q in O(M) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::pattern_matching::RabinKarp;
    ///
    /// let rk = RabinKarp::from_bytes(b"AACAA");
    /// assert_eq!(rk.search_bytes(b"AABRAACADABRAACAADABRA"), Some(12));
    /// ```
    pub fn from_bytes(pattern: &[u8]) -> Self {
        let m = pattern.len();
        let radix = 256u64;
        let q = Self::PRIME;

        // Compute radix^(m-1) % q for removing leading digit
        let mut rm = 1u64;
        for _ in 1..m {
            rm = (radix * rm) % q;
        }

        // Compute pattern hash
        let pattern_hash = Self::hash(pattern, m, radix, q);

        RabinKarp {
            pattern: pattern.to_vec(),
            pattern_hash,
            m,
            q,
            radix,
            rm,
        }
    }

    /// Creates a new Rabin-Karp matcher for the given pattern string.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::pattern_matching::RabinKarp;
    ///
    /// let rk = RabinKarp::new("AACAA");
    /// assert_eq!(rk.search("AABRAACADABRAACAADABRA"), Some(12));
    /// ```
    pub fn new(pattern: &str) -> Self {
        Self::from_bytes(pattern.as_bytes())
    }

    /// Computes the hash of the first m characters of the given key.
    fn hash(key: &[u8], m: usize, radix: u64, q: u64) -> u64 {
        let mut h = 0u64;
        for i in 0..m.min(key.len()) {
            h = (radix * h + key[i] as u64) % q;
        }
        h
    }

    /// Searches for the pattern in the given text (as bytes).
    ///
    /// Returns the index of the first occurrence of the pattern in the text,
    /// or None if the pattern is not found.
    ///
    /// Uses Monte Carlo version (assumes hash collision is very unlikely).
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::pattern_matching::RabinKarp;
    ///
    /// let rk = RabinKarp::from_bytes(b"AACAA");
    /// assert_eq!(rk.search_bytes(b"AABRAACADABRAACAADABRA"), Some(12));
    /// assert_eq!(rk.search_bytes(b"HAYSTACK"), None);
    /// ```
    pub fn search_bytes(&self, text: &[u8]) -> Option<usize> {
        let n = text.len();

        if n < self.m {
            return None;
        }

        // Compute hash of first m characters in text
        let mut text_hash = Self::hash(text, self.m, self.radix, self.q);

        // Check for match at position 0
        if self.pattern_hash == text_hash && self.check(text, 0) {
            return Some(0);
        }

        // Roll hash over text
        for i in self.m..n {
            // Remove leading digit, add trailing digit
            text_hash =
                (text_hash + self.q - self.rm * (text[i - self.m] as u64) % self.q) % self.q;
            text_hash = (text_hash * self.radix + text[i] as u64) % self.q;

            // Check for match
            let offset = i - self.m + 1;
            if self.pattern_hash == text_hash && self.check(text, offset) {
                return Some(offset);
            }
        }

        None // Not found
    }

    /// Las Vegas version: verify the match by comparing characters
    fn check(&self, text: &[u8], i: usize) -> bool {
        if i + self.m > text.len() {
            return false;
        }
        for j in 0..self.m {
            if self.pattern[j] != text[i + j] {
                return false;
            }
        }
        true
    }

    /// Searches for the pattern in the given text string.
    ///
    /// Returns the index of the first occurrence of the pattern in the text,
    /// or None if the pattern is not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::pattern_matching::RabinKarp;
    ///
    /// let rk = RabinKarp::new("AACAA");
    /// assert_eq!(rk.search("AABRAACADABRAACAADABRA"), Some(12));
    /// assert_eq!(rk.search("HAYSTACK"), None);
    /// ```
    pub fn search(&self, text: &str) -> Option<usize> {
        self.search_bytes(text.as_bytes())
    }

    /// Returns the pattern being searched for.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::pattern_matching::RabinKarp;
    ///
    /// let rk = RabinKarp::new("AACAA");
    /// assert_eq!(rk.pattern(), "AACAA");
    /// ```
    pub fn pattern(&self) -> &str {
        std::str::from_utf8(&self.pattern).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rabin_karp_basic() {
        let rk = RabinKarp::new("AACAA");
        assert_eq!(rk.search("AABRAACADABRAACAADABRA"), Some(12));
        assert_eq!(rk.search("HAYSTACK"), None);
    }

    #[test]
    fn test_rabin_karp_pattern_at_start() {
        let rk = RabinKarp::new("ABC");
        assert_eq!(rk.search("ABCDEF"), Some(0));
    }

    #[test]
    fn test_rabin_karp_pattern_at_end() {
        let rk = RabinKarp::new("DEF");
        assert_eq!(rk.search("ABCDEF"), Some(3));
    }

    #[test]
    fn test_rabin_karp_single_char() {
        let rk = RabinKarp::new("A");
        assert_eq!(rk.search("BCDA"), Some(3));
    }

    #[test]
    fn test_rabin_karp_not_found() {
        let rk = RabinKarp::new("XYZ");
        assert_eq!(rk.search("ABCDEF"), None);
    }

    #[test]
    fn test_rabin_karp_repeated_pattern() {
        let rk = RabinKarp::new("AAA");
        assert_eq!(rk.search("BAAAAB"), Some(1));
    }

    #[test]
    fn test_rabin_karp_overlapping() {
        let rk = RabinKarp::new("ABAB");
        assert_eq!(rk.search("ABABAB"), Some(0));
    }

    #[test]
    fn test_rabin_karp_empty_text() {
        let rk = RabinKarp::new("ABC");
        assert_eq!(rk.search(""), None);
    }

    #[test]
    fn test_rabin_karp_pattern_longer_than_text() {
        let rk = RabinKarp::new("ABCDEF");
        assert_eq!(rk.search("ABC"), None);
    }

    #[test]
    fn test_rabin_karp_bytes() {
        let rk = RabinKarp::from_bytes(b"AACAA");
        assert_eq!(rk.search_bytes(b"AABRAACADABRAACAADABRA"), Some(12));
    }

    #[test]
    fn test_rabin_karp_needle_haystack() {
        let rk = RabinKarp::new("NEEDLE");
        assert_eq!(rk.search("FINDINANEEDLEINTHEHAYSTACK"), Some(7));
    }
}
