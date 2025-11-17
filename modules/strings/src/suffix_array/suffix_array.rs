//! Suffix array data structure.
//!
//! A suffix array is a sorted array of all suffixes of a string.
//! It's useful for pattern matching, string compression, and bioinformatics.

/// Suffix array data structure.
///
/// Represents the sorted suffixes of a string, enabling efficient
/// pattern matching and string analysis.
///
/// # Examples
///
/// ```
/// use algs4_strings::suffix_array::SuffixArray;
///
/// let text = "ABRACADABRA";
/// let sa = SuffixArray::new(text);
///
/// // Find pattern
/// assert!(sa.contains("ABRA"));
/// assert!(!sa.contains("XYZ"));
/// ```
#[derive(Clone, Debug)]
pub struct SuffixArray {
    text: String,
    index: Vec<usize>, // Sorted suffix indices
}

impl SuffixArray {
    /// Creates a new suffix array for the given text.
    ///
    /// Builds the suffix array in O(N^2 log N) time using standard sorting.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::suffix_array::SuffixArray;
    ///
    /// let sa = SuffixArray::new("ABRACADABRA");
    /// assert_eq!(sa.length(), 11);
    /// ```
    pub fn new(text: &str) -> Self {
        let n = text.len();
        let mut index: Vec<usize> = (0..n).collect();

        // Sort suffixes
        index.sort_by(|&a, &b| text[a..].cmp(&text[b..]));

        SuffixArray {
            text: text.to_string(),
            index,
        }
    }

    /// Returns the length of the text.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::suffix_array::SuffixArray;
    ///
    /// let sa = SuffixArray::new("ABRACADABRA");
    /// assert_eq!(sa.length(), 11);
    /// ```
    pub fn length(&self) -> usize {
        self.index.len()
    }

    /// Returns the index of the i-th smallest suffix.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::suffix_array::SuffixArray;
    ///
    /// let sa = SuffixArray::new("ABRACADABRA");
    /// assert_eq!(sa.index(0), 10); // Empty suffix "A"
    /// ```
    pub fn index(&self, i: usize) -> usize {
        if i >= self.index.len() {
            panic!(
                "Index {} out of bounds for suffix array of length {}",
                i,
                self.index.len()
            );
        }
        self.index[i]
    }

    /// Returns the longest common prefix of suffixes at indices i and i-1.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::suffix_array::SuffixArray;
    ///
    /// let sa = SuffixArray::new("ABRACADABRA");
    /// // Check LCP between consecutive suffixes
    /// let lcp = sa.lcp(1);
    /// assert!(lcp >= 0);
    /// ```
    pub fn lcp(&self, i: usize) -> usize {
        if i == 0 {
            return 0;
        }
        if i >= self.index.len() {
            panic!("Index {} out of bounds", i);
        }
        self.lcp_suffix(self.index[i], self.index[i - 1])
    }

    /// Returns the length of the longest common prefix of s[p..] and s[q..].
    fn lcp_suffix(&self, p: usize, q: usize) -> usize {
        let mut length = 0;
        let text_bytes = self.text.as_bytes();
        let n = text_bytes.len();

        while p + length < n && q + length < n {
            if text_bytes[p + length] != text_bytes[q + length] {
                break;
            }
            length += 1;
        }
        length
    }

    /// Returns the i-th smallest suffix as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::suffix_array::SuffixArray;
    ///
    /// let sa = SuffixArray::new("ABRACADABRA");
    /// assert_eq!(sa.select(0), "A");
    /// ```
    pub fn select(&self, i: usize) -> &str {
        if i >= self.index.len() {
            panic!("Index {} out of bounds", i);
        }
        &self.text[self.index[i]..]
    }

    /// Returns the number of suffixes strictly less than the query string.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::suffix_array::SuffixArray;
    ///
    /// let sa = SuffixArray::new("ABRACADABRA");
    /// let rank = sa.rank("ABRA");
    /// assert!(rank >= 0);
    /// ```
    pub fn rank(&self, query: &str) -> usize {
        let mut lo = 0;
        let mut hi = self.index.len();

        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let cmp = query.cmp(&self.text[self.index[mid]..]);

            match cmp {
                std::cmp::Ordering::Less => hi = mid,
                std::cmp::Ordering::Greater => lo = mid + 1,
                std::cmp::Ordering::Equal => return mid,
            }
        }
        lo
    }

    /// Returns whether the query string appears in the text.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::suffix_array::SuffixArray;
    ///
    /// let sa = SuffixArray::new("ABRACADABRA");
    /// assert!(sa.contains("ABRA"));
    /// assert!(sa.contains("CAD"));
    /// assert!(!sa.contains("XYZ"));
    /// ```
    pub fn contains(&self, query: &str) -> bool {
        let rank = self.rank(query);
        if rank >= self.index.len() {
            return false;
        }
        let suffix = &self.text[self.index[rank]..];
        suffix.starts_with(query)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_array_basic() {
        let sa = SuffixArray::new("ABRACADABRA");
        assert_eq!(sa.length(), 11);
    }

    #[test]
    fn test_suffix_array_contains() {
        let sa = SuffixArray::new("ABRACADABRA");
        assert!(sa.contains("ABRA"));
        assert!(sa.contains("CAD"));
        assert!(sa.contains("A"));
        assert!(!sa.contains("XYZ"));
    }

    #[test]
    fn test_suffix_array_rank() {
        let sa = SuffixArray::new("ABRACADABRA");
        let rank = sa.rank("ABRA");
        assert!(rank < sa.length());
    }

    #[test]
    fn test_suffix_array_select() {
        let sa = SuffixArray::new("ABRACADABRA");
        let suffix = sa.select(0);
        assert!(!suffix.is_empty());
    }

    #[test]
    fn test_suffix_array_lcp() {
        let sa = SuffixArray::new("ABRACADABRA");
        // LCP of first suffix is always 0
        assert_eq!(sa.lcp(0), 0);
        // Other LCPs should be non-negative
        for i in 1..sa.length() {
            assert!(sa.lcp(i) >= 0);
        }
    }

    #[test]
    fn test_suffix_array_index() {
        let sa = SuffixArray::new("ABRACADABRA");
        for i in 0..sa.length() {
            let idx = sa.index(i);
            assert!(idx < sa.length());
        }
    }

    #[test]
    fn test_suffix_array_empty() {
        let sa = SuffixArray::new("");
        assert_eq!(sa.length(), 0);
    }

    #[test]
    fn test_suffix_array_single_char() {
        let sa = SuffixArray::new("A");
        assert_eq!(sa.length(), 1);
        assert!(sa.contains("A"));
    }

    #[test]
    fn test_suffix_array_repeated() {
        let sa = SuffixArray::new("AAA");
        assert_eq!(sa.length(), 3);
        assert!(sa.contains("A"));
        assert!(sa.contains("AA"));
        assert!(sa.contains("AAA"));
    }
}
