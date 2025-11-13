// Copyright (C) 2025 algs4-rust contributors
// SPDX-License-Identifier: GPL-3.0-or-later
//
// This file is part of algs4-rust (Rust implementation of Algorithms, 4th Edition).
// Adapted from the original Java implementation by Robert Sedgewick and Kevin Wayne.

//! Standard output utilities.
//!
//! This module provides convenience functions for writing to standard output,
//! similar to Java's `StdOut` class. In Rust, you can also use the standard
//! `println!`, `print!`, and `format!` macros directly.

use std::fmt::Display;
use std::io::{self, Write};

/// Prints a value to standard output and flushes.
///
/// This is equivalent to `print!()` followed by a flush.
///
/// # Examples
///
/// ```
/// use algs4_fundamentals::io::stdout;
///
/// stdout::print("Hello");
/// stdout::print(" ");
/// stdout::print("World");
/// stdout::println("!"); // Outputs: Hello World!
/// ```
pub fn print<T: Display>(x: T) {
    print!("{}", x);
    let _ = io::stdout().flush();
}

/// Prints a value to standard output followed by a newline.
///
/// This is equivalent to Rust's `println!()` macro.
///
/// # Examples
///
/// ```
/// use algs4_fundamentals::io::stdout;
///
/// stdout::println("Hello, world!");
/// stdout::println(42);
/// stdout::println(3.14159);
/// ```
pub fn println<T: Display>(x: T) {
    println!("{}", x);
}

/// Prints a newline to standard output.
///
/// # Examples
///
/// ```
/// use algs4_fundamentals::io::stdout;
///
/// stdout::print("Hello");
/// stdout::println_empty();
/// stdout::print("World");
/// ```
pub fn println_empty() {
    println!();
}

/// Prints formatted output to standard output.
///
/// This uses Rust's standard formatting syntax.
///
/// # Examples
///
/// ```
/// use algs4_fundamentals::io::stdout;
///
/// stdout::printf("Hello, {}!", &[&"World"]);
/// stdout::printf("Value: {:.2}", &[&3.14159]);
/// stdout::printf("{} + {} = {}", &[&1, &2, &3]);
/// ```
///
/// Note: In Rust, it's more idiomatic to use `print!()` and `println!()` macros directly:
/// ```
/// println!("Hello, {}!", "World");
/// println!("Value: {:.2}", 3.14159);
/// ```
pub fn printf(format_str: &str, args: &[&dyn Display]) {
    // Note: This is a simplified version. For full printf-style formatting,
    // users should use Rust's print!/println! macros with {} placeholders.
    let mut result = format_str.to_string();
    for arg in args {
        result = result.replacen("{}", &format!("{}", arg), 1);
    }
    print!("{}", result);
    let _ = io::stdout().flush();
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests don't actually verify output since that would require
    // capturing stdout. In practice, these functions are thin wrappers around
    // Rust's standard printing facilities.

    #[test]
    fn test_print() {
        // Just verify it doesn't panic
        print("test");
    }

    #[test]
    fn test_println() {
        // Just verify it doesn't panic
        println("test");
        println(42);
        println(3.14);
    }

    #[test]
    fn test_println_empty() {
        // Just verify it doesn't panic
        println_empty();
    }

    #[test]
    fn test_printf() {
        // Just verify it doesn't panic
        printf("Hello, {}!", &[&"World"]);
    }
}
