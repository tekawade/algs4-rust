# Quick Start Guide - Next Phase

**Status:** Phase 2 Complete ✅
**Current Progress:** 18/160 files (11.25%)
**Last Updated:** 2025-11-15

---

## 🎯 Starting the Next Phase

Choose which phase to start based on project priorities:

### Option 1: Phase 4 - Priority Queues ⭐ RECOMMENDED

**Why:** Completes fundamentals module, enables graph algorithms

**Files:** 10 implementations
- MaxPQ, MinPQ (basic heaps)
- IndexMaxPQ, IndexMinPQ (indexed heaps)
- BinomialMinPQ, FibonacciMinPQ (advanced heaps)
- IndexBinomialMinPQ, IndexFibonacciMinPQ (indexed advanced)
- MultiwayMinPQ, IndexMultiwayMinPQ (multiway heaps)

**Estimated effort:** 2 sessions

**Start command:**
```bash
# See detailed instructions in START_PHASE4.md
```

---

### Option 2: Phase 3 - Sorting Algorithms

**Why:** Classic algorithms, builds on collections

**Files:** 18 sorting algorithms
- Basic: Selection, Insertion, Shell (5 variants)
- Merge: Top-down, Bottom-up, Optimized (3 variants)
- Quick: Standard, 3-way, Optimized (4 variants)
- Other: Heap, LSD, MSD, String sorts (6 variants)

**Estimated effort:** 2-3 sessions

---

### Option 3: Phase 5 - Searching & Symbol Tables

**Why:** High practical value, foundational data structures

**Files:** 20 data structures
- Binary search trees
- Red-black trees
- Hash tables
- Tries
- Applications

**Estimated effort:** 3-4 sessions

---

## 📋 Before Starting

### 1. Review Phase 2 Work

```bash
# Read the session summary
cat SESSION_PHASE2_SUMMARY.md

# Review the code structure
tree modules/fundamentals/src/collections/
tree modules/fundamentals/src/union_find/

# Look at test patterns
grep -r "#\[test\]" modules/fundamentals/src/collections/ | head -20
```

### 2. Verify Clean State

```bash
# Ensure you're on the feature branch
git branch --show-current

# Check for uncommitted changes
git status

# Run all tests to confirm baseline
cargo test --workspace

# Verify clippy is clean
cargo clippy --workspace -- -D warnings
```

### 3. Check References

**Textbook resources:**
- Main site: https://algs4.cs.princeton.edu/
- Java source: https://github.com/kevin-wayne/algs4
- Chapter listings: https://algs4.cs.princeton.edu/code/

**Project documentation:**
- Conversion plan: `CONVERSION_PLAN.md`
- Progress tracker: `PROGRESS.md`
- Phase 2 summary: `SESSION_PHASE2_SUMMARY.md`

---

## 🏗️ Code Patterns to Follow

### Pattern 1: File Structure

```rust
//! Module documentation
//!
//! Description of the data structure
//!
//! # Examples
//! ```
//! use algs4_fundamentals::module::Type;
//! // Example code
//! ```
//!
//! **Reference:** <https://algs4.cs.princeton.edu/section>

use std::fmt;

/// Main struct documentation
#[derive(Debug)]
pub struct MyStruct<T> {
    // fields
}

impl<T> MyStruct<T> {
    /// Constructor documentation
    pub fn new() -> Self {
        // implementation
    }

    // Other methods...
}

// Trait implementations (Display, Default, etc.)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        // tests
    }
}
```

### Pattern 2: Iterator Implementation

```rust
/// An iterator over references
#[derive(Debug)]
pub struct Iter<'a, T> {
    // iterator state
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // implementation
    }
}

/// An iterator that moves out
#[derive(Debug)]
pub struct IntoIter<T> {
    // iterator state
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // implementation
    }
}

// IntoIterator implementations for both &Type and Type
```

### Pattern 3: Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        // Test constructor
    }

    #[test]
    fn test_basic_operations() {
        // Test main functionality
    }

    #[test]
    fn test_edge_cases() {
        // Empty, single element, large datasets
    }

    #[test]
    fn test_iterator() {
        // Iterator behavior
    }

    #[test]
    #[should_panic(expected = "error message")]
    fn test_invalid_input() {
        // Panic conditions
    }
}
```

---

## 🧪 Development Workflow

### 1. Create Module Structure

```bash
# For new module (e.g., priority queues)
mkdir -p modules/fundamentals/src/priority_queue
touch modules/fundamentals/src/priority_queue/mod.rs
touch modules/fundamentals/src/priority_queue/max_pq.rs

# Add to lib.rs
# pub mod priority_queue;
```

### 2. Implement One File at a Time

```bash
# 1. Fetch Java source for reference
# Browse to: https://github.com/kevin-wayne/algs4/blob/master/src/main/java/...

# 2. Create Rust implementation
# Write the struct, methods, and docs

# 3. Add tests
cargo test -p algs4-fundamentals -- test_name

# 4. Check documentation
cargo doc -p algs4-fundamentals --open

# 5. Run clippy
cargo clippy -p algs4-fundamentals -- -D warnings

# 6. Format code
cargo fmt --all
```

