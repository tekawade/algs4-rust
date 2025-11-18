//! Lazy Prim's algorithm for computing a minimum spanning tree.

use crate::{Edge, EdgeWeightedGraph};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Computes a minimum spanning tree (MST) of an edge-weighted graph using
/// the lazy version of Prim's algorithm.
///
/// This implementation uses a lazy approach where edges are added to a priority
/// queue and obsolete edges are not removed until they are dequeued.
///
/// The constructor takes O(E log E) time and O(E) space, where E is the number
/// of edges and V is the number of vertices.
///
/// # Examples
///
/// ```
/// use algs4_graphs::{EdgeWeightedGraph, Edge, LazyPrimMST};
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
/// let mst = LazyPrimMST::new(&graph);
///
/// assert_eq!(mst.weight(), 1.81);
/// assert_eq!(mst.edges().len(), 7);
/// ```
#[derive(Debug, Clone)]
pub struct LazyPrimMST {
    weight: f64,    // total weight of MST
    mst: Vec<Edge>, // edges in the MST
    #[allow(dead_code)]
    marked: Vec<bool>, // marked[v] = true if v on tree
}

impl LazyPrimMST {
    /// Computes a minimum spanning tree (or forest) of an edge-weighted graph.
    ///
    /// # Arguments
    ///
    /// * `graph` - The edge-weighted graph
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedGraph, Edge, LazyPrimMST};
    ///
    /// let mut graph = EdgeWeightedGraph::new(5);
    /// graph.add_edge(Edge::new(0, 1, 1.0));
    /// graph.add_edge(Edge::new(1, 2, 2.0));
    /// graph.add_edge(Edge::new(2, 3, 3.0));
    /// graph.add_edge(Edge::new(3, 4, 4.0));
    /// graph.add_edge(Edge::new(0, 4, 5.0));
    ///
    /// let mst = LazyPrimMST::new(&graph);
    /// assert_eq!(mst.weight(), 10.0);
    /// ```
    pub fn new(graph: &EdgeWeightedGraph) -> Self {
        let v = graph.v();
        let mut mst = Vec::new();
        let mut marked = vec![false; v];
        let mut pq = BinaryHeap::new();
        let mut weight = 0.0;

        // Run Prim from all vertices to get minimum spanning forest
        for s in 0..v {
            if !marked[s] {
                Self::prim(graph, s, &mut marked, &mut pq, &mut mst, &mut weight);
            }
        }

        LazyPrimMST {
            weight,
            mst,
            marked,
        }
    }

    // Run Prim's algorithm from vertex s
    fn prim(
        graph: &EdgeWeightedGraph,
        s: usize,
        marked: &mut [bool],
        pq: &mut BinaryHeap<Reverse<Edge>>,
        mst: &mut Vec<Edge>,
        weight: &mut f64,
    ) {
        Self::visit(graph, s, marked, pq);

        while let Some(Reverse(e)) = pq.pop() {
            let v = e.either();
            let w = e.other(v);

            // Skip if both endpoints are already in tree (obsolete edge)
            if marked[v] && marked[w] {
                continue;
            }

            // Add edge to MST
            mst.push(e);
            *weight += e.weight();

            // Add vertex to tree
            if !marked[v] {
                Self::visit(graph, v, marked, pq);
            }
            if !marked[w] {
                Self::visit(graph, w, marked, pq);
            }
        }
    }

    // Add vertex v to tree; add all edges incident to v to priority queue
    fn visit(
        graph: &EdgeWeightedGraph,
        v: usize,
        marked: &mut [bool],
        pq: &mut BinaryHeap<Reverse<Edge>>,
    ) {
        marked[v] = true;
        for &e in graph.adj(v) {
            let w = e.other(v);
            if !marked[w] {
                pq.push(Reverse(e));
            }
        }
    }

    /// Returns the edges in a minimum spanning tree (or forest).
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedGraph, Edge, LazyPrimMST};
    ///
    /// let mut graph = EdgeWeightedGraph::new(5);
    /// graph.add_edge(Edge::new(0, 1, 1.0));
    /// graph.add_edge(Edge::new(1, 2, 2.0));
    /// graph.add_edge(Edge::new(2, 3, 3.0));
    /// graph.add_edge(Edge::new(3, 4, 4.0));
    ///
    /// let mst = LazyPrimMST::new(&graph);
    /// assert_eq!(mst.edges().len(), 4);
    /// ```
    pub fn edges(&self) -> &[Edge] {
        &self.mst
    }

    /// Returns the sum of the edge weights in a minimum spanning tree (or forest).
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedGraph, Edge, LazyPrimMST};
    ///
    /// let mut graph = EdgeWeightedGraph::new(5);
    /// graph.add_edge(Edge::new(0, 1, 1.0));
    /// graph.add_edge(Edge::new(1, 2, 2.0));
    /// graph.add_edge(Edge::new(2, 3, 3.0));
    /// graph.add_edge(Edge::new(3, 4, 4.0));
    ///
    /// let mst = LazyPrimMST::new(&graph);
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
    fn test_lazy_prim_basic() {
        let graph = create_test_graph();
        let mst = LazyPrimMST::new(&graph);

        assert!((mst.weight() - 1.81).abs() < 1e-10);
        assert_eq!(mst.edges().len(), 7);
    }

    #[test]
    fn test_mst_edges() {
        let graph = create_test_graph();
        let mst = LazyPrimMST::new(&graph);

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

        let mst = LazyPrimMST::new(&graph);
        assert_eq!(mst.weight(), 3.0);
        assert_eq!(mst.edges().len(), 2);
    }

    #[test]
    fn test_disconnected_graph() {
        let mut graph = EdgeWeightedGraph::new(5);
        graph.add_edge(Edge::new(0, 1, 1.0));
        graph.add_edge(Edge::new(3, 4, 2.0));

        let mst = LazyPrimMST::new(&graph);
        assert_eq!(mst.weight(), 3.0);
        assert_eq!(mst.edges().len(), 2);
    }

    #[test]
    fn test_single_vertex() {
        let graph = EdgeWeightedGraph::new(1);
        let mst = LazyPrimMST::new(&graph);

        assert_eq!(mst.weight(), 0.0);
        assert_eq!(mst.edges().len(), 0);
    }

    #[test]
    fn test_linear_graph() {
        let mut graph = EdgeWeightedGraph::new(4);
        graph.add_edge(Edge::new(0, 1, 1.0));
        graph.add_edge(Edge::new(1, 2, 2.0));
        graph.add_edge(Edge::new(2, 3, 3.0));

        let mst = LazyPrimMST::new(&graph);
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

        let mst = LazyPrimMST::new(&graph);
        assert_eq!(mst.weight(), 6.0);
        assert_eq!(mst.edges().len(), 3);
    }

    #[test]
    fn test_empty_graph() {
        let graph = EdgeWeightedGraph::new(5);
        let mst = LazyPrimMST::new(&graph);

        assert_eq!(mst.weight(), 0.0);
        assert_eq!(mst.edges().len(), 0);
    }
}
