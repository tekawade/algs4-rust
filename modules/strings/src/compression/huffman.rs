//! Huffman coding data compression.
//!
//! Huffman coding is an optimal prefix-free compression algorithm that
//! assigns variable-length codes to characters based on their frequencies.

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

/// A node in the Huffman trie.
#[derive(Clone, Debug)]
struct Node {
    ch: Option<u8>, // Character (None for internal nodes)
    freq: usize,    // Frequency
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new_leaf(ch: u8, freq: usize) -> Self {
        Node {
            ch: Some(ch),
            freq,
            left: None,
            right: None,
        }
    }

    fn new_internal(left: Node, right: Node) -> Self {
        Node {
            ch: None,
            freq: left.freq + right.freq,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }

    fn is_leaf(&self) -> bool {
        self.ch.is_some()
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
        other.freq.cmp(&self.freq)
    }
}

/// Huffman coding compression and decompression.
///
/// Builds an optimal prefix-free code based on character frequencies.
///
/// # Examples
///
/// ```
/// use algs4_strings::compression::Huffman;
///
/// let data = b"ABRACADABRA!";
/// let compressed = Huffman::compress(data);
/// let decompressed = Huffman::decompress(&compressed);
/// assert_eq!(decompressed, data);
/// ```
pub struct Huffman;

impl Huffman {
    /// Compresses data using Huffman coding.
    ///
    /// Returns a compressed representation that includes both the trie
    /// and the encoded data.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::compression::Huffman;
    ///
    /// let data = b"ABRACADABRA!";
    /// let compressed = Huffman::compress(data);
    /// assert!(compressed.len() > 0);
    /// ```
    pub fn compress(data: &[u8]) -> Vec<u8> {
        if data.is_empty() {
            return Vec::new();
        }

        // Build frequency table
        let mut freq = HashMap::new();
        for &ch in data {
            *freq.entry(ch).or_insert(0) += 1;
        }

        // Build Huffman trie
        let root = Self::build_trie(&freq);

        // Build code table
        let mut code_table = HashMap::new();
        Self::build_code(&root, String::new(), &mut code_table);

        // Encode data
        let mut bits = String::new();
        for &ch in data {
            bits.push_str(&code_table[&ch]);
        }

        // Serialize: [trie_size][trie][data_length][encoded_bits]
        let mut result = Vec::new();
        let trie_bytes = Self::serialize_trie(&root);

        // Write trie size (4 bytes)
        result.extend_from_slice(&(trie_bytes.len() as u32).to_be_bytes());
        // Write trie
        result.extend_from_slice(&trie_bytes);
        // Write original data length (4 bytes)
        result.extend_from_slice(&(data.len() as u32).to_be_bytes());
        // Write encoded bits as bytes
        let encoded_bytes = Self::bits_to_bytes(&bits);
        result.extend(encoded_bytes);

        result
    }

    /// Decompresses Huffman-encoded data.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::compression::Huffman;
    ///
    /// let data = b"ABRACADABRA!";
    /// let compressed = Huffman::compress(data);
    /// let decompressed = Huffman::decompress(&compressed);
    /// assert_eq!(decompressed, data);
    /// ```
    pub fn decompress(compressed: &[u8]) -> Vec<u8> {
        if compressed.is_empty() {
            return Vec::new();
        }

        let mut pos = 0;

        // Read trie size
        let trie_size = u32::from_be_bytes([
            compressed[pos],
            compressed[pos + 1],
            compressed[pos + 2],
            compressed[pos + 3],
        ]) as usize;
        pos += 4;

        // Read and deserialize trie
        let trie_bytes = &compressed[pos..pos + trie_size];
        let (root, _) = Self::deserialize_trie(trie_bytes, 0);
        pos += trie_size;

        // Read original data length
        let data_len = u32::from_be_bytes([
            compressed[pos],
            compressed[pos + 1],
            compressed[pos + 2],
            compressed[pos + 3],
        ]) as usize;
        pos += 4;

        // Decode bits
        let encoded_bytes = &compressed[pos..];
        let bits = Self::bytes_to_bits(encoded_bytes);

        let mut result = Vec::new();

        // Special case: single character (root is leaf)
        if root.is_leaf() {
            let ch = root.ch.unwrap();
            for _ in 0..data_len {
                result.push(ch);
            }
            return result;
        }

        let mut node = &root;
        let mut count = 0;

        for bit in bits.chars() {
            if count >= data_len {
                break;
            }

            node = if bit == '0' {
                node.left.as_ref().unwrap()
            } else {
                node.right.as_ref().unwrap()
            };

            if node.is_leaf() {
                result.push(node.ch.unwrap());
                node = &root;
                count += 1;
            }
        }

        result
    }

