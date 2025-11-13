# algs4-rust: Java to Rust Conversion Plan

## Project Overview

This project converts the Java implementation from [kevin-wayne/algs4](https://github.com/kevin-wayne/algs4) (Algorithms, 4th Edition by Robert Sedgewick and Kevin Wayne) to idiomatic Rust.

**Source Repository:** https://github.com/kevin-wayne/algs4
**Target Repository:** algs4-rust
**Total Files to Convert:** ~201 Java source files
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
├── LICENSE                     # GPLv3 license
├── .claude/                    # Claude Code context
│   ├── commands/              # Custom slash commands
│   └── context/               # Additional context files
├── src/
│   └── lib.rs                 # Library root
├── modules/                    # Main algorithm modules (workspace members)
│   ├── fundamentals/          # Basic data structures
│   ├── sorting/               # Sorting algorithms
│   ├── searching/             # Search algorithms
│   ├── graphs/                # Graph algorithms
│   ├── strings/               # String processing
│   ├── geometry/              # Geometric algorithms
│   └── advanced/              # Advanced topics
├── examples/                   # Example programs (equivalent to Java clients)
├── tests/                      # Integration tests
└── benches/                    # Performance benchmarks
```

## Conversion Phases

The work is divided into **10 phases** to manage complexity and context windows. Each phase can be handled by a single session or sub-agent.

---

### **Phase 0: Project Foundation** ✓ (Current Phase)
**Estimated Effort:** 1 session
**Deliverables:**
- [x] Project structure setup
- [x] Cargo workspace configuration
- [x] .gitignore and basic files
- [x] Claude context files
- [x] Conversion plan document
- [x] Initial README
- [x] CI/CD setup (optional)

**Files Created:**
- `Cargo.toml` (workspace)
- `.gitignore`
- `README.md`
- `CONVERSION_PLAN.md`
- `.claude/` context files
- `LICENSE`

---

### **Phase 1: Fundamentals - I/O & Utilities**
**Estimated Effort:** 2-3 sessions
**Priority:** HIGH (Required by all other modules)
**Module:** `modules/fundamentals/`

**Java Files to Convert (22 files):**
- I/O: `In`, `Out`, `StdIn`, `StdOut`, `BinaryIn`, `BinaryOut`, `BinaryStdIn`, `BinaryStdOut`
- Binary utilities: `BinaryDump`, `HexDump`, `PictureDump`, `Cat`
- Statistics: `StdRandom`, `StdStats`, `Stopwatch`, `StopwatchCPU`
- Basic types: `Counter`, `Accumulator`, `Date`, `Transaction`
- Array I/O: `StdArrayIO`
- Testing: `DoublingTest`, `DoublingRatio`, `RandomSeq`

**Key Challenges:**
- Standard input/output handling (use `std::io`)
- Random number generation (use `rand` crate)
- Binary I/O (use `byteorder` crate)
- Timer utilities (use `std::time`)

**Dependencies:**
- `rand` - Random number generation
- `byteorder` - Binary I/O

**Rust Adaptations:**
- Replace Java's `StdIn` with Rust's `std::io::stdin()`
- Use iterators for array processing
- Implement `Display` and `Debug` traits for custom types
- Use `Result<T, E>` for error handling

---

### **Phase 2: Fundamentals - Basic Collections**
**Estimated Effort:** 2-3 sessions
**Priority:** HIGH (Foundation for other data structures)
**Module:** `modules/fundamentals/`

**Java Files to Convert (12 files):**
- Bags: `Bag`, `LinkedBag`, `ResizingArrayBag`
- Queues: `Queue`, `LinkedQueue`, `ResizingArrayQueue`
- Stacks: `Stack`, `LinkedStack`, `ResizingArrayStack`
- Generic: `SET`, `ST`
- Testing: `Knuth` (shuffling)

**Key Challenges:**
- Generic type parameters and trait bounds
- Iterator implementation for all collections
- Memory-safe linked structures (use `Box<T>` and `Option<Box<Node<T>>>`)
- Resizing array implementation (use `Vec<T>` or custom)

**Rust Adaptations:**
- Implement `Iterator` trait for all collections
- Use `Option<T>` for nullable references
- Implement common traits: `Clone`, `Debug`, `Default`
- Use Rust's `Vec<T>` where appropriate but show educational implementations

---

### **Phase 3: Sorting Algorithms**
**Estimated Effort:** 2 sessions
**Priority:** MEDIUM
**Module:** `modules/sorting/`

**Java Files to Convert (18 files):**
- Basic: `Selection`, `Insertion`, `InsertionX`, `BinaryInsertion`, `Shell`
- Merge sorts: `Merge`, `MergeBU`, `MergeX`
- Quick sorts: `Quick`, `Quick3way`, `QuickX`, `QuickBentleyMcIlroy`
- Heap sort: `Heap`
- String sorts: `LSD`, `MSD`, `InplaceMSD`, `Quick3string`
- Advanced: `AmericanFlag`, `AmericanFlagX`
- Utility: `Inversions`

**Key Challenges:**
- Generic comparison functions (use `Ord` trait)
- In-place sorting with slices
- String-specific optimizations
- Performance benchmarking

**Rust Adaptations:**
- Use trait bounds: `T: Ord`, `T: PartialOrd`
- Leverage slice methods: `&mut [T]`
- Implement as generic functions, not classes
- Create comprehensive benchmarks

---

### **Phase 4: Priority Queues**
**Estimated Effort:** 2 sessions
**Priority:** MEDIUM
**Module:** `modules/fundamentals/`

**Java Files to Convert (10 files):**
- Basic: `MaxPQ`, `MinPQ`
- Indexed: `IndexMaxPQ`, `IndexMinPQ`
- Advanced: `BinomialMinPQ`, `FibonacciMinPQ`
- Indexed advanced: `IndexBinomialMinPQ`, `IndexFibonacciMinPQ`
- Multiway: `MultiwayMinPQ`, `IndexMultiwayMinPQ`

**Key Challenges:**
- Heap implementation with generics
- Index-based operations
- Advanced heap structures (binomial, Fibonacci)
- Efficient decrease-key operations

**Rust Adaptations:**
- Use `Vec<T>` for heap storage
- Implement `Ord` comparisons
- Consider using Rust's `std::collections::BinaryHeap` as reference
- Implement both min and max heaps generically

---

### **Phase 5: Searching & Symbol Tables**
**Estimated Effort:** 3 sessions
**Priority:** HIGH
**Module:** `modules/searching/`

**Java Files to Convert (20 files):**
- Binary search: `BinarySearch`, `BinarySearchST`
- Sequential: `SequentialSearchST`
- Binary search trees: `BST`, `RedBlackBST`, `AVLTreeST`, `BTree`
- Hash tables: `SeparateChainingHashST`, `LinearProbingHashST`
- Tries: `TrieSET`, `TrieST`, `PatriciaSET`, `PatriciaST`
- Applications: `FrequencyCounter`, `DeDup`, `Count`, `FileIndex`, `LookupCSV`, `LookupIndex`

**Key Challenges:**
- Self-balancing tree implementations
- Hash function design (use Rust's `Hash` trait)
- Trie implementation with ownership
- Generic key-value storage

**Rust Adaptations:**
- Use `std::collections::HashMap` as reference but provide educational implementations
- Implement `Hash` and `Eq` traits for keys
- Use `Option<Box<Node>>` for tree nodes
- Provide trait-based API: `SymbolTable` trait

---

### **Phase 6: Graph Fundamentals**
**Estimated Effort:** 3 sessions
**Priority:** HIGH
**Module:** `modules/graphs/`

**Java Files to Convert (18 files):**
- Graph structures: `Graph`, `Digraph`, `EdgeWeightedGraph`, `EdgeWeightedDigraph`
- Special: `AdjMatrixEdgeWeightedDigraph`, `SymbolGraph`, `SymbolDigraph`
- Generators: `GraphGenerator`, `DigraphGenerator`
- Basic traversal: `DepthFirstSearch`, `DepthFirstPaths`, `BreadthFirstPaths`
- Directed: `DepthFirstDirectedPaths`, `BreadthFirstDirectedPaths`, `NonrecursiveDFS`, `NonrecursiveDirectedDFS`, `DirectedDFS`
- Components: `CC` (connected components)

**Key Challenges:**
- Graph representation (adjacency list)
- Edge and vertex types
- Iterator patterns for graph traversal
- Memory-efficient structures

**Rust Adaptations:**
- Use `Vec<Vec<usize>>` for adjacency lists
- Create `Edge` and `DirectedEdge` structs
- Implement graph traits: `GraphTrait`, `DigraphTrait`
- Use `VecDeque` for BFS queues

---

### **Phase 7: Advanced Graph Algorithms**
**Estimated Effort:** 3-4 sessions
**Priority:** MEDIUM
**Module:** `modules/graphs/`

**Java Files to Convert (30 files):**
- Cycles: `Cycle`, `DirectedCycle`, `DirectedCycleX`, `EdgeWeightedDirectedCycle`, `EulerianCycle`, `EulerianPath`, `DirectedEulerianCycle`, `DirectedEulerianPath`
- Topological: `Topological`, `TopologicalX`, `DepthFirstOrder`
- SCC: `KosarajuSharirSCC`, `TarjanSCC`, `GabowSCC`
- Bipartite: `Bipartite`, `BipartiteX`, `BipartiteMatching`, `HopcroftKarp`
- Shortest paths: `DijkstraSP`, `DijkstraUndirectedSP`, `DijkstraAllPairsSP`, `BellmanFordSP`, `AcyclicSP`, `AcyclicLP`, `FloydWarshall`, `TransitiveClosure`
- MST: `LazyPrimMST`, `PrimMST`, `KruskalMST`, `BoruvkaMST`
- Flow: `FlowEdge`, `FlowNetwork`, `FordFulkerson`, `GlobalMincut`
- Applications: `CPM`, `DegreesOfSeparation`, `Arbitrage`, `AssignmentProblem`

**Key Challenges:**
- Complex graph algorithms
- Shortest path with different edge weights
- Maximum flow implementation
- Bipartite matching algorithms

**Rust Adaptations:**
- Use priority queues from Phase 4
- Implement efficient path reconstruction
- Use `f64` for weighted edges
- Create dedicated edge types for flow networks

---

### **Phase 8: String Processing**
**Estimated Effort:** 3 sessions
**Priority:** MEDIUM
**Module:** `modules/strings/`

**Java Files to Convert (18 files):**
- Pattern matching: `KMP`, `BoyerMoore`, `RabinKarp`
- Regular expressions: `NFA`, `GREP`
- Compression: `LZW`, `Huffman`, `RunLength`
- Suffix structures: `SuffixArray`, `SuffixArrayX`
- String utilities: `Alphabet`, `Genome`
- Applications: `LongestCommonSubstring`, `LongestRepeatedSubstring`, `KWIK`

**Key Challenges:**
- String pattern matching with Rust strings (UTF-8)
- Compression algorithms
- Suffix array implementation
- Regular expression engine

**Rust Adaptations:**
- Work with `&str` and `String` types
- Handle UTF-8 encoding properly
- Use `Vec<u8>` for binary data
- Consider using `regex` crate as reference but implement educational versions

---

### **Phase 9: Geometry & Union-Find**
**Estimated Effort:** 2 sessions
**Priority:** LOW
**Module:** `modules/geometry/`

**Java Files to Convert (13 files):**
- Union-Find: `UF`, `QuickFindUF`, `QuickUnionUF`, `WeightedQuickUnionUF`
- Geometry: `Point2D`, `RectHV`, `Interval1D`, `Interval2D`
- Geometric algorithms: `ClosestPair`, `FarthestPair`, `GrahamScan`
- Visualization: `Draw`, `DrawListener`, `StdDraw`

**Key Challenges:**
- Union-Find optimization (path compression, union by rank)
- Geometric data structures
- Graphics/visualization (consider optional feature)

**Rust Adaptations:**
- Implement union-find with Vec<usize>
- Create geometric primitives with `f64`
- Consider `plotters` crate for visualization (optional)
- Implement geometric traits: `Point`, `Shape`

---

### **Phase 10: Advanced Topics & Utilities**
**Estimated Effort:** 3 sessions
**Priority:** LOW
**Module:** `modules/advanced/`

**Java Files to Convert (40 files):**
- Linear algebra: `Vector`, `SparseVector`, `Complex`, `Polynomial`, `FFT`
- Linear programming: `LinearProgramming`, `GaussianElimination`, `GaussJordanElimination`
- Data structures: `SegmentTree`, `FenwickTree`
- Physics: `CollisionSystem`, `Particle`
- Multimedia: `Picture`, `StdPicture`, `GrayscalePicture`, `StdAudio`, `StdAudioStereo`
- Games: `TwoPersonZeroSumGame`
- Misc: `ThreeSum`, `ThreeSumFast`, `AllowFilter`, `BlockFilter`, `TopM`

**Key Challenges:**
- Numerical computing (consider `num` crate)
- Complex multimedia (images, audio)
- Advanced mathematical algorithms

**Rust Adaptations:**
- Use `num-complex` for complex numbers
- Use `image` crate for picture processing (optional)
- Use `rodio` for audio (optional)
- Implement core algorithms, make multimedia optional features

---

## Conversion Guidelines

### General Principles

1. **Naming Conventions:**
   - Java classes → Rust structs/modules
   - `camelCase` → `snake_case` for functions and variables
   - Keep algorithm names recognizable (e.g., `DijkstraSP` → `dijkstra_sp`)

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
]
```

### Module Dependencies

```
fundamentals (no dependencies)
    ↓
sorting, searching (depends on fundamentals)
    ↓
graphs (depends on fundamentals, searching)
    ↓
strings (depends on fundamentals, searching)
    ↓
geometry (depends on fundamentals)
    ↓
advanced (depends on all)
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
   - Use textbook test cases
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

Consider setting up:
- GitHub Actions for CI
- Automated testing on push
- Documentation deployment
- Release automation

---

## Timeline Estimates

| Phase | Description | Estimated Time |
|-------|-------------|----------------|
| 0 | Project Foundation | 1 day |
| 1 | Fundamentals - I/O & Utilities | 3-5 days |
| 2 | Fundamentals - Basic Collections | 3-5 days |
| 3 | Sorting Algorithms | 2-4 days |
| 4 | Priority Queues | 2-4 days |
| 5 | Searching & Symbol Tables | 4-6 days |
| 6 | Graph Fundamentals | 4-6 days |
| 7 | Advanced Graph Algorithms | 5-8 days |
| 8 | String Processing | 4-6 days |
| 9 | Geometry & Union-Find | 3-4 days |
| 10 | Advanced Topics & Utilities | 4-6 days |
| **Total** | | **35-55 days** |

*Note: Times are estimates for focused work. Actual time may vary based on experience level and available time.*

---

## Session Management

### Starting a New Phase

1. Read the relevant section of this plan
2. Check out a new branch: `git checkout -b phase-N-description`
3. Review Java source files for the phase
4. Create module structure
5. Implement, test, document
6. Commit regularly with clear messages
7. Push and create PR when phase is complete

### Continuing Work

1. Review previous session's progress
2. Check TODO comments in code
3. Run tests to ensure nothing broke
4. Continue with next component
5. Update this plan if scope changes

### Phase Completion Checklist

- [ ] All Java files converted
- [ ] Unit tests written and passing
- [ ] Integration tests written and passing
- [ ] Documentation complete
- [ ] Examples/demos created
- [ ] Benchmarks implemented (where appropriate)
- [ ] Code formatted and linted
- [ ] Phase branch merged to main

---

## Open Questions / Decisions Needed

1. **Multimedia Support:** Should we include Picture, StdDraw, StdAudio?
   - **Option A:** Include with feature flags (optional dependencies)
   - **Option B:** Skip multimedia, focus on algorithms
   - **Recommendation:** Feature flags for optional multimedia

2. **Standard Library Usage:**
   - **Option A:** Re-implement everything for educational value
   - **Option B:** Use std library but provide custom implementations as examples
   - **Recommendation:** Both - use std lib in real code, provide educational implementations in examples

3. **API Design:**
   - **Option A:** Mirror Java API closely for familiarity
   - **Option B:** Idiomatic Rust API that differs from Java
   - **Recommendation:** Idiomatic Rust, but document Java equivalents

4. **Performance vs. Clarity:**
   - Priority: Clarity first, optimize later
   - Document where Rust version differs from Java for performance

5. **Error Handling Philosophy:**
   - Use `Result` for I/O operations
   - Use `Option` for search operations
   - Use `panic!` only for contract violations

---

## Success Criteria

The project will be considered successful when:

1. ✅ All core algorithms (phases 1-8) are converted
2. ✅ Comprehensive test suite with >80% coverage
3. ✅ Documentation for all public APIs
4. ✅ Performance benchmarks showing competitive performance
5. ✅ Example programs demonstrating usage
6. ✅ CI/CD pipeline passing
7. ✅ Published to crates.io (optional)

---

## Resources

- **Original Repository:** https://github.com/kevin-wayne/algs4
- **Textbook:** "Algorithms, 4th Edition" by Sedgewick & Wayne
- **Booksite:** https://algs4.cs.princeton.edu/
- **Rust Book:** https://doc.rust-lang.org/book/
- **Rust API Guidelines:** https://rust-lang.github.io/api-guidelines/

---

## Contributing

(To be defined based on project goals)

---

## License

GPLv3 - Inherited from the original algs4 Java implementation.

---

**Last Updated:** 2025-11-13
**Version:** 1.0
**Status:** Phase 0 - Project Foundation

---

## Notes for Sub-Agents / Future Sessions

- This plan is a living document - update as needed
- Each phase is designed to fit in typical context window
- Phases can be parallelized if multiple developers/sessions available
- Focus on one phase at a time for single developer
- Keep this document updated with progress
- Add notes about challenges encountered for future reference
