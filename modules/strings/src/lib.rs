//! # algs4-strings
//!
//! String processing algorithms from *Algorithms, 4th Edition*
//! by Robert Sedgewick and Kevin Wayne.
//!
//! This module contains:
//! - String pattern matching (KMP, Boyer-Moore, Rabin-Karp)
//! - Regular expressions (NFA)
//! - Data compression (LZW, Huffman, Run-length)
//! - Suffix arrays
//! - Alphabet data types
//!
//! ## Pattern Matching Example
//!
//! ```
//! use algs4_strings::pattern_matching::KMP;
//!
//! let kmp = KMP::new("NEEDLE");
//! let text = "FINDINANEEDLEINTHEHAYSTACK";
//! assert_eq!(kmp.search(text), Some(7));
//! ```
//!
//! ## Compression Example
//!
//! ```
//! use algs4_strings::compression::Huffman;
//!
//! let data = b"ABRACADABRA!";
//! let compressed = Huffman::compress(data);
//! let decompressed = Huffman::decompress(&compressed);
//! assert_eq!(decompressed, data);
//! ```
//!
//! ## Regular Expression Example
//!
//! ```
//! use algs4_strings::regex::NFA;
//!
//! let nfa = NFA::new("(A*B|AC)D");
//! assert!(nfa.recognizes("AABD"));
//! assert!(nfa.recognizes("ACD"));
//! ```

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

pub mod alphabet;
pub mod compression;
pub mod pattern_matching;
pub mod regex;
pub mod substring;
pub mod suffix_array;
