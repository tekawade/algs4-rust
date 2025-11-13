# algs4-rust: Java to Rust Conversion Plan

## Project Overview

This project converts the Java implementation from [kevin-wayne/algs4](https://github.com/kevin-wayne/algs4) (Algorithms, 4th Edition by Robert Sedgewick and Kevin Wayne) to idiomatic Rust.

**Source Repository:** https://github.com/kevin-wayne/algs4
**Target Repository:** algs4-rust
**Total Files in Original:** ~201 Java source files
**Core Files to Convert:** ~160 files (41 skipped or optional)
**License:** GPLv3 (inherited from source)

## Design Philosophy

Following the original design principles while adapting to Rust idioms:
- **Clarity:** Code should be easy to read and understand
- **Portability:** Standard Rust with minimal dependencies
- **Efficiency:** Leverage Rust's zero-cost abstractions and memory safety
- **Safety:** Use Rust's type system to prevent common errors
- **Idiomatic:** Follow Rust best practices (ownership, borrowing, traits)

## Project Structure

```
algs4-rust/
├── Cargo.toml                  # Main workspace configuration
├── .gitignore                  # Ignore Cargo artifacts
├── README.md                   # Project documentation
├── CONVERSION_PLAN.md          # This file
├── PROGRESS.md                 # Detailed progress tracking
├── API_GUIDE.md                # API design and usage examples
├── LICENSE                     # GPLv3 license
├── .claude/                    # Claude Code context
│   ├── commands/              # Custom slash commands
│   └── context/               # Additional context files
├── modules/                    # Main algorithm modules (workspace members)
│   ├── fundamentals/          # Basic data structures
│   ├── sorting/               # Sorting algorithms
│   ├── searching/             # Search algorithms
│   ├── graphs/                # Graph algorithms
│   ├── strings/               # String processing
│   ├── geometry/              # Geometric algorithms
│   └── advanced/              # Advanced topics
├── examples/                   # Example programs (converted from Java clients)
├── tests/                      # Integration tests
│   └── data/                  # Test data files (.txt from original repo)
└── benches/                    # Performance benchmarks
```

## Key Decisions Made

### 1. API Structure
**Decision:** Modular with optional unified re-export crate

```rust
// Users can import from individual modules
use algs4_fundamentals::{Stack, Queue};
use algs4_sorting::quick_sort;

// Or (future) use unified crate that re-exports everything
use algs4::prelude::*;
```

**Rationale:** Allows users to include only what they need, reducing compile times.

### 2. String Algorithm Strategy
**Decision:** Hybrid approach based on algorithm requirements

- **UTF-8 aware algorithms** (search, pattern matching): Use `&str` and `String`
- **Byte-level algorithms** (compression, low-level): Use `&[u8]` and `Vec<u8>`
- **Character-indexed algorithms** (suffix arrays): Use `Vec<char>` when needed

**Rationale:** Respects Rust's UTF-8 strings while supporting byte-level operations where needed.

### 3. Test Data Files
**Decision:** Include in repository with `include_str!` macro

```rust
const TEST_DATA: &str = include_str!("../tests/data/tinyG.txt");
```

**Location:** `tests/data/` directory with original .txt files from algs4 repo

### 4. Client Programs (Java main() methods)
**Decision:** Convert to both examples and integration tests

- **Examples** (`examples/`): Interactive demonstrations
- **Integration tests** (`tests/`): Automated correctness verification
- **Doc tests**: Simple usage in documentation

**Rationale:** Provides both learning resources and test coverage.

### 5. Multimedia & Visualization
**Decision:** Phase 11 (optional) with feature flags

```toml
[features]
default = []
graphics = ["plotters"]
audio = ["rodio"]
image = ["image"]
```

**Rationale:** Core algorithms don't need multimedia; power users can opt-in.

### 6. Error Handling Philosophy
**Decision:**
- `Result<T, E>` for I/O and file operations
- `Option<T>` for search operations (found/not found)
- `panic!` only for contract violations (e.g., invalid indices with debug assertions)

### 7. Performance vs Clarity
**Decision:** Clarity first, then optimize with benchmarks

- Initial implementations prioritize readability
- Add `#[inline]` and optimizations after correctness verified
- Document any deviations from textbook for performance
- Benchmark against std library and Java

---

## Files to Skip

The following files from the original repository will **not** be converted:

### Testing/Benchmarking Utilities (use Cargo alternatives)
- `DoublingTest` - Use `cargo bench` instead
- `DoublingRatio` - Use `criterion` benchmarks
- `RandomSeq` - Can be done with `rand` crate directly

### System Utilities (use standard tools)
- `Cat` - Use shell `cat` command
- `BinaryDump` - Use `hexdump` or `xxd`
- `HexDump` - Use `hexdump` or `xxd`
- `PictureDump` - Not applicable without Picture

### Multimedia (moved to optional Phase 11)
- `Picture`, `StdPicture`, `GrayscalePicture` (25 files total - see Phase 11)
- `StdDraw`, `Draw`, `DrawListener`
- `StdAudio`, `StdAudioStereo`

### Java-Specific or Redundant
- Multiple "X" variants where one Rust implementation suffices
- Some test clients (consolidate into integration tests)

**Total Skipped:** ~41 files
**Core Conversion Target:** ~160 files

---

## Conversion Phases

The work is divided into **11 phases** (Phase 0-10, plus optional Phase 11) to manage complexity and context windows.

---

### **Phase 0: Project Foundation** ✅ COMPLETED
**Estimated Effort:** 1 session
**Status:** DONE - 2025-11-13

**Deliverables:**
- [x] Project structure setup
- [x] Cargo workspace configuration
- [x] .gitignore and basic files
- [x] Claude context files
- [x] Conversion plan document
- [x] Initial README
- [x] CI/CD setup

**Files Created:**
- `Cargo.toml` (workspace)
- `.gitignore`
- `README.md`
- `CONVERSION_PLAN.md`
- `.claude/` context files
- `LICENSE`
- GitHub Actions CI/CD

---

### **Phase 1: Core I/O & Basic Types**
**Estimated Effort:** 2 sessions
**Priority:** HIGH (Required by all other modules)
**Module:** `modules/fundamentals/`

**Java Files to Convert (8 files):**
1. `StdIn` - Standard input utilities
2. `StdOut` - Standard output utilities (formatted printing)
3. `In` - File and URL input
4. `Out` - File output
5. `StdRandom` - Random number generation
6. `Stopwatch` - Elapsed time measurement
7. `Counter` - Simple counter with name
8. `Accumulator` - Running statistics (mean, variance)

**Files to Skip:**
- `BinaryIn`, `BinaryOut`, `BinaryStdIn`, `BinaryStdOut` (Phase 1B if needed)
- `StdArrayIO` (not commonly used)
- `DoublingTest`, `DoublingRatio` (use `cargo bench`)

**Key Challenges:**
- Standard input/output handling in Rust (`std::io`)
- Random number generation (`rand` crate)
- Timer utilities (`std::time::Instant`)
- Making I/O convenient like Java's utilities

**Dependencies:**
- `rand = "0.8"` - Random number generation

**Rust Adaptations:**
```rust
// StdIn equivalent
pub fn read_int() -> io::Result<i32> { ... }
pub fn read_line() -> io::Result<String> { ... }

// StdRandom equivalent
pub fn uniform_int(a: i32, b: i32) -> i32 { ... }
pub fn shuffle<T>(arr: &mut [T]) { ... }

// Stopwatch
pub struct Stopwatch { start: Instant }
impl Stopwatch {
    pub fn elapsed(&self) -> f64 { ... }
}
```

**Testing:**
- Unit tests for each utility
- Doc tests showing usage
- Example programs reading/writing data

---

### **Phase 2: Collections & Union-Find**
**Estimated Effort:** 3 sessions
**Priority:** HIGH (Foundation for other data structures)
**Module:** `modules/fundamentals/`

**Java Files to Convert (16 files):**

**Collections (12 files):**
1. `Bag` - Multiset (unordered collection)
2. `LinkedBag` - Linked-list implementation
3. `ResizingArrayBag` - Array-based implementation
4. `Queue` - FIFO queue
5. `LinkedQueue` - Linked-list queue
6. `ResizingArrayQueue` - Circular array queue
7. `Stack` - LIFO stack
8. `LinkedStack` - Linked-list stack
9. `ResizingArrayStack` - Array-based stack
10. `SET` - Ordered set
11. `ST` - Ordered symbol table
12. `Knuth` - Knuth shuffle

**Union-Find (4 files):** ⭐ MOVED FROM PHASE 9
13. `UF` - Union-Find interface
14. `QuickFindUF` - Quick-find implementation
15. `QuickUnionUF` - Quick-union implementation
16. `WeightedQuickUnionUF` - Weighted with path compression

**Key Challenges:**
- Generic type parameters and trait bounds
- Iterator implementation for all collections
- Memory-safe linked structures (`Box<T>` and `Option<Box<Node<T>>>`)
- Resizing array implementation
- Union-Find with path compression

**Rust Adaptations:**
```rust
// Stack example
pub struct Stack<T> {
    first: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> Stack<T> {
    pub fn push(&mut self, item: T) { ... }
    pub fn pop(&mut self) -> Option<T> { ... }
}

impl<T> Iterator for StackIter<T> { ... }

// Union-Find
pub struct WeightedQuickUnionUF {
    parent: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl WeightedQuickUnionUF {
    pub fn union(&mut self, p: usize, q: usize) { ... }
    pub fn find(&mut self, p: usize) -> usize { ... }
}
```

**Why Union-Find is in Phase 2:**
- Needed for Kruskal's MST algorithm (Phase 7A)
- Fundamental data structure, not geometric
- Simple enough to implement early
- No dependencies on other structures

**Testing:**
- Unit tests for each collection
- Iterator tests
- Union-Find correctness tests
- Property tests (e.g., union-find maintains equivalence)

---

### **Phase 3: Sorting Algorithms**
**Estimated Effort:** 2-3 sessions
**Priority:** MEDIUM
**Module:** `modules/sorting/`

**Java Files to Convert (18 files):**
1. `Selection` - Selection sort
2. `Insertion` - Insertion sort
3. `InsertionX` - Insertion with sentinel
4. `BinaryInsertion` - Binary insertion sort
5. `Shell` - Shellsort
6. `Merge` - Top-down mergesort
7. `MergeBU` - Bottom-up mergesort
8. `MergeX` - Optimized mergesort
9. `Quick` - Quicksort
10. `Quick3way` - 3-way quicksort
11. `QuickX` - Optimized quicksort
12. `QuickBentleyMcIlroy` - Bentley-McIlroy 3-way
13. `Heap` - Heapsort
14. `LSD` - LSD radix sort
15. `MSD` - MSD radix sort
16. `InplaceMSD` - In-place MSD
17. `Quick3string` - 3-way string quicksort
18. `Inversions` - Count inversions

**Files to Skip:**
- `AmericanFlag`, `AmericanFlagX` (complex, rarely used)

**Key Challenges:**
- Generic comparison functions (use `Ord` trait)
- In-place sorting with slices
- String-specific optimizations
- Performance benchmarking

**Rust Adaptations:**
```rust
// Generic sorting functions
pub fn selection_sort<T: Ord>(arr: &mut [T]) { ... }
pub fn merge_sort<T: Ord + Clone>(arr: &mut [T]) { ... }
pub fn quick_sort<T: Ord>(arr: &mut [T]) { ... }

// String sorting
pub fn lsd_sort(arr: &mut [String], w: usize) { ... }
pub fn msd_sort(arr: &mut [String]) { ... }
```

**Testing:**
- Correctness: sorted output, stable sorts preserve order
- Property tests: output is permutation of input
- Benchmark against `slice::sort()` and `slice::sort_unstable()`
- Test edge cases: empty, single element, all equal, reverse sorted

---

### **Phase 4: Priority Queues**
**Estimated Effort:** 2-3 sessions
**Priority:** MEDIUM-HIGH
**Module:** `modules/fundamentals/`

**Java Files to Convert (10 files):**
1. `MaxPQ` - Maximum priority queue (binary heap)
2. `MinPQ` - Minimum priority queue
3. `IndexMaxPQ` - Index-based max PQ
4. `IndexMinPQ` - Index-based min PQ
5. `BinomialMinPQ` - Binomial heap
6. `FibonacciMinPQ` - Fibonacci heap
7. `IndexBinomialMinPQ` - Indexed binomial heap
8. `IndexFibonacciMinPQ` - Indexed Fibonacci heap
9. `MultiwayMinPQ` - Multiway heap
10. `IndexMultiwayMinPQ` - Indexed multiway heap

**Key Challenges:**
- Heap implementation with generics
- Index-based operations (map external keys to heap positions)
- Advanced heap structures (binomial, Fibonacci)
- Efficient decrease-key operations
- Complex pointer manipulation in safe Rust

**Rust Adaptations:**
```rust
// Basic priority queue
pub struct MaxPQ<T: Ord> {
    pq: Vec<Option<T>>,
    n: usize,
}

impl<T: Ord> MaxPQ<T> {
    pub fn insert(&mut self, item: T) { ... }
    pub fn del_max(&mut self) -> Option<T> { ... }
}

// Indexed priority queue
pub struct IndexMinPQ<T: Ord> {
    pq: Vec<usize>,    // heap of indices
    qp: Vec<isize>,    // inverse: qp[i] = position of i in pq
    keys: Vec<Option<T>>,
}

impl<T: Ord> IndexMinPQ<T> {
    pub fn insert(&mut self, i: usize, key: T) { ... }
    pub fn decrease_key(&mut self, i: usize, key: T) { ... }
}
```

**Note:** Advanced heaps (Fibonacci, Binomial) are complex. May take extra time.

**Testing:**
- Heap property maintained after operations
- Correct ordering of extracted elements
- Index-based operations work correctly
- Benchmark against `std::collections::BinaryHeap`

---

### **Phase 5: Searching & Symbol Tables**
**Estimated Effort:** 3-4 sessions
**Priority:** HIGH
**Module:** `modules/searching/`

**Java Files to Convert (20 files):**

**Search Algorithms (2):**
1. `BinarySearch` - Binary search in sorted array
2. `BinarySearchST` - Binary search symbol table

**Trees (4):**
3. `SequentialSearchST` - Unordered linked list
4. `BST` - Binary search tree
5. `RedBlackBST` - Red-black BST
6. `AVLTreeST` - AVL tree
7. `BTree` - B-tree

**Hash Tables (2):**
8. `SeparateChainingHashST` - Separate chaining
9. `LinearProbingHashST` - Linear probing

**Tries (4):**
10. `TrieSET` - R-way trie set
11. `TrieST` - R-way trie symbol table
12. `PatriciaSET` - Patricia trie set
13. `PatriciaST` - Patricia trie symbol table

**Applications (8):**
14. `FrequencyCounter` - Count word frequencies
15. `DeDup` - Remove duplicates
16. `Count` - Count occurrences
17. `FileIndex` - File indexing
18. `LookupCSV` - CSV lookup
19. `LookupIndex` - Index lookup
20. `KWIK` - Keyword in context (moved from strings)

**Key Challenges:**
- Self-balancing tree implementations (Red-Black especially complex)
- Hash function design (use Rust's `Hash` trait)
- Trie implementation with ownership
- Generic key-value storage with trait bounds

**Rust Adaptations:**
```rust
// Symbol table trait
pub trait SymbolTable<K, V> {
    fn put(&mut self, key: K, value: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn delete(&mut self, key: &K);
    fn size(&self) -> usize;
}

// BST implementation
pub struct BST<K: Ord, V> {
    root: Option<Box<Node<K, V>>>,
}

// Red-Black BST (complex!)
pub struct RedBlackBST<K: Ord, V> {
    root: Option<Box<RBNode<K, V>>>,
}

enum Color { Red, Black }

struct RBNode<K, V> {
    key: K,
    val: V,
    left: Option<Box<RBNode<K, V>>>,
    right: Option<Box<RBNode<K, V>>>,
    color: Color,
    size: usize,
}
```

**Testing:**
- BST property maintained
- Red-black tree invariants
- Hash table load factor and resizing
- Trie correctness
- Application programs work correctly

---

### **Phase 6: Graph Fundamentals**
**Estimated Effort:** 3 sessions
**Priority:** HIGH
**Module:** `modules/graphs/`

**Java Files to Convert (18 files):**

**Graph Structures (4):**
1. `Graph` - Undirected graph
2. `Digraph` - Directed graph
3. `EdgeWeightedGraph` - Weighted undirected graph
4. `EdgeWeightedDigraph` - Weighted directed graph

**Special Graphs (3):**
5. `AdjMatrixEdgeWeightedDigraph` - Adjacency matrix representation
6. `SymbolGraph` - String-vertex mapping
7. `SymbolDigraph` - Directed symbol graph

**Generators (2):**
8. `GraphGenerator` - Random graph generation
9. `DigraphGenerator` - Random digraph generation

**Undirected Traversal (3):**
10. `DepthFirstSearch` - DFS
11. `DepthFirstPaths` - DFS paths
12. `BreadthFirstPaths` - BFS paths

**Directed Traversal (5):**
13. `DepthFirstDirectedPaths` - Directed DFS paths
14. `BreadthFirstDirectedPaths` - Directed BFS paths
15. `NonrecursiveDFS` - Iterative DFS
16. `NonrecursiveDirectedDFS` - Directed iterative DFS
17. `DirectedDFS` - Reachability

**Components (1):**
18. `CC` - Connected components (undirected)

**Key Challenges:**
- Graph representation (adjacency list)
- Edge and vertex types
- Iterator patterns for graph traversal
- Memory-efficient structures
- Avoiding excessive cloning

**Rust Adaptations:**
```rust
// Basic graph
pub struct Graph {
    v: usize,              // number of vertices
    e: usize,              // number of edges
    adj: Vec<Vec<usize>>,  // adjacency lists
}

impl Graph {
    pub fn add_edge(&mut self, v: usize, w: usize) { ... }
    pub fn adj(&self, v: usize) -> &[usize] { ... }
}

// Weighted edge
#[derive(Clone, Copy)]
pub struct Edge {
    v: usize,
    w: usize,
    weight: f64,
}

// Directed edge
#[derive(Clone, Copy)]
pub struct DirectedEdge {
    from: usize,
    to: usize,
    weight: f64,
}

// Graph traits
pub trait GraphTrait {
    fn v(&self) -> usize;
    fn e(&self) -> usize;
    fn add_edge(&mut self, v: usize, w: usize);
    fn adj(&self, v: usize) -> Box<dyn Iterator<Item = usize> + '_>;
}
```

**Testing:**
- Graph construction correctness
- Traversal visits all reachable vertices
- Path reconstruction
- Connected components
- Test with standard graph files (tinyG.txt, etc.)

---

### **Phase 7A: Shortest Paths & MST**
**Estimated Effort:** 3-4 sessions
**Priority:** HIGH
**Module:** `modules/graphs/`

**Java Files to Convert (12 files):**

**Shortest Paths (8):**
1. `DijkstraSP` - Dijkstra's algorithm
2. `DijkstraUndirectedSP` - Dijkstra for undirected graphs
3. `DijkstraAllPairsSP` - All-pairs Dijkstra
4. `BellmanFordSP` - Bellman-Ford (handles negative weights)
5. `AcyclicSP` - Shortest paths in DAG
6. `AcyclicLP` - Longest paths in DAG
7. `FloydWarshall` - Floyd-Warshall all-pairs
8. `TransitiveClosure` - Transitive closure

**Minimum Spanning Trees (4):**
9. `LazyPrimMST` - Lazy Prim's algorithm
10. `PrimMST` - Eager Prim's algorithm
11. `KruskalMST` - Kruskal's algorithm ⭐ **Uses Union-Find from Phase 2**
12. `BoruvkaMST` - Boruvka's algorithm

**Key Challenges:**
- Shortest path with different edge weights (positive, negative, DAG)
- Priority queue usage for Dijkstra and Prim
- Union-Find for Kruskal
- Path reconstruction
- Handling negative cycles

**Dependencies:**
- Priority queues from Phase 4
- Union-Find from Phase 2
- Graph structures from Phase 6

**Rust Adaptations:**
```rust
// Dijkstra's algorithm
pub struct DijkstraSP {
    dist_to: Vec<f64>,
    edge_to: Vec<Option<DirectedEdge>>,
}

impl DijkstraSP {
    pub fn new(g: &EdgeWeightedDigraph, s: usize) -> Self { ... }
    pub fn dist_to(&self, v: usize) -> f64 { ... }
    pub fn path_to(&self, v: usize) -> Option<Vec<DirectedEdge>> { ... }
}

// Kruskal's MST
pub struct KruskalMST {
    mst: Vec<Edge>,
    weight: f64,
}

impl KruskalMST {
    pub fn new(g: &EdgeWeightedGraph) -> Self {
        let mut edges: Vec<Edge> = g.edges().collect();
        edges.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());

        let mut uf = WeightedQuickUnionUF::new(g.v());
        // ... Kruskal's algorithm
    }
}
```

**Testing:**
- Shortest path distances match expected values
- MST weight is correct
- Handles negative weights (Bellman-Ford)
- Detects negative cycles
- Test with standard graph files (tinyEWG.txt, etc.)

---

### **Phase 7B: Flow & Advanced Graph Analysis**
**Estimated Effort:** 3-4 sessions
**Priority:** MEDIUM
**Module:** `modules/graphs/`

**Java Files to Convert (18 files):**

**Cycles (8):**
1. `Cycle` - Cycle detection (undirected)
2. `DirectedCycle` - Cycle detection (directed)
3. `DirectedCycleX` - Nonrecursive cycle detection
4. `EdgeWeightedDirectedCycle` - Weighted directed cycle
5. `EulerianCycle` - Eulerian cycle (undirected)
6. `EulerianPath` - Eulerian path (undirected)
7. `DirectedEulerianCycle` - Directed Eulerian cycle
8. `DirectedEulerianPath` - Directed Eulerian path

**Topological & Ordering (2):**
9. `Topological` - Topological sort
10. `DepthFirstOrder` - DFS preorder/postorder/reverse postorder

**Strongly Connected Components (3):**
11. `KosarajuSharirSCC` - Kosaraju-Sharir algorithm
12. `TarjanSCC` - Tarjan's algorithm
13. `GabowSCC` - Gabow's algorithm

**Bipartite (2):**
14. `Bipartite` - Bipartite detection
15. `BipartiteX` - Nonrecursive bipartite

**Maximum Flow (2):**
16. `FlowEdge` - Flow edge data type
17. `FlowNetwork` - Flow network
18. `FordFulkerson` - Ford-Fulkerson algorithm

**Files moved to Phase 7C:**
- `BipartiteMatching`, `HopcroftKarp` (can be Phase 7C if needed)
- `GlobalMincut` (part of flow)
- `CPM`, `DegreesOfSeparation`, `Arbitrage`, `AssignmentProblem` (applications)

**Key Challenges:**
- Cycle detection in various graph types
- Topological ordering
- SCC algorithms (complex)
- Bipartite matching
- Maximum flow implementation
- Residual graph handling

**Rust Adaptations:**
```rust
// Topological sort
pub struct Topological {
    order: Option<Vec<usize>>,
}

impl Topological {
    pub fn new(g: &Digraph) -> Self {
        let cycle = DirectedCycle::new(g);
        if !cycle.has_cycle() {
            let dfo = DepthFirstOrder::new(g);
            Self { order: Some(dfo.reverse_post()) }
        } else {
            Self { order: None }
        }
    }
}

// Flow edge
pub struct FlowEdge {
    v: usize,
    w: usize,
    capacity: f64,
    flow: f64,
}

impl FlowEdge {
    pub fn residual_capacity_to(&self, vertex: usize) -> f64 { ... }
    pub fn add_residual_flow_to(&mut self, vertex: usize, delta: f64) { ... }
}

// Ford-Fulkerson
pub struct FordFulkerson {
    value: f64,
    marked: Vec<bool>,
}
```

**Testing:**
- Cycle detection correctness
- Topological order is valid
- SCC correctness
- Bipartite detection
- Max flow value matches expected
- Min-cut correctness

---

### **Phase 7C: Graph Applications** (Optional Extension)
**Estimated Effort:** 2 sessions
**Priority:** LOW
**Module:** `modules/graphs/`

**Java Files to Convert (6 files):**
1. `BipartiteMatching` - Bipartite matching
2. `HopcroftKarp` - Hopcroft-Karp algorithm
3. `GlobalMincut` - Global minimum cut
4. `AssignmentProblem` - Assignment problem
5. `CPM` - Critical path method
6. `DegreesOfSeparation` - Degrees of separation
7. `Arbitrage` - Arbitrage detection

**Note:** These are application programs that can be done later or as examples.

---

### **Phase 8: String Processing**
**Estimated Effort:** 3 sessions
**Priority:** MEDIUM
**Module:** `modules/strings/`

**Java Files to Convert (17 files):**

**Pattern Matching (3):**
1. `KMP` - Knuth-Morris-Pratt
2. `BoyerMoore` - Boyer-Moore
3. `RabinKarp` - Rabin-Karp

**Regular Expressions (2):**
4. `NFA` - Nondeterministic finite automaton
5. `GREP` - Pattern matching with NFA

**Compression (3):**
6. `LZW` - LZW compression
7. `Huffman` - Huffman coding
8. `RunLength` - Run-length encoding

**Suffix Structures (2):**
9. `SuffixArray` - Suffix array
10. `SuffixArrayX` - Optimized suffix array

**String Utilities (2):**
11. `Alphabet` - Alphabet data type
12. `Genome` - Genome data type

**Applications (5):**
13. `LongestCommonSubstring` - LCS
14. `LongestRepeatedSubstring` - LRS
15. `TopologicalX` - String topological sort (if not in Phase 7B)

**Files Moved:**
- `KWIK` → Phase 5 (applications)

**Key Challenges:**
- String pattern matching with Rust strings (UTF-8 vs bytes)
- Compression algorithms
- Suffix array implementation
- Regular expression engine
- Handling both text and binary data

**Rust Adaptations:**
```rust
// Pattern matching (byte-level)
pub struct KMP {
    pattern: Vec<u8>,
    dfa: Vec<Vec<usize>>,
}

impl KMP {
    pub fn new(pattern: &[u8]) -> Self { ... }
    pub fn search(&self, text: &[u8]) -> Option<usize> { ... }
}

// Compression
pub fn compress(input: &[u8]) -> Vec<u8> { ... }
pub fn decompress(input: &[u8]) -> Vec<u8> { ... }

// Suffix array
pub struct SuffixArray {
    text: Vec<u8>,
    index: Vec<usize>,
}

impl SuffixArray {
    pub fn lcp(&self, i: usize) -> usize { ... }
    pub fn rank(&self, query: &[u8]) -> usize { ... }
}
```

**String Handling Decision:**
- UTF-8 strings: Use `&str` for text processing
- Byte-level: Use `&[u8]` for compression and low-level algorithms
- Document when to use each

**Testing:**
- Pattern matching finds correct positions
- Compression/decompression roundtrip
- Suffix array LCP values correct
- Regex matching works
- Test with both ASCII and UTF-8 data

---

### **Phase 9: Geometric Algorithms**
**Estimated Effort:** 2 sessions
**Priority:** LOW
**Module:** `modules/geometry/`

**Java Files to Convert (9 files):**

**Geometric Primitives (4):**
1. `Point2D` - 2D point
2. `RectHV` - Axis-aligned rectangle
3. `Interval1D` - 1D interval
4. `Interval2D` - 2D interval

**Geometric Algorithms (3):**
5. `ClosestPair` - Closest pair of points
6. `FarthestPair` - Farthest pair of points
7. `GrahamScan` - Graham scan convex hull

**Visualization (2):**
8. `Draw` - Basic drawing (optional, feature-gated)
9. `DrawListener` - Drawing events (optional)

**Files Removed:**
- Union-Find moved to Phase 2
- `StdDraw` moved to Phase 11 (multimedia)

**Key Challenges:**
- Geometric data structures
- Floating-point precision
- Convex hull algorithm
- Visualization (optional feature)

**Rust Adaptations:**
```rust
// Point
#[derive(Clone, Copy, Debug)]
pub struct Point2D {
    x: f64,
    y: f64,
}

impl Point2D {
    pub fn distance_to(&self, that: &Point2D) -> f64 { ... }
    pub fn ccw(a: &Point2D, b: &Point2D, c: &Point2D) -> i32 { ... }
}

// Convex hull
pub struct GrahamScan {
    hull: Vec<Point2D>,
}

impl GrahamScan {
    pub fn new(points: &[Point2D]) -> Self { ... }
    pub fn hull(&self) -> &[Point2D] { ... }
}
```

**Testing:**
- Geometric calculations correct
- Convex hull properties verified
- Edge cases: collinear points, all same point

---

### **Phase 10: Advanced Core Algorithms**
**Estimated Effort:** 3 sessions
**Priority:** LOW
**Module:** `modules/advanced/`

**Java Files to Convert (15 files):**

**Linear Algebra (5):**
1. `Vector` - Vector operations
2. `SparseVector` - Sparse vector
3. `Complex` - Complex numbers (or use `num-complex` crate)
4. `Polynomial` - Polynomial
5. `FFT` - Fast Fourier Transform

**Linear Programming & Linear Algebra (3):**
6. `GaussianElimination` - Gaussian elimination
7. `GaussJordanElimination` - Gauss-Jordan elimination
8. `LinearProgramming` - Simplex algorithm

**Advanced Data Structures (2):**
9. `SegmentTree` - Segment tree
10. `FenwickTree` - Fenwick tree (Binary Indexed Tree)

**Miscellaneous Algorithms (5):**
11. `ThreeSum` - 3-sum problem
12. `ThreeSumFast` - Optimized 3-sum
13. `AllowFilter` - Allow filter
14. `BlockFilter` - Block filter
15. `TopM` - Top M elements

**Files Moved to Phase 11 (Multimedia):**
- `Picture`, `StdPicture`, `GrayscalePicture`
- `StdAudio`, `StdAudioStereo`
- `CollisionSystem`, `Particle` (physics simulation with graphics)
- `TwoPersonZeroSumGame` (game theory - could stay if no graphics)

**Optional Dependencies:**
```toml
[dependencies]
num-complex = { version = "0.4", optional = true }

[features]
complex = ["num-complex"]
```

**Key Challenges:**
- Numerical computing precision
- FFT implementation
- Simplex algorithm complexity
- Advanced data structures

**Rust Adaptations:**
```rust
// FFT
pub fn fft(x: &mut [Complex<f64>]) { ... }
pub fn ifft(x: &mut [Complex<f64>]) { ... }

// Segment tree
pub struct SegmentTree<T> {
    tree: Vec<T>,
    n: usize,
}

impl<T: Copy + Default> SegmentTree<T> {
    pub fn query(&self, left: usize, right: usize) -> T { ... }
    pub fn update(&mut self, index: usize, value: T) { ... }
}
```

**Testing:**
- Numerical algorithms within precision tolerance
- FFT correctness
- Segment tree range queries
- Linear programming solutions

---

### **Phase 11: Multimedia & Visualization** (OPTIONAL)
**Estimated Effort:** 4+ sessions
**Priority:** OPTIONAL
**Module:** `modules/multimedia/` (new module)

**Feature-Gated:** Users opt-in via Cargo features

```toml
[features]
default = []
graphics = ["plotters"]
audio = ["rodio"]
image = ["image"]
full-multimedia = ["graphics", "audio", "image"]
```

**Java Files to Convert (25+ files):**

**Image Processing (3):**
- `Picture` - Image manipulation
- `StdPicture` - Standard picture I/O
- `GrayscalePicture` - Grayscale images

**Graphics/Visualization (3):**
- `StdDraw` - 2D drawing
- `Draw` - Drawing canvas (if not in Phase 9)
- `DrawListener` - Event handling

**Audio (2):**
- `StdAudio` - Audio playback
- `StdAudioStereo` - Stereo audio

**Physics Simulation (2):**
- `CollisionSystem` - Particle collisions
- `Particle` - Particle physics

**Game Theory (1):**
- `TwoPersonZeroSumGame` - Zero-sum games

**Remaining Utilities (~14 files):**
- Binary I/O if not done: `BinaryIn`, `BinaryOut`, etc.
- `Date`, `Transaction` if not in Phase 1
- Other miscellaneous utilities

**Implementation Strategy:**
1. Use existing Rust crates as backends:
   - `plotters` for 2D graphics
   - `image` for picture processing
   - `rodio` for audio playback
2. Provide thin wrapper API matching original where possible
3. Document differences from Java version
4. All multimedia is opt-in via features

**Why Optional:**
- Not core algorithms
- Platform-dependent (audio/graphics)
- Heavy dependencies
- May not work in all environments
- Educational value lower than core algorithms

**Testing:**
- Unit tests for API
- Visual tests (manual)
- Skip in CI if dependencies unavailable

---

## Module Dependencies (Updated)

```
fundamentals (Phase 1-2)
    - No dependencies
    - Includes: I/O, collections, union-find, priority queues
    ↓
sorting (Phase 3)
    - Depends on: fundamentals (for I/O, testing)
    ↓
searching (Phase 5)
    - Depends on: fundamentals
    ↓
graphs (Phase 6, 7A, 7B, 7C)
    - Depends on: fundamentals (priority queues, union-find)
    - Depends on: searching (symbol tables for SymbolGraph)
    ↓
strings (Phase 8)
    - Depends on: fundamentals
    - Depends on: searching (tries)
    ↓
geometry (Phase 9)
    - Depends on: fundamentals
    ↓
advanced (Phase 10)
    - Depends on: fundamentals
    - May depend on: graphs (for some applications)
    ↓
multimedia (Phase 11 - OPTIONAL)
    - Depends on: fundamentals
    - Optional external crates
```

---

## Conversion Guidelines

### General Principles

1. **Naming Conventions:**
   - Java classes → Rust structs/modules
   - `camelCase` → `snake_case` for functions and variables
   - Keep algorithm names recognizable (e.g., `DijkstraSP` → `dijkstra_sp` module with `DijkstraSP` struct)

2. **Type System:**
   - `int` → `i32` or `usize` (for indices)
   - `double` → `f64`
   - `boolean` → `bool`
   - `String` → `String` or `&str`
   - Generic types: `<T>` → `<T>`

3. **Memory Management:**
   - No `null` → Use `Option<T>`
   - Linked structures → `Box<T>` and `Option<Box<Node<T>>>`
   - Arrays → `Vec<T>` or slices `&[T]`, `&mut [T]`
   - References → `&T` (immutable) or `&mut T` (mutable)

4. **Error Handling:**
   - Exceptions → `Result<T, E>`
   - Implement custom error types where needed
   - Use `?` operator for error propagation

5. **Iterators:**
   - Implement `Iterator` trait for collections
   - Use iterator adapters: `map`, `filter`, `fold`, etc.
   - Provide both consuming and borrowing iterators

6. **Traits:**
   - Common traits: `Debug`, `Clone`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`, `Display`
   - Custom traits for algorithms: `Graph`, `SymbolTable`, `PriorityQueue`

7. **Testing:**
   - Unit tests in same file: `#[cfg(test)] mod tests { ... }`
   - Integration tests in `tests/` directory
   - Doc tests in documentation comments
   - Property-based testing (consider `proptest` crate)

8. **Documentation:**
   - Use `///` for documentation comments
   - Include examples in doc comments
   - Document time/space complexity
   - Reference original textbook sections

9. **Performance:**
   - Use `#[inline]` for hot paths
   - Benchmark critical algorithms
   - Profile and optimize after correctness is verified
   - Compare with standard library implementations

10. **Dependencies:**
    - Minimize external dependencies
    - Prefer standard library when possible
    - Document why each dependency is needed

---

## Workspace Organization

### Cargo Workspace Structure

```toml
[workspace]
members = [
    "modules/fundamentals",
    "modules/sorting",
    "modules/searching",
    "modules/graphs",
    "modules/strings",
    "modules/geometry",
    "modules/advanced",
    "modules/multimedia",  # Optional
]
```

---

## Testing Strategy

1. **Unit Tests:**
   - Test each algorithm with small inputs
   - Edge cases: empty, single element, duplicates
   - Verify invariants and postconditions

2. **Integration Tests:**
   - Test algorithm combinations
   - Use example data from textbook
   - Compare outputs with known correct results

3. **Property-Based Tests:**
   - Sorting: output is sorted, permutation of input
   - Search: find returns correct element or None
   - Graph: paths are valid, MST properties hold

4. **Performance Tests:**
   - Benchmark against standard library
   - Verify time complexity empirically
   - Track performance regressions

5. **Correctness:**
   - Cross-reference with Java implementation
   - Use textbook test cases (in `tests/data/`)
   - Validate against algorithm specifications

---

## Build Instructions

### Prerequisites

- Rust 1.70+ (stable)
- Cargo (comes with Rust)

### Building

```bash
# Clone repository
git clone <repo-url>
cd algs4-rust

# Build all modules
cargo build --release

# Build specific module
cargo build -p algs4-fundamentals

# Run tests
cargo test --all

# Run benchmarks
cargo bench --all

# Generate documentation
cargo doc --open --no-deps

# Build with multimedia features
cargo build --features full-multimedia
```

### Development

```bash
# Check code
cargo check --all

# Format code
cargo fmt --all

# Lint code
cargo clippy --all -- -D warnings

# Watch mode (requires cargo-watch)
cargo watch -x check -x test
```

---

## Quality Standards

### Code Quality

- **Formatting:** Use `rustfmt` with default settings
- **Linting:** Pass `clippy` with no warnings
- **Testing:** Minimum 80% code coverage
- **Documentation:** All public APIs documented
- **Safety:** No `unsafe` unless absolutely necessary (and well documented)

### Git Workflow

- **Commits:** One logical change per commit
- **Messages:** Descriptive commit messages
- **Branches:** Feature branches for each phase
- **Reviews:** All code should be reviewed (if working with team)

### CI/CD

Configured in `.github/workflows/ci.yml`:
- Automated testing on push
- Format and lint checks
- Documentation building
- Multiple Rust versions (stable, beta)

---

## Timeline Estimates (Updated)

| Phase | Description | Files | Estimated Time |
|-------|-------------|-------|----------------|
| 0 | Project Foundation | - | 1 day ✅ |
| 1 | Core I/O & Basic Types | 8 | 2 days |
| 2 | Collections & Union-Find | 16 | 3 days |
| 3 | Sorting Algorithms | 18 | 3 days |
| 4 | Priority Queues | 10 | 3 days |
| 5 | Searching & Symbol Tables | 20 | 4 days |
| 6 | Graph Fundamentals | 18 | 3 days |
| 7A | Shortest Paths & MST | 12 | 4 days |
| 7B | Flow & Advanced Graph | 18 | 4 days |
| 7C | Graph Applications (opt) | 6 | 2 days |
| 8 | String Processing | 17 | 3 days |
| 9 | Geometric Algorithms | 9 | 2 days |
| 10 | Advanced Core | 15 | 3 days |
| **Core Total** | **Phases 0-10** | **~160** | **37 days** |
| 11 | Multimedia (optional) | 25+ | 4+ days |
| **Complete Total** | **All Phases** | **185+** | **41+ days** |

*Note: Times are estimates for focused work. Actual time may vary based on experience level and available time.*

---

## Session Management

### Starting a New Phase

1. Read the relevant section of this plan
2. Review Java source files for the phase at https://github.com/kevin-wayne/algs4
3. Create module structure if not exists
4. Implement, test, document
5. Commit regularly with clear messages: `git commit -m "Phase X: Implement Y"`
6. Update `PROGRESS.md` with completed files
7. Run `cargo test --all && cargo clippy --all` before finishing
8. Push when phase is complete

### Continuing Work

1. Review previous session's progress in `PROGRESS.md`
2. Check TODO comments in code
3. Run tests to ensure nothing broke
4. Continue with next component
5. Update this plan if scope changes

### Phase Completion Checklist

For each phase, ensure:

- [ ] All planned files converted
- [ ] Unit tests written and passing
- [ ] Integration tests written and passing (where applicable)
- [ ] Documentation complete (doc comments + examples)
- [ ] Benchmarks implemented (where appropriate)
- [ ] Code formatted: `cargo fmt --all`
- [ ] Code linted: `cargo clippy --all -- -D warnings`
- [ ] CI passing
- [ ] `PROGRESS.md` updated
- [ ] Commit and push

---

## Progress Tracking

Create a `PROGRESS.md` file to track detailed progress:

```markdown
# algs4-rust Progress Tracker

## Phase 0: Project Foundation ✅ COMPLETE
- 2025-11-13: Initial setup

## Phase 1: Core I/O & Basic Types (0/8)
- [ ] StdIn
- [ ] StdOut
- [ ] In
- [ ] Out
- [ ] StdRandom
- [ ] Stopwatch
- [ ] Counter
- [ ] Accumulator

## Phase 2: Collections & Union-Find (0/16)
...
```

---

## Success Criteria

The project will be considered successful when:

1. ✅ All core algorithms (phases 1-10) are converted (~160 files)
2. ✅ Comprehensive test suite with >80% coverage
3. ✅ Documentation for all public APIs with examples
4. ✅ Performance benchmarks showing competitive performance
5. ✅ Example programs demonstrating usage
6. ✅ CI/CD pipeline passing
7. ✅ Published to crates.io (optional)
8. ✅ API guide document created

---

## Resources

- **Original Repository:** https://github.com/kevin-wayne/algs4
- **Java Source Files:** https://github.com/kevin-wayne/algs4/tree/master/src/main/java/edu/princeton/cs/algs4
- **Textbook:** "Algorithms, 4th Edition" by Sedgewick & Wayne
- **Booksite:** https://algs4.cs.princeton.edu/
- **Test Data Files:** https://algs4.cs.princeton.edu/code/ (various .txt files)
- **Rust Book:** https://doc.rust-lang.org/book/
- **Rust API Guidelines:** https://rust-lang.github.io/api-guidelines/
- **Rust By Example:** https://doc.rust-lang.org/rust-by-example/

---

## Contributing

(To be defined based on project goals)

### For Future Contributors:
1. Read this plan thoroughly
2. Pick a phase or file to work on
3. Follow the conversion guidelines
4. Write tests and documentation
5. Submit PR with clear description
6. Update `PROGRESS.md`

---

## License

GPLv3 - Inherited from the original algs4 Java implementation.

Copyright notice to be maintained in all source files.

---

**Last Updated:** 2025-11-13
**Version:** 2.0
**Status:** Phase 0 Complete - Ready for Phase 1
**Next Phase:** Phase 1 - Core I/O & Basic Types

---

## Notes for Sub-Agents / Future Sessions

### Important Changes from v1.0:
- ✅ Union-Find moved from Phase 9 to Phase 2 (needed for Kruskal's)
- ✅ Phase 7 split into 7A (Shortest Paths & MST) and 7B (Flow & Advanced)
- ✅ Phase 10 split into core (10) and multimedia (11)
- ✅ Phase 1 reduced to essential I/O only (8 files)
- ✅ Explicit decisions made on API, strings, multimedia, testing
- ✅ Files to skip documented (~41 files)
- ✅ Core conversion target: ~160 files (was 201)
- ✅ Timeline revised to 37 days for core (was 35-55)

### Key Points:
- This plan is a living document - update as needed
- Each phase is designed to fit in typical context window
- Phases can be parallelized if multiple developers/sessions available
- Focus on one phase at a time for single developer
- Keep `PROGRESS.md` updated with completed files
- Add notes about challenges encountered for future reference
- Refer to `.claude/context/` files for detailed conversion patterns

### Starting Phase 1:
1. Read Phase 1 section above
2. Review Java files in original repo
3. Create I/O modules in `modules/fundamentals/src/io/`
4. Start with `StdIn` and `StdOut` (most commonly used)
5. Write comprehensive tests with example usage
6. Document with examples from textbook

**Good luck! 🦀**
