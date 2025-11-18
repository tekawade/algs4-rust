//! Shortest paths in edge-weighted DAGs (Directed Acyclic Graphs).

use crate::{DirectedEdge, EdgeWeightedDigraph};

/// Computes shortest paths from a source vertex to all other vertices in an
/// edge-weighted DAG (directed acyclic graph).
///
/// This implementation uses topological sort followed by one pass of relaxation.
/// The constructor takes O(V + E) time in the worst case, where V is the number
/// of vertices and E is the number of edges.
///
/// # Panics
///
/// Panics if the digraph is not a DAG (contains a cycle).
///
/// # Examples
///
/// ```
/// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, AcyclicSP};
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
/// let sp = AcyclicSP::new(&graph, 5);
///
/// assert!(sp.has_path_to(0));
/// assert_eq!(sp.dist_to(0), Some(0.73));
/// ```
#[derive(Debug, Clone)]
pub struct AcyclicSP {
    dist_to: Vec<f64>,                  // dist_to[v] = distance of shortest s->v path
    edge_to: Vec<Option<DirectedEdge>>, // edge_to[v] = last edge on shortest s->v path
}

impl AcyclicSP {
    /// Computes a shortest-paths tree from the source vertex `s` to every other
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
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, AcyclicSP};
    ///
    /// let mut graph = EdgeWeightedDigraph::new(5);
    /// graph.add_edge(DirectedEdge::new(0, 1, 1.0));
    /// graph.add_edge(DirectedEdge::new(0, 2, 3.0));
    /// graph.add_edge(DirectedEdge::new(1, 2, 1.0));
    ///
    /// let sp = AcyclicSP::new(&graph, 0);
    /// assert_eq!(sp.dist_to(2), Some(2.0));
    /// ```
    pub fn new(graph: &EdgeWeightedDigraph, s: usize) -> Self {
        let v = graph.v();
        let mut dist_to = vec![f64::INFINITY; v];
        let mut edge_to = vec![None; v];
        dist_to[s] = 0.0;

        // Topological sort
        let topological = Topological::new(graph);
        if !topological.has_order() {
            panic!("Digraph is not acyclic");
        }

        // Relax vertices in topological order
        for &v in topological.order().unwrap() {
            for &e in graph.adj(v) {
                Self::relax(&mut dist_to, &mut edge_to, e);
            }
        }

        AcyclicSP { dist_to, edge_to }
    }

    // Relax edge e
    fn relax(dist_to: &mut [f64], edge_to: &mut [Option<DirectedEdge>], e: DirectedEdge) {
        let v = e.from();
        let w = e.to();
        if dist_to[w] > dist_to[v] + e.weight() {
            dist_to[w] = dist_to[v] + e.weight();
            edge_to[w] = Some(e);
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
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, AcyclicSP};
    ///
    /// let mut graph = EdgeWeightedDigraph::new(5);
    /// graph.add_edge(DirectedEdge::new(0, 1, 1.0));
    /// graph.add_edge(DirectedEdge::new(1, 2, 2.0));
    ///
    /// let sp = AcyclicSP::new(&graph, 0);
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
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, AcyclicSP};
    ///
    /// let mut graph = EdgeWeightedDigraph::new(5);
    /// graph.add_edge(DirectedEdge::new(0, 1, 1.0));
    ///
    /// let sp = AcyclicSP::new(&graph, 0);
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
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, AcyclicSP};
    ///
    /// let mut graph = EdgeWeightedDigraph::new(5);
    /// graph.add_edge(DirectedEdge::new(0, 1, 1.0));
    /// graph.add_edge(DirectedEdge::new(1, 2, 2.0));
    ///
    /// let sp = AcyclicSP::new(&graph, 0);
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

// Helper struct for topological sorting
struct Topological {
    order: Option<Vec<usize>>,
}

impl Topological {
    fn new(graph: &EdgeWeightedDigraph) -> Self {
        let finder = DirectedCycle::new(graph);
        if finder.has_cycle() {
            return Topological { order: None };
        }

        let dfo = DepthFirstOrder::new(graph);
        Topological {
            order: Some(dfo.reverse_post()),
        }
    }

    fn has_order(&self) -> bool {
        self.order.is_some()
    }

    fn order(&self) -> Option<&[usize]> {
        self.order.as_deref()
    }
}

// Helper struct for cycle detection
struct DirectedCycle {
    marked: Vec<bool>,
    on_stack: Vec<bool>,
    has_cycle: bool,
}

impl DirectedCycle {
    fn new(graph: &EdgeWeightedDigraph) -> Self {
        let v = graph.v();
        let mut dc = DirectedCycle {
            marked: vec![false; v],
            on_stack: vec![false; v],
            has_cycle: false,
        };

        for s in 0..v {
            if !dc.marked[s] {
                dc.dfs(graph, s);
            }
        }

        dc
    }

