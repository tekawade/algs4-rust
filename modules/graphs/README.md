# algs4-graphs

Graph algorithms and data structures from *Algorithms, 4th Edition* by Robert Sedgewick and Kevin Wayne.

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

## Features

This crate provides comprehensive graph algorithms and data structures:

### Graph Representations
- **Graph** - Undirected graph with adjacency lists
- **Digraph** - Directed graph with adjacency lists
- **EdgeWeightedGraph** - Edge-weighted undirected graph
- **EdgeWeightedDigraph** - Edge-weighted directed graph
- **SymbolGraph** - Graph with string vertex names
- **SymbolDigraph** - Directed graph with string vertex names

### Graph Generators
- **GraphGenerator** - Random graph generation (simple, complete, bipartite, trees, regular graphs, etc.)
- **DigraphGenerator** - Random digraph generation (DAGs, tournaments, strong digraphs, etc.)

### Graph Traversal
- **DepthFirstPaths** - Find paths using depth-first search
- **BreadthFirstPaths** - Find shortest paths using breadth-first search
- **DepthFirstDirectedPaths** - DFS for directed graphs
- **BreadthFirstDirectedPaths** - BFS for directed graphs

### Connectivity
- **CC** - Connected components in undirected graphs
- **KosarajuSharirSCC** - Strongly connected components (Kosaraju-Sharir algorithm)
- **TarjanSCC** - Strongly connected components (Tarjan algorithm)
- **GabowSCC** - Strongly connected components (Gabow algorithm)

### Shortest Paths
- **DijkstraSP** - Dijkstra's algorithm for shortest paths
- **DijkstraUndirectedSP** - Dijkstra for undirected graphs
- **DijkstraAllPairsSP** - All-pairs shortest paths
- **BellmanFordSP** - Bellman-Ford algorithm (handles negative weights)
- **AcyclicSP** - Shortest paths in DAGs
- **AcyclicLP** - Longest paths in DAGs
- **FloydWarshall** - All-pairs shortest paths with negative weights

### Minimum Spanning Trees
- **PrimMST** - Prim's algorithm (eager version)
- **LazyPrimMST** - Prim's algorithm (lazy version)
- **KruskalMST** - Kruskal's algorithm
- **BoruvkaMST** - Borůvka's algorithm

### Topological Sort & DAGs
- **Topological** - Topological sort for DAGs
- **DepthFirstOrder** - DFS-based vertex orderings

### Cycle Detection
- **Cycle** - Find cycles in undirected graphs
- **DirectedCycle** - Find cycles in directed graphs
- **DirectedCycleX** - Cycle detection with edge tracking
- **EdgeWeightedDirectedCycle** - Cycle detection in edge-weighted digraphs

### Special Graph Problems
- **Bipartite** - Bipartite graph detection
- **BipartiteX** - Enhanced bipartite detection
- **EulerianCycle** - Find Eulerian cycles
- **EulerianPath** - Find Eulerian paths
- **DirectedEulerianCycle** - Eulerian cycles in digraphs
- **DirectedEulerianPath** - Eulerian paths in digraphs

### Maximum Flow
- **FordFulkerson** - Ford-Fulkerson algorithm for maximum flow
- **FlowNetwork** - Flow network representation
- **FlowEdge** - Edge with flow and capacity

### Other
- **TransitiveClosure** - Compute transitive closure

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
algs4-graphs = "0.1"
```

## Usage

### Basic Graph Operations

```rust
use algs4_graphs::Graph;

// Create an undirected graph with 6 vertices
let mut graph = Graph::new(6);
graph.add_edge(0, 1);
graph.add_edge(0, 2);
graph.add_edge(1, 3);
graph.add_edge(2, 3);
graph.add_edge(4, 5);

println!("Vertices: {}", graph.v());
println!("Edges: {}", graph.e());
```

### Finding Paths

```rust
use algs4_graphs::{Graph, DepthFirstPaths, BreadthFirstPaths};

