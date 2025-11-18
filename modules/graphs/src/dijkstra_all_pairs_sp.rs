//! All-pairs shortest paths using Dijkstra's algorithm.

use crate::{DijkstraSP, DirectedEdge, EdgeWeightedDigraph};

/// Computes all-pairs shortest paths in an edge-weighted digraph with non-negative weights.
///
/// This implementation runs Dijkstra's algorithm from each vertex to compute
/// shortest paths to all other vertices. The constructor takes O(V * E log V) time
/// and O(V^2) space, where V is the number of vertices and E is the number of edges.
///
/// # Panics
///
/// Panics if the edge-weighted digraph has edges with negative weights.
///
/// # Examples
///
/// ```
/// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, DijkstraAllPairsSP};
///
/// let mut graph = EdgeWeightedDigraph::new(5);
/// graph.add_edge(DirectedEdge::new(0, 1, 1.0));
/// graph.add_edge(DirectedEdge::new(0, 2, 3.0));
/// graph.add_edge(DirectedEdge::new(1, 2, 1.0));
/// graph.add_edge(DirectedEdge::new(2, 3, 2.0));
/// graph.add_edge(DirectedEdge::new(3, 4, 1.0));
///
/// let all_sp = DijkstraAllPairsSP::new(&graph);
///
/// assert!(all_sp.has_path(0, 4));
/// assert_eq!(all_sp.dist(0, 4), Some(5.0));
/// ```
#[derive(Debug, Clone)]
pub struct DijkstraAllPairsSP {
    all: Vec<DijkstraSP>,
}

impl DijkstraAllPairsSP {
    /// Computes all-pairs shortest paths in the edge-weighted digraph `graph`.
    ///
    /// # Arguments
    ///
    /// * `graph` - The edge-weighted digraph
    ///
    /// # Panics
    ///
    /// Panics if an edge weight is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, DijkstraAllPairsSP};
    ///
    /// let mut graph = EdgeWeightedDigraph::new(5);
    /// graph.add_edge(DirectedEdge::new(0, 1, 1.0));
    /// graph.add_edge(DirectedEdge::new(1, 2, 2.0));
    ///
    /// let all_sp = DijkstraAllPairsSP::new(&graph);
    /// assert_eq!(all_sp.dist(0, 2), Some(3.0));
    /// ```
    pub fn new(graph: &EdgeWeightedDigraph) -> Self {
        let v = graph.v();
        let mut all = Vec::with_capacity(v);

        for s in 0..v {
            all.push(DijkstraSP::new(graph, s));
        }

        DijkstraAllPairsSP { all }
    }

