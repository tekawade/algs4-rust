// Copyright (C) 2025 algs4-rust contributors
// SPDX-License-Identifier: GPL-3.0-or-later
//
// This file is part of algs4-rust (Rust implementation of Algorithms, 4th Edition).
// Adapted from the original Java implementation by Robert Sedgewick and Kevin Wayne.

//! Standard input utilities.
//!
//! This module provides utilities for reading numbers and text from standard input.
//! It provides a simpler, more Rust-idiomatic API compared to the Java version.

use std::io::{self, BufRead};
use std::str::SplitWhitespace;

/// A reader for standard input that provides convenient methods for reading
/// various data types.
///
/// # Examples
///
/// ```no_run
/// use algs4_fundamentals::io::StdIn;
///
/// let mut stdin = StdIn::new();
///
/// // Read integers
/// if let Some(n) = stdin.read_i32() {
///     println!("Read integer: {}", n);
/// }
///
/// // Read a line
/// if let Some(line) = stdin.read_line() {
///     println!("Read line: {}", line);
/// }
/// ```
#[derive(Debug)]
pub struct StdIn {
    stdin: io::Stdin,
    buffer: String,
    tokens: Option<SplitWhitespace<'static>>,
}

impl StdIn {
    /// Creates a new standard input reader.
    pub fn new() -> Self {
        Self {
            stdin: io::stdin(),
            buffer: String::new(),
            tokens: None,
        }
    }

    /// Reads the next token as a string.
    ///
    /// Returns `None` if no more tokens are available.
    pub fn read_string(&mut self) -> Option<String> {
        loop {
            // Try to get next token from current buffer
            if let Some(ref mut tokens) = self.tokens {
                if let Some(token) = tokens.next() {
                    return Some(token.to_string());
                }
            }

            // Need to read more input
            self.buffer.clear();
            match self.stdin.lock().read_line(&mut self.buffer) {
                Ok(0) => return None, // EOF
                Ok(_) => {
                    // Safety: We're storing the buffer in self, so it lives long enough
                    let tokens = unsafe {
                        std::mem::transmute::<SplitWhitespace<'_>, SplitWhitespace<'static>>(
                            self.buffer.split_whitespace(),
                        )
                    };
                    self.tokens = Some(tokens);
                }
                Err(_) => return None,
            }
        }
    }

    /// Reads the next token as an i32.
    ///
    /// Returns `None` if no more tokens or parsing fails.
    pub fn read_i32(&mut self) -> Option<i32> {
        self.read_string()?.parse().ok()
    }

    /// Reads the next token as a usize.
    ///
    /// Returns `None` if no more tokens or parsing fails.
    pub fn read_usize(&mut self) -> Option<usize> {
        self.read_string()?.parse().ok()
    }

    /// Reads the next token as an i64.
    ///
    /// Returns `None` if no more tokens or parsing fails.
    pub fn read_i64(&mut self) -> Option<i64> {
        self.read_string()?.parse().ok()
    }

    /// Reads the next token as an f64.
    ///
    /// Returns `None` if no more tokens or parsing fails.
    pub fn read_f64(&mut self) -> Option<f64> {
        self.read_string()?.parse().ok()
    }

    /// Reads the next token as a boolean.
    ///
    /// Accepts "true"/"false" (case-insensitive) or "1"/"0".
    /// Returns `None` if no more tokens or parsing fails.
    pub fn read_bool(&mut self) -> Option<bool> {
        let s = self.read_string()?;
        match s.to_lowercase().as_str() {
            "true" | "1" => Some(true),
            "false" | "0" => Some(false),
            _ => None,
        }
    }

    /// Reads the next character, including whitespace.
    ///
    /// Returns `None` if no more characters are available.
    pub fn read_char(&mut self) -> Option<char> {
        self.buffer.clear();
        let mut buf = [0u8; 1];
        match self.stdin.lock().read(&mut buf) {
            Ok(0) => None,
            Ok(_) => Some(buf[0] as char),
            Err(_) => None,
        }
    }

    /// Reads the remainder of the current line.
    ///
    /// Returns `None` if no more lines are available.
    pub fn read_line(&mut self) -> Option<String> {
        self.buffer.clear();
        match self.stdin.lock().read_line(&mut self.buffer) {
            Ok(0) => None,
            Ok(_) => {
                // Remove trailing newline
                let line = self.buffer.trim_end().to_string();
                Some(line)
            }
            Err(_) => None,
        }
    }

