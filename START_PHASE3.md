# Quick Start Guide - Phase 3: Sorting Algorithms

**Status:** Ready to Start
**Priority:** HIGH
**Module:** `modules/sorting/`
**Files:** 18 sorting algorithms
**Estimated effort:** 2-3 sessions
**Last Updated:** 2025-11-15

---

## 🎯 Phase 3 Overview

Implement 18 sorting algorithms from *Algorithms, 4th Edition*. This phase covers fundamental sorting techniques that are foundational for computer science education and practical applications.

### Why Phase 3 Next?

1. **Natural progression** - Heap sort uses priority queues from Phase 4
2. **Independent module** - No dependencies on unimplemented phases
3. **Foundational algorithms** - Essential for many later algorithms
4. **Classic CS curriculum** - Core algorithms every programmer should know
5. **Testing utilities** - Provides sorting utilities needed for other phases

---

## 📦 Files to Implement (18 Total)

### Basic Sorts (5 files)
Simple sorting algorithms, primarily for educational purposes:

1. **Selection.rs** - Selection sort (O(n²) comparisons, O(n) swaps)
2. **Insertion.rs** - Insertion sort (O(n²) average, O(n) best case)
3. **InsertionX.rs** - Insertion with sentinel optimization
4. **BinaryInsertion.rs** - Binary search for insertion point
5. **Shell.rs** - Shellsort with h-sorted subsequences

### Merge Sorts (3 files)
Divide-and-conquer sorting with guaranteed O(n log n):

6. **Merge.rs** - Top-down mergesort (recursive)
7. **MergeBU.rs** - Bottom-up mergesort (iterative)
8. **MergeX.rs** - Optimized merge with cutoff to insertion sort

### Quick Sorts (4 files)
In-place sorting with expected O(n log n):

9. **Quick.rs** - Standard quicksort with partitioning
10. **Quick3way.rs** - 3-way partitioning for duplicate keys
11. **QuickX.rs** - Optimized with median-of-3 partitioning
12. **QuickBentleyMcIlroy.rs** - Bentley-McIlroy 3-way partitioning

### Other Sorts (6 files)
Specialized sorting algorithms:

13. **Heap.rs** - Heapsort using sink operations
14. **LSD.rs** - LSD radix sort for fixed-length strings
15. **MSD.rs** - MSD radix sort with recursive partitioning
16. **InplaceMSD.rs** - In-place MSD radix sort
17. **Quick3string.rs** - 3-way string quicksort
18. **Inversions.rs** - Count inversions using mergesort

---

## 🏗️ Implementation Strategy

### Step 1: Module Setup

```bash
# Create module structure
mkdir -p modules/sorting/src/basic
mkdir -p modules/sorting/src/merge
mkdir -p modules/sorting/src/quick
mkdir -p modules/sorting/src/other

# Update lib.rs
# Add: pub mod basic; pub mod merge; pub mod quick; pub mod other;
```

### Step 2: Implementation Order

**Recommended order (easiest to hardest):**

1. Start with **basic sorts** (Selection, Insertion) - simplest implementations
2. Move to **merge sorts** (Merge, MergeBU) - introduce recursion
3. Implement **quick sorts** (Quick, Quick3way) - partitioning technique
4. Add **specialized sorts** (Heap, LSD, MSD) - more complex algorithms
5. Finish with **optimizations** (InsertionX, MergeX, QuickX) - refinements

### Step 3: Common Patterns

**All sorting functions should:**
- Accept `&mut [T]` where `T: Ord`
- Sort in-place (except merge sort auxiliary array)
- Be generic over any ordered type
- Include comprehensive tests
- Have doc examples

**Example function signature:**
```rust
/// Sorts the slice using selection sort.
///
/// Time complexity: O(n²)
/// Space complexity: O(1)
///
/// # Examples
/// ```
/// use algs4_sorting::basic::selection_sort;
///
/// let mut arr = vec![3, 1, 4, 1, 5];
/// selection_sort(&mut arr);
/// assert_eq!(arr, vec![1, 1, 3, 4, 5]);
/// ```
pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    // implementation
}
```

---

## 📝 Implementation Template

### Basic File Structure

```rust
//! [Algorithm Name] sorting implementation.
//!
//! [Description of algorithm]
//!
//! # Performance
//! - Time complexity: [Big-O notation]
//! - Space complexity: [Big-O notation]
//! - Stable: [Yes/No]
//!
//! # Examples
//! ```
//! use algs4_sorting::basic::selection_sort;
//! let mut arr = vec![3, 1, 4, 1, 5];
//! selection_sort(&mut arr);
//! assert_eq!(arr, vec![1, 1, 3, 4, 5]);
//! ```
//!
//! **Reference:** <https://algs4.cs.princeton.edu/2[section]>

