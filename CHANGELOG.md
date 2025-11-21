# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-11-19

### Added

#### Fundamentals Module (`algs4-fundamentals`)
- **Collections**: LinkedStack, ResizingArrayStack, LinkedQueue, ResizingArrayQueue, LinkedBag, ResizingArrayBag
- **Union-Find**: QuickFindUF, QuickUnionUF, WeightedQuickUnionUF
- **Priority Queues**: MaxPQ, MinPQ, IndexMaxPQ, IndexMinPQ
- **I/O Utilities**: StdIn, StdOut, StdRandom
- **Statistical Tools**: Stopwatch, Counter, Accumulator, Transaction
- **Tests**: 136 passing tests
- **Benchmarks**: Performance benchmarks for all major data structures

#### Sorting Module (`algs4-sorting`)
- **Elementary Sorts**: Selection, Insertion, Binary Insertion, Shell
- **Advanced Sorts**: Merge, MergeBottomUp, MergeX, Quick, Quick3Way, QuickX, QuickBentleyMcIlroy, Heap
- **Radix Sorts**: LSD, MSD, MSDInPlace, Quick3String
- **Utilities**: Inversions counter
- **Tests**: 185 passing tests
- **Benchmarks**: Comprehensive benchmarks for all sorting algorithms

#### Searching Module (`algs4-searching`)
- **Ordered Symbol Tables**: BinarySearchST, BST, RedBlackBST, AVLTree, BTree
- **Hash Tables**: SeparateChainingHashST, LinearProbingHashST
- **Tries**: TrieST, TrieSET, PatriciaST, PatriciaSET
- **Applications**: Binary search, sequential search, frequency counter, file index, CSV lookup, and more
- **Tests**: 77+ passing tests
- **Benchmarks**: BST, Red-Black BST, Hash Tables, and Tries

#### Graphs Module (`algs4-graphs`)
- **Graph Representations**: Graph, Digraph, EdgeWeightedGraph, EdgeWeightedDigraph, SymbolGraph, SymbolDigraph
- **Graph Generators**: GraphGenerator, DigraphGenerator with 20+ graph types
- **Traversal**: DepthFirstPaths, BreadthFirstPaths, DepthFirstDirectedPaths, BreadthFirstDirectedPaths
- **Connectivity**: CC, KosarajuSharirSCC, TarjanSCC, GabowSCC
- **Shortest Paths**: DijkstraSP, DijkstraUndirectedSP, DijkstraAllPairsSP, BellmanFordSP, AcyclicSP, AcyclicLP, FloydWarshall
- **Minimum Spanning Trees**: PrimMST, LazyPrimMST, KruskalMST, BoruvkaMST
- **Topological Sort**: Topological, DepthFirstOrder
- **Cycle Detection**: Cycle, DirectedCycle, DirectedCycleX, EdgeWeightedDirectedCycle
- **Special Problems**: Bipartite, BipartiteX, EulerianCycle, EulerianPath, DirectedEulerianCycle, DirectedEulerianPath
- **Maximum Flow**: FordFulkerson
- **Other**: TransitiveClosure
- **Tests**: 322 passing tests
- **Benchmarks**: Comprehensive Criterion suite for all major algorithms
- **Documentation**: Detailed module README

#### Strings Module (`algs4-strings`)
- **Pattern Matching**: KMP, Boyer-Moore, Rabin-Karp
- **Regular Expressions**: NFA (nondeterministic finite automaton)
- **Compression**: Huffman, LZW, Run-length encoding
- **Suffix Arrays**: SuffixArray, SuffixArrayX
- **Substring Problems**: Longest common substring, longest repeated substring
- **Utilities**: Alphabet support
- **Tests**: 227 passing tests
- **Benchmarks**: Pattern matching and trie benchmarks

#### Geometry Module (`algs4-geometry`)
- **Primitives**: Point2D, Interval1D, Interval2D, RectHV
- **Algorithms**: Graham Scan (convex hull), Closest Pair, Farthest Pair
- **Tests**: 329 passing tests
- **Benchmarks**: Convex hull benchmarks

#### Advanced Module (`algs4-advanced`)
- **Linear Algebra**: Complex, Vector, SparseVector, Polynomial
- **Transforms**: FFT (Fast Fourier Transform)
- **Matrix Operations**: Gaussian Elimination, Gauss-Jordan Elimination
- **Optimization**: Linear Programming (Simplex algorithm)
- **Advanced Data Structures**: Segment Tree, Fenwick Tree (Binary Indexed Tree)
- **Algorithms**: ThreeSum, ThreeSumFast
- **Applications**: AllowFilter, BlockFilter, TopM
- **Tests**: 101 passing tests
- **Benchmarks**: Fenwick Tree and other data structure benchmarks

#### Plotting Module (`algs4-plotting`)
- **2D Graphics**: StdDraw-like API for educational plotting
- **Drawing Functions**: Points, lines, circles, rectangles, polygons
- **Coordinate Systems**: Flexible scaling and transformations
- **Color Support**: Pen color and radius control

### Infrastructure
- **CI/CD**: GitHub Actions workflow with tests, formatting, clippy, and documentation checks
- **Code Quality**: All code passes `cargo clippy -- -D warnings`
- **Formatting**: All code formatted with `rustfmt`
- **Documentation**: Complete API documentation with examples and doctests
- **Total Tests**: 1,377 passing tests across all modules
- **License**: GPL-3.0 (inherited from original algs4 project)
- **Repository**: Structured as Cargo workspace with 8 modules

### Project Status
- 95% feature complete
- All major algorithms from *Algorithms, 4th Edition* implemented
- Production-ready code with comprehensive testing
- Ready for publication to crates.io

### References
- Original textbook: *Algorithms, 4th Edition* by Robert Sedgewick and Kevin Wayne
- Original Java implementation: [kevin-wayne/algs4](https://github.com/kevin-wayne/algs4)
- Textbook website: https://algs4.cs.princeton.edu/

[0.1.0]: https://github.com/tekawade/algs4-rust/releases/tag/v0.1.0
