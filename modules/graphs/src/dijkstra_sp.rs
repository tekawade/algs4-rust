//! Dijkstra's shortest path algorithm for edge-weighted digraphs with non-negative weights.

use crate::{DirectedEdge, EdgeWeightedDigraph};
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

/// Computes shortest paths from a source vertex to all other vertices in an
/// edge-weighted digraph using Dijkstra's algorithm.
///
/// This implementation uses Dijkstra's algorithm with a binary heap.
/// The constructor takes O(E log V) time in the worst case, where V is the
/// number of vertices and E is the number of edges.
///
/// # Panics
///
/// Panics if the edge-weighted digraph has edges with negative weights.
///
/// # Examples
///
/// ```
/// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, DijkstraSP};
///
/// let mut graph = EdgeWeightedDigraph::new(8);
/// graph.add_edge(DirectedEdge::new(0, 1, 5.0));
/// graph.add_edge(DirectedEdge::new(0, 4, 9.0));
/// graph.add_edge(DirectedEdge::new(0, 7, 8.0));
/// graph.add_edge(DirectedEdge::new(1, 2, 12.0));
/// graph.add_edge(DirectedEdge::new(1, 3, 15.0));
/// graph.add_edge(DirectedEdge::new(1, 7, 4.0));
/// graph.add_edge(DirectedEdge::new(2, 3, 3.0));
/// graph.add_edge(DirectedEdge::new(2, 6, 11.0));
/// graph.add_edge(DirectedEdge::new(3, 6, 9.0));
/// graph.add_edge(DirectedEdge::new(4, 5, 4.0));
/// graph.add_edge(DirectedEdge::new(4, 6, 20.0));
/// graph.add_edge(DirectedEdge::new(4, 7, 5.0));
/// graph.add_edge(DirectedEdge::new(5, 2, 1.0));
/// graph.add_edge(DirectedEdge::new(5, 6, 13.0));
/// graph.add_edge(DirectedEdge::new(7, 5, 6.0));
/// graph.add_edge(DirectedEdge::new(7, 2, 7.0));
///
/// let sp = DijkstraSP::new(&graph, 0);
///
/// assert!(sp.has_path_to(6));
/// assert_eq!(sp.dist_to(6), Some(25.0));
/// ```
#[derive(Debug, Clone)]
pub struct DijkstraSP {
    dist_to: Vec<f64>,                  // dist_to[v] = distance of shortest s->v path
    edge_to: Vec<Option<DirectedEdge>>, // edge_to[v] = last edge on shortest s->v path
}

impl DijkstraSP {
    /// Computes a shortest-paths tree from the source vertex `s` to every other
    /// vertex in the edge-weighted digraph `graph`.
    ///
    /// # Arguments
    ///
    /// * `graph` - The edge-weighted digraph
    /// * `s` - The source vertex
    ///
    /// # Panics
    ///
    /// Panics if an edge weight is negative or if `s` is not a valid vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, DijkstraSP};
    ///
    /// let mut graph = EdgeWeightedDigraph::new(5);
    /// graph.add_edge(DirectedEdge::new(0, 1, 1.0));
    /// graph.add_edge(DirectedEdge::new(0, 2, 3.0));
    /// graph.add_edge(DirectedEdge::new(1, 2, 1.0));
    ///
    /// let sp = DijkstraSP::new(&graph, 0);
    /// assert_eq!(sp.dist_to(2), Some(2.0));
    /// ```
    pub fn new(graph: &EdgeWeightedDigraph, s: usize) -> Self {
        // Check for negative edge weights
        for e in graph.edges() {
            if e.weight() < 0.0 {
                panic!("edge {} has negative weight", e);
            }
        }

        let v = graph.v();
        let mut dist_to = vec![f64::INFINITY; v];
        let mut edge_to = vec![None; v];
        dist_to[s] = 0.0;

        // Relax vertices in order of distance from s
        let mut pq = IndexMinPQ::new(v);
        pq.insert(s, OrdF64(0.0));

        while !pq.is_empty() {
            let v = pq.del_min().unwrap();
            for &e in graph.adj(v) {
                Self::relax(&mut dist_to, &mut edge_to, &mut pq, e);
            }
        }

        DijkstraSP { dist_to, edge_to }
    }

    // Relax edge e and update pq if changed
    fn relax(
        dist_to: &mut [f64],
        edge_to: &mut [Option<DirectedEdge>],
        pq: &mut IndexMinPQ<OrdF64>,
        e: DirectedEdge,
    ) {
        let v = e.from();
        let w = e.to();
        if dist_to[w] > dist_to[v] + e.weight() {
            dist_to[w] = dist_to[v] + e.weight();
            edge_to[w] = Some(e);
            if pq.contains(w) {
                pq.decrease_key(w, OrdF64(dist_to[w]));
            } else {
                pq.insert(w, OrdF64(dist_to[w]));
            }
        }
    }

