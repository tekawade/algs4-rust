/// KWIK - Keyword in Context
///
/// Generates a keyword-in-context (KWIK) index, showing each occurrence of
/// keywords with surrounding context. Useful for concordance generation and
/// text analysis.
///
/// # Examples
///
/// ```
/// use algs4_searching::kwik::KWIK;
///
/// let text = "the quick brown fox jumps over the lazy dog";
/// let kwik = KWIK::new(text, 5);
///
/// let contexts = kwik.get_contexts("the");
/// assert_eq!(contexts.len(), 2);
/// ```
use std::collections::HashMap;

/// KWIK (Keyword in Context) index.
#[derive(Debug, Clone, Default)]
pub struct KWIK {
    index: HashMap<String, Vec<Context>>,
}

/// Represents a keyword occurrence with its surrounding context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Context {
    /// Text before the keyword
    pub before: String,
    /// The keyword itself
    pub keyword: String,
    /// Text after the keyword
    pub after: String,
    /// Position of the keyword in the original text (word index)
    pub position: usize,
}

impl KWIK {
    /// Creates a new KWIK index from the given text.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to index
    /// * `context_length` - Number of words to include before/after each keyword
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::kwik::KWIK;
    ///
    /// let text = "the quick brown fox";
    /// let kwik = KWIK::new(text, 2);
    /// assert!(!kwik.is_empty());
    /// ```
    pub fn new(text: &str, context_length: usize) -> Self {
        let mut index = HashMap::new();
        let words: Vec<&str> = text.split_whitespace().collect();

        for (i, word) in words.iter().enumerate() {
            let keyword = word.to_lowercase();

            // Extract context
            let before_start = i.saturating_sub(context_length);
            let after_end = (i + 1 + context_length).min(words.len());

            let before = words[before_start..i].join(" ");
            let after = words[i + 1..after_end].join(" ");

            let context = Context {
                before,
                keyword: keyword.clone(),
                after,
                position: i,
            };

            index.entry(keyword).or_insert_with(Vec::new).push(context);
        }

        KWIK { index }
    }

    /// Returns all contexts for a given keyword.
    ///
    /// # Arguments
    ///
    /// * `keyword` - The keyword to look up (case-insensitive)
    ///
    /// # Returns
    ///
    /// A Vec of Context objects showing each occurrence of the keyword.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::kwik::KWIK;
    ///
    /// let text = "the quick brown fox jumps over the lazy dog";
    /// let kwik = KWIK::new(text, 3);
    ///
    /// let contexts = kwik.get_contexts("the");
    /// assert_eq!(contexts.len(), 2);
    /// ```
    pub fn get_contexts(&self, keyword: &str) -> Vec<&Context> {
        let key = keyword.to_lowercase();
        self.index
            .get(&key)
            .map(|contexts| contexts.iter().collect())
            .unwrap_or_default()
    }

    /// Returns the number of times a keyword appears.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::kwik::KWIK;
    ///
    /// let text = "the quick brown fox jumps over the lazy dog";
    /// let kwik = KWIK::new(text, 2);
    ///
    /// assert_eq!(kwik.count("the"), 2);
    /// assert_eq!(kwik.count("fox"), 1);
    /// assert_eq!(kwik.count("cat"), 0);
    /// ```
    pub fn count(&self, keyword: &str) -> usize {
        let key = keyword.to_lowercase();
        self.index.get(&key).map(|v| v.len()).unwrap_or(0)
    }

    /// Returns true if the index contains the given keyword.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::kwik::KWIK;
    ///
    /// let text = "the quick brown fox";
    /// let kwik = KWIK::new(text, 2);
    ///
    /// assert!(kwik.contains("quick"));
    /// assert!(!kwik.contains("slow"));
    /// ```
    pub fn contains(&self, keyword: &str) -> bool {
        let key = keyword.to_lowercase();
        self.index.contains_key(&key)
    }

    /// Returns all keywords in the index.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::kwik::KWIK;
    ///
    /// let text = "the quick brown fox";
    /// let kwik = KWIK::new(text, 2);
    ///
    /// let keywords = kwik.keywords();
    /// assert_eq!(keywords.len(), 4);
    /// ```
    pub fn keywords(&self) -> Vec<&String> {
        self.index.keys().collect()
    }

    /// Returns the total number of unique keywords.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::kwik::KWIK;
    ///
    /// let text = "the quick brown fox jumps over the lazy dog";
    /// let kwik = KWIK::new(text, 2);
    ///
    /// assert_eq!(kwik.size(), 8); // 8 unique words
    /// ```
    pub fn size(&self) -> usize {
        self.index.len()
    }

    /// Returns true if the index is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::kwik::KWIK;
    ///
    /// let kwik = KWIK::new("", 2);
    /// assert!(kwik.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }
}

