# Phase 6: Searching & Symbol Tables - Implementation Plan

**Status:** Ready to Start
**Priority:** HIGH
**Module:** `modules/searching/`
**Estimated Effort:** 2-3 sessions
**Target Files:** 20 total

---

## Overview

Phase 6 focuses on implementing fundamental search algorithms and symbol table (map/dictionary) data structures. These are critical components for efficient data retrieval and are used extensively in real-world applications.

### Key Concepts
- **Symbol Tables:** Key-value pair data structures (maps/dictionaries)
- **Binary Search:** O(log n) search in sorted arrays
- **Binary Search Trees:** Dynamic ordered symbol tables
- **Balanced Trees:** Self-balancing BSTs (Red-Black, AVL)
- **Hash Tables:** O(1) average-case search/insert
- **Tries:** String-specific symbol tables with prefix operations

---

## Implementation Priority

### High Priority (Core - Session 1)
1. **SequentialSearchST** - Baseline unordered symbol table
2. **BinarySearch** - Classic binary search algorithm
3. **BinarySearchST** - Ordered array-based symbol table
4. **BST** - Basic binary search tree
5. **SeparateChainingHashST** - Hash table with chaining

### Medium Priority (Advanced - Session 2)
6. **RedBlackBST** - Balanced BST with guaranteed O(log n)
7. **LinearProbingHashST** - Open addressing hash table
8. **TrieST** - R-way trie symbol table
9. **TrieSET** - R-way trie set

### Lower Priority (Specialized - Session 3)
10. **AVLTreeST** - AVL tree (alternative balanced BST)
11. **BTree** - B-tree for external storage
12. **PatriciaST** - Practical Algorithm to Retrieve Information
13. **PatriciaSET** - Patricia trie set

### Applications (As Time Permits)
14. **FrequencyCounter** - Count word frequencies
15. **DeDup** - Remove duplicates from input
16. **Count** - Count occurrences
17. **FileIndex** - Inverted index for file search
18. **LookupCSV** - CSV database lookup
19. **LookupIndex** - General index lookup
20. **KWIK** - Keyword in context search

---

## Session 1: Basic Search & Symbol Tables

### Goals
- Implement foundational search algorithms
- Create basic symbol table implementations
- Establish API patterns for all symbol tables

### Files to Implement

#### 1. BinarySearch (Standalone Algorithm)
```rust
// modules/searching/src/binary_search.rs
pub fn binary_search<T: Ord>(arr: &[T], key: &T) -> Option<usize>
pub fn rank<T: Ord>(arr: &[T], key: &T) -> usize  // number of keys < key
```

**Features:**
- Generic binary search on sorted slices
- Return index if found, None otherwise
- Rank function for insertion point

**Tests:**
- Empty array
- Single element
- Key at boundaries (first, last, middle)
- Key not present
- Duplicate keys

---

#### 2. SequentialSearchST (Unordered Linked List)
```rust
// modules/searching/src/sequential_search_st.rs
pub struct SequentialSearchST<K, V> {
    head: Option<Box<Node<K, V>>>,
    n: usize,
}

struct Node<K, V> {
    key: K,
    val: V,
    next: Option<Box<Node<K, V>>>,
}
```

**API:**
- `new()` - Create empty table
- `put(key, val)` - Insert/update
- `get(&key) -> Option<&V>` - Retrieve value
- `delete(&key)` - Remove key
- `contains(&key) -> bool`
- `is_empty() -> bool`
- `size() -> usize`
- `keys() -> impl Iterator<Item = &K>`

**Performance:** O(n) for all operations (baseline)

---

#### 3. BinarySearchST (Ordered Array)
```rust
// modules/searching/src/binary_search_st.rs
pub struct BinarySearchST<K, V> {
    keys: Vec<K>,
    vals: Vec<V>,
}
```

**API:** Same as SequentialSearchST plus:
- `min() -> Option<&K>` - Smallest key
- `max() -> Option<&K>` - Largest key
- `floor(&key) -> Option<&K>` - Largest key ≤ given key
- `ceiling(&key) -> Option<&K>` - Smallest key ≥ given key
- `rank(&key) -> usize` - Number of keys < given key
- `select(k: usize) -> Option<&K>` - Key of rank k
- `delete_min()` - Remove smallest key
- `delete_max()` - Remove largest key
- `range(&lo, &hi) -> impl Iterator` - Keys in range

**Performance:**
- Search: O(log n)
- Insert: O(n) (array shifting)
- Delete: O(n)

---

