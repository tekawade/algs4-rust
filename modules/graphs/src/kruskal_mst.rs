//! Kruskal's algorithm for computing a minimum spanning tree.

use crate::{Edge, EdgeWeightedGraph};
use algs4_fundamentals::union_find::WeightedQuickUnionUF;

/// Computes a minimum spanning tree (MST) of an edge-weighted graph using
/// Kruskal's algorithm.
///
/// This implementation uses a union-find data structure to detect cycles.
/// It processes edges in order of their weight (from smallest to largest)
/// and adds an edge to the MST if it doesn't create a cycle.
///
/// The constructor takes O(E log E) time and O(E) space, where E is the number
/// of edges and V is the number of vertices.
///
/// # Examples
///
/// ```
/// use algs4_graphs::{EdgeWeightedGraph, Edge, KruskalMST};
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
/// let mst = KruskalMST::new(&graph);
///
/// assert_eq!(mst.weight(), 1.81);
/// assert_eq!(mst.edges().len(), 7);
/// ```
#[derive(Debug, Clone)]
pub struct KruskalMST {
    weight: f64,    // total weight of MST
    mst: Vec<Edge>, // edges in the MST
}

impl KruskalMST {
    /// Computes a minimum spanning tree (or forest) of an edge-weighted graph.
    ///
    /// # Arguments
    ///
    /// * `graph` - The edge-weighted graph
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedGraph, Edge, KruskalMST};
    ///
    /// let mut graph = EdgeWeightedGraph::new(5);
    /// graph.add_edge(Edge::new(0, 1, 1.0));
    /// graph.add_edge(Edge::new(1, 2, 2.0));
    /// graph.add_edge(Edge::new(2, 3, 3.0));
    /// graph.add_edge(Edge::new(3, 4, 4.0));
    /// graph.add_edge(Edge::new(0, 4, 5.0));
    ///
    /// let mst = KruskalMST::new(&graph);
    /// assert_eq!(mst.weight(), 10.0);
    /// ```
    pub fn new(graph: &EdgeWeightedGraph) -> Self {
        let v = graph.v();
        let mut mst = Vec::new();
        let mut weight = 0.0;

        // Sort edges by weight
        let mut edges = graph.edges();
        edges.sort();

        // Use union-find to detect cycles
        let mut uf = WeightedQuickUnionUF::new(v);

        // Greedily add edges to MST
        for e in edges {
            let v = e.either();
            let w = e.other(v);

            // Skip edge if it would create a cycle
            if uf.connected(v, w) {
                continue;
            }

            // Add edge to MST
            uf.union(v, w);
            mst.push(e);
            weight += e.weight();

            // Stop if we have V-1 edges (MST is complete for connected component)
            if mst.len() == graph.v() - 1 {
                break;
            }
        }

        KruskalMST { weight, mst }
    }

    /// Returns the edges in a minimum spanning tree (or forest).
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedGraph, Edge, KruskalMST};
    ///
    /// let mut graph = EdgeWeightedGraph::new(5);
    /// graph.add_edge(Edge::new(0, 1, 1.0));
    /// graph.add_edge(Edge::new(1, 2, 2.0));
    /// graph.add_edge(Edge::new(2, 3, 3.0));
    /// graph.add_edge(Edge::new(3, 4, 4.0));
    ///
    /// let mst = KruskalMST::new(&graph);
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
    /// use algs4_graphs::{EdgeWeightedGraph, Edge, KruskalMST};
    ///
    /// let mut graph = EdgeWeightedGraph::new(5);
    /// graph.add_edge(Edge::new(0, 1, 1.0));
    /// graph.add_edge(Edge::new(1, 2, 2.0));
    /// graph.add_edge(Edge::new(2, 3, 3.0));
    /// graph.add_edge(Edge::new(3, 4, 4.0));
    ///
    /// let mst = KruskalMST::new(&graph);
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
    fn test_kruskal_basic() {
        let graph = create_test_graph();
        let mst = KruskalMST::new(&graph);

        assert!((mst.weight() - 1.81).abs() < 1e-10);
        assert_eq!(mst.edges().len(), 7);
    }

    #[test]
    fn test_mst_edges() {
        let graph = create_test_graph();
        let mst = KruskalMST::new(&graph);

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

        let mst = KruskalMST::new(&graph);
        assert_eq!(mst.weight(), 3.0);
        assert_eq!(mst.edges().len(), 2);
    }

    #[test]
    fn test_disconnected_graph() {
        let mut graph = EdgeWeightedGraph::new(5);
        graph.add_edge(Edge::new(0, 1, 1.0));
        graph.add_edge(Edge::new(3, 4, 2.0));

        let mst = KruskalMST::new(&graph);
        assert_eq!(mst.weight(), 3.0);
        assert_eq!(mst.edges().len(), 2);
    }

    #[test]
    fn test_single_vertex() {
        let graph = EdgeWeightedGraph::new(1);
        let mst = KruskalMST::new(&graph);

        assert_eq!(mst.weight(), 0.0);
        assert_eq!(mst.edges().len(), 0);
    }

    #[test]
    fn test_linear_graph() {
        let mut graph = EdgeWeightedGraph::new(4);
        graph.add_edge(Edge::new(0, 1, 1.0));
        graph.add_edge(Edge::new(1, 2, 2.0));
        graph.add_edge(Edge::new(2, 3, 3.0));

        let mst = KruskalMST::new(&graph);
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

        let mst = KruskalMST::new(&graph);
        assert_eq!(mst.weight(), 6.0);
        assert_eq!(mst.edges().len(), 3);
    }

    #[test]
    fn test_empty_graph() {
        let graph = EdgeWeightedGraph::new(5);
        let mst = KruskalMST::new(&graph);

        assert_eq!(mst.weight(), 0.0);
        assert_eq!(mst.edges().len(), 0);
    }

    #[test]
    fn test_edges_sorted() {
        let graph = create_test_graph();
        let mst = KruskalMST::new(&graph);

        // Verify edges are added in order of weight
        let edges = mst.edges();
        for i in 1..edges.len() {
            assert!(edges[i - 1].weight() <= edges[i].weight());
        }
    }
}