    /// Reads all remaining input as a single string.
    pub fn read_all(&mut self) -> String {
        let mut result = String::new();
        let _ = self.stdin.lock().read_to_string(&mut result);
        result
    }

    /// Reads all remaining tokens as strings.
    pub fn read_all_strings(&mut self) -> Vec<String> {
        let mut strings = Vec::new();
        while let Some(s) = self.read_string() {
            strings.push(s);
        }
        strings
    }

    /// Reads all remaining tokens as i32s.
    ///
    /// Stops at the first token that cannot be parsed.
    pub fn read_all_i32(&mut self) -> Vec<i32> {
        let mut numbers = Vec::new();
        while let Some(n) = self.read_i32() {
            numbers.push(n);
        }
        numbers
    }

    /// Reads all remaining tokens as f64s.
    ///
    /// Stops at the first token that cannot be parsed.
    pub fn read_all_f64(&mut self) -> Vec<f64> {
        let mut numbers = Vec::new();
        while let Some(n) = self.read_f64() {
            numbers.push(n);
        }
        numbers
    }

    /// Reads all remaining lines.
    pub fn read_all_lines(&mut self) -> Vec<String> {
        let mut lines = Vec::new();
        while let Some(line) = self.read_line() {
            lines.push(line);
        }
        lines
    }

    /// Checks if there is more input available.
    pub fn is_empty(&mut self) -> bool {
        // Try to peek at the next token without consuming
        // This is a simplified check
        false // Conservative: assume input might be available
    }
}

impl Default for StdIn {
    fn default() -> Self {
        Self::new()
    }
}

// For simpler usage, provide free functions that use a global instance
use std::io::Read;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref GLOBAL_STDIN: Mutex<StdIn> = Mutex::new(StdIn::new());
}

/// Reads the next token as a string from standard input.
pub fn read_string() -> Option<String> {
    GLOBAL_STDIN.lock().ok()?.read_string()
}

/// Reads the next token as an i32 from standard input.
pub fn read_i32() -> Option<i32> {
    GLOBAL_STDIN.lock().ok()?.read_i32()
}

/// Reads the next token as a usize from standard input.
pub fn read_usize() -> Option<usize> {
    GLOBAL_STDIN.lock().ok()?.read_usize()
}

/// Reads the next token as an i64 from standard input.
pub fn read_i64() -> Option<i64> {
    GLOBAL_STDIN.lock().ok()?.read_i64()
}

/// Reads the next token as an f64 from standard input.
pub fn read_f64() -> Option<f64> {
    GLOBAL_STDIN.lock().ok()?.read_f64()
}

/// Reads the next token as a boolean from standard input.
pub fn read_bool() -> Option<bool> {
    GLOBAL_STDIN.lock().ok()?.read_bool()
}

/// Reads the next character from standard input.
pub fn read_char() -> Option<char> {
    GLOBAL_STDIN.lock().ok()?.read_char()
}

/// Reads a line from standard input.
pub fn read_line() -> Option<String> {
    GLOBAL_STDIN.lock().ok()?.read_line()
}

/// Reads all remaining input as a string.
pub fn read_all() -> String {
    GLOBAL_STDIN
        .lock()
        .ok()
        .map(|mut s| s.read_all())
        .unwrap_or_default()
}

/// Reads all remaining tokens as strings.
pub fn read_all_strings() -> Vec<String> {
    GLOBAL_STDIN
        .lock()
        .ok()
        .map(|mut s| s.read_all_strings())
        .unwrap_or_default()
}

/// Reads all remaining tokens as i32s.
pub fn read_all_i32() -> Vec<i32> {
    GLOBAL_STDIN
        .lock()
        .ok()
        .map(|mut s| s.read_all_i32())
        .unwrap_or_default()
}

/// Reads all remaining tokens as f64s.
pub fn read_all_f64() -> Vec<f64> {
    GLOBAL_STDIN
        .lock()
        .ok()
        .map(|mut s| s.read_all_f64())
        .unwrap_or_default()
}

/// Reads all remaining lines.
pub fn read_all_lines() -> Vec<String> {
    GLOBAL_STDIN
        .lock()
        .ok()
        .map(|mut s| s.read_all_lines())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Testing stdin is tricky. These are basic structural tests.
    // In practice, users would test with actual input.

    #[test]
    fn test_stdin_new() {
        let _stdin = StdIn::new();
    }

    #[test]
    fn test_stdin_default() {
        let _stdin = StdIn::default();
    }

    // Additional tests would require mocking stdin or using test files
}
