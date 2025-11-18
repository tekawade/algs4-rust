//! Transaction data type for commercial transactions.
//!
//! An immutable data type representing a commercial transaction
//! with a customer name, date, and amount.
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

/// Represents a commercial transaction with customer, date, and amount.
///
/// # Examples
///
/// ```
/// use algs4_fundamentals::util::Transaction;
///
/// let t1 = Transaction::new("Alice".to_string(), "3/1/2000".to_string(), 100.0);
/// let t2 = Transaction::new("Bob".to_string(), "2/15/2000".to_string(), 200.0);
///
/// // Transactions are ordered by amount
/// assert!(t2 > t1);
/// assert_eq!(t1.who(), "Alice");
/// assert_eq!(t1.amount(), 100.0);
/// ```
#[derive(Debug, Clone)]
pub struct Transaction {
    who: String,
    when: String,
    amount: f64,
}

impl Transaction {
    /// Creates a new transaction.
    ///
    /// # Arguments
    ///
    /// * `who` - The customer name
    /// * `when` - The date (as a string)
    /// * `amount` - The transaction amount
    ///
    /// # Panics
    ///
    /// Panics if amount is NaN or infinite.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::util::Transaction;
    ///
    /// let transaction = Transaction::new("Alice".to_string(), "3/1/2000".to_string(), 100.0);
    /// assert_eq!(transaction.amount(), 100.0);
    /// ```
    pub fn new(who: String, when: String, amount: f64) -> Self {
        if amount.is_nan() || amount.is_infinite() {
            panic!("Amount cannot be NaN or infinite");
        }
        Transaction { who, when, amount }
    }

    /// Creates a transaction by parsing a string.
    ///
    /// The string should contain: name date amount (whitespace separated)
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_fundamentals::util::Transaction;
    ///
    /// let transaction = Transaction::from_string("Alice 3/1/2000 100.00");
    /// assert_eq!(transaction.who(), "Alice");
    /// assert_eq!(transaction.when(), "3/1/2000");
    /// assert_eq!(transaction.amount(), 100.0);
    /// ```
    pub fn from_string(s: &str) -> Self {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 3 {
            panic!("Invalid transaction format: expected 'who when amount'");
        }

        let who = parts[0].to_string();
        let when = parts[1].to_string();
        let amount = parts[2].parse::<f64>().expect("Invalid amount format");

        Transaction::new(who, when, amount)
    }

    /// Returns the customer name.
    pub fn who(&self) -> &str {
        &self.who
    }

    /// Returns the transaction date.
    pub fn when(&self) -> &str {
        &self.when
    }

    /// Returns the transaction amount.
    pub fn amount(&self) -> f64 {
        self.amount
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:<10} {:>10} {:>8.2}", self.who, self.when, self.amount)
    }
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.who == other.who
            && self.when == other.when
            && (self.amount - other.amount).abs() < f64::EPSILON
    }
}

impl Eq for Transaction {}

impl Hash for Transaction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.who.hash(state);
        self.when.hash(state);
        // For f64, we use the bits representation for hashing
        self.amount.to_bits().hash(state);
    }
}

impl PartialOrd for Transaction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Transaction {
    /// Compare transactions by amount.
    fn cmp(&self, other: &Self) -> Ordering {
        self.amount
            .partial_cmp(&other.amount)
            .unwrap_or(Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let t = Transaction::new("Alice".to_string(), "3/1/2000".to_string(), 100.0);
        assert_eq!(t.who(), "Alice");
        assert_eq!(t.when(), "3/1/2000");
        assert_eq!(t.amount(), 100.0);
    }

    #[test]
    #[should_panic(expected = "Amount cannot be NaN or infinite")]
    fn test_new_nan() {
        Transaction::new("Alice".to_string(), "3/1/2000".to_string(), f64::NAN);
    }

    #[test]
    #[should_panic(expected = "Amount cannot be NaN or infinite")]
    fn test_new_infinite() {
        Transaction::new("Alice".to_string(), "3/1/2000".to_string(), f64::INFINITY);
    }

    #[test]
    fn test_from_string() {
        let t = Transaction::from_string("Alice 3/1/2000 100.00");
        assert_eq!(t.who(), "Alice");
        assert_eq!(t.when(), "3/1/2000");
        assert_eq!(t.amount(), 100.0);
    }

    #[test]
    fn test_display() {
        let t = Transaction::new("Alice".to_string(), "3/1/2000".to_string(), 100.0);
        let s = format!("{}", t);
        assert!(s.contains("Alice"));
        assert!(s.contains("3/1/2000"));
        assert!(s.contains("100.00"));
    }

    #[test]
    fn test_equality() {
        let t1 = Transaction::new("Alice".to_string(), "3/1/2000".to_string(), 100.0);
        let t2 = Transaction::new("Alice".to_string(), "3/1/2000".to_string(), 100.0);
        let t3 = Transaction::new("Bob".to_string(), "3/1/2000".to_string(), 100.0);

        assert_eq!(t1, t2);
        assert_ne!(t1, t3);
    }

    #[test]
    fn test_ordering() {
        let t1 = Transaction::new("Alice".to_string(), "3/1/2000".to_string(), 100.0);
        let t2 = Transaction::new("Bob".to_string(), "2/15/2000".to_string(), 200.0);
        let t3 = Transaction::new("Charlie".to_string(), "1/10/2000".to_string(), 50.0);

        assert!(t2 > t1);
        assert!(t1 > t3);
        assert!(t2 > t3);
    }

    #[test]
    fn test_sorting() {
        let mut transactions = vec![
            Transaction::new("Alice".to_string(), "3/1/2000".to_string(), 100.0),
            Transaction::new("Bob".to_string(), "2/15/2000".to_string(), 200.0),
            Transaction::new("Charlie".to_string(), "1/10/2000".to_string(), 50.0),
        ];

        transactions.sort();

        assert_eq!(transactions[0].amount(), 50.0);
        assert_eq!(transactions[1].amount(), 100.0);
        assert_eq!(transactions[2].amount(), 200.0);
    }
}
