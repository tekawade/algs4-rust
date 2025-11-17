//! Alphabet data type for working with different character sets.
//!
//! The Alphabet data type provides a way to work with a specific alphabet,
//! converting between characters and indices. This is particularly useful
//! for string algorithms that need to work with character arrays or tries.

use std::collections::HashMap;
use std::fmt;

/// Represents an alphabet - a finite set of characters.
///
/// An alphabet defines a mapping between characters and indices (0 to R-1,
/// where R is the radix or size of the alphabet).
///
/// # Examples
///
/// ```
/// use algs4_strings::alphabet::Alphabet;
///
/// // Create a binary alphabet
/// let binary = Alphabet::binary();
/// assert_eq!(binary.radix(), 2);
/// assert!(binary.contains('0'));
/// assert!(!binary.contains('2'));
///
/// // Convert between characters and indices
/// assert_eq!(binary.to_index('0'), 0);
/// assert_eq!(binary.to_index('1'), 1);
/// assert_eq!(binary.to_char(0), '0');
/// assert_eq!(binary.to_char(1), '1');
/// ```
#[derive(Clone, Debug)]
pub struct Alphabet {
    chars: Vec<char>,
    inverse: HashMap<char, usize>,
    radix: usize,
}

impl Alphabet {
    /// Creates a new alphabet from the given characters.
    ///
    /// # Panics
    ///
    /// Panics if the alphabet contains duplicate characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::alphabet::Alphabet;
    ///
    /// let dna = Alphabet::new("ACGT");
    /// assert_eq!(dna.radix(), 4);
    /// ```
    pub fn new(s: &str) -> Self {
        let chars: Vec<char> = s.chars().collect();
        let radix = chars.len();

        let mut inverse = HashMap::new();
        for (i, &c) in chars.iter().enumerate() {
            if inverse.insert(c, i).is_some() {
                panic!("Alphabet contains duplicate character: '{}'", c);
            }
        }

        Alphabet {
            chars,
            inverse,
            radix,
        }
    }

    /// Creates a binary alphabet (0-1).
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::alphabet::Alphabet;
    ///
    /// let binary = Alphabet::binary();
    /// assert_eq!(binary.radix(), 2);
    /// ```
    pub fn binary() -> Self {
        Self::new("01")
    }

    /// Creates an octal alphabet (0-7).
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::alphabet::Alphabet;
    ///
    /// let octal = Alphabet::octal();
    /// assert_eq!(octal.radix(), 8);
    /// ```
    pub fn octal() -> Self {
        Self::new("01234567")
    }

    /// Creates a decimal alphabet (0-9).
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::alphabet::Alphabet;
    ///
    /// let decimal = Alphabet::decimal();
    /// assert_eq!(decimal.radix(), 10);
    /// ```
    pub fn decimal() -> Self {
        Self::new("0123456789")
    }

    /// Creates a hexadecimal alphabet (0-9A-F).
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::alphabet::Alphabet;
    ///
    /// let hex = Alphabet::hexadecimal();
    /// assert_eq!(hex.radix(), 16);
    /// ```
    pub fn hexadecimal() -> Self {
        Self::new("0123456789ABCDEF")
    }

    /// Creates a DNA alphabet (ACGT).
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::alphabet::Alphabet;
    ///
    /// let dna = Alphabet::dna();
    /// assert_eq!(dna.radix(), 4);
    /// ```
    pub fn dna() -> Self {
        Self::new("ACGT")
    }

    /// Creates a lowercase alphabet (a-z).
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::alphabet::Alphabet;
    ///
    /// let lowercase = Alphabet::lowercase();
    /// assert_eq!(lowercase.radix(), 26);
    /// ```
    pub fn lowercase() -> Self {
        Self::new("abcdefghijklmnopqrstuvwxyz")
    }

    /// Creates an uppercase alphabet (A-Z).
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::alphabet::Alphabet;
    ///
    /// let uppercase = Alphabet::uppercase();
    /// assert_eq!(uppercase.radix(), 26);
    /// ```
    pub fn uppercase() -> Self {
        Self::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
    }

    /// Creates a protein alphabet (20 amino acids).
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::alphabet::Alphabet;
    ///
    /// let protein = Alphabet::protein();
    /// assert_eq!(protein.radix(), 20);
    /// ```
    pub fn protein() -> Self {
        Self::new("ACDEFGHIKLMNPQRSTVWY")
    }

    /// Creates an extended ASCII alphabet (0-127).
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::alphabet::Alphabet;
    ///
    /// let ascii = Alphabet::ascii();
    /// assert_eq!(ascii.radix(), 128);
    /// ```
    pub fn ascii() -> Self {
        let mut s = String::new();
        for i in 0..128 {
            s.push(char::from_u32(i).unwrap());
        }
        Self::new(&s)
    }

    /// Creates an extended ASCII alphabet (0-255).
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::alphabet::Alphabet;
    ///
    /// let extended_ascii = Alphabet::extended_ascii();
    /// assert_eq!(extended_ascii.radix(), 256);
    /// ```
    pub fn extended_ascii() -> Self {
        let mut s = String::new();
        for i in 0..256 {
            s.push(char::from_u32(i).unwrap());
        }
        Self::new(&s)
    }

