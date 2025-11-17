//! # algs4-graphs
//!
//! Graph algorithms from *Algorithms, 4th Edition*
//! by Robert Sedgewick and Kevin Wayne.
//!
//! This module contains:
//! - Graph representations (undirected, directed, weighted)
//! - Graph traversal (DFS, BFS)
//! - Connected components
//! - Shortest paths (Dijkstra, Bellman-Ford, Floyd-Warshall) - coming in Phase 7A
//! - Minimum spanning trees (Prim, Kruskal, Boruvka) - coming in Phase 7A
//! - Maximum flow (Ford-Fulkerson) - coming in Phase 7B
//! - Topological sort, strongly connected components - coming in Phase 7B
//!
//! ## Example
//!
//! ```
//! use algs4_graphs::{Graph, DepthFirstPaths, BreadthFirstPaths, CC};
//!
//! // Create an undirected graph
//! let mut graph = Graph::new(6);
//! graph.add_edge(0, 1);
//! graph.add_edge(0, 2);
//! graph.add_edge(1, 3);
//! graph.add_edge(2, 3);
//! graph.add_edge(4, 5);
//!
//! // Find paths using DFS
//! let dfs = DepthFirstPaths::new(&graph, 0);
//! if let Some(path) = dfs.path_to(3) {
//!     println!("Path from 0 to 3: {:?}", path);
//! }
//!
//! // Find shortest paths using BFS
//! let bfs = BreadthFirstPaths::new(&graph, 0);
//! if let Some(dist) = bfs.dist_to(3) {
//!     println!("Distance from 0 to 3: {}", dist);
//! }
//!
//! // Find connected components
//! let cc = CC::new(&graph);
//! println!("Number of connected components: {}", cc.count());
//! println!("0 and 3 connected? {}", cc.connected(0, 3));
//! println!("0 and 4 connected? {}", cc.connected(0, 4));
//! ```

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

// Graph data structures
mod digraph;
mod directed_edge;
mod edge;
mod edge_weighted_digraph;
mod edge_weighted_graph;
mod graph;

// Undirected graph traversal
mod breadth_first_paths;
mod cc;
mod depth_first_paths;

// Directed graph traversal
mod breadth_first_directed_paths;
mod depth_first_directed_paths;

// Public exports
pub use digraph::Digraph;
pub use directed_edge::DirectedEdge;
pub use edge::Edge;
pub use edge_weighted_digraph::EdgeWeightedDigraph;
pub use edge_weighted_graph::EdgeWeightedGraph;
pub use graph::Graph;

pub use breadth_first_paths::BreadthFirstPaths;
pub use cc::CC;
pub use depth_first_paths::DepthFirstPaths;

pub use breadth_first_directed_paths::BreadthFirstDirectedPaths;
pub use depth_first_directed_paths::DepthFirstDirectedPaths;
