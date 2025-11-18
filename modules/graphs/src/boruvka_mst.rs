//! Boruvka's algorithm for computing a minimum spanning tree.

use crate::{Edge, EdgeWeightedGraph};
use algs4_fundamentals::union_find::WeightedQuickUnionUF;

/// Computes a minimum spanning tree (MST) of an edge-weighted graph using
/// Boruvka's algorithm.
///
/// Boruvka's algorithm works by repeatedly finding the minimum-weight edge
/// connecting each tree in the forest to another tree, and adding all such
/// edges to the MST.
///
/// The constructor takes O(E log V) time and O(V) space, where E is the number
/// of edges and V is the number of vertices.
///
/// # Examples
///
/// ```
/// use algs4_graphs::{EdgeWeightedGraph, Edge, BoruvkaMST};
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
/// let mst = BoruvkaMST::new(&graph);
///
/// assert!((mst.weight() - 1.81).abs() < 0.01);
/// assert_eq!(mst.edges().len(), 7);
/// ```
#[derive(Debug, Clone)]
pub struct BoruvkaMST {
    weight: f64,    // total weight of MST
    mst: Vec<Edge>, // edges in the MST
}

impl BoruvkaMST {
    /// Computes a minimum spanning tree (or forest) of an edge-weighted graph.
    ///
    /// # Arguments
    ///
    /// * `graph` - The edge-weighted graph
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedGraph, Edge, BoruvkaMST};
    ///
    /// let mut graph = EdgeWeightedGraph::new(5);
    /// graph.add_edge(Edge::new(0, 1, 1.0));
    /// graph.add_edge(Edge::new(1, 2, 2.0));
    /// graph.add_edge(Edge::new(2, 3, 3.0));
    /// graph.add_edge(Edge::new(3, 4, 4.0));
    /// graph.add_edge(Edge::new(0, 4, 5.0));
    ///
    /// let mst = BoruvkaMST::new(&graph);
    /// assert_eq!(mst.weight(), 10.0);
    /// ```
    pub fn new(graph: &EdgeWeightedGraph) -> Self {
        let v = graph.v();
        let mut mst = Vec::new();
        let mut weight = 0.0;
        let mut uf = WeightedQuickUnionUF::new(v);

        // Repeat until we have V-1 edges or cannot find more edges
        let mut num_trees = v;
        while num_trees > 1 && !mst.is_empty() || num_trees == v {
            // Find the closest edge for each tree
            let mut closest: Vec<Option<Edge>> = vec![None; v];

            for e in graph.edges() {
                let v = e.either();
                let w = e.other(v);

                let comp_v = uf.find(v);
                let comp_w = uf.find(w);

                // Skip if both endpoints are in the same tree
                if comp_v == comp_w {
                    continue;
                }

                // Update closest edge for component v
                if closest[comp_v].is_none() || e.weight() < closest[comp_v].unwrap().weight() {
                    closest[comp_v] = Some(e);
                }

                // Update closest edge for component w
                if closest[comp_w].is_none() || e.weight() < closest[comp_w].unwrap().weight() {
                    closest[comp_w] = Some(e);
                }
            }

            // Add all closest edges to MST
            let mut added = false;
            for e in closest.iter().flatten() {
                let v = e.either();
                let w = e.other(v);

                // Skip if both endpoints are now in the same tree
                if uf.connected(v, w) {
                    continue;
                }

                // Add edge to MST
                mst.push(*e);
                weight += e.weight();
                uf.union(v, w);
                num_trees -= 1;
                added = true;
            }

            // If no edges were added, we're done
            if !added {
                break;
            }
        }

        BoruvkaMST { weight, mst }
    }

    /// Returns the edges in a minimum spanning tree (or forest).
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedGraph, Edge, BoruvkaMST};
    ///
    /// let mut graph = EdgeWeightedGraph::new(5);
    /// graph.add_edge(Edge::new(0, 1, 1.0));
    /// graph.add_edge(Edge::new(1, 2, 2.0));
    /// graph.add_edge(Edge::new(2, 3, 3.0));
    /// graph.add_edge(Edge::new(3, 4, 4.0));
    ///
    /// let mst = BoruvkaMST::new(&graph);
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
    /// use algs4_graphs::{EdgeWeightedGraph, Edge, BoruvkaMST};
    ///
    /// let mut graph = EdgeWeightedGraph::new(5);
    /// graph.add_edge(Edge::new(0, 1, 1.0));
    /// graph.add_edge(Edge::new(1, 2, 2.0));
    /// graph.add_edge(Edge::new(2, 3, 3.0));
    /// graph.add_edge(Edge::new(3, 4, 4.0));
    ///
    /// let mst = BoruvkaMST::new(&graph);
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
    fn test_boruvka_basic() {
        let graph = create_test_graph();
        let mst = BoruvkaMST::new(&graph);

        assert!((mst.weight() - 1.81).abs() < 1e-10);
        assert_eq!(mst.edges().len(), 7);
    }

    #[test]
    fn test_mst_edges() {
        let graph = create_test_graph();
        let mst = BoruvkaMST::new(&graph);

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

        let mst = BoruvkaMST::new(&graph);
        assert_eq!(mst.weight(), 3.0);
        assert_eq!(mst.edges().len(), 2);
    }

    #[test]
    fn test_disconnected_graph() {
        let mut graph = EdgeWeightedGraph::new(5);
        graph.add_edge(Edge::new(0, 1, 1.0));
        graph.add_edge(Edge::new(3, 4, 2.0));

        let mst = BoruvkaMST::new(&graph);
        assert_eq!(mst.weight(), 3.0);
        assert_eq!(mst.edges().len(), 2);
    }

    #[test]
    fn test_single_vertex() {
        let graph = EdgeWeightedGraph::new(1);
        let mst = BoruvkaMST::new(&graph);

        assert_eq!(mst.weight(), 0.0);
        assert_eq!(mst.edges().len(), 0);
    }

    #[test]
    fn test_linear_graph() {
        let mut graph = EdgeWeightedGraph::new(4);
        graph.add_edge(Edge::new(0, 1, 1.0));
        graph.add_edge(Edge::new(1, 2, 2.0));
        graph.add_edge(Edge::new(2, 3, 3.0));

        let mst = BoruvkaMST::new(&graph);
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

        let mst = BoruvkaMST::new(&graph);
        assert_eq!(mst.weight(), 6.0);
        assert_eq!(mst.edges().len(), 3);
    }

    #[test]
    fn test_empty_graph() {
        let graph = EdgeWeightedGraph::new(5);
        let mst = BoruvkaMST::new(&graph);

        assert_eq!(mst.weight(), 0.0);
        assert_eq!(mst.edges().len(), 0);
    }
}
