/// Lookup CSV
///
/// A utility for performing lookups in CSV (comma-separated value) data.
/// Maps keys to values from CSV-like data structures.
///
/// # Examples
///
/// ```
/// use algs4_searching::lookup_csv::LookupCSV;
///
/// let mut lookup = LookupCSV::new(',');
/// lookup.add_row(vec!["apple", "fruit", "red"]);
/// lookup.add_row(vec!["carrot", "vegetable", "orange"]);
///
/// let value = lookup.get(1, "apple");
/// assert_eq!(value, Some(&"fruit".to_string()));
/// ```
use std::collections::HashMap;
use std::fmt::{self, Display};

/// Lookup CSV data structure for key-value lookups from CSV-like data.
#[derive(Debug, Clone)]
pub struct LookupCSV {
    separator: char,
    data: HashMap<String, Vec<String>>,
    key_column: usize,
}

impl LookupCSV {
    /// Creates a new CSV lookup with the specified separator character.
    ///
    /// # Arguments
    ///
    /// * `separator` - The character used to separate fields (e.g., ',' or '\t')
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::lookup_csv::LookupCSV;
    ///
    /// let lookup = LookupCSV::new(',');
    /// assert!(lookup.is_empty());
    /// ```
    pub fn new(separator: char) -> Self {
        LookupCSV {
            separator,
            data: HashMap::new(),
            key_column: 0,
        }
    }

    /// Sets the column index to use as the key for lookups.
    ///
    /// # Arguments
    ///
    /// * `column` - The zero-indexed column number to use as keys
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::lookup_csv::LookupCSV;
    ///
    /// let mut lookup = LookupCSV::new(',');
    /// lookup.set_key_column(1); // Use second column as key
    /// ```
    pub fn set_key_column(&mut self, column: usize) {
        self.key_column = column;
    }

    /// Adds a row of data to the lookup table.
    ///
    /// # Arguments
    ///
    /// * `fields` - Vector of field values for this row
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::lookup_csv::LookupCSV;
    ///
    /// let mut lookup = LookupCSV::new(',');
    /// lookup.add_row(vec!["key1", "value1", "value2"]);
    /// ```
    pub fn add_row<S: AsRef<str>>(&mut self, fields: Vec<S>) {
        if fields.len() <= self.key_column {
            return; // Invalid row
        }

        let key = fields[self.key_column].as_ref().to_string();
        let values: Vec<String> = fields.iter().map(|s| s.as_ref().to_string()).collect();

        self.data.insert(key, values);
    }

    /// Parses and adds a CSV line to the lookup table.
    ///
    /// # Arguments
    ///
    /// * `line` - A CSV-formatted string
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::lookup_csv::LookupCSV;
    ///
    /// let mut lookup = LookupCSV::new(',');
    /// lookup.add_line("apple,fruit,red");
    /// assert_eq!(lookup.get(1, "apple"), Some(&"fruit".to_string()));
    /// ```
    pub fn add_line(&mut self, line: &str) {
        let fields: Vec<&str> = line.split(self.separator).collect();
        self.add_row(fields);
    }

    /// Retrieves a specific field value for a given key.
    ///
    /// # Arguments
    ///
    /// * `field_index` - The column index of the desired field
    /// * `key` - The key to look up
    ///
    /// # Returns
    ///
    /// An Option containing a reference to the field value, or None if not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::lookup_csv::LookupCSV;
    ///
    /// let mut lookup = LookupCSV::new(',');
    /// lookup.add_row(vec!["apple", "fruit", "red"]);
    ///
    /// assert_eq!(lookup.get(1, "apple"), Some(&"fruit".to_string()));
    /// assert_eq!(lookup.get(2, "apple"), Some(&"red".to_string()));
    /// assert_eq!(lookup.get(1, "banana"), None);
    /// ```
    pub fn get(&self, field_index: usize, key: &str) -> Option<&String> {
        self.data.get(key).and_then(|row| row.get(field_index))
    }

    /// Retrieves all fields for a given key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up
    ///
    /// # Returns
    ///
    /// An Option containing a reference to the row data, or None if not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::lookup_csv::LookupCSV;
    ///
    /// let mut lookup = LookupCSV::new(',');
    /// lookup.add_row(vec!["apple", "fruit", "red"]);
    ///
    /// let row = lookup.get_row("apple");
    /// assert_eq!(row.unwrap().len(), 3);
    /// ```
    pub fn get_row(&self, key: &str) -> Option<&Vec<String>> {
        self.data.get(key)
    }

