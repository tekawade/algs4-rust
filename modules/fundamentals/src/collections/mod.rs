//! Collection data structures.
//!
//! This module provides fundamental collection data structures including:
//! - **Stack**: LIFO (Last-In-First-Out) data structure
//! - **Queue**: FIFO (First-In-First-Out) data structure
//! - **Bag**: Unordered collection (multiset) that allows duplicates
//!
//! Each collection type has multiple implementations:
//! - **Linked implementations**: Use singly linked lists, no capacity limits
//! - **Array implementations**: Use resizable arrays, better cache locality
//!
//! **Reference:** <https://algs4.cs.princeton.edu/13stacks>

// Linked implementations
mod linked_bag;
mod linked_queue;
mod linked_stack;

pub use linked_bag::LinkedBag;
pub use linked_queue::LinkedQueue;
pub use linked_stack::LinkedStack;

// Array implementations
mod resizing_array_bag;
mod resizing_array_queue;
mod resizing_array_stack;

pub use resizing_array_bag::ResizingArrayBag;
pub use resizing_array_queue::ResizingArrayQueue;
pub use resizing_array_stack::ResizingArrayStack;