#### 4. BST (Binary Search Tree)
```rust
// modules/searching/src/bst.rs
pub struct BST<K, V> {
    root: Option<Box<Node<K, V>>>,
}

struct Node<K, V> {
    key: K,
    val: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
    n: usize,  // subtree size
}
```

**API:** Same as BinarySearchST (all ordered operations)

**Additional:**
- `height() -> usize` - Tree height
- Recursive implementation
- Size-based operations

**Performance:**
- Average: O(log n)
- Worst: O(n) (unbalanced)

**Tests:**
- BST invariant (left < parent < right)
- Size consistency
- Ordered iteration
- Range queries
- Floor/ceiling correctness

---

#### 5. SeparateChainingHashST (Hash Table with Chaining)
```rust
// modules/searching/src/separate_chaining_hash_st.rs
pub struct SeparateChainingHashST<K, V> {
    chains: Vec<SequentialSearchST<K, V>>,
    n: usize,  // total keys
    m: usize,  // number of chains
}
```

**Features:**
- Dynamic resizing (double/halve)
- Load factor α = N/M ≈ 5-10
- Uses SequentialSearchST for chains

**API:** Basic symbol table API (unordered)

**Performance:** O(1) average case

---

### Testing Strategy

1. **Unit Tests** for each data structure
   - Empty table operations
   - Single element
   - Multiple insertions
   - Deletions
   - Iterator correctness

2. **Correctness Tests**
   - BST invariants
   - Ordered operations (min, max, floor, ceiling, rank, select)
   - Range queries
   - Hash table resizing

3. **Performance Benchmarks**
   - Compare against std::collections::HashMap
   - Compare against std::collections::BTreeMap
   - Measure resize overhead

4. **Documentation Tests**
   - Example usage in doc comments
   - Show idiomatic Rust patterns

---

## Session 2: Balanced Trees & Tries

### Goals
- Implement self-balancing BSTs
- Create trie-based symbol tables
- Ensure guaranteed O(log n) performance

### Files to Implement

#### 6. RedBlackBST (Red-Black Tree)
```rust
pub struct RedBlackBST<K, V> {
    root: Option<Box<Node<K, V>>>,
}

struct Node<K, V> {
    key: K,
    val: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
    n: usize,
    color: Color,  // RED or BLACK
}

enum Color { Red, Black }
```

**Features:**
- Left-leaning red-black BST (Sedgewick variant)
- Guaranteed O(log n) for all operations
- Self-balancing rotations

**Core Operations:**
- `rotate_left()`, `rotate_right()`
- `flip_colors()`
- Maintain red-black invariants

**Tests:**
- Red-black properties:
  1. Root is black
  2. No two consecutive red nodes
  3. All paths have same number of black nodes
- Performance vs BST on random data

---

#### 7. LinearProbingHashST (Open Addressing)
```rust
pub struct LinearProbingHashST<K, V> {
    keys: Vec<Option<K>>,
    vals: Vec<Option<V>>,
    n: usize,  // number of key-value pairs
    m: usize,  // size of arrays
}
```

**Features:**
- Open addressing with linear probing
- Resize when α > 0.5 or α < 0.125
- Tombstone-free deletion (rehash cluster)

**Performance:** O(1) average, sensitive to clustering

---

#### 8. TrieST (R-way Trie Symbol Table)
```rust
pub struct TrieST<V> {
    root: Option<Box<Node<V>>>,
    n: usize,
}

struct Node<V> {
    val: Option<V>,
    next: [Option<Box<Node<V>>>; R],  // R = 256 for extended ASCII
}
```

**API:** Standard ST API plus:
- `keys_with_prefix(&prefix)` - All keys starting with prefix
- `longest_prefix_of(&s)` - Longest key that is a prefix of s
- `keys_that_match(&pattern)` - Wildcard matching (. matches any char)

**Performance:**
- Search hit: O(L) where L = key length
- Search miss: O(log_R N) average
- Space: O(RNL) worst case

---

#### 9. TrieSET (R-way Trie Set)
Similar to TrieST but stores only keys (no values).

---

## Session 3: Advanced Trees & Applications

### Files to Implement

#### 10. AVLTreeST (AVL Tree)
Alternative balanced BST with stricter balance (height difference ≤ 1).

#### 11. BTree (B-tree)
Multi-way balanced tree for external storage simulation.

#### 12-13. Patricia Tries
Space-optimized tries (radix trees).

#### 14-20. Applications
Practical programs demonstrating symbol table usage.

---

## Common API Pattern

All symbol tables should implement a common trait:

