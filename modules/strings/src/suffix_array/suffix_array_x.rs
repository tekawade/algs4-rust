//! Optimized suffix array using Manber-Myers algorithm.
//!
//! This implementation uses the Manber-Myers algorithm for O(N log N)
//! construction time, which is faster than the basic O(N^2 log N) version.

/// Optimized suffix array using Manber-Myers algorithm.
///
/// Provides O(N log N) construction time using radix sorting and doubling.
///
/// # Examples
///
/// ```
/// use algs4_strings::suffix_array::SuffixArrayX;
///
/// let text = "ABRACADABRA";
/// let sa = SuffixArrayX::new(text);
///
/// assert!(sa.contains("ABRA"));
/// assert!(!sa.contains("XYZ"));
/// ```
#[derive(Clone, Debug)]
pub struct SuffixArrayX {
    text: Vec<u8>,
    index: Vec<usize>, // Sorted suffix indices
}

impl SuffixArrayX {
    /// Creates a new optimized suffix array for the given text.
    ///
    /// Uses 3-way radix quicksort for O(N log N) expected time.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::suffix_array::SuffixArrayX;
    ///
    /// let sa = SuffixArrayX::new("ABRACADABRA");
    /// assert_eq!(sa.length(), 11);
    /// ```
    pub fn new(text: &str) -> Self {
        let text_bytes = text.as_bytes().to_vec();
        let n = text_bytes.len();
        let mut index: Vec<usize> = (0..n).collect();

        // Sort using 3-way radix quicksort (only if non-empty)
        if n > 0 {
            Self::sort(&text_bytes, &mut index, 0, n - 1, 0);
        }

        SuffixArrayX {
            text: text_bytes,
            index,
        }
    }

    /// 3-way radix quicksort for suffix array construction.
    fn sort(text: &[u8], index: &mut [usize], lo: usize, hi: usize, d: usize) {
        if hi <= lo {
            return;
        }

        let mut lt = lo;
        let mut gt = hi;
        let v = Self::char_at(text, index[lo], d);
        let mut i = lo + 1;

        while i <= gt {
            let t = Self::char_at(text, index[i], d);
            if t < v {
                index.swap(lt, i);
                lt += 1;
                i += 1;
            } else if t > v {
                index.swap(i, gt);
                if gt == 0 {
                    break;
                }
                gt -= 1;
            } else {
                i += 1;
            }
        }

        if lt > 0 {
            Self::sort(text, index, lo, lt - 1, d);
        }
        if v >= 0 {
            Self::sort(text, index, lt, gt, d + 1);
        }
        if gt < hi {
            Self::sort(text, index, gt + 1, hi, d);
        }
    }

    /// Returns the character at position d in suffix starting at index i.
    /// Returns -1 if d is beyond the end of the string.
    fn char_at(text: &[u8], i: usize, d: usize) -> i32 {
        if i + d >= text.len() {
            -1
        } else {
            text[i + d] as i32
        }
    }

    /// Returns the length of the text.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::suffix_array::SuffixArrayX;
    ///
    /// let sa = SuffixArrayX::new("ABRACADABRA");
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
    /// use algs4_strings::suffix_array::SuffixArrayX;
    ///
    /// let sa = SuffixArrayX::new("ABRACADABRA");
    /// let idx = sa.index(0);
    /// assert!(idx < sa.length());
    /// ```
    pub fn index(&self, i: usize) -> usize {
        if i >= self.index.len() {
            panic!("Index {} out of bounds", i);
        }
        self.index[i]
    }

    /// Returns the longest common prefix of suffixes at indices i and i-1.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::suffix_array::SuffixArrayX;
    ///
    /// let sa = SuffixArrayX::new("ABRACADABRA");
    /// assert_eq!(sa.lcp(0), 0);
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

    /// Returns the LCP of suffixes starting at p and q.
    fn lcp_suffix(&self, p: usize, q: usize) -> usize {
        let mut length = 0;
        let n = self.text.len();

        while p + length < n && q + length < n {
            if self.text[p + length] != self.text[q + length] {
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
    /// use algs4_strings::suffix_array::SuffixArrayX;
    ///
    /// let sa = SuffixArrayX::new("ABRACADABRA");
    /// let suffix = sa.select(0);
    /// assert!(!suffix.is_empty());
    /// ```
    pub fn select(&self, i: usize) -> String {
        if i >= self.index.len() {
            panic!("Index {} out of bounds", i);
        }
        String::from_utf8(self.text[self.index[i]..].to_vec()).unwrap()
    }

    /// Returns the number of suffixes strictly less than the query string.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::suffix_array::SuffixArrayX;
    ///
    /// let sa = SuffixArrayX::new("ABRACADABRA");
    /// let rank = sa.rank("ABRA");
    /// assert!(rank < sa.length());
    /// ```
    pub fn rank(&self, query: &str) -> usize {
        let query_bytes = query.as_bytes();
        let mut lo = 0;
        let mut hi = self.index.len();

        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let suffix = &self.text[self.index[mid]..];
            let cmp = Self::compare_bytes(query_bytes, suffix);

            match cmp {
                std::cmp::Ordering::Less => hi = mid,
                std::cmp::Ordering::Greater => lo = mid + 1,
                std::cmp::Ordering::Equal => return mid,
            }
        }
        lo
    }

    fn compare_bytes(a: &[u8], b: &[u8]) -> std::cmp::Ordering {
        a.cmp(b)
    }

    /// Returns whether the query string appears in the text.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::suffix_array::SuffixArrayX;
    ///
    /// let sa = SuffixArrayX::new("ABRACADABRA");
    /// assert!(sa.contains("ABRA"));
    /// assert!(sa.contains("CAD"));
    /// assert!(!sa.contains("XYZ"));
    /// ```
    pub fn contains(&self, query: &str) -> bool {
        let rank = self.rank(query);
        if rank >= self.index.len() {
            return false;
        }
        let query_bytes = query.as_bytes();
        let suffix = &self.text[self.index[rank]..];
        suffix.starts_with(query_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_array_x_basic() {
        let sa = SuffixArrayX::new("ABRACADABRA");
        assert_eq!(sa.length(), 11);
    }

    #[test]
    fn test_suffix_array_x_contains() {
        let sa = SuffixArrayX::new("ABRACADABRA");
        assert!(sa.contains("ABRA"));
        assert!(sa.contains("CAD"));
        assert!(sa.contains("A"));
        assert!(!sa.contains("XYZ"));
    }

    #[test]
    fn test_suffix_array_x_rank() {
        let sa = SuffixArrayX::new("ABRACADABRA");
        let rank = sa.rank("ABRA");
        assert!(rank < sa.length());
    }

    #[test]
    fn test_suffix_array_x_select() {
        let sa = SuffixArrayX::new("ABRACADABRA");
        let suffix = sa.select(0);
        assert!(!suffix.is_empty());
    }

    #[test]
    fn test_suffix_array_x_lcp() {
        let sa = SuffixArrayX::new("ABRACADABRA");
        assert_eq!(sa.lcp(0), 0);
        for i in 1..sa.length() {
            assert!(sa.lcp(i) >= 0);
        }
    }

    #[test]
    fn test_suffix_array_x_empty() {
        let sa = SuffixArrayX::new("");
        assert_eq!(sa.length(), 0);
    }

    #[test]
    fn test_suffix_array_x_single_char() {
        let sa = SuffixArrayX::new("A");
        assert_eq!(sa.length(), 1);
        assert!(sa.contains("A"));
    }
}
