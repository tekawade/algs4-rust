//! Transitive closure of a digraph using Floyd-Warshall algorithm.

use crate::Digraph;

/// Computes the transitive closure of a digraph.
///
/// The transitive closure provides a constant-time method for answering
/// reachability queries: is vertex v reachable from vertex w?
///
/// This implementation uses the Floyd-Warshall algorithm and takes O(V^3) time
/// and O(V^2) space, where V is the number of vertices.
///
/// # Examples
///
/// ```
/// use algs4_graphs::{Digraph, TransitiveClosure};
///
/// let mut graph = Digraph::new(13);
/// graph.add_edge(0, 1);
/// graph.add_edge(0, 5);
/// graph.add_edge(2, 0);
/// graph.add_edge(2, 3);
/// graph.add_edge(3, 2);
/// graph.add_edge(3, 5);
/// graph.add_edge(4, 2);
/// graph.add_edge(4, 3);
/// graph.add_edge(5, 4);
/// graph.add_edge(6, 0);
/// graph.add_edge(6, 4);
/// graph.add_edge(6, 9);
/// graph.add_edge(7, 6);
/// graph.add_edge(7, 8);
/// graph.add_edge(8, 7);
/// graph.add_edge(8, 9);
/// graph.add_edge(9, 10);
/// graph.add_edge(9, 11);
/// graph.add_edge(10, 12);
/// graph.add_edge(11, 4);
/// graph.add_edge(11, 12);
/// graph.add_edge(12, 9);
///
/// let tc = TransitiveClosure::new(&graph);
///
/// assert!(tc.reachable(0, 2));
/// assert!(tc.reachable(0, 4));
/// assert!(!tc.reachable(1, 0));
/// ```
#[derive(Debug, Clone)]
pub struct TransitiveClosure {
    tc: Vec<Vec<bool>>, // tc[v][w] = true if there is a path from v to w
}