```rust
pub trait SymbolTable<K, V> {
    fn new() -> Self;
    fn put(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn delete(&mut self, key: &K);
    fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
    fn keys(&self) -> Box<dyn Iterator<Item = &K> + '_>;
}

pub trait OrderedSymbolTable<K, V>: SymbolTable<K, V> {
    fn min(&self) -> Option<&K>;
    fn max(&self) -> Option<&K>;
    fn floor(&self, key: &K) -> Option<&K>;
    fn ceiling(&self, key: &K) -> Option<&K>;
    fn rank(&self, key: &K) -> usize;
    fn select(&self, k: usize) -> Option<&K>;
    // ... range operations
}
```

---

## Module Structure

```
modules/searching/
├── src/
│   ├── lib.rs
│   ├── binary_search.rs
│   ├── sequential_search_st.rs
│   ├── binary_search_st.rs
│   ├── bst.rs
│   ├── red_black_bst.rs
│   ├── avl_tree_st.rs
│   ├── btree.rs
│   ├── separate_chaining_hash_st.rs
│   ├── linear_probing_hash_st.rs
│   ├── trie_st.rs
│   ├── trie_set.rs
│   ├── patricia_st.rs
│   ├── patricia_set.rs
│   └── apps/
│       ├── frequency_counter.rs
│       ├── dedup.rs
│       ├── count.rs
│       ├── file_index.rs
│       ├── lookup_csv.rs
│       ├── lookup_index.rs
│       └── kwik.rs
├── tests/
│   └── integration_tests.rs
├── benches/
│   └── benchmarks.rs
└── Cargo.toml
```

---

## Success Criteria

### Phase 6 Complete When:
- [x] All 20 files implemented
- [x] 200+ unit tests passing
- [x] BST invariants verified
- [x] Red-black tree properties verified
- [x] Hash table resizing tested
- [x] Trie prefix operations tested
- [x] All ordered operations correct (floor, ceiling, rank, select, range)
- [x] Documentation complete with examples
- [x] Code formatted and linted (0 warnings)
- [x] Benchmarks comparing different implementations
- [x] CI passing

---

## Key Challenges & Solutions

### Challenge 1: Ownership in Tree Structures
**Solution:** Use `Option<Box<Node>>` for child pointers, ensuring unique ownership.

### Challenge 2: Iterator Lifetimes
**Solution:** Return boxed trait objects or implement custom iterator types with proper lifetimes.

### Challenge 3: Generic Hash Functions
**Solution:** Require `K: Hash + Eq` trait bounds, use `std::collections::hash_map::DefaultHasher`.

### Challenge 4: Trie Memory Usage
**Solution:** Start with R=256 for simplicity, optionally implement compressed tries (Patricia) later.

### Challenge 5: Red-Black Balancing
**Solution:** Follow Sedgewick's left-leaning red-black BST algorithm exactly, with comprehensive tests.

---

## Testing Data

Use these standard test files from algs4:
- `tinyST.txt` - Small symbol table
- `tale.txt` - Word frequency counting
- `leipzig1M.txt` - Large text file (1M words)

---

## Performance Targets

| Operation | Sequential | BinarySearchST | BST (avg) | RedBlackBST | Hash Table |
|-----------|-----------|----------------|-----------|-------------|------------|
| Search    | O(n)      | O(log n)       | O(log n)  | O(log n)    | O(1)*      |
| Insert    | O(n)      | O(n)           | O(log n)  | O(log n)    | O(1)*      |
| Delete    | O(n)      | O(n)           | O(log n)  | O(log n)    | O(1)*      |
| Min/Max   | O(n)      | O(1)           | O(h)      | O(log n)    | O(n)       |
| Floor     | O(n)      | O(log n)       | O(h)      | O(log n)    | N/A        |
| Range     | O(n)      | O(log n + k)   | O(h + k)  | O(log n + k)| O(n)       |

*Average case; worst case can be O(n) if poor hash function or clustering

---

## References

- **Textbook:** Algorithms, 4th Edition, Sections 3.1-3.5
- **Java Source:** https://github.com/kevin-wayne/algs4 (edu.princeton.cs.algs4)
- **Online Course:** https://algs4.cs.princeton.edu/30searching/
- **Rust Collections:** std::collections::{HashMap, BTreeMap}

---

## Next Phase Preview

**Phase 7: Graph Fundamentals** will build on symbol tables for:
- Symbol graphs (string-named vertices)
- Adjacency list representations (using hash maps)
- Efficient graph traversals
