//! # algs4-graphs
//!
//! Graph algorithms from *Algorithms, 4th Edition*
//! by Robert Sedgewick and Kevin Wayne.
//!
//! This module contains:
//! - Graph representations (undirected, directed, weighted)
//! - Graph traversal (DFS, BFS)
//! - Connected components
//! - Shortest paths (Dijkstra, Bellman-Ford, Floyd-Warshall)
//! - Minimum spanning trees (Prim, Kruskal, Boruvka)
//! - Maximum flow (Ford-Fulkerson)
//! - Topological sort, strongly connected components
//! - Cycle detection (directed and undirected)
//! - Eulerian paths and cycles
//! - Bipartite detection
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

// Shortest paths
mod acyclic_lp;
mod acyclic_sp;
mod bellman_ford_sp;
mod dijkstra_all_pairs_sp;
mod dijkstra_sp;
mod dijkstra_undirected_sp;
mod floyd_warshall;
mod transitive_closure;

// Minimum spanning trees
mod boruvka_mst;
mod kruskal_mst;
mod lazy_prim_mst;
mod prim_mst;

// Cycle detection
mod cycle;
mod directed_cycle;
mod directed_cycle_x;
mod edge_weighted_directed_cycle;

// Topological sort and DFS orders
mod depth_first_order;
mod topological;

// Strongly connected components
mod gabow_scc;
mod kosaraju_sharir_scc;
mod tarjan_scc;

// Bipartite detection
mod bipartite;
mod bipartite_x;

// Eulerian paths and cycles
mod directed_eulerian_cycle;
mod directed_eulerian_path;
mod eulerian_cycle;
mod eulerian_path;

// Maximum flow
mod flow_edge;
mod flow_network;
mod ford_fulkerson;

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

pub use acyclic_lp::AcyclicLP;
pub use acyclic_sp::AcyclicSP;
pub use bellman_ford_sp::BellmanFordSP;
pub use dijkstra_all_pairs_sp::DijkstraAllPairsSP;
pub use dijkstra_sp::DijkstraSP;
pub use dijkstra_undirected_sp::DijkstraUndirectedSP;
pub use floyd_warshall::FloydWarshall;
pub use transitive_closure::TransitiveClosure;

pub use boruvka_mst::BoruvkaMST;
pub use kruskal_mst::KruskalMST;
pub use lazy_prim_mst::LazyPrimMST;
pub use prim_mst::PrimMST;

pub use cycle::Cycle;
pub use directed_cycle::DirectedCycle;
pub use directed_cycle_x::DirectedCycleX;
pub use edge_weighted_directed_cycle::EdgeWeightedDirectedCycle;

pub use depth_first_order::DepthFirstOrder;
pub use topological::Topological;

pub use gabow_scc::GabowSCC;
pub use kosaraju_sharir_scc::KosarajuSharirSCC;
pub use tarjan_scc::TarjanSCC;

pub use bipartite::Bipartite;
pub use bipartite_x::BipartiteX;

pub use directed_eulerian_cycle::DirectedEulerianCycle;
pub use directed_eulerian_path::DirectedEulerianPath;
pub use eulerian_cycle::EulerianCycle;
pub use eulerian_path::EulerianPath;

pub use flow_edge::FlowEdge;
pub use flow_network::FlowNetwork;
pub use ford_fulkerson::FordFulkerson;
