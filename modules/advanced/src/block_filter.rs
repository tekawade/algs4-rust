//! Block filter application - prints words from stdin that are NOT in the blocklist.
//!
//! Reads a blocklist of words from a file, then reads words from standard input
//! and prints all those words that do NOT appear in the blocklist. Useful for
//! filtering out common words or spam.
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Filters words from standard input against a blocklist.
///
/// # Examples
///
/// ```no_run
/// use algs4_advanced::block_filter::BlockFilter;
///
/// // Filter stdin against blocklist in "blocked.txt"
/// let filter = BlockFilter::from_file("blocked.txt")
///     .expect("Failed to read blocklist");
///
/// // Check if a word should be blocked
/// assert!(!filter.is_blocked("hello"));
/// ```
#[derive(Debug, Clone)]
pub struct BlockFilter {
    blocklist: HashSet<String>,
}

impl BlockFilter {
    /// Creates a new block filter from a file.
    ///
    /// # Arguments
    ///
    /// * `filename` - Path to file containing blocklist words (one per line or whitespace separated)
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use algs4_advanced::block_filter::BlockFilter;
    ///
    /// let filter = BlockFilter::from_file("blocklist.txt")?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn from_file(filename: &str) -> io::Result<Self> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let mut blocklist = HashSet::new();

        for line in reader.lines() {
            let line = line?;
            for word in line.split_whitespace() {
                blocklist.insert(word.to_string());
            }
        }

        Ok(BlockFilter { blocklist })
    }

    /// Creates a new block filter from a collection of words.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::block_filter::BlockFilter;
    ///
    /// let words = vec!["spam".to_string(), "bad".to_string(), "blocked".to_string()];
    /// let filter = BlockFilter::from_words(words);
    /// assert!(filter.is_blocked("spam"));
    /// assert!(!filter.is_blocked("good"));
    /// ```
    pub fn from_words<I>(words: I) -> Self
    where
        I: IntoIterator<Item = String>,
    {
        BlockFilter {
            blocklist: words.into_iter().collect(),
        }
    }

    /// Checks if a word is in the blocklist.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::block_filter::BlockFilter;
    ///
    /// let filter = BlockFilter::from_words(vec!["blocked".to_string()]);
    /// assert!(filter.is_blocked("blocked"));
    /// assert!(!filter.is_blocked("allowed"));
    /// ```
    pub fn is_blocked(&self, word: &str) -> bool {
        self.blocklist.contains(word)
    }

    /// Filters words from an iterator, returning only non-blocked words.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::block_filter::BlockFilter;
    ///
    /// let filter = BlockFilter::from_words(vec!["the".to_string(), "of".to_string()]);
    /// let words = vec!["the", "quick", "brown", "of", "fox"];
    /// let allowed: Vec<_> = filter.filter_words(words).collect();
    /// assert_eq!(allowed, vec!["quick", "brown", "fox"]);
    /// ```
    pub fn filter_words<'a, I>(&'a self, words: I) -> impl Iterator<Item = &'a str> + 'a
    where
        I: IntoIterator<Item = &'a str> + 'a,
    {
        words.into_iter().filter(move |word| !self.is_blocked(word))
    }

    /// Returns the number of words in the blocklist.
    pub fn len(&self) -> usize {
        self.blocklist.len()
    }

    /// Returns true if the blocklist is empty.
    pub fn is_empty(&self) -> bool {
        self.blocklist.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_words() {
        let words = vec!["the".to_string(), "of".to_string(), "and".to_string()];
        let filter = BlockFilter::from_words(words);

        assert_eq!(filter.len(), 3);
        assert!(filter.is_blocked("the"));
        assert!(filter.is_blocked("of"));
        assert!(filter.is_blocked("and"));
        assert!(!filter.is_blocked("hello"));
    }

    #[test]
    fn test_filter_words() {
        let filter = BlockFilter::from_words(vec![
            "was".to_string(),
            "it".to_string(),
            "the".to_string(),
            "of".to_string(),
        ]);

        let input = vec!["it", "was", "the", "best", "of", "times"];
        let filtered: Vec<_> = filter.filter_words(input).collect();

        // Should get everything EXCEPT the blocked words
        assert_eq!(filtered, vec!["best", "times"]);
    }

    #[test]
    fn test_empty_filter() {
        let filter = BlockFilter::from_words(Vec::<String>::new());

        assert!(filter.is_empty());
        assert_eq!(filter.len(), 0);
        // Nothing is blocked with empty filter
        assert!(!filter.is_blocked("anything"));
    }

    #[test]
    fn test_case_sensitive() {
        let filter = BlockFilter::from_words(vec!["Spam".to_string()]);

        assert!(filter.is_blocked("Spam"));
        assert!(!filter.is_blocked("spam"));
    }

    #[test]
    fn test_all_blocked() {
        let filter = BlockFilter::from_words(vec![
            "all".to_string(),
            "words".to_string(),
            "blocked".to_string(),
        ]);

        let input = vec!["all", "words", "blocked"];
        let filtered: Vec<_> = filter.filter_words(input).collect();

        assert!(filtered.is_empty());
    }
}
