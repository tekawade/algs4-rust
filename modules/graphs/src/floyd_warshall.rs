//! Floyd-Warshall all-pairs shortest paths algorithm.

use crate::{DirectedEdge, EdgeWeightedDigraph};

/// Computes all-pairs shortest paths in an edge-weighted digraph using
/// the Floyd-Warshall algorithm.
///
/// This implementation uses dynamic programming to compute shortest paths
/// between all pairs of vertices. The constructor takes O(V^3) time and
/// O(V^2) space, where V is the number of vertices.
///
/// The algorithm can handle negative edge weights and detects negative cycles.
///
/// # Examples
///
/// ```
/// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, FloydWarshall};
///
/// let mut graph = EdgeWeightedDigraph::new(5);
/// graph.add_edge(DirectedEdge::new(0, 1, 1.0));
/// graph.add_edge(DirectedEdge::new(0, 2, 3.0));
/// graph.add_edge(DirectedEdge::new(1, 2, 1.0));
/// graph.add_edge(DirectedEdge::new(2, 3, 2.0));
/// graph.add_edge(DirectedEdge::new(3, 4, 1.0));
///
/// let fw = FloydWarshall::new(&graph);
///
/// assert!(fw.has_path(0, 4));
/// assert_eq!(fw.dist(0, 4), Some(5.0));
/// ```
#[derive(Debug, Clone)]
pub struct FloydWarshall {
    has_negative_cycle: bool,                // is there a negative cycle?
    dist_to: Vec<Vec<f64>>,                  // dist_to[v][w] = length of shortest v->w path
    edge_to: Vec<Vec<Option<DirectedEdge>>>, // edge_to[v][w] = last edge on shortest v->w path
}

