//! Eager Prim's algorithm for computing a minimum spanning tree.

use crate::{Edge, EdgeWeightedGraph};
use algs4_fundamentals::priority_queue::IndexMinPQ;
use std::cmp::Ordering;

/// Wrapper for f64 that implements Ord by using partial_cmp
#[derive(Debug, Clone, Copy, PartialEq)]
struct OrdF64(f64);

impl Eq for OrdF64 {}

impl PartialOrd for OrdF64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrdF64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(Ordering::Equal)
    }
}

impl From<f64> for OrdF64 {
    fn from(f: f64) -> Self {
        OrdF64(f)
    }
}

/// Computes a minimum spanning tree (MST) of an edge-weighted graph using
/// the eager version of Prim's algorithm.
///
/// This implementation uses an indexed priority queue to efficiently track
/// the minimum-weight edge connecting each vertex to the tree.
///
/// The constructor takes O(E log V) time and O(V) space, where E is the number
/// of edges and V is the number of vertices.
///
/// # Examples
///
/// ```
/// use algs4_graphs::{EdgeWeightedGraph, Edge, PrimMST};
///
/// let mut graph = EdgeWeightedGraph::new(8);
/// graph.add_edge(Edge::new(0, 7, 0.16));
/// graph.add_edge(Edge::new(2, 3, 0.17));
/// graph.add_edge(Edge::new(1, 7, 0.19));
/// graph.add_edge(Edge::new(0, 2, 0.26));
/// graph.add_edge(Edge::new(5, 7, 0.28));
/// graph.add_edge(Edge::new(1, 3, 0.29));
/// graph.add_edge(Edge::new(1, 5, 0.32));
/// graph.add_edge(Edge::new(2, 7, 0.34));
/// graph.add_edge(Edge::new(4, 5, 0.35));
/// graph.add_edge(Edge::new(1, 2, 0.36));
/// graph.add_edge(Edge::new(4, 7, 0.37));
/// graph.add_edge(Edge::new(0, 4, 0.38));
/// graph.add_edge(Edge::new(6, 2, 0.40));
/// graph.add_edge(Edge::new(3, 6, 0.52));
/// graph.add_edge(Edge::new(6, 0, 0.58));
/// graph.add_edge(Edge::new(6, 4, 0.93));
///
/// let mst = PrimMST::new(&graph);
///
/// assert_eq!(mst.weight(), 1.81);
/// assert_eq!(mst.edges().len(), 7);
/// ```
#[derive(Debug, Clone)]
pub struct PrimMST {
    edge_to: Vec<Option<Edge>>, // edge_to[v] = shortest edge from tree vertex to non-tree vertex
    #[allow(dead_code)]
    dist_to: Vec<f64>, // dist_to[v] = weight of shortest such edge
    #[allow(dead_code)]
    marked: Vec<bool>, // marked[v] = true if v on tree
    weight: f64,                // total weight of MST
}