### 3. Commit Frequently

```bash
# After implementing each file
git add modules/fundamentals/src/priority_queue/max_pq.rs
git commit -m "feat(priority_queue): implement MaxPQ with binary heap"

# Or batch related files
git add modules/fundamentals/src/priority_queue/
git commit -m "feat(priority_queue): implement basic priority queues (MaxPQ, MinPQ)"
```

---

## 📚 Common Implementation Tips

### Tip 1: Start with Java Source

Always reference the Java implementation:
```bash
# Example for MaxPQ
curl https://raw.githubusercontent.com/kevin-wayne/algs4/master/src/main/java/edu/princeton/cs/algs4/MaxPQ.java
```

### Tip 2: Handle Generics Carefully

```rust
// Java generic: class MyClass<Key>
// Rust generic: struct MyStruct<T>

// Java bound: Key extends Comparable<Key>
// Rust bound: where T: Ord

impl<T: Ord> MyStruct<T> {
    // implementation
}
```

### Tip 3: Use Vec for Resizing Arrays

```rust
// Java: Key[] items = (Key[]) new Object[capacity];
// Rust:
let items: Vec<Option<T>> = Vec::with_capacity(capacity);

// Or for non-Option types:
let mut items = Vec::with_capacity(capacity);
items.resize_with(capacity, Default::default);
```

### Tip 4: Handle Optional Values

```rust
// Java: return null
// Rust: return None

// Java: if (item == null)
// Rust: if item.is_none()

// Java: item != null ? item : default
// Rust: item.unwrap_or(default)
```

---

## ✅ Quality Checklist

Before committing each file:

- [ ] Code compiles without warnings
- [ ] All tests pass
- [ ] Clippy is clean (with `-D warnings`)
- [ ] Code is formatted
- [ ] Public items have doc comments
- [ ] Doc tests work
- [ ] Performance characteristics documented
- [ ] Edge cases tested
- [ ] Display/Debug traits implemented (where appropriate)
- [ ] Iterator traits implemented (for collections)

---

## 🚀 Quick Commands Reference

```bash
# Test single module
cargo test -p algs4-fundamentals priority_queue

# Test with output
cargo test -p algs4-fundamentals -- --nocapture

# Check specific file
cargo check -p algs4-fundamentals

# Run clippy on one package
cargo clippy -p algs4-fundamentals -- -D warnings

# Build documentation
cargo doc -p algs4-fundamentals --open

# Format all code
cargo fmt --all

# Clean build artifacts
cargo clean

# Check compilation time
cargo build -p algs4-fundamentals --timings
```

---

## 📊 Progress Tracking

After completing each file:

1. **Update PROGRESS.md**
   - Mark file as complete `[x]`
   - Update completion counts
   - Add any notable implementation details

2. **Update README.md** (if needed)
   - Add new module to features list
   - Update examples

3. **Commit with clear message**
   ```bash
   git commit -m "feat(module): implement StructName - description"
   ```

---

## 🆘 Getting Help

If you encounter issues:

1. **Check existing implementations**
   ```bash
   # Look at similar implementations
   cat modules/fundamentals/src/collections/linked_stack.rs
   ```

2. **Review test patterns**
   ```bash
   # See how others test similar functionality
   grep -A 10 "#\[test\]" modules/fundamentals/src/union_find/*.rs
   ```

3. **Consult textbook**
   - Online: https://algs4.cs.princeton.edu/
   - Java source: https://github.com/kevin-wayne/algs4

4. **Check Rust patterns**
   - Rust book: https://doc.rust-lang.org/book/
   - Rust by example: https://doc.rust-lang.org/rust-by-example/

---

## 📝 Session Management

### Starting a Session

1. Create a new feature branch (if needed):
   ```bash
   git checkout -b claude/start-phase-X-<session-id>
   ```

2. Set up context:
   - Read session summary from previous phase
   - Review CONVERSION_PLAN.md for current phase
   - Check PROGRESS.md for what's next

### Ending a Session

1. Update documentation:
   - Update PROGRESS.md with completed files
   - Create session summary
   - Document any challenges or decisions

2. Commit and push:
   ```bash
   git add -A
   git commit -m "Phase X: Implement <description> (N/M files)"
   git push -u origin <branch-name>
   ```

3. Create session handoff:
   - Write SESSION_PHASE<N>_SUMMARY.md
   - Include next steps and context

---

## 🎯 Success Criteria

Each phase is complete when:

- ✅ All planned files implemented
- ✅ All tests passing
- ✅ Zero clippy warnings
- ✅ Complete documentation
- ✅ PROGRESS.md updated
- ✅ Session summary created
- ✅ Changes committed and pushed

---

**Ready to start the next phase! Choose your adventure above. 🚀**

For detailed instructions on Phase 4 (recommended), see **START_PHASE4.md** (to be created).