    /// Returns true if the lookup contains the given key.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::lookup_csv::LookupCSV;
    ///
    /// let mut lookup = LookupCSV::new(',');
    /// lookup.add_row(vec!["apple", "fruit"]);
    ///
    /// assert!(lookup.contains("apple"));
    /// assert!(!lookup.contains("banana"));
    /// ```
    pub fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    /// Returns the number of rows in the lookup table.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::lookup_csv::LookupCSV;
    ///
    /// let mut lookup = LookupCSV::new(',');
    /// lookup.add_row(vec!["key1", "val1"]);
    /// lookup.add_row(vec!["key2", "val2"]);
    ///
    /// assert_eq!(lookup.size(), 2);
    /// ```
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the lookup table is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::lookup_csv::LookupCSV;
    ///
    /// let lookup = LookupCSV::new(',');
    /// assert!(lookup.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns all keys in the lookup table.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_searching::lookup_csv::LookupCSV;
    ///
    /// let mut lookup = LookupCSV::new(',');
    /// lookup.add_row(vec!["apple", "fruit"]);
    /// lookup.add_row(vec!["carrot", "vegetable"]);
    ///
    /// let keys = lookup.keys();
    /// assert_eq!(keys.len(), 2);
    /// ```
    pub fn keys(&self) -> Vec<&String> {
        self.data.keys().collect()
    }
}

impl Default for LookupCSV {
    fn default() -> Self {
        Self::new(',')
    }
}

impl Display for LookupCSV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (key, row) in &self.data {
            write!(f, "{}: ", key)?;
            for (i, field) in row.iter().enumerate() {
                if i > 0 {
                    write!(f, "{}", self.separator)?;
                }
                write!(f, "{}", field)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let lookup = LookupCSV::new(',');
        assert!(lookup.is_empty());
        assert_eq!(lookup.size(), 0);
    }

    #[test]
    fn test_add_row_and_get() {
        let mut lookup = LookupCSV::new(',');
        lookup.add_row(vec!["apple", "fruit", "red"]);
        lookup.add_row(vec!["carrot", "vegetable", "orange"]);

        assert_eq!(lookup.get(1, "apple"), Some(&"fruit".to_string()));
        assert_eq!(lookup.get(2, "apple"), Some(&"red".to_string()));
        assert_eq!(lookup.get(1, "carrot"), Some(&"vegetable".to_string()));
        assert_eq!(lookup.get(2, "carrot"), Some(&"orange".to_string()));
    }

    #[test]
    fn test_add_line() {
        let mut lookup = LookupCSV::new(',');
        lookup.add_line("apple,fruit,red");
        lookup.add_line("carrot,vegetable,orange");

        assert_eq!(lookup.get(1, "apple"), Some(&"fruit".to_string()));
        assert_eq!(lookup.get(1, "carrot"), Some(&"vegetable".to_string()));
    }

    #[test]
    fn test_set_key_column() {
        let mut lookup = LookupCSV::new(',');
        lookup.set_key_column(1); // Use second column as key
        lookup.add_row(vec!["fruit", "apple", "red"]);

        assert_eq!(lookup.get(0, "apple"), Some(&"fruit".to_string()));
        assert_eq!(lookup.get(2, "apple"), Some(&"red".to_string()));
    }

    #[test]
    fn test_get_row() {
        let mut lookup = LookupCSV::new(',');
        lookup.add_row(vec!["apple", "fruit", "red"]);

        let row = lookup.get_row("apple");
        assert!(row.is_some());
        assert_eq!(row.unwrap().len(), 3);
        assert_eq!(row.unwrap()[1], "fruit");
    }

    #[test]
    fn test_contains() {
        let mut lookup = LookupCSV::new(',');
        lookup.add_row(vec!["apple", "fruit"]);

        assert!(lookup.contains("apple"));
        assert!(!lookup.contains("banana"));
    }

    #[test]
    fn test_size() {
        let mut lookup = LookupCSV::new(',');
        assert_eq!(lookup.size(), 0);

        lookup.add_row(vec!["key1", "val1"]);
        assert_eq!(lookup.size(), 1);

        lookup.add_row(vec!["key2", "val2"]);
        assert_eq!(lookup.size(), 2);
    }

    #[test]
    fn test_keys() {
        let mut lookup = LookupCSV::new(',');
        lookup.add_row(vec!["apple", "fruit"]);
        lookup.add_row(vec!["carrot", "vegetable"]);

        let keys = lookup.keys();
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&&"apple".to_string()));
        assert!(keys.contains(&&"carrot".to_string()));
    }

    #[test]
    fn test_tab_separator() {
        let mut lookup = LookupCSV::new('\t');
        lookup.add_line("apple\tfruit\tred");

        assert_eq!(lookup.get(1, "apple"), Some(&"fruit".to_string()));
    }

    #[test]
    fn test_overwrite_key() {
        let mut lookup = LookupCSV::new(',');
        lookup.add_row(vec!["key", "value1"]);
        lookup.add_row(vec!["key", "value2"]);

        // Second row should overwrite first
        assert_eq!(lookup.get(1, "key"), Some(&"value2".to_string()));
        assert_eq!(lookup.size(), 1);
    }
}
