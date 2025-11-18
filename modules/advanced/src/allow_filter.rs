///! Allow filter application - prints words from stdin that are in the allowlist.
///!
///! Reads an allowlist of words from a file, then reads words from standard input
///! and prints all those words that appear in the allowlist. Useful for demonstrating
///! set data structures and membership testing.
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Filters words from standard input against an allowlist.
///
/// # Examples
///
/// ```no_run
/// use algs4_advanced::allow_filter::AllowFilter;
///
/// // Filter stdin against allowlist in "allowed.txt"
/// let filter = AllowFilter::from_file("allowed.txt")
///     .expect("Failed to read allowlist");
///
/// // Check if a word is allowed
/// assert!(filter.is_allowed("the"));
/// ```
#[derive(Debug, Clone)]
pub struct AllowFilter {
    allowlist: HashSet<String>,
}

impl AllowFilter {
    /// Creates a new allow filter from a file.
    ///
    /// # Arguments
    ///
    /// * `filename` - Path to file containing allowlist words (one per line or whitespace separated)
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use algs4_advanced::allow_filter::AllowFilter;
    ///
    /// let filter = AllowFilter::from_file("allowlist.txt")?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn from_file(filename: &str) -> io::Result<Self> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let mut allowlist = HashSet::new();

        for line in reader.lines() {
            let line = line?;
            for word in line.split_whitespace() {
                allowlist.insert(word.to_string());
            }
        }

        Ok(AllowFilter { allowlist })
    }

    /// Creates a new allow filter from a collection of words.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::allow_filter::AllowFilter;
    ///
    /// let words = vec!["the".to_string(), "of".to_string(), "and".to_string()];
    /// let filter = AllowFilter::from_words(words);
    /// assert!(filter.is_allowed("the"));
    /// assert!(!filter.is_allowed("hello"));
    /// ```
    pub fn from_words<I>(words: I) -> Self
    where
        I: IntoIterator<Item = String>,
    {
        AllowFilter {
            allowlist: words.into_iter().collect(),
        }
    }

    /// Checks if a word is in the allowlist.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::allow_filter::AllowFilter;
    ///
    /// let filter = AllowFilter::from_words(vec!["allowed".to_string()]);
    /// assert!(filter.is_allowed("allowed"));
    /// assert!(!filter.is_allowed("blocked"));
    /// ```
    pub fn is_allowed(&self, word: &str) -> bool {
        self.allowlist.contains(word)
    }

    /// Filters words from an iterator, returning only allowed words.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::allow_filter::AllowFilter;
    ///
    /// let filter = AllowFilter::from_words(vec!["the".to_string(), "of".to_string()]);
    /// let words = vec!["the", "quick", "brown", "of"];
    /// let allowed: Vec<_> = filter.filter_words(words).collect();
    /// assert_eq!(allowed, vec!["the", "of"]);
    /// ```
    pub fn filter_words<'a, I>(&'a self, words: I) -> impl Iterator<Item = &'a str> + 'a
    where
        I: IntoIterator<Item = &'a str> + 'a,
    {
        words.into_iter().filter(move |word| self.is_allowed(word))
    }

    /// Returns the number of words in the allowlist.
    pub fn len(&self) -> usize {
        self.allowlist.len()
    }

    /// Returns true if the allowlist is empty.
    pub fn is_empty(&self) -> bool {
        self.allowlist.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_words() {
        let words = vec!["the".to_string(), "of".to_string(), "and".to_string()];
        let filter = AllowFilter::from_words(words);

        assert_eq!(filter.len(), 3);
        assert!(filter.is_allowed("the"));
        assert!(filter.is_allowed("of"));
        assert!(filter.is_allowed("and"));
        assert!(!filter.is_allowed("hello"));
    }

    #[test]
    fn test_filter_words() {
        let filter = AllowFilter::from_words(vec![
            "was".to_string(),
            "it".to_string(),
            "the".to_string(),
            "of".to_string(),
        ]);

        let input = vec!["it", "was", "the", "best", "of", "times"];
        let filtered: Vec<_> = filter.filter_words(input).collect();

        assert_eq!(filtered, vec!["it", "was", "the", "of"]);
    }

    #[test]
    fn test_empty_filter() {
        let filter = AllowFilter::from_words(Vec::<String>::new());

        assert!(filter.is_empty());
        assert_eq!(filter.len(), 0);
        assert!(!filter.is_allowed("anything"));
    }

    #[test]
    fn test_case_sensitive() {
        let filter = AllowFilter::from_words(vec!["Hello".to_string()]);

        assert!(filter.is_allowed("Hello"));
        assert!(!filter.is_allowed("hello"));
    }
}
