//! Longest paths in edge-weighted DAGs (Directed Acyclic Graphs).

use crate::{AcyclicSP, DirectedEdge, EdgeWeightedDigraph};

/// Computes longest paths from a source vertex to all other vertices in an
/// edge-weighted DAG (directed acyclic graph).
///
/// This implementation uses topological sort followed by one pass of relaxation
/// for negated weights. The constructor takes O(V + E) time in the worst case,
/// where V is the number of vertices and E is the number of edges.
///
/// # Panics
///
/// Panics if the digraph is not a DAG (contains a cycle).
///
/// # Examples
///
/// ```
/// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, AcyclicLP};
///
/// let mut graph = EdgeWeightedDigraph::new(8);
/// graph.add_edge(DirectedEdge::new(5, 4, 0.35));
/// graph.add_edge(DirectedEdge::new(4, 7, 0.37));
/// graph.add_edge(DirectedEdge::new(5, 7, 0.28));
/// graph.add_edge(DirectedEdge::new(5, 1, 0.32));
/// graph.add_edge(DirectedEdge::new(4, 0, 0.38));
/// graph.add_edge(DirectedEdge::new(0, 2, 0.26));
/// graph.add_edge(DirectedEdge::new(3, 7, 0.39));
/// graph.add_edge(DirectedEdge::new(1, 3, 0.29));
/// graph.add_edge(DirectedEdge::new(7, 2, 0.34));
/// graph.add_edge(DirectedEdge::new(6, 2, 0.40));
/// graph.add_edge(DirectedEdge::new(3, 6, 0.52));
/// graph.add_edge(DirectedEdge::new(6, 0, 0.58));
/// graph.add_edge(DirectedEdge::new(6, 4, 0.93));
///
/// let lp = AcyclicLP::new(&graph, 5);
///
/// assert!(lp.has_path_to(0));
/// assert_eq!(lp.dist_to(0), Some(2.44));
/// ```
#[derive(Debug, Clone)]
pub struct AcyclicLP {
    dist_to: Vec<f64>,                  // dist_to[v] = distance of longest s->v path
    edge_to: Vec<Option<DirectedEdge>>, // edge_to[v] = last edge on longest s->v path
}

impl AcyclicLP {
    /// Computes a longest-paths tree from the source vertex `s` to every other
    /// vertex in the edge-weighted DAG `graph`.
    ///
    /// # Arguments
    ///
    /// * `graph` - The edge-weighted DAG
    /// * `s` - The source vertex
    ///
    /// # Panics
    ///
    /// Panics if the digraph is not a DAG or if `s` is not a valid vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, AcyclicLP};
    ///
    /// let mut graph = EdgeWeightedDigraph::new(5);
    /// graph.add_edge(DirectedEdge::new(0, 1, 1.0));
    /// graph.add_edge(DirectedEdge::new(0, 2, 3.0));
    /// graph.add_edge(DirectedEdge::new(1, 2, 1.0));
    ///
    /// let lp = AcyclicLP::new(&graph, 0);
    /// assert_eq!(lp.dist_to(2), Some(3.0));
    /// ```
    pub fn new(graph: &EdgeWeightedDigraph, s: usize) -> Self {
        // Create a new graph with negated weights
        let mut negated = EdgeWeightedDigraph::new(graph.v());
        for e in graph.edges() {
            negated.add_edge(DirectedEdge::new(e.from(), e.to(), -e.weight()));
        }

        // Run shortest path algorithm on negated graph
        let sp = AcyclicSP::new(&negated, s);

        // Convert back to longest paths
        let v = graph.v();
        let mut dist_to = vec![f64::NEG_INFINITY; v];
        let mut edge_to = vec![None; v];

        for (i, dist) in dist_to.iter_mut().enumerate() {
            if let Some(d) = sp.dist_to(i) {
                *dist = -d;
            }
        }

        // Reconstruct edge_to with original edges
        for (i, edge) in edge_to.iter_mut().enumerate() {
            if let Some(path) = sp.path_to(i) {
                if !path.is_empty() {
                    let last_edge = path.last().unwrap();
                    // Find the corresponding edge in the original graph
                    for e in graph.adj(last_edge.from()) {
                        if e.to() == last_edge.to() {
                            *edge = Some(*e);
                            break;
                        }
                    }
                }
            }
        }

        dist_to[s] = 0.0;

        AcyclicLP { dist_to, edge_to }
    }