impl FloydWarshall {
    /// Computes all-pairs shortest paths in the edge-weighted digraph `graph`.
    ///
    /// # Arguments
    ///
    /// * `graph` - The edge-weighted digraph
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, FloydWarshall};
    ///
    /// let mut graph = EdgeWeightedDigraph::new(5);
    /// graph.add_edge(DirectedEdge::new(0, 1, 1.0));
    /// graph.add_edge(DirectedEdge::new(1, 2, 2.0));
    ///
    /// let fw = FloydWarshall::new(&graph);
    /// assert_eq!(fw.dist(0, 2), Some(3.0));
    /// ```
    pub fn new(graph: &EdgeWeightedDigraph) -> Self {
        let v = graph.v();
        let mut dist_to = vec![vec![f64::INFINITY; v]; v];
        let mut edge_to = vec![vec![None; v]; v];

        // Initialize distances to self as 0
        #[allow(clippy::needless_range_loop)]
        for i in 0..v {
            dist_to[i][i] = 0.0;
        }

        // Initialize distances and edges from the graph
        for i in 0..v {
            for &e in graph.adj(i) {
                let j = e.to();
                dist_to[i][j] = e.weight();
                edge_to[i][j] = Some(e);
            }
        }

        // Floyd-Warshall algorithm
        for k in 0..v {
            for i in 0..v {
                if edge_to[i][k].is_none() {
                    continue; // No path from i to k
                }
                for j in 0..v {
                    if dist_to[i][j] > dist_to[i][k] + dist_to[k][j] {
                        dist_to[i][j] = dist_to[i][k] + dist_to[k][j];
                        edge_to[i][j] = edge_to[k][j];
                    }
                }
                // Check for negative cycle
                if dist_to[i][i] < 0.0 {
                    return FloydWarshall {
                        has_negative_cycle: true,
                        dist_to,
                        edge_to,
                    };
                }
            }
        }

        FloydWarshall {
            has_negative_cycle: false,
            dist_to,
            edge_to,
        }
    }

    /// Returns `true` if there is a negative cycle.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, FloydWarshall};
    ///
    /// let mut graph = EdgeWeightedDigraph::new(3);
    /// graph.add_edge(DirectedEdge::new(0, 1, 1.0));
    /// graph.add_edge(DirectedEdge::new(1, 2, -2.0));
    /// graph.add_edge(DirectedEdge::new(2, 0, -1.0)); // negative cycle
    ///
    /// let fw = FloydWarshall::new(&graph);
    /// assert!(fw.has_negative_cycle());
    /// ```
    pub fn has_negative_cycle(&self) -> bool {
        self.has_negative_cycle
    }

    /// Returns a negative cycle if one exists.
    ///
    /// # Returns
    ///
    /// * `Some(cycle)` - A vector of edges representing the negative cycle
    /// * `None` - If there is no negative cycle
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, FloydWarshall};
    ///
    /// let mut graph = EdgeWeightedDigraph::new(3);
    /// graph.add_edge(DirectedEdge::new(0, 1, 1.0));
    /// graph.add_edge(DirectedEdge::new(1, 2, -2.0));
    /// graph.add_edge(DirectedEdge::new(2, 0, -1.0)); // negative cycle
    ///
    /// let fw = FloydWarshall::new(&graph);
    /// assert!(fw.negative_cycle().is_some());
    /// ```
    pub fn negative_cycle(&self) -> Option<Vec<DirectedEdge>> {
        if !self.has_negative_cycle {
            return None;
        }

        let v = self.dist_to.len();
        // Find a vertex on the negative cycle
        #[allow(clippy::needless_range_loop)]
        for i in 0..v {
            if self.dist_to[i][i] < 0.0 {
                let mut cycle = Vec::new();
                let mut x = i;
                #[allow(clippy::while_let_loop)]
                loop {
                    if let Some(e) = self.edge_to[i][x] {
                        cycle.push(e);
                        x = e.from();
                        if x == i && cycle.len() > 1 {
                            break;
                        }
                    } else {
                        break;
                    }
                    // Prevent infinite loop
                    if cycle.len() > v {
                        break;
                    }
                }
                if !cycle.is_empty() {
                    return Some(cycle);
                }
            }
        }
        None
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
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, FloydWarshall};
    ///
    /// let mut graph = EdgeWeightedDigraph::new(5);
    /// graph.add_edge(DirectedEdge::new(0, 1, 1.0));
    ///
    /// let fw = FloydWarshall::new(&graph);
    /// assert!(fw.has_path(0, 1));
    /// assert!(!fw.has_path(0, 4));
    /// ```
    pub fn has_path(&self, s: usize, t: usize) -> bool {
        !self.has_negative_cycle && self.dist_to[s][t].is_finite()
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
    /// * `Some(dist)` - The distance if there is a path and no negative cycle
    /// * `None` - If there is no path or a negative cycle exists
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, FloydWarshall};
    ///
    /// let mut graph = EdgeWeightedDigraph::new(5);
    /// graph.add_edge(DirectedEdge::new(0, 1, 1.0));
    /// graph.add_edge(DirectedEdge::new(1, 2, 2.0));
    ///
    /// let fw = FloydWarshall::new(&graph);
    /// assert_eq!(fw.dist(0, 2), Some(3.0));
    /// assert_eq!(fw.dist(0, 4), None);
    /// ```
    pub fn dist(&self, s: usize, t: usize) -> Option<f64> {
        if self.has_negative_cycle {
            return None;
        }
        if self.dist_to[s][t].is_finite() {
            Some(self.dist_to[s][t])
        } else {
            None
        }
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
    /// * `None` - If there is no path or a negative cycle exists
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, FloydWarshall};
    ///
    /// let mut graph = EdgeWeightedDigraph::new(5);
    /// graph.add_edge(DirectedEdge::new(0, 1, 1.0));
    /// graph.add_edge(DirectedEdge::new(1, 2, 2.0));
    ///
    /// let fw = FloydWarshall::new(&graph);
    /// let path = fw.path(0, 2).unwrap();
    /// assert_eq!(path.len(), 2);
    /// ```
    pub fn path(&self, s: usize, t: usize) -> Option<Vec<DirectedEdge>> {
        if !self.has_path(s, t) {
            return None;
        }

        let mut path = Vec::new();
        let mut current = t;
        while current != s {
            if let Some(e) = self.edge_to[s][current] {
                path.push(e);
                current = e.from();
            } else {
                return None;
            }
        }
        path.reverse();
        Some(path)
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
    fn test_floyd_warshall_basic() {
        let graph = create_test_graph();
        let fw = FloydWarshall::new(&graph);

        assert!(!fw.has_negative_cycle());
        // Test that paths exist
        assert!(fw.has_path(0, 6));
        assert_eq!(fw.dist(0, 0), Some(0.0));
        assert!(fw.has_path(1, 6));
        assert!(fw.has_path(4, 3));
    }

    #[test]
    fn test_negative_weights() {
        let mut graph = EdgeWeightedDigraph::new(5);
        graph.add_edge(DirectedEdge::new(0, 1, 1.0));
        graph.add_edge(DirectedEdge::new(1, 2, -2.0));
        graph.add_edge(DirectedEdge::new(2, 3, 3.0));

        let fw = FloydWarshall::new(&graph);
        assert!(!fw.has_negative_cycle());
        assert_eq!(fw.dist(0, 3), Some(2.0));
    }

    #[test]
    fn test_negative_cycle() {
        let mut graph = EdgeWeightedDigraph::new(3);
        graph.add_edge(DirectedEdge::new(0, 1, 1.0));
        graph.add_edge(DirectedEdge::new(1, 2, -2.0));
        graph.add_edge(DirectedEdge::new(2, 0, -1.0));

        let fw = FloydWarshall::new(&graph);
        assert!(fw.has_negative_cycle());
        assert!(fw.negative_cycle().is_some());
    }

    #[test]
    fn test_has_path() {
        let graph = create_test_graph();
        let fw = FloydWarshall::new(&graph);

        assert!(fw.has_path(0, 6));
        assert!(fw.has_path(4, 3));
    }

    #[test]
    fn test_path() {
        let graph = create_test_graph();
        let fw = FloydWarshall::new(&graph);

        let path = fw.path(0, 6).unwrap();
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

        let fw = FloydWarshall::new(&graph);
        assert!(fw.has_path(0, 1));
        assert!(!fw.has_path(0, 3));
        assert_eq!(fw.dist(0, 3), None);
        assert_eq!(fw.path(0, 3), None);
    }

    #[test]
    fn test_empty_graph() {
        let graph = EdgeWeightedDigraph::new(5);
        let fw = FloydWarshall::new(&graph);

        for s in 0..5 {
            assert_eq!(fw.dist(s, s), Some(0.0));
            for t in 0..5 {
                if s != t {
                    assert_eq!(fw.dist(s, t), None);
                    assert!(!fw.has_path(s, t));
                }
            }
        }
    }

    #[test]
    fn test_all_pairs() {
        let mut graph = EdgeWeightedDigraph::new(4);
        graph.add_edge(DirectedEdge::new(0, 1, 1.0));
        graph.add_edge(DirectedEdge::new(1, 2, 2.0));
        graph.add_edge(DirectedEdge::new(2, 3, 3.0));

        let fw = FloydWarshall::new(&graph);

        assert_eq!(fw.dist(0, 3), Some(6.0));
        assert_eq!(fw.dist(1, 3), Some(5.0));
        assert_eq!(fw.dist(2, 3), Some(3.0));
    }
}