impl TransitiveClosure {
    /// Computes the transitive closure of the digraph `graph`.
    ///
    /// # Arguments
    ///
    /// * `graph` - The digraph
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Digraph, TransitiveClosure};
    ///
    /// let mut graph = Digraph::new(5);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    /// graph.add_edge(2, 3);
    ///
    /// let tc = TransitiveClosure::new(&graph);
    /// assert!(tc.reachable(0, 3));
    /// ```
    pub fn new(graph: &Digraph) -> Self {
        let v = graph.v();
        let mut tc = vec![vec![false; v]; v];

        // Initialize with direct edges
        for (i, tc_row) in tc.iter_mut().enumerate().take(v) {
            tc_row[i] = true; // Every vertex is reachable from itself
            for &j in graph.adj(i) {
                tc_row[j] = true;
            }
        }

        // Floyd-Warshall algorithm for reachability
        for k in 0..v {
            // Clone the k-th row to avoid borrowing issues
            let tc_k_row = tc[k].clone();
            for tc_row in &mut tc {
                let tc_i_k = tc_row[k];
                if tc_i_k {
                    for (j, tc_ij) in tc_row.iter_mut().enumerate() {
                        if tc_k_row[j] {
                            *tc_ij = true;
                        }
                    }
                }
            }
        }

        TransitiveClosure { tc }
    }

    /// Returns `true` if vertex `w` is reachable from vertex `v`.
    ///
    /// # Arguments
    ///
    /// * `v` - The source vertex
    /// * `w` - The destination vertex
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Digraph, TransitiveClosure};
    ///
    /// let mut graph = Digraph::new(5);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    ///
    /// let tc = TransitiveClosure::new(&graph);
    /// assert!(tc.reachable(0, 2));
    /// assert!(!tc.reachable(2, 0));
    /// ```
    pub fn reachable(&self, v: usize, w: usize) -> bool {
        self.tc[v][w]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_graph() -> Digraph {
        let mut graph = Digraph::new(13);
        graph.add_edge(0, 1);
        graph.add_edge(0, 5);
        graph.add_edge(2, 0);
        graph.add_edge(2, 3);
        graph.add_edge(3, 2);
        graph.add_edge(3, 5);
        graph.add_edge(4, 2);
        graph.add_edge(4, 3);
        graph.add_edge(5, 4);
        graph.add_edge(6, 0);
        graph.add_edge(6, 4);
        graph.add_edge(6, 9);
        graph.add_edge(7, 6);
        graph.add_edge(7, 8);
        graph.add_edge(8, 7);
        graph.add_edge(8, 9);
        graph.add_edge(9, 10);
        graph.add_edge(9, 11);
        graph.add_edge(10, 12);
        graph.add_edge(11, 4);
        graph.add_edge(11, 12);
        graph.add_edge(12, 9);
        graph
    }

    #[test]
    fn test_transitive_closure_basic() {
        let graph = create_test_graph();
        let tc = TransitiveClosure::new(&graph);

        // Test some reachability queries
        assert!(tc.reachable(0, 2));
        assert!(tc.reachable(0, 4));
        assert!(tc.reachable(2, 0));
        assert!(tc.reachable(6, 12));
        assert!(tc.reachable(7, 9));
    }

    #[test]
    fn test_self_reachability() {
        let graph = create_test_graph();
        let tc = TransitiveClosure::new(&graph);

        // Every vertex should be reachable from itself
        for v in 0..13 {
            assert!(tc.reachable(v, v));
        }
    }

    #[test]
    fn test_non_reachable() {
        let graph = create_test_graph();
        let tc = TransitiveClosure::new(&graph);

        // Test some non-reachable pairs
        assert!(!tc.reachable(1, 0));
        assert!(!tc.reachable(5, 6));
        assert!(!tc.reachable(10, 7));
    }

    #[test]
    fn test_simple_path() {
        let mut graph = Digraph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let tc = TransitiveClosure::new(&graph);

        assert!(tc.reachable(0, 4));
        assert!(tc.reachable(1, 4));
        assert!(tc.reachable(2, 4));
        assert!(tc.reachable(3, 4));
        assert!(!tc.reachable(4, 0));
    }

    #[test]
    fn test_cycle() {
        let mut graph = Digraph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let tc = TransitiveClosure::new(&graph);

        // In a cycle, all vertices are reachable from each other
        for v in 0..3 {
            for w in 0..3 {
                assert!(tc.reachable(v, w));
            }
        }
    }

    #[test]
    fn test_disconnected_graph() {
        let mut graph = Digraph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(3, 4);

        let tc = TransitiveClosure::new(&graph);

        assert!(tc.reachable(0, 1));
        assert!(tc.reachable(3, 4));
        assert!(!tc.reachable(0, 3));
        assert!(!tc.reachable(0, 4));
        assert!(!tc.reachable(1, 3));
        assert!(!tc.reachable(3, 0));
    }

    #[test]
    fn test_empty_graph() {
        let graph = Digraph::new(5);
        let tc = TransitiveClosure::new(&graph);

        // Only self-reachability
        for v in 0..5 {
            assert!(tc.reachable(v, v));
            for w in 0..5 {
                if v != w {
                    assert!(!tc.reachable(v, w));
                }
            }
        }
    }

    #[test]
    fn test_complete_graph() {
        let mut graph = Digraph::new(4);
        for i in 0..4 {
            for j in 0..4 {
                if i != j {
                    graph.add_edge(i, j);
                }
            }
        }

        let tc = TransitiveClosure::new(&graph);

        // All vertices are reachable from all other vertices
        for v in 0..4 {
            for w in 0..4 {
                assert!(tc.reachable(v, w));
            }
        }
    }

    #[test]
    fn test_transitive_path() {
        let mut graph = Digraph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);

        let tc = TransitiveClosure::new(&graph);

        // 0 can reach 3 transitively
        assert!(tc.reachable(0, 3));
        assert!(tc.reachable(0, 2));
        assert!(tc.reachable(1, 3));
    }
}