    /// Returns the length of a longest path from the source vertex `s` to vertex `v`.
    ///
    /// # Arguments
    ///
    /// * `v` - The destination vertex
    ///
    /// # Returns
    ///
    /// * `Some(dist)` - The distance if there is a path
    /// * `None` - If there is no path
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, AcyclicLP};
    ///
    /// let mut graph = EdgeWeightedDigraph::new(5);
    /// graph.add_edge(DirectedEdge::new(0, 1, 1.0));
    /// graph.add_edge(DirectedEdge::new(1, 2, 2.0));
    ///
    /// let lp = AcyclicLP::new(&graph, 0);
    /// assert_eq!(lp.dist_to(2), Some(3.0));
    /// assert_eq!(lp.dist_to(4), None);
    /// ```
    pub fn dist_to(&self, v: usize) -> Option<f64> {
        if self.dist_to[v].is_finite() {
            Some(self.dist_to[v])
        } else {
            None
        }
    }

    /// Returns `true` if there is a path from the source vertex to vertex `v`.
    ///
    /// # Arguments
    ///
    /// * `v` - The destination vertex
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, AcyclicLP};
    ///
    /// let mut graph = EdgeWeightedDigraph::new(5);
    /// graph.add_edge(DirectedEdge::new(0, 1, 1.0));
    ///
    /// let lp = AcyclicLP::new(&graph, 0);
    /// assert!(lp.has_path_to(1));
    /// assert!(!lp.has_path_to(4));
    /// ```
    pub fn has_path_to(&self, v: usize) -> bool {
        self.dist_to[v].is_finite()
    }

    /// Returns a longest path from the source vertex to vertex `v`.
    ///
    /// # Arguments
    ///
    /// * `v` - The destination vertex
    ///
    /// # Returns
    ///
    /// * `Some(path)` - A vector of edges representing the longest path
    /// * `None` - If there is no path
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, AcyclicLP};
    ///
    /// let mut graph = EdgeWeightedDigraph::new(5);
    /// graph.add_edge(DirectedEdge::new(0, 1, 1.0));
    /// graph.add_edge(DirectedEdge::new(1, 2, 2.0));
    ///
    /// let lp = AcyclicLP::new(&graph, 0);
    /// let path = lp.path_to(2).unwrap();
    /// assert_eq!(path.len(), 2);
    /// ```
    pub fn path_to(&self, v: usize) -> Option<Vec<DirectedEdge>> {
        if !self.has_path_to(v) {
            return None;
        }

        let mut path = Vec::new();
        let mut current = v;
        while let Some(e) = self.edge_to[current] {
            path.push(e);
            current = e.from();
        }
        path.reverse();
        Some(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_dag() -> EdgeWeightedDigraph {
        let mut graph = EdgeWeightedDigraph::new(8);
        graph.add_edge(DirectedEdge::new(5, 4, 0.35));
        graph.add_edge(DirectedEdge::new(4, 7, 0.37));
        graph.add_edge(DirectedEdge::new(5, 7, 0.28));
        graph.add_edge(DirectedEdge::new(5, 1, 0.32));
        graph.add_edge(DirectedEdge::new(4, 0, 0.38));
        graph.add_edge(DirectedEdge::new(0, 2, 0.26));
        graph.add_edge(DirectedEdge::new(3, 7, 0.39));
        graph.add_edge(DirectedEdge::new(1, 3, 0.29));
        graph.add_edge(DirectedEdge::new(7, 2, 0.34));
        graph.add_edge(DirectedEdge::new(6, 2, 0.40));
        graph.add_edge(DirectedEdge::new(3, 6, 0.52));
        graph.add_edge(DirectedEdge::new(6, 0, 0.58));
        graph.add_edge(DirectedEdge::new(6, 4, 0.93));
        graph
    }

    #[test]
    fn test_acyclic_lp_basic() {
        let graph = create_dag();
        let lp = AcyclicLP::new(&graph, 5);

        // Test that longest paths are computed
        assert!(lp.has_path_to(0));
        assert!(lp.has_path_to(1));
        assert!(lp.has_path_to(2));
        assert!(lp.has_path_to(3));
        assert!(lp.has_path_to(4));
        assert_eq!(lp.dist_to(5), Some(0.0));
        assert!(lp.has_path_to(6));
        assert!(lp.has_path_to(7));

        // Verify longest path to 4 is longer than direct path
        assert!(lp.dist_to(4).unwrap() >= 0.35);
    }

    #[test]
    fn test_has_path_to() {
        let graph = create_dag();
        let lp = AcyclicLP::new(&graph, 5);

        for v in 0..8 {
            assert!(lp.has_path_to(v));
        }
    }

    #[test]
    fn test_path_to() {
        let graph = create_dag();
        let lp = AcyclicLP::new(&graph, 5);

        let path = lp.path_to(0).unwrap();
        let mut total_weight = 0.0;
        for e in &path {
            total_weight += e.weight();
        }
        assert!((total_weight - 2.44).abs() < 1e-10);
    }

    #[test]
    #[should_panic(expected = "not acyclic")]
    fn test_cyclic_graph() {
        let mut graph = EdgeWeightedDigraph::new(3);
        graph.add_edge(DirectedEdge::new(0, 1, 1.0));
        graph.add_edge(DirectedEdge::new(1, 2, 2.0));
        graph.add_edge(DirectedEdge::new(2, 0, 3.0));

        AcyclicLP::new(&graph, 0);
    }

    #[test]
    fn test_disconnected_dag() {
        let mut graph = EdgeWeightedDigraph::new(5);
        graph.add_edge(DirectedEdge::new(0, 1, 1.0));
        graph.add_edge(DirectedEdge::new(3, 4, 1.0));

        let lp = AcyclicLP::new(&graph, 0);
        assert!(lp.has_path_to(1));
        assert!(!lp.has_path_to(3));
        assert_eq!(lp.dist_to(3), None);
    }

    #[test]
    fn test_empty_graph() {
        let graph = EdgeWeightedDigraph::new(5);
        let lp = AcyclicLP::new(&graph, 0);

        assert_eq!(lp.dist_to(0), Some(0.0));
        for v in 1..5 {
            assert_eq!(lp.dist_to(v), None);
            assert!(!lp.has_path_to(v));
        }
    }

    #[test]
    fn test_simple_longest_path() {
        let mut graph = EdgeWeightedDigraph::new(4);
        graph.add_edge(DirectedEdge::new(0, 1, 5.0));
        graph.add_edge(DirectedEdge::new(0, 2, 1.0));
        graph.add_edge(DirectedEdge::new(2, 1, 3.0));

        let lp = AcyclicLP::new(&graph, 0);
        assert_eq!(lp.dist_to(1), Some(5.0)); // Direct path is longer
    }

    #[test]
    fn test_multiple_paths() {
        let mut graph = EdgeWeightedDigraph::new(4);
        graph.add_edge(DirectedEdge::new(0, 1, 1.0));
        graph.add_edge(DirectedEdge::new(0, 2, 2.0));
        graph.add_edge(DirectedEdge::new(1, 3, 3.0));
        graph.add_edge(DirectedEdge::new(2, 3, 1.0));

        let lp = AcyclicLP::new(&graph, 0);
        assert_eq!(lp.dist_to(3), Some(4.0)); // Path through vertex 1
    }
}