let mut graph = Graph::new(6);
graph.add_edge(0, 1);
graph.add_edge(0, 2);
graph.add_edge(1, 3);
graph.add_edge(2, 3);

// DFS paths from vertex 0
let dfs = DepthFirstPaths::new(&graph, 0);
if let Some(path) = dfs.path_to(3) {
    println!("Path: {:?}", path);
}

// BFS shortest paths from vertex 0
let bfs = BreadthFirstPaths::new(&graph, 0);
if let Some(dist) = bfs.dist_to(3) {
    println!("Distance: {}", dist);
}
```

### Connected Components

```rust
use algs4_graphs::{Graph, CC};

let mut graph = Graph::new(6);
graph.add_edge(0, 1);
graph.add_edge(0, 2);
graph.add_edge(4, 5);

let cc = CC::new(&graph);
println!("Components: {}", cc.count());
println!("0 and 3 connected? {}", cc.connected(0, 3));
```

### Shortest Paths with Dijkstra

```rust
use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, DijkstraSP};

let mut graph = EdgeWeightedDigraph::new(5);
graph.add_edge(DirectedEdge::new(0, 1, 5.0));
graph.add_edge(DirectedEdge::new(0, 2, 3.0));
graph.add_edge(DirectedEdge::new(1, 3, 2.0));
graph.add_edge(DirectedEdge::new(2, 3, 7.0));

let sp = DijkstraSP::new(&graph, 0);
if let Some(dist) = sp.dist_to(3) {
    println!("Shortest distance: {}", dist);
}
```

### Minimum Spanning Tree

```rust
use algs4_graphs::{EdgeWeightedGraph, Edge, PrimMST};

let mut graph = EdgeWeightedGraph::new(5);
graph.add_edge(Edge::new(0, 1, 1.0));
graph.add_edge(Edge::new(0, 2, 2.0));
graph.add_edge(Edge::new(1, 2, 3.0));
graph.add_edge(Edge::new(1, 3, 4.0));
graph.add_edge(Edge::new(2, 3, 5.0));

let mst = PrimMST::new(&graph);
println!("MST weight: {}", mst.weight());
```

### Generating Random Graphs

```rust
use algs4_graphs::{GraphGenerator, DigraphGenerator};

// Generate a random simple graph with 10 vertices and 15 edges
let graph = GraphGenerator::simple(10, 15);

// Generate a random tree with 10 vertices
let tree = GraphGenerator::tree(10);

// Generate a random DAG with 10 vertices and 20 edges
let dag = DigraphGenerator::dag(10, 20);

// Generate a complete bipartite graph
let bipartite = GraphGenerator::complete_bipartite(3, 4);
```

### Symbol Graphs

```rust
use algs4_graphs::SymbolGraph;

let mut sg = SymbolGraph::new();
sg.add_vertex("Alice".to_string());
sg.add_vertex("Bob".to_string());
sg.add_vertex("Charlie".to_string());
sg.add_edge("Alice", "Bob");
sg.add_edge("Bob", "Charlie");

assert_eq!(sg.degree("Bob"), 2);
```

## Performance

All algorithms are implemented with performance in mind, using efficient data structures and following the textbook's proven approaches. Benchmarks are included in the `benches/` directory.

## Documentation

For detailed API documentation, run:

```bash
cargo doc --open
```

## Testing

Run tests with:

```bash
cargo test
```

Run benchmarks with:

```bash
cargo bench
```

## License

This project is licensed under the GNU General Public License v3.0 - see the LICENSE file for details.

This license is inherited from the original [kevin-wayne/algs4](https://github.com/kevin-wayne/algs4) repository.

## Credits

- **Original Authors:** Robert Sedgewick and Kevin Wayne
- **Textbook:** *Algorithms, 4th Edition* by Sedgewick & Wayne
- **Original Java Implementation:** [kevin-wayne/algs4](https://github.com/kevin-wayne/algs4)

## Contributing

Contributions are welcome! Please ensure all tests pass and documentation is updated.
