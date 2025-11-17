//! Data compression algorithms.
//!
//! This module provides three classic compression algorithms:
//! - **Run-length encoding**: Compresses runs of identical bits/bytes
//! - **Huffman coding**: Optimal prefix-free compression based on frequencies
//! - **LZW**: Dictionary-based compression algorithm

pub mod huffman;
pub mod lzw;
pub mod run_length;

pub use huffman::Huffman;
pub use lzw::LZW;
pub use run_length::RunLength;