impl PrimMST {
    /// Computes a minimum spanning tree (or forest) of an edge-weighted graph.
    ///
    /// # Arguments
    ///
    /// * `graph` - The edge-weighted graph
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedGraph, Edge, PrimMST};
    ///
    /// let mut graph = EdgeWeightedGraph::new(5);
    /// graph.add_edge(Edge::new(0, 1, 1.0));
    /// graph.add_edge(Edge::new(1, 2, 2.0));
    /// graph.add_edge(Edge::new(2, 3, 3.0));
    /// graph.add_edge(Edge::new(3, 4, 4.0));
    /// graph.add_edge(Edge::new(0, 4, 5.0));
    ///
    /// let mst = PrimMST::new(&graph);
    /// assert_eq!(mst.weight(), 10.0);
    /// ```
    pub fn new(graph: &EdgeWeightedGraph) -> Self {
        let v = graph.v();
        let mut edge_to = vec![None; v];
        let mut dist_to = vec![f64::INFINITY; v];
        let mut marked = vec![false; v];
        let mut weight = 0.0;

        // Run Prim from all vertices to get minimum spanning forest
        for s in 0..v {
            if !marked[s] {
                Self::prim(
                    graph,
                    s,
                    &mut edge_to,
                    &mut dist_to,
                    &mut marked,
                    &mut weight,
                );
            }
        }

        PrimMST {
            edge_to,
            dist_to,
            marked,
            weight,
        }
    }

    // Run Prim's algorithm from vertex s
    fn prim(
        graph: &EdgeWeightedGraph,
        s: usize,
        edge_to: &mut [Option<Edge>],
        dist_to: &mut [f64],
        marked: &mut [bool],
        weight: &mut f64,
    ) {
        let v = graph.v();
        let mut pq = IndexMinPQ::new(v);

        dist_to[s] = 0.0;
        pq.insert(s, OrdF64(0.0));

        while !pq.is_empty() {
            let v = pq.del_min().unwrap();
            *weight += dist_to[v];
            Self::visit(graph, v, edge_to, dist_to, marked, &mut pq);
        }
    }

    // Add vertex v to tree; update data structures
    fn visit(
        graph: &EdgeWeightedGraph,
        v: usize,
        edge_to: &mut [Option<Edge>],
        dist_to: &mut [f64],
        marked: &mut [bool],
        pq: &mut IndexMinPQ<OrdF64>,
    ) {
        marked[v] = true;
        for &e in graph.adj(v) {
            let w = e.other(v);

            // Skip if w is already in tree
            if marked[w] {
                continue;
            }

            // Update edge and distance if this is a better connection
            if e.weight() < dist_to[w] {
                edge_to[w] = Some(e);
                dist_to[w] = e.weight();

                if pq.contains(w) {
                    pq.decrease_key(w, OrdF64(dist_to[w]));
                } else {
                    pq.insert(w, OrdF64(dist_to[w]));
                }
            }
        }
    }

    /// Returns the edges in a minimum spanning tree (or forest).
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedGraph, Edge, PrimMST};
    ///
    /// let mut graph = EdgeWeightedGraph::new(5);
    /// graph.add_edge(Edge::new(0, 1, 1.0));
    /// graph.add_edge(Edge::new(1, 2, 2.0));
    /// graph.add_edge(Edge::new(2, 3, 3.0));
    /// graph.add_edge(Edge::new(3, 4, 4.0));
    ///
    /// let mst = PrimMST::new(&graph);
    /// assert_eq!(mst.edges().len(), 4);
    /// ```
    pub fn edges(&self) -> Vec<Edge> {
        self.edge_to.iter().filter_map(|&e| e).collect()
    }

    /// Returns the sum of the edge weights in a minimum spanning tree (or forest).
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedGraph, Edge, PrimMST};
    ///
    /// let mut graph = EdgeWeightedGraph::new(5);
    /// graph.add_edge(Edge::new(0, 1, 1.0));
    /// graph.add_edge(Edge::new(1, 2, 2.0));
    /// graph.add_edge(Edge::new(2, 3, 3.0));
    /// graph.add_edge(Edge::new(3, 4, 4.0));
    ///
    /// let mst = PrimMST::new(&graph);
    /// assert_eq!(mst.weight(), 10.0);
    /// ```
    pub fn weight(&self) -> f64 {
        self.weight
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_graph() -> EdgeWeightedGraph {
        let mut graph = EdgeWeightedGraph::new(8);
        graph.add_edge(Edge::new(0, 7, 0.16));
        graph.add_edge(Edge::new(2, 3, 0.17));
        graph.add_edge(Edge::new(1, 7, 0.19));
        graph.add_edge(Edge::new(0, 2, 0.26));
        graph.add_edge(Edge::new(5, 7, 0.28));
        graph.add_edge(Edge::new(1, 3, 0.29));
        graph.add_edge(Edge::new(1, 5, 0.32));
        graph.add_edge(Edge::new(2, 7, 0.34));
        graph.add_edge(Edge::new(4, 5, 0.35));
        graph.add_edge(Edge::new(1, 2, 0.36));
        graph.add_edge(Edge::new(4, 7, 0.37));
        graph.add_edge(Edge::new(0, 4, 0.38));
        graph.add_edge(Edge::new(6, 2, 0.40));
        graph.add_edge(Edge::new(3, 6, 0.52));
        graph.add_edge(Edge::new(6, 0, 0.58));
        graph.add_edge(Edge::new(6, 4, 0.93));
        graph
    }

    #[test]
    fn test_prim_basic() {
        let graph = create_test_graph();
        let mst = PrimMST::new(&graph);

        assert!((mst.weight() - 1.81).abs() < 1e-10);
        assert_eq!(mst.edges().len(), 7);
    }

    #[test]
    fn test_mst_edges() {
        let graph = create_test_graph();
        let mst = PrimMST::new(&graph);

        // Verify the total weight
        let total_weight: f64 = mst.edges().iter().map(|e| e.weight()).sum();
        assert!((total_weight - 1.81).abs() < 1e-10);
    }

    #[test]
    fn test_simple_graph() {
        let mut graph = EdgeWeightedGraph::new(3);
        graph.add_edge(Edge::new(0, 1, 1.0));
        graph.add_edge(Edge::new(1, 2, 2.0));
        graph.add_edge(Edge::new(0, 2, 5.0));

        let mst = PrimMST::new(&graph);
        assert_eq!(mst.weight(), 3.0);
        assert_eq!(mst.edges().len(), 2);
    }

    #[test]
    fn test_disconnected_graph() {
        let mut graph = EdgeWeightedGraph::new(5);
        graph.add_edge(Edge::new(0, 1, 1.0));
        graph.add_edge(Edge::new(3, 4, 2.0));

        let mst = PrimMST::new(&graph);
        assert_eq!(mst.weight(), 3.0);
        assert_eq!(mst.edges().len(), 2);
    }

    #[test]
    fn test_single_vertex() {
        let graph = EdgeWeightedGraph::new(1);
        let mst = PrimMST::new(&graph);

        assert_eq!(mst.weight(), 0.0);
        assert_eq!(mst.edges().len(), 0);
    }

    #[test]
    fn test_linear_graph() {
        let mut graph = EdgeWeightedGraph::new(4);
        graph.add_edge(Edge::new(0, 1, 1.0));
        graph.add_edge(Edge::new(1, 2, 2.0));
        graph.add_edge(Edge::new(2, 3, 3.0));

        let mst = PrimMST::new(&graph);
        assert_eq!(mst.weight(), 6.0);
        assert_eq!(mst.edges().len(), 3);
    }

    #[test]
    fn test_complete_graph() {
        let mut graph = EdgeWeightedGraph::new(4);
        graph.add_edge(Edge::new(0, 1, 1.0));
        graph.add_edge(Edge::new(0, 2, 2.0));
        graph.add_edge(Edge::new(0, 3, 3.0));
        graph.add_edge(Edge::new(1, 2, 4.0));
        graph.add_edge(Edge::new(1, 3, 5.0));
        graph.add_edge(Edge::new(2, 3, 6.0));

        let mst = PrimMST::new(&graph);
        assert_eq!(mst.weight(), 6.0);
        assert_eq!(mst.edges().len(), 3);
    }

    #[test]
    fn test_empty_graph() {
        let graph = EdgeWeightedGraph::new(5);
        let mst = PrimMST::new(&graph);

        assert_eq!(mst.weight(), 0.0);
        assert_eq!(mst.edges().len(), 0);
    }
}