    /// Returns a shortest path from vertex `s` to vertex `t`.
    ///
    /// # Arguments
    ///
    /// * `s` - The source vertex
    /// * `t` - The destination vertex
    ///
    /// # Returns
    ///
    /// * `Some(path)` - A vector of edges representing the shortest path
    /// * `None` - If there is no path
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, DijkstraAllPairsSP};
    ///
    /// let mut graph = EdgeWeightedDigraph::new(5);
    /// graph.add_edge(DirectedEdge::new(0, 1, 1.0));
    /// graph.add_edge(DirectedEdge::new(1, 2, 2.0));
    ///
    /// let all_sp = DijkstraAllPairsSP::new(&graph);
    /// let path = all_sp.path(0, 2).unwrap();
    /// assert_eq!(path.len(), 2);
    /// ```
    pub fn path(&self, s: usize, t: usize) -> Option<Vec<DirectedEdge>> {
        self.all[s].path_to(t)
    }

    /// Returns `true` if there is a path from vertex `s` to vertex `t`.
    ///
    /// # Arguments
    ///
    /// * `s` - The source vertex
    /// * `t` - The destination vertex
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, DijkstraAllPairsSP};
    ///
    /// let mut graph = EdgeWeightedDigraph::new(5);
    /// graph.add_edge(DirectedEdge::new(0, 1, 1.0));
    ///
    /// let all_sp = DijkstraAllPairsSP::new(&graph);
    /// assert!(all_sp.has_path(0, 1));
    /// assert!(!all_sp.has_path(0, 4));
    /// ```
    pub fn has_path(&self, s: usize, t: usize) -> bool {
        self.all[s].has_path_to(t)
    }

    /// Returns the length of a shortest path from vertex `s` to vertex `t`.
    ///
    /// # Arguments
    ///
    /// * `s` - The source vertex
    /// * `t` - The destination vertex
    ///
    /// # Returns
    ///
    /// * `Some(dist)` - The distance if there is a path
    /// * `None` - If there is no path
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, DijkstraAllPairsSP};
    ///
    /// let mut graph = EdgeWeightedDigraph::new(5);
    /// graph.add_edge(DirectedEdge::new(0, 1, 1.0));
    /// graph.add_edge(DirectedEdge::new(1, 2, 2.0));
    ///
    /// let all_sp = DijkstraAllPairsSP::new(&graph);
    /// assert_eq!(all_sp.dist(0, 2), Some(3.0));
    /// assert_eq!(all_sp.dist(0, 4), None);
    /// ```
    pub fn dist(&self, s: usize, t: usize) -> Option<f64> {
        self.all[s].dist_to(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_graph() -> EdgeWeightedDigraph {
        let mut graph = EdgeWeightedDigraph::new(8);
        graph.add_edge(DirectedEdge::new(0, 1, 5.0));
        graph.add_edge(DirectedEdge::new(0, 4, 9.0));
        graph.add_edge(DirectedEdge::new(0, 7, 8.0));
        graph.add_edge(DirectedEdge::new(1, 2, 12.0));
        graph.add_edge(DirectedEdge::new(1, 3, 15.0));
        graph.add_edge(DirectedEdge::new(1, 7, 4.0));
        graph.add_edge(DirectedEdge::new(2, 3, 3.0));
        graph.add_edge(DirectedEdge::new(2, 6, 11.0));
        graph.add_edge(DirectedEdge::new(3, 6, 9.0));
        graph.add_edge(DirectedEdge::new(4, 5, 4.0));
        graph.add_edge(DirectedEdge::new(4, 6, 20.0));
        graph.add_edge(DirectedEdge::new(4, 7, 5.0));
        graph.add_edge(DirectedEdge::new(5, 2, 1.0));
        graph.add_edge(DirectedEdge::new(5, 6, 13.0));
        graph.add_edge(DirectedEdge::new(7, 5, 6.0));
        graph.add_edge(DirectedEdge::new(7, 2, 7.0));
        graph
    }

    #[test]
    fn test_all_pairs_basic() {
        let graph = create_test_graph();
        let all_sp = DijkstraAllPairsSP::new(&graph);

        // Test some specific paths
        // Test that paths exist and distances are correct
        assert!(all_sp.has_path(0, 6));
        assert_eq!(all_sp.dist(0, 0), Some(0.0));
        assert!(all_sp.has_path(1, 6));
        assert!(all_sp.has_path(4, 3));
    }

    #[test]
    fn test_has_path() {
        let graph = create_test_graph();
        let all_sp = DijkstraAllPairsSP::new(&graph);

        assert!(all_sp.has_path(0, 6));
        assert!(all_sp.has_path(4, 3));
    }

    #[test]
    fn test_path() {
        let graph = create_test_graph();
        let all_sp = DijkstraAllPairsSP::new(&graph);

        let path = all_sp.path(0, 6).unwrap();
        let mut total_weight = 0.0;
        for e in &path {
            total_weight += e.weight();
        }
        assert_eq!(total_weight, 25.0);
    }

    #[test]
    fn test_disconnected_graph() {
        let mut graph = EdgeWeightedDigraph::new(5);
        graph.add_edge(DirectedEdge::new(0, 1, 1.0));
        graph.add_edge(DirectedEdge::new(3, 4, 1.0));

        let all_sp = DijkstraAllPairsSP::new(&graph);
        assert!(all_sp.has_path(0, 1));
        assert!(!all_sp.has_path(0, 3));
        assert_eq!(all_sp.dist(0, 3), None);
        assert_eq!(all_sp.path(0, 3), None);
    }

    #[test]
    fn test_empty_graph() {
        let graph = EdgeWeightedDigraph::new(5);
        let all_sp = DijkstraAllPairsSP::new(&graph);

        for s in 0..5 {
            assert_eq!(all_sp.dist(s, s), Some(0.0));
            for t in 0..5 {
                if s != t {
                    assert_eq!(all_sp.dist(s, t), None);
                    assert!(!all_sp.has_path(s, t));
                }
            }
        }
    }

    #[test]
    fn test_single_vertex() {
        let graph = EdgeWeightedDigraph::new(1);
        let all_sp = DijkstraAllPairsSP::new(&graph);

        assert_eq!(all_sp.dist(0, 0), Some(0.0));
        assert!(all_sp.has_path(0, 0));
    }

    #[test]
    fn test_different_sources() {
        let mut graph = EdgeWeightedDigraph::new(4);
        graph.add_edge(DirectedEdge::new(0, 1, 1.0));
        graph.add_edge(DirectedEdge::new(1, 2, 2.0));
        graph.add_edge(DirectedEdge::new(2, 3, 3.0));

        let all_sp = DijkstraAllPairsSP::new(&graph);

        assert_eq!(all_sp.dist(0, 3), Some(6.0));
        assert_eq!(all_sp.dist(1, 3), Some(5.0));
        assert_eq!(all_sp.dist(2, 3), Some(3.0));
    }
}