    fn dfs(&mut self, graph: &EdgeWeightedDigraph, v: usize) {
        self.marked[v] = true;
        self.on_stack[v] = true;

        for &e in graph.adj(v) {
            let w = e.to();
            if self.has_cycle {
                return;
            }
            if !self.marked[w] {
                self.dfs(graph, w);
            } else if self.on_stack[w] {
                self.has_cycle = true;
                return;
            }
        }

        self.on_stack[v] = false;
    }

    fn has_cycle(&self) -> bool {
        self.has_cycle
    }
}

// Helper struct for depth-first order
struct DepthFirstOrder {
    marked: Vec<bool>,
    reverse_post: Vec<usize>,
}

impl DepthFirstOrder {
    fn new(graph: &EdgeWeightedDigraph) -> Self {
        let v = graph.v();
        let mut dfo = DepthFirstOrder {
            marked: vec![false; v],
            reverse_post: Vec::new(),
        };

        for s in 0..v {
            if !dfo.marked[s] {
                dfo.dfs(graph, s);
            }
        }

        dfo
    }

    fn dfs(&mut self, graph: &EdgeWeightedDigraph, v: usize) {
        self.marked[v] = true;
        for &e in graph.adj(v) {
            let w = e.to();
            if !self.marked[w] {
                self.dfs(graph, w);
            }
        }
        self.reverse_post.push(v);
    }

    fn reverse_post(mut self) -> Vec<usize> {
        self.reverse_post.reverse();
        self.reverse_post
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
    fn test_acyclic_sp_basic() {
        let graph = create_dag();
        let sp = AcyclicSP::new(&graph, 5);

        // Test key distances (allowing for floating point precision)
        assert!((sp.dist_to(0).unwrap() - 0.73).abs() < 1e-10);
        assert!((sp.dist_to(1).unwrap() - 0.32).abs() < 1e-10);
        assert!((sp.dist_to(2).unwrap() - 0.62).abs() < 1e-10);
        assert!((sp.dist_to(3).unwrap() - 0.61).abs() < 1e-10);
        assert!((sp.dist_to(4).unwrap() - 0.35).abs() < 1e-10);
        assert_eq!(sp.dist_to(5), Some(0.0));
        assert!((sp.dist_to(6).unwrap() - 1.13).abs() < 1e-10);
        assert!((sp.dist_to(7).unwrap() - 0.28).abs() < 1e-10);
    }

    #[test]
    fn test_has_path_to() {
        let graph = create_dag();
        let sp = AcyclicSP::new(&graph, 5);

        for v in 0..8 {
            assert!(sp.has_path_to(v));
        }
    }

    #[test]
    fn test_path_to() {
        let graph = create_dag();
        let sp = AcyclicSP::new(&graph, 5);

        let path = sp.path_to(0).unwrap();
        let mut total_weight = 0.0;
        for e in &path {
            total_weight += e.weight();
        }
        assert!((total_weight - 0.73).abs() < 1e-10);
    }

    #[test]
    #[should_panic(expected = "not acyclic")]
    fn test_cyclic_graph() {
        let mut graph = EdgeWeightedDigraph::new(3);
        graph.add_edge(DirectedEdge::new(0, 1, 1.0));
        graph.add_edge(DirectedEdge::new(1, 2, 2.0));
        graph.add_edge(DirectedEdge::new(2, 0, 3.0));

        AcyclicSP::new(&graph, 0);
    }

    #[test]
    fn test_disconnected_dag() {
        let mut graph = EdgeWeightedDigraph::new(5);
        graph.add_edge(DirectedEdge::new(0, 1, 1.0));
        graph.add_edge(DirectedEdge::new(3, 4, 1.0));

        let sp = AcyclicSP::new(&graph, 0);
        assert!(sp.has_path_to(1));
        assert!(!sp.has_path_to(3));
        assert_eq!(sp.dist_to(3), None);
    }

    #[test]
    fn test_empty_graph() {
        let graph = EdgeWeightedDigraph::new(5);
        let sp = AcyclicSP::new(&graph, 0);

        assert_eq!(sp.dist_to(0), Some(0.0));
        for v in 1..5 {
            assert_eq!(sp.dist_to(v), None);
            assert!(!sp.has_path_to(v));
        }
    }

    #[test]
    fn test_negative_weights() {
        let mut graph = EdgeWeightedDigraph::new(4);
        graph.add_edge(DirectedEdge::new(0, 1, 5.0));
        graph.add_edge(DirectedEdge::new(0, 2, 1.0));
        graph.add_edge(DirectedEdge::new(2, 1, -3.0));

        let sp = AcyclicSP::new(&graph, 0);
        assert_eq!(sp.dist_to(1), Some(-2.0));
    }
}