/// Sorts the slice using [algorithm name].
pub fn [algorithm_name]<T: Ord>(arr: &mut [T]) {
    // implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut arr: Vec<i32> = vec![];
        [algorithm_name](&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single() {
        let mut arr = vec![1];
        [algorithm_name](&mut arr);
        assert_eq!(arr, vec![1]);
    }

    #[test]
    fn test_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        [algorithm_name](&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        [algorithm_name](&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_duplicates() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        [algorithm_name](&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 5, 6, 9]);
    }

    #[test]
    fn test_strings() {
        let mut arr = vec!["dog", "cat", "apple", "zebra"];
        [algorithm_name](&mut arr);
        assert_eq!(arr, vec!["apple", "cat", "dog", "zebra"]);
    }
}
```

---

## 🧪 Testing Guidelines

### Required Tests for Each Sort

1. **Empty array** - Should handle gracefully
2. **Single element** - No changes needed
3. **Already sorted** - Best case performance
4. **Reverse sorted** - Worst case for some algorithms
5. **Random data** - Typical use case
6. **Duplicates** - Many duplicate values
7. **Strings** - Test with different types
8. **Large dataset** - Performance verification (100-1000 elements)

### Stability Testing (for stable sorts)

```rust
#[test]
fn test_stability() {
    #[derive(Debug, PartialEq, Eq)]
    struct Item { key: i32, value: char }

    impl Ord for Item {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.key.cmp(&other.key)
        }
    }

    impl PartialOrd for Item {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut arr = vec![
        Item { key: 2, value: 'a' },
        Item { key: 1, value: 'b' },
        Item { key: 2, value: 'c' },
    ];

    stable_sort(&mut arr);

    // Check that items with equal keys maintain relative order
    assert_eq!(arr[1].value, 'a');
    assert_eq!(arr[2].value, 'c');
}
```

---

## 📚 Reference Materials

### Textbook Sections
- **Section 2.1:** Elementary Sorts (Selection, Insertion, Shell)
- **Section 2.2:** Mergesort
- **Section 2.3:** Quicksort
- **Section 2.4:** Priority Queues (Heapsort)
- **Section 2.5:** Applications
- **Section 5.1:** String Sorts (LSD, MSD, 3-way quicksort)

### Java Source Code
```bash
# Base URL for Java implementations
https://github.com/kevin-wayne/algs4/tree/master/src/main/java/edu/princeton/cs/algs4

# Example files:
# Selection.java, Insertion.java, Shell.java
# Merge.java, MergeBU.java, MergeX.java
# Quick.java, Quick3way.java, QuickX.java
# Heap.java, LSD.java, MSD.java
```

### Online Resources
- Textbook website: https://algs4.cs.princeton.edu/
- Sorting visualizations: https://algs4.cs.princeton.edu/20sorting
- Data files: https://algs4.cs.princeton.edu/21elementary (tiny.txt, words3.txt)

---

## 🚀 Quick Commands

```bash
# Create sorting module
cd modules/sorting

# Run tests for specific sort
cargo test -p algs4-sorting selection

# Run all sorting tests
cargo test -p algs4-sorting

# Check clippy
cargo clippy -p algs4-sorting -- -D warnings

# Build documentation
cargo doc -p algs4-sorting --open

# Format code
cargo fmt --all
```

---

## 💡 Implementation Tips

### Tip 1: Start Simple
Begin with Selection and Insertion sorts - they're the simplest and will help you establish patterns for the rest.

### Tip 2: Use Helper Functions
Many sorts share common operations:
```rust
// Swap helper
fn swap<T>(arr: &mut [T], i: usize, j: usize) {
    arr.swap(i, j);
}

// Less-than comparison
fn less<T: Ord>(arr: &[T], i: usize, j: usize) -> bool {
    arr[i] < arr[j]
}

// Check if sorted (for testing)
fn is_sorted<T: Ord>(arr: &[T]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}
```

### Tip 3: Mergesort Needs Auxiliary Array
```rust
pub fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    let mut aux = arr.to_vec();
    merge_sort_helper(arr, &mut aux, 0, arr.len());
}

fn merge_sort_helper<T: Ord + Clone>(
    arr: &mut [T],
    aux: &mut [T],
    lo: usize,
    hi: usize,
) {
    // recursive implementation
}
```

### Tip 4: Quicksort Partitioning
```rust
fn partition<T: Ord>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    let pivot = lo;
    let mut i = lo + 1;
    let mut j = hi;

    loop {
        while i <= hi && arr[i] <= arr[pivot] { i += 1; }
        while j > lo && arr[j] >= arr[pivot] { j -= 1; }
        if i >= j { break; }
        arr.swap(i, j);
    }
    arr.swap(pivot, j);
    j
}
```

---

## ✅ Quality Checklist

Before committing each sort:

- [ ] Function is generic over `T: Ord`
- [ ] Sorts in-place (except merge sort)
- [ ] Handles empty and single-element arrays
- [ ] All 8 required tests pass
- [ ] Clippy clean with `-D warnings`
- [ ] Doc comment with complexity analysis
- [ ] Doc test example works
- [ ] Code formatted
- [ ] Stability documented (if applicable)

---

## 📊 Progress Tracking

After completing each file:

1. Update PROGRESS.md - mark file as `[x]` complete
2. Update completion count (e.g., "5/18 complete")
3. Commit with descriptive message:
   ```bash
   git commit -m "feat(sorting): implement Selection and Insertion sorts (2/18)"
   ```

---

## 🎯 Success Criteria

Phase 3 is complete when:

- ✅ All 18 sorting algorithms implemented
- ✅ Comprehensive test suite (144+ tests minimum, 8 per algorithm)
- ✅ Zero clippy warnings
- ✅ Complete documentation with examples
- ✅ Complexity analysis for each algorithm
- ✅ Stability documented where applicable
- ✅ PROGRESS.md updated
- ✅ Session summary created

---

## 🆘 Getting Help

If you encounter issues:

1. **Check the Java source:**
   ```bash
   curl https://raw.githubusercontent.com/kevin-wayne/algs4/master/src/main/java/edu/princeton/cs/algs4/Selection.java
   ```

2. **Review existing implementations:**
   ```bash
   # Look at priority queue patterns
   cat modules/fundamentals/src/priority_queue/max_pq.rs
   ```

3. **Consult textbook:**
   - Online: https://algs4.cs.princeton.edu/20sorting
   - Visualizations help understand the algorithms

4. **Test incrementally:**
   ```bash
   cargo test -p algs4-sorting selection -- --nocapture
   ```

---

**Ready to start Phase 3! 🚀**

Begin with Selection and Insertion sorts for a solid foundation.