impl Context {
    /// Formats the context as a single line with the keyword highlighted.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::kwik::{KWIK, Context};
    ///
    /// let context = Context {
    ///     before: "the quick".to_string(),
    ///     keyword: "brown".to_string(),
    ///     after: "fox jumps".to_string(),
    ///     position: 2,
    /// };
    ///
    /// let line = context.format();
    /// assert!(line.contains("brown"));
    /// ```
    pub fn format(&self) -> String {
        format!("{} [{}] {}", self.before, self.keyword, self.after)
    }

    /// Formats the context with a custom keyword marker.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::kwik::Context;
    ///
    /// let context = Context {
    ///     before: "hello".to_string(),
    ///     keyword: "world".to_string(),
    ///     after: "today".to_string(),
    ///     position: 1,
    /// };
    ///
    /// let line = context.format_with("<", ">");
    /// assert_eq!(line, "hello <world> today");
    /// ```
    pub fn format_with(&self, left_marker: &str, right_marker: &str) -> String {
        format!(
            "{} {}{}{} {}",
            self.before, left_marker, self.keyword, right_marker, self.after
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let text = "the quick brown fox";
        let kwik = KWIK::new(text, 2);

        assert!(!kwik.is_empty());
        assert_eq!(kwik.size(), 4);
    }

    #[test]
    fn test_get_contexts() {
        let text = "the quick brown fox jumps over the lazy dog";
        let kwik = KWIK::new(text, 3);

        let contexts = kwik.get_contexts("the");
        assert_eq!(contexts.len(), 2);

        // First "the"
        assert_eq!(contexts[0].before, "");
        assert_eq!(contexts[0].keyword, "the");
        assert!(contexts[0].after.starts_with("quick"));

        // Second "the"
        assert!(contexts[1].before.contains("over"));
        assert_eq!(contexts[1].keyword, "the");
        assert!(contexts[1].after.starts_with("lazy"));
    }

    #[test]
    fn test_count() {
        let text = "the quick brown fox jumps over the lazy dog";
        let kwik = KWIK::new(text, 2);

        assert_eq!(kwik.count("the"), 2);
        assert_eq!(kwik.count("fox"), 1);
        assert_eq!(kwik.count("cat"), 0);
    }

    #[test]
    fn test_contains() {
        let text = "the quick brown fox";
        let kwik = KWIK::new(text, 2);

        assert!(kwik.contains("quick"));
        assert!(kwik.contains("QUICK")); // Case-insensitive
        assert!(!kwik.contains("slow"));
    }

    #[test]
    fn test_keywords() {
        let text = "the quick brown fox";
        let kwik = KWIK::new(text, 2);

        let keywords = kwik.keywords();
        assert_eq!(keywords.len(), 4);
        assert!(keywords.contains(&&"the".to_string()));
        assert!(keywords.contains(&&"quick".to_string()));
        assert!(keywords.contains(&&"brown".to_string()));
        assert!(keywords.contains(&&"fox".to_string()));
    }

    #[test]
    fn test_context_format() {
        let context = Context {
            before: "hello".to_string(),
            keyword: "world".to_string(),
            after: "today".to_string(),
            position: 1,
        };

        let formatted = context.format();
        assert_eq!(formatted, "hello [world] today");
    }

    #[test]
    fn test_context_format_with() {
        let context = Context {
            before: "hello".to_string(),
            keyword: "world".to_string(),
            after: "today".to_string(),
            position: 1,
        };

        let formatted = context.format_with("<", ">");
        assert_eq!(formatted, "hello <world> today");
    }

    #[test]
    fn test_empty_text() {
        let kwik = KWIK::new("", 2);
        assert!(kwik.is_empty());
        assert_eq!(kwik.size(), 0);
    }

    #[test]
    fn test_single_word() {
        let kwik = KWIK::new("hello", 2);
        assert_eq!(kwik.size(), 1);

        let contexts = kwik.get_contexts("hello");
        assert_eq!(contexts.len(), 1);
        assert_eq!(contexts[0].before, "");
        assert_eq!(contexts[0].after, "");
    }

    #[test]
    fn test_case_insensitive() {
        let text = "The Quick Brown FOX";
        let kwik = KWIK::new(text, 2);

        assert_eq!(kwik.count("the"), 1);
        assert_eq!(kwik.count("THE"), 1);
        assert_eq!(kwik.count("fox"), 1);
        assert_eq!(kwik.count("FOX"), 1);
    }

    #[test]
    fn test_context_length() {
        let text = "a b c d e f g h i j";
        let kwik = KWIK::new(text, 2);

        let contexts = kwik.get_contexts("e");
        assert_eq!(contexts.len(), 1);

        let context = contexts[0];
        assert_eq!(context.before, "c d");
        assert_eq!(context.after, "f g");
    }
}