    fn build_trie(freq: &HashMap<u8, usize>) -> Node {
        let mut pq = BinaryHeap::new();

        for (&ch, &f) in freq {
            pq.push(Node::new_leaf(ch, f));
        }

        while pq.len() > 1 {
            let left = pq.pop().unwrap();
            let right = pq.pop().unwrap();
            pq.push(Node::new_internal(left, right));
        }

        pq.pop().unwrap()
    }

    fn build_code(node: &Node, prefix: String, table: &mut HashMap<u8, String>) {
        if node.is_leaf() {
            table.insert(
                node.ch.unwrap(),
                if prefix.is_empty() {
                    "0".to_string()
                } else {
                    prefix
                },
            );
        } else {
            if let Some(ref left) = node.left {
                Self::build_code(left, prefix.clone() + "0", table);
            }
            if let Some(ref right) = node.right {
                Self::build_code(right, prefix + "1", table);
            }
        }
    }

    fn serialize_trie(node: &Node) -> Vec<u8> {
        let mut result = Vec::new();
        if node.is_leaf() {
            result.push(1); // Leaf marker
            result.push(node.ch.unwrap());
        } else {
            result.push(0); // Internal node marker
            if let Some(ref left) = node.left {
                result.extend(Self::serialize_trie(left));
            }
            if let Some(ref right) = node.right {
                result.extend(Self::serialize_trie(right));
            }
        }
        result
    }

    fn deserialize_trie(data: &[u8], pos: usize) -> (Node, usize) {
        let marker = data[pos];
        if marker == 1 {
            // Leaf node
            let ch = data[pos + 1];
            (Node::new_leaf(ch, 0), pos + 2)
        } else {
            // Internal node
            let (left, pos1) = Self::deserialize_trie(data, pos + 1);
            let (right, pos2) = Self::deserialize_trie(data, pos1);
            (Node::new_internal(left, right), pos2)
        }
    }

    fn bits_to_bytes(bits: &str) -> Vec<u8> {
        let mut result = Vec::new();
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

    fn bytes_to_bits(bytes: &[u8]) -> String {
        let mut result = String::new();
        for &byte in bytes {
            for i in (0..8).rev() {
                result.push(if (byte >> i) & 1 == 1 { '1' } else { '0' });
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_huffman_basic() {
        let data = b"ABRACADABRA!";
        let compressed = Huffman::compress(data);
        let decompressed = Huffman::decompress(&compressed);
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_huffman_single_char() {
        let data = b"AAAA";
        let compressed = Huffman::compress(data);
        let decompressed = Huffman::decompress(&compressed);
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_huffman_empty() {
        let data = b"";
        let compressed = Huffman::compress(data);
        let decompressed = Huffman::decompress(&compressed);
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_huffman_long_text() {
        let data = b"The quick brown fox jumps over the lazy dog.";
        let compressed = Huffman::compress(data);
        let decompressed = Huffman::decompress(&compressed);
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_huffman_binary() {
        let data: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7];
        let compressed = Huffman::compress(data);
        let decompressed = Huffman::decompress(&compressed);
        assert_eq!(decompressed, data);
    }
}
