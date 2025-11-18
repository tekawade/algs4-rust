/// Frequency Counter
///
/// A utility for counting word frequencies in text using symbol tables.
/// Demonstrates practical usage of symbol table implementations.
///
/// # Examples
///
/// ```
/// use algs4_searching::frequency_counter::count_frequencies;
///
/// let text = "the quick brown fox jumps over the lazy dog the fox";
/// let words: Vec<&str> = text.split_whitespace().collect();
/// let freq = count_frequencies(&words, 2);
///
/// assert_eq!(freq.get("the"), Some(&3));
/// assert_eq!(freq.get("fox"), Some(&2));
/// ```
use std::collections::HashMap;
use std::fmt::{self, Display};

/// Counts the frequency of each word in the input that meets the minimum length requirement.
///
/// # Arguments
///
/// * `words` - Iterator of words to count
/// * `min_len` - Minimum word length to include in the count
///
/// # Returns
///
/// A HashMap mapping each word to its frequency count.
///
/// # Examples
///
/// ```
/// use algs4_searching::frequency_counter::count_frequencies;
///
/// let words = vec!["the", "quick", "brown", "fox", "the", "fox"];
/// let freq = count_frequencies(&words, 3);
///
/// assert_eq!(freq.get("the"), Some(&2));
/// assert_eq!(freq.get("fox"), Some(&2));
/// assert_eq!(freq.get("quick"), Some(&1));
/// ```
pub fn count_frequencies<S: AsRef<str>>(words: &[S], min_len: usize) -> HashMap<String, usize> {
    let mut freq = HashMap::new();

    for word in words {
        let w = word.as_ref();
        if w.len() >= min_len {
            *freq.entry(w.to_lowercase()).or_insert(0) += 1;
        }
    }

    freq
}

/// Finds the most frequent word that meets the minimum length requirement.
///
/// # Arguments
///
/// * `words` - Iterator of words to analyze
/// * `min_len` - Minimum word length to consider
///
/// # Returns
///
/// An Option containing a tuple of (word, frequency) for the most common word,
/// or None if no words meet the criteria.
///
/// # Examples
///
/// ```
/// use algs4_searching::frequency_counter::most_frequent;
///
/// let words = vec!["the", "quick", "brown", "fox", "the", "fox", "the"];
/// let result = most_frequent(&words, 2);
///
/// assert_eq!(result, Some(("the".to_string(), 3)));
/// ```
pub fn most_frequent<S: AsRef<str>>(words: &[S], min_len: usize) -> Option<(String, usize)> {
    let freq = count_frequencies(words, min_len);

    freq.into_iter().max_by_key(|(_, count)| *count)
}

/// Statistics about word frequency analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrequencyStats {
    /// The most frequent word
    pub most_frequent_word: String,
    /// The frequency of the most common word
    pub max_frequency: usize,
    /// Total number of distinct words (meeting length requirement)
    pub distinct_words: usize,
    /// Total number of words (meeting length requirement)
    pub total_words: usize,
}

impl Display for FrequencyStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.most_frequent_word, self.max_frequency)
    }
}

/// Analyzes word frequencies and returns comprehensive statistics.
///
/// # Arguments
///
/// * `words` - Iterator of words to analyze
/// * `min_len` - Minimum word length to consider
///
/// # Returns
///
/// An Option containing FrequencyStats, or None if no words meet the criteria.
///
/// # Examples
///
/// ```
/// use algs4_searching::frequency_counter::analyze_frequencies;
///
/// let text = "the quick brown fox jumps over the lazy dog the fox";
/// let words: Vec<&str> = text.split_whitespace().collect();
/// let stats = analyze_frequencies(&words, 2).unwrap();
///
/// assert_eq!(stats.most_frequent_word, "the");
/// assert_eq!(stats.max_frequency, 3);
/// assert_eq!(stats.distinct_words, 8);
/// assert_eq!(stats.total_words, 11);
/// ```
pub fn analyze_frequencies<S: AsRef<str>>(words: &[S], min_len: usize) -> Option<FrequencyStats> {
    let freq = count_frequencies(words, min_len);

    if freq.is_empty() {
        return None;
    }

    let total_words: usize = freq.values().sum();
    let distinct_words = freq.len();

    let (most_frequent_word, max_frequency) = freq.into_iter().max_by_key(|(_, count)| *count)?;

    Some(FrequencyStats {
        most_frequent_word,
        max_frequency,
        distinct_words,
        total_words,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_frequencies() {
        let words = vec!["the", "quick", "brown", "fox", "the", "fox", "the"];
        let freq = count_frequencies(&words, 2);

        assert_eq!(freq.get("the"), Some(&3));
        assert_eq!(freq.get("fox"), Some(&2));
        assert_eq!(freq.get("quick"), Some(&1));
        assert_eq!(freq.get("brown"), Some(&1));
    }

    #[test]
    fn test_count_frequencies_min_length() {
        let words = vec!["a", "an", "the", "quick", "brown", "fox"];
        let freq = count_frequencies(&words, 3);

        // "a" and "an" should be filtered out
        assert_eq!(freq.get("a"), None);
        assert_eq!(freq.get("an"), None);
        assert_eq!(freq.get("the"), Some(&1));
        assert_eq!(freq.get("quick"), Some(&1));
    }

    #[test]
    fn test_most_frequent() {
        let words = vec!["the", "quick", "brown", "fox", "the", "fox", "the"];
        let result = most_frequent(&words, 2);

        assert_eq!(result, Some(("the".to_string(), 3)));
    }

    #[test]
    fn test_most_frequent_empty() {
        let words: Vec<&str> = vec![];
        let result = most_frequent(&words, 1);

        assert_eq!(result, None);
    }

    #[test]
    fn test_most_frequent_all_filtered() {
        let words = vec!["a", "b", "c"];
        let result = most_frequent(&words, 5);

        assert_eq!(result, None);
    }

    #[test]
    fn test_analyze_frequencies() {
        let text = "the quick brown fox jumps over the lazy dog the fox";
        let words: Vec<&str> = text.split_whitespace().collect();
        let stats = analyze_frequencies(&words, 2).unwrap();

        assert_eq!(stats.most_frequent_word, "the");
        assert_eq!(stats.max_frequency, 3);
        assert_eq!(stats.distinct_words, 8); // the, quick, brown, fox, jumps, over, lazy, dog
        assert_eq!(stats.total_words, 11); // total words with length >= 2
    }

    #[test]
    fn test_analyze_frequencies_case_insensitive() {
        let words = vec!["The", "THE", "the", "Quick", "quick"];
        let stats = analyze_frequencies(&words, 2).unwrap();

        assert_eq!(stats.most_frequent_word, "the");
        assert_eq!(stats.max_frequency, 3);
    }

    #[test]
    fn test_frequency_stats_display() {
        let stats = FrequencyStats {
            most_frequent_word: "hello".to_string(),
            max_frequency: 42,
            distinct_words: 10,
            total_words: 100,
        };

        assert_eq!(format!("{}", stats), "hello 42");
    }

    #[test]
    fn test_large_text() {
        let mut words = Vec::new();
        words.extend(std::iter::repeat_n("common", 100));
        words.extend(std::iter::repeat_n("less", 50));
        words.extend(std::iter::repeat_n("rare", 10));

        let stats = analyze_frequencies(&words, 1).unwrap();
        assert_eq!(stats.most_frequent_word, "common");
        assert_eq!(stats.max_frequency, 100);
        assert_eq!(stats.distinct_words, 3);
        assert_eq!(stats.total_words, 160);
    }
}