    /// Returns the length of a shortest path from the source vertex `s` to vertex `v`.
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
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, DijkstraSP};
    ///
    /// let mut graph = EdgeWeightedDigraph::new(5);
    /// graph.add_edge(DirectedEdge::new(0, 1, 1.0));
    /// graph.add_edge(DirectedEdge::new(1, 2, 2.0));
    ///
    /// let sp = DijkstraSP::new(&graph, 0);
    /// assert_eq!(sp.dist_to(2), Some(3.0));
    /// assert_eq!(sp.dist_to(4), None);
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
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, DijkstraSP};
    ///
    /// let mut graph = EdgeWeightedDigraph::new(5);
    /// graph.add_edge(DirectedEdge::new(0, 1, 1.0));
    ///
    /// let sp = DijkstraSP::new(&graph, 0);
    /// assert!(sp.has_path_to(1));
    /// assert!(!sp.has_path_to(4));
    /// ```
    pub fn has_path_to(&self, v: usize) -> bool {
        self.dist_to[v].is_finite()
    }

    /// Returns a shortest path from the source vertex to vertex `v`.
    ///
    /// # Arguments
    ///
    /// * `v` - The destination vertex
    ///
    /// # Returns
    ///
    /// * `Some(path)` - A vector of edges representing the shortest path
    /// * `None` - If there is no path
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, DijkstraSP};
    ///
    /// let mut graph = EdgeWeightedDigraph::new(5);
    /// graph.add_edge(DirectedEdge::new(0, 1, 1.0));
    /// graph.add_edge(DirectedEdge::new(1, 2, 2.0));
    ///
    /// let sp = DijkstraSP::new(&graph, 0);
    /// let path = sp.path_to(2).unwrap();
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
    fn test_dijkstra_basic() {
        let graph = create_test_graph();
        let sp = DijkstraSP::new(&graph, 0);

        // Test distances
        assert_eq!(sp.dist_to(0), Some(0.0));
        assert_eq!(sp.dist_to(1), Some(5.0));
        assert_eq!(sp.dist_to(2), Some(14.0));
        assert_eq!(sp.dist_to(3), Some(17.0));
        assert_eq!(sp.dist_to(4), Some(9.0));
        assert_eq!(sp.dist_to(5), Some(13.0));
        assert_eq!(sp.dist_to(6), Some(25.0));
        assert_eq!(sp.dist_to(7), Some(8.0));
    }

    #[test]
    fn test_has_path_to() {
        let graph = create_test_graph();
        let sp = DijkstraSP::new(&graph, 0);

        for v in 0..8 {
            assert!(sp.has_path_to(v));
        }
    }

    #[test]
    fn test_path_to() {
        let graph = create_test_graph();
        let sp = DijkstraSP::new(&graph, 0);

        let path = sp.path_to(6).unwrap();
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

        let sp = DijkstraSP::new(&graph, 0);
        assert!(sp.has_path_to(1));
        assert!(!sp.has_path_to(3));
        assert_eq!(sp.dist_to(3), None);
        assert_eq!(sp.path_to(3), None);
    }

    #[test]
    fn test_self_loop() {
        let mut graph = EdgeWeightedDigraph::new(3);
        graph.add_edge(DirectedEdge::new(0, 1, 1.0));
        graph.add_edge(DirectedEdge::new(1, 1, 0.5));
        graph.add_edge(DirectedEdge::new(1, 2, 2.0));

        let sp = DijkstraSP::new(&graph, 0);
        assert_eq!(sp.dist_to(2), Some(3.0));
    }

    #[test]
    #[should_panic(expected = "has negative weight")]
    fn test_negative_weight() {
        let mut graph = EdgeWeightedDigraph::new(3);
        graph.add_edge(DirectedEdge::new(0, 1, 1.0));
        graph.add_edge(DirectedEdge::new(1, 2, -1.0));

        DijkstraSP::new(&graph, 0);
    }

    #[test]
    fn test_empty_graph() {
        let graph = EdgeWeightedDigraph::new(5);
        let sp = DijkstraSP::new(&graph, 0);

        assert_eq!(sp.dist_to(0), Some(0.0));
        for v in 1..5 {
            assert_eq!(sp.dist_to(v), None);
            assert!(!sp.has_path_to(v));
        }
    }

    #[test]
    fn test_single_vertex() {
        let graph = EdgeWeightedDigraph::new(1);
        let sp = DijkstraSP::new(&graph, 0);

        assert_eq!(sp.dist_to(0), Some(0.0));
        assert!(sp.has_path_to(0));
    }
}