    /// Creates a Unicode16 alphabet (0-65535).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use algs4_strings::alphabet::Alphabet;
    ///
    /// let unicode16 = Alphabet::unicode16();
    /// assert_eq!(unicode16.radix(), 65536);
    /// ```
    pub fn unicode16() -> Self {
        let mut s = String::new();
        for i in 0..65536 {
            s.push(char::from_u32(i).unwrap());
        }
        Self::new(&s)
    }

    /// Returns the character at the given index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::alphabet::Alphabet;
    ///
    /// let binary = Alphabet::binary();
    /// assert_eq!(binary.to_char(0), '0');
    /// assert_eq!(binary.to_char(1), '1');
    /// ```
    pub fn to_char(&self, index: usize) -> char {
        if index >= self.radix {
            panic!(
                "Index {} is out of bounds for alphabet of size {}",
                index, self.radix
            );
        }
        self.chars[index]
    }

    /// Returns the index of the given character.
    ///
    /// # Panics
    ///
    /// Panics if the character is not in the alphabet.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::alphabet::Alphabet;
    ///
    /// let dna = Alphabet::dna();
    /// assert_eq!(dna.to_index('A'), 0);
    /// assert_eq!(dna.to_index('C'), 1);
    /// ```
    pub fn to_index(&self, c: char) -> usize {
        *self
            .inverse
            .get(&c)
            .unwrap_or_else(|| panic!("Character '{}' is not in the alphabet", c))
    }

    /// Returns whether the alphabet contains the given character.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::alphabet::Alphabet;
    ///
    /// let binary = Alphabet::binary();
    /// assert!(binary.contains('0'));
    /// assert!(binary.contains('1'));
    /// assert!(!binary.contains('2'));
    /// ```
    pub fn contains(&self, c: char) -> bool {
        self.inverse.contains_key(&c)
    }

    /// Returns the radix (number of characters) in the alphabet.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::alphabet::Alphabet;
    ///
    /// let dna = Alphabet::dna();
    /// assert_eq!(dna.radix(), 4);
    /// ```
    pub fn radix(&self) -> usize {
        self.radix
    }

    /// Returns the number of bits needed to represent an index.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::alphabet::Alphabet;
    ///
    /// let binary = Alphabet::binary();
    /// assert_eq!(binary.lg_radix(), 1);
    ///
    /// let dna = Alphabet::dna();
    /// assert_eq!(dna.lg_radix(), 2);
    /// ```
    pub fn lg_radix(&self) -> usize {
        let mut lg = 0;
        let mut r = self.radix - 1;
        while r > 0 {
            lg += 1;
            r >>= 1;
        }
        lg
    }

    /// Converts a string to an array of indices.
    ///
    /// # Panics
    ///
    /// Panics if any character in the string is not in the alphabet.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::alphabet::Alphabet;
    ///
    /// let dna = Alphabet::dna();
    /// let indices = dna.to_indices("ACGT");
    /// assert_eq!(indices, vec![0, 1, 2, 3]);
    /// ```
    pub fn to_indices(&self, s: &str) -> Vec<usize> {
        s.chars().map(|c| self.to_index(c)).collect()
    }

    /// Converts an array of indices to a string.
    ///
    /// # Panics
    ///
    /// Panics if any index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::alphabet::Alphabet;
    ///
    /// let dna = Alphabet::dna();
    /// let s = dna.to_string(&[0, 1, 2, 3]);
    /// assert_eq!(s, "ACGT");
    /// ```
    pub fn to_string(&self, indices: &[usize]) -> String {
        indices.iter().map(|&i| self.to_char(i)).collect()
    }
}

impl fmt::Display for Alphabet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.chars.iter().collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary() {
        let alpha = Alphabet::binary();
        assert_eq!(alpha.radix(), 2);
        assert_eq!(alpha.to_char(0), '0');
        assert_eq!(alpha.to_char(1), '1');
        assert_eq!(alpha.to_index('0'), 0);
        assert_eq!(alpha.to_index('1'), 1);
        assert!(alpha.contains('0'));
        assert!(!alpha.contains('2'));
    }

    #[test]
    fn test_dna() {
        let alpha = Alphabet::dna();
        assert_eq!(alpha.radix(), 4);
        assert_eq!(alpha.to_char(0), 'A');
        assert_eq!(alpha.to_char(1), 'C');
        assert_eq!(alpha.to_char(2), 'G');
        assert_eq!(alpha.to_char(3), 'T');
    }

    #[test]
    fn test_to_indices() {
        let alpha = Alphabet::dna();
        let indices = alpha.to_indices("ACGT");
        assert_eq!(indices, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_to_string() {
        let alpha = Alphabet::dna();
        let s = alpha.to_string(&[0, 1, 2, 3]);
        assert_eq!(s, "ACGT");
    }

    #[test]
    fn test_lg_radix() {
        assert_eq!(Alphabet::binary().lg_radix(), 1);
        assert_eq!(Alphabet::dna().lg_radix(), 2);
        assert_eq!(Alphabet::octal().lg_radix(), 3);
        assert_eq!(Alphabet::hexadecimal().lg_radix(), 4);
    }

    #[test]
    #[should_panic(expected = "duplicate character")]
    fn test_duplicate_chars() {
        Alphabet::new("AAB");
    }

    #[test]
    #[should_panic(expected = "not in the alphabet")]
    fn test_invalid_char() {
        let alpha = Alphabet::binary();
        alpha.to_index('2');
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn test_invalid_index() {
        let alpha = Alphabet::binary();
        alpha.to_char(5);
    }
}
