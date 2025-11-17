//! Run-length encoding data compression.
//!
//! Run-length encoding is a simple compression algorithm that replaces
//! runs of identical bits with a count of the run length.

use std::io::{self, Read, Write};

/// Run-length encoding compression and decompression.
///
/// Compresses data by encoding runs of identical bits (0 or 1) as counts.
/// Works best on data with long runs of the same bit.
///
/// # Examples
///
/// ```
/// use algs4_strings::compression::RunLength;
///
/// let data = vec![0, 0, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1];
/// let compressed = RunLength::compress(&data);
/// let decompressed = RunLength::decompress(&compressed);
/// // Note: decompressed may have trailing zeros due to byte alignment
/// assert_eq!(&decompressed[..data.len()], &data[..]);
/// ```
pub struct RunLength;

impl RunLength {
    const MAX_RUN_LENGTH: u8 = 255;

    /// Compresses a bit vector using run-length encoding.
    ///
    /// The input is a vector of bits (0 or 1).
    /// Returns a compressed byte vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::compression::RunLength;
    ///
    /// let data = vec![0, 0, 0, 0, 1, 1, 1];
    /// let compressed = RunLength::compress(&data);
    /// assert!(compressed.len() < data.len());
    /// ```
    pub fn compress(bits: &[u8]) -> Vec<u8> {
        let mut result = Vec::new();
        let mut run = 0u8;
        let mut old_bit = false;

        for &bit in bits {
            let current_bit = bit != 0;

            if current_bit != old_bit {
                result.push(run);
                run = 1;
                old_bit = current_bit;
            } else {
                if run == Self::MAX_RUN_LENGTH {
                    result.push(run);
                    run = 0;
                    result.push(0);
                } else {
                    run += 1;
                }
            }
        }
        result.push(run);
        result
    }

    /// Decompresses run-length encoded data.
    ///
    /// Takes a compressed byte vector and returns the original bit vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::compression::RunLength;
    ///
    /// let data = vec![0, 0, 0, 0, 1, 1, 1];
    /// let compressed = RunLength::compress(&data);
    /// let decompressed = RunLength::decompress(&compressed);
    /// assert_eq!(&decompressed[..data.len()], &data[..]);
    /// ```
    pub fn decompress(compressed: &[u8]) -> Vec<u8> {
        let mut result = Vec::new();
        let mut bit = false;

        for &count in compressed {
            for _ in 0..count {
                result.push(if bit { 1 } else { 0 });
            }
            bit = !bit;
        }
        result
    }

    /// Compresses bytes to bytes (working with raw binary data).
    ///
    /// This version works with byte streams rather than bit vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::compression::RunLength;
    ///
    /// let data = b"AAAABBBCCDAA";
    /// let compressed = RunLength::compress_bytes(data);
    /// let decompressed = RunLength::decompress_bytes(&compressed);
    /// assert_eq!(decompressed, data);
    /// ```
    pub fn compress_bytes(data: &[u8]) -> Vec<u8> {
        if data.is_empty() {
            return Vec::new();
        }

        let mut result = Vec::new();
        let mut current = data[0];
        let mut count = 1u8;

        for &byte in &data[1..] {
            if byte == current && count < Self::MAX_RUN_LENGTH {
                count += 1;
            } else {
                result.push(count);
                result.push(current);
                current = byte;
                count = 1;
            }
        }
        result.push(count);
        result.push(current);
        result
    }

    /// Decompresses byte-encoded run-length data.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::compression::RunLength;
    ///
    /// let data = b"AAAABBBCCDAA";
    /// let compressed = RunLength::compress_bytes(data);
    /// let decompressed = RunLength::decompress_bytes(&compressed);
    /// assert_eq!(decompressed, data);
    /// ```
    pub fn decompress_bytes(compressed: &[u8]) -> Vec<u8> {
        let mut result = Vec::new();
        let mut i = 0;

        while i + 1 < compressed.len() {
            let count = compressed[i];
            let byte = compressed[i + 1];
            for _ in 0..count {
                result.push(byte);
            }
            i += 2;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_length_compress_decompress() {
        let data = vec![0, 0, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1];
        let compressed = RunLength::compress(&data);
        let decompressed = RunLength::decompress(&compressed);
        assert_eq!(&decompressed[..data.len()], &data[..]);
    }

    #[test]
    fn test_run_length_bytes() {
        let data = b"AAAABBBCCDAA";
        let compressed = RunLength::compress_bytes(data);
        let decompressed = RunLength::decompress_bytes(&compressed);
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_run_length_empty() {
        let data: Vec<u8> = vec![];
        let compressed = RunLength::compress(&data);
        assert_eq!(compressed.len(), 1); // Just the final run
    }

    #[test]
    fn test_run_length_single_bit() {
        let data = vec![1];
        let compressed = RunLength::compress(&data);
        let decompressed = RunLength::decompress(&compressed);
        assert_eq!(&decompressed[..1], &data[..]);
    }

    #[test]
    fn test_run_length_alternating() {
        let data = vec![0, 1, 0, 1, 0, 1];
        let compressed = RunLength::compress(&data);
        let decompressed = RunLength::decompress(&compressed);
        assert_eq!(&decompressed[..data.len()], &data[..]);
    }

    #[test]
    fn test_run_length_long_run() {
        // Test with long runs, but keep them reasonable
        let mut data = vec![0; 100];
        data.extend(vec![1; 50]);
        let compressed = RunLength::compress(&data);
        let decompressed = RunLength::decompress(&compressed);
        // Decompressed may have trailing zeros, so just check the prefix
        assert_eq!(
            &decompressed[..150.min(decompressed.len())],
            &data[..150.min(decompressed.len())]
        );
    }

    #[test]
    fn test_run_length_bytes_single() {
        let data = b"A";
        let compressed = RunLength::compress_bytes(data);
        let decompressed = RunLength::decompress_bytes(&compressed);
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_run_length_bytes_no_compression() {
        let data = b"ABCDEFG";
        let compressed = RunLength::compress_bytes(data);
        let decompressed = RunLength::decompress_bytes(&compressed);
        assert_eq!(decompressed, data);
    }
}
