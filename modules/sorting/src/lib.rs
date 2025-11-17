//! # algs4-sorting
//!
//! Sorting algorithms from *Algorithms, 4th Edition*
//! by Robert Sedgewick and Kevin Wayne.
//!
//! This module contains:
//! - Elementary sorts (Selection, Insertion, Shell)
//! - Mergesort and variants
//! - Quicksort and variants
//! - Heapsort
//! - String sorts (LSD, MSD, 3-way string quicksort)
//!
//! ## Example
//!
//! ```
//! use algs4_sorting::{selection, insertion, shell, insertion_x, binary_insertion};
//! use algs4_sorting::{merge, merge_bu, merge_x};
//! use algs4_sorting::{quick, quick_3way, quick_x, quick_bentley_mcilroy};
//! use algs4_sorting::{heap, inversions};
//! use algs4_sorting::{lsd, msd, inplace_msd, quick_3string};
//!
//! let mut data = vec![5, 2, 8, 1, 9];
//! selection::sort(&mut data);
//! assert_eq!(data, vec![1, 2, 5, 8, 9]);
//!
//! let mut data = vec![64, 25, 12, 22, 11];
//! insertion::sort(&mut data);
//! assert_eq!(data, vec![11, 12, 22, 25, 64]);
//!
//! let mut data = vec![5, 2, 8, 1, 9, 3, 7, 4, 6];
//! shell::sort(&mut data);
//! assert_eq!(data, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
//!
//! let mut data = vec![9, 7, 5, 3, 1];
//! insertion_x::sort(&mut data);
//! assert_eq!(data, vec![1, 3, 5, 7, 9]);
//!
//! let mut data = vec![4, 2, 7, 1, 5];
//! binary_insertion::sort(&mut data);
//! assert_eq!(data, vec![1, 2, 4, 5, 7]);
//!
//! let mut data = vec![64, 25, 12, 22, 11];
//! merge::sort(&mut data);
//! assert_eq!(data, vec![11, 12, 22, 25, 64]);
//!
//! let mut data = vec![5, 2, 8, 1, 9];
//! merge_bu::sort(&mut data);
//! assert_eq!(data, vec![1, 2, 5, 8, 9]);
//!
//! let mut data = vec![9, 7, 5, 3, 1];
//! merge_x::sort(&mut data);
//! assert_eq!(data, vec![1, 3, 5, 7, 9]);
//!
//! let mut data = vec![64, 25, 12, 22, 11];
//! quick::sort(&mut data);
//! assert_eq!(data, vec![11, 12, 22, 25, 64]);
//!
//! let mut data = vec![5, 2, 5, 2, 5];
//! quick_3way::sort(&mut data);
//! assert_eq!(data, vec![2, 2, 5, 5, 5]);
//!
//! let mut data = vec![64, 25, 12, 22, 11];
//! quick_x::sort(&mut data);
//! assert_eq!(data, vec![11, 12, 22, 25, 64]);
//!
//! let mut data = vec![5, 2, 5, 2, 5];
//! quick_bentley_mcilroy::sort(&mut data);
//! assert_eq!(data, vec![2, 2, 5, 5, 5]);
//!
//! let mut data = vec![64, 25, 12, 22, 11];
//! heap::sort(&mut data);
//! assert_eq!(data, vec![11, 12, 22, 25, 64]);
//!
//! let data = vec![2, 4, 1, 3, 5];
//! let count = inversions::count(&data);
//! assert_eq!(count, 3); // (2,1), (4,1), (4,3)
//!
//! // String/Radix sorts
//! let mut strings = vec!["she".to_string(), "sells".to_string(), "seashells".to_string()];
//! msd::sort(&mut strings);
//! assert_eq!(strings, vec!["seashells", "sells", "she"]);
//!
//! let mut fixed = vec!["bed".to_string(), "bug".to_string(), "dad".to_string()];
//! lsd::sort(&mut fixed, 3);
//! assert_eq!(fixed, vec!["bed", "bug", "dad"]);
//!
//! let mut strings = vec!["by".to_string(), "the".to_string(), "sea".to_string()];
//! quick_3string::sort(&mut strings);
//! assert_eq!(strings, vec!["by", "sea", "the"]);
//!
//! let mut strings = vec!["hello".to_string(), "world".to_string(), "abc".to_string()];
//! inplace_msd::sort(&mut strings);
//! assert_eq!(strings, vec!["abc", "hello", "world"]);
//! ```

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

// Elementary sorts
pub mod insertion;
pub mod selection;

// Optimized insertion sort variants
pub mod binary_insertion;
pub mod insertion_x;

// Shellsort
pub mod shell;

// Mergesort and variants
pub mod merge;
pub mod merge_bu;
pub mod merge_x;

// Quicksort and variants
pub mod quick;
pub mod quick_3way;
pub mod quick_bentley_mcilroy;
pub mod quick_x;

// Heapsort
pub mod heap;

// Advanced algorithms
pub mod inversions;

// String/Radix sorts
pub mod inplace_msd;
pub mod lsd;
pub mod msd;
pub mod quick_3string;
