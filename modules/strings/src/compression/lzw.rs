//! LZW (Lempel-Ziv-Welch) compression algorithm.
//!
//! LZW is a dictionary-based compression algorithm that builds a dictionary
//! of strings dynamically during compression and decompression.

use std::collections::HashMap;

/// LZW compression and decompression.
///
/// Uses a dictionary-based approach where common strings are replaced with
/// shorter codes.
///
/// # Examples
///
/// ```
/// use algs4_strings::compression::LZW;
///
/// let data = b"TOBEORNOTTOBEORTOBEORNOT";
/// let compressed = LZW::compress(data);
/// let decompressed = LZW::decompress(&compressed);
/// assert_eq!(decompressed, data);
/// ```
#[derive(Debug)]
pub struct LZW;

impl LZW {
    const RADIX: usize = 256; // Number of input characters
    const CODE_WIDTH: usize = 12; // Code width (bits)
    const MAX_CODE: usize = 4096; // 2^12

    /// Compresses data using LZW algorithm.
    ///
    /// Returns a vector of 12-bit codes encoded as bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::compression::LZW;
    ///
    /// let data = b"TOBEORNOTTOBEORTOBEORNOT";
    /// let compressed = LZW::compress(data);
    /// assert!(compressed.len() > 0);
    /// ```
    pub fn compress(data: &[u8]) -> Vec<u8> {
        if data.is_empty() {
            return Vec::new();
        }

        // Build initial dictionary with single characters
        let mut dict = HashMap::new();
        for i in 0..Self::RADIX {
            dict.insert(vec![i as u8], i);
        }

        let mut next_code = Self::RADIX;
        let mut current = Vec::new();
        let mut codes = Vec::new();

        for &byte in data {
            let mut next = current.clone();
            next.push(byte);

            if dict.contains_key(&next) {
                current = next;
            } else {
                // Output code for current
                codes.push(dict[&current]);

                // Add new string to dictionary
                if next_code < Self::MAX_CODE {
                    dict.insert(next, next_code);
                    next_code += 1;
                }

                current = vec![byte];
            }
        }

        // Output code for remaining string
        if !current.is_empty() {
            codes.push(dict[&current]);
        }

        // Convert codes to bytes
        Self::codes_to_bytes(&codes)
    }

    /// Decompresses LZW-encoded data.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::compression::LZW;
    ///
    /// let data = b"TOBEORNOTTOBEORTOBEORNOT";
    /// let compressed = LZW::compress(data);
    /// let decompressed = LZW::decompress(&compressed);
    /// assert_eq!(decompressed, data);
    /// ```
    pub fn decompress(compressed: &[u8]) -> Vec<u8> {
        if compressed.is_empty() {
            return Vec::new();
        }

        let codes = Self::bytes_to_codes(compressed);
        if codes.is_empty() {
            return Vec::new();
        }

        // Build initial dictionary with single characters
        let mut dict = HashMap::new();
        for i in 0..Self::RADIX {
            dict.insert(i, vec![i as u8]);
        }

        let mut next_code = Self::RADIX;
        let mut result = Vec::new();

        // Read first code
        let first_code = codes[0];
        let mut current = dict[&first_code].clone();
        result.extend_from_slice(&current);

        for &code in &codes[1..] {
            let entry = if dict.contains_key(&code) {
                dict[&code].clone()
            } else if code == next_code {
                // Special case: code not in dictionary yet
                let mut entry = current.clone();
                entry.push(current[0]);
                entry
            } else {
                panic!("Invalid LZW code: {}", code);
            };

            result.extend_from_slice(&entry);

            // Add new string to dictionary
            if next_code < Self::MAX_CODE {
                let mut new_entry = current;
                new_entry.push(entry[0]);
                dict.insert(next_code, new_entry);
                next_code += 1;
            }

            current = entry;
        }

        result
    }

    /// Converts 12-bit codes to bytes for storage.
    fn codes_to_bytes(codes: &[usize]) -> Vec<u8> {
        let mut result = Vec::new();
        let mut bits = String::new();

        for &code in codes {
            // Convert code to 12-bit binary string
            for i in (0..Self::CODE_WIDTH).rev() {
                bits.push(if (code >> i) & 1 == 1 { '1' } else { '0' });
            }
        }

        // Convert bits to bytes
        let mut byte = 0u8;
        let mut count = 0;

        for bit in bits.chars() {
            byte = (byte << 1) | if bit == '1' { 1 } else { 0 };
            count += 1;
            if count == 8 {
                result.push(byte);
                byte = 0;
                count = 0;
            }
        }

        if count > 0 {
            byte <<= 8 - count;
            result.push(byte);
        }

        result
    }

    /// Converts bytes back to 12-bit codes.
    fn bytes_to_codes(bytes: &[u8]) -> Vec<usize> {
        let mut bits = String::new();

        // Convert bytes to bit string
        for &byte in bytes {
            for i in (0..8).rev() {
                bits.push(if (byte >> i) & 1 == 1 { '1' } else { '0' });
            }
        }

        // Parse 12-bit codes
        let mut codes = Vec::new();
        let mut i = 0;

        while i + Self::CODE_WIDTH <= bits.len() {
            let code_str = &bits[i..i + Self::CODE_WIDTH];
            let mut code = 0usize;

            for bit in code_str.chars() {
                code = (code << 1) | if bit == '1' { 1 } else { 0 };
            }

            codes.push(code);
            i += Self::CODE_WIDTH;
        }

        codes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lzw_basic() {
        let data = b"TOBEORNOTTOBEORTOBEORNOT";
        let compressed = LZW::compress(data);
        let decompressed = LZW::decompress(&compressed);
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_lzw_simple() {
        let data = b"ABABABA";
        let compressed = LZW::compress(data);
        let decompressed = LZW::decompress(&compressed);
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_lzw_single_char() {
        let data = b"A";
        let compressed = LZW::compress(data);
        let decompressed = LZW::decompress(&compressed);
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_lzw_repeated() {
        let data = b"AAAAAAAAAA";
        let compressed = LZW::compress(data);
        let decompressed = LZW::decompress(&compressed);
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_lzw_text() {
        let data = b"the quick brown fox jumps over the lazy dog";
        let compressed = LZW::compress(data);
        let decompressed = LZW::decompress(&compressed);
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_lzw_empty() {
        let data = b"";
        let compressed = LZW::compress(data);
        let decompressed = LZW::decompress(&compressed);
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_lzw_textbook_example() {
        // Example from the textbook
        let data = b"ABABABABABABABABABA";
        let compressed = LZW::compress(data);
        let decompressed = LZW::decompress(&compressed);
        assert_eq!(decompressed, data);
    }
}
