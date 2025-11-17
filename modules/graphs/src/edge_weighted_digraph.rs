//! Edge-weighted directed graph implementation.

use crate::directed_edge::DirectedEdge;
use std::fmt;

/// An edge-weighted directed graph, implemented using adjacency lists.
///
/// Supports adding weighted directed edges and iterating over edges adjacent
/// from a given vertex. Parallel edges and self-loops are permitted.
///
/// # Examples
///
/// ```
/// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge};
///
/// let mut digraph = EdgeWeightedDigraph::new(6);
/// digraph.add_edge(DirectedEdge::new(0, 1, 0.5));
/// digraph.add_edge(DirectedEdge::new(0, 2, 0.3));
/// digraph.add_edge(DirectedEdge::new(1, 2, 0.7));
///
/// assert_eq!(digraph.v(), 6);
/// assert_eq!(digraph.e(), 3);
/// ```
#[derive(Debug, Clone)]
pub struct EdgeWeightedDigraph {
    v: usize,                      // number of vertices
    e: usize,                      // number of edges
    adj: Vec<Vec<DirectedEdge>>,   // adjacency lists
    indegree: Vec<usize>,          // indegree of each vertex
}

impl EdgeWeightedDigraph {
    /// Creates a new edge-weighted digraph with `v` vertices and no edges.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::EdgeWeightedDigraph;
    ///
    /// let digraph = EdgeWeightedDigraph::new(10);
    /// assert_eq!(digraph.v(), 10);
    /// assert_eq!(digraph.e(), 0);
    /// ```
    pub fn new(v: usize) -> Self {
        EdgeWeightedDigraph {
            v,
            e: 0,
            adj: vec![Vec::new(); v],
            indegree: vec![0; v],
        }
    }

    /// Returns the number of vertices in the digraph.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::EdgeWeightedDigraph;
    ///
    /// let digraph = EdgeWeightedDigraph::new(5);
    /// assert_eq!(digraph.v(), 5);
    /// ```
    pub fn v(&self) -> usize {
        self.v
    }

    /// Returns the number of edges in the digraph.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge};
    ///
    /// let mut digraph = EdgeWeightedDigraph::new(5);
    /// digraph.add_edge(DirectedEdge::new(0, 1, 0.5));
    /// digraph.add_edge(DirectedEdge::new(1, 2, 0.3));
    /// assert_eq!(digraph.e(), 2);
    /// ```
    pub fn e(&self) -> usize {
        self.e
    }

    /// Validates that vertex v is a valid vertex in the digraph.
    fn validate_vertex(&self, v: usize) {
        if v >= self.v {
            panic!("vertex {} is not between 0 and {}", v, self.v - 1);
        }
    }

    /// Adds the weighted directed edge to the digraph.
    ///
    /// # Arguments
    ///
    /// * `e` - The directed edge to add
    ///
    /// # Panics
    ///
    /// Panics if either vertex is not between 0 and V-1.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge};
    ///
    /// let mut digraph = EdgeWeightedDigraph::new(3);
    /// digraph.add_edge(DirectedEdge::new(0, 1, 0.5));
    /// digraph.add_edge(DirectedEdge::new(1, 2, 0.3));
    /// assert_eq!(digraph.e(), 2);
    /// ```
    pub fn add_edge(&mut self, e: DirectedEdge) {
        let v = e.from();
        let w = e.to();
        self.validate_vertex(v);
        self.validate_vertex(w);
        self.adj[v].push(e);
        self.indegree[w] += 1;
        self.e += 1;
    }

    /// Returns the directed edges incident from vertex `v`.
    ///
    /// # Arguments
    ///
    /// * `v` - The vertex
    ///
    /// # Returns
    ///
    /// A slice containing the directed edges leaving vertex `v`
    ///
    /// # Panics
    ///
    /// Panics if vertex is not between 0 and V-1.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge};
    ///
    /// let mut digraph = EdgeWeightedDigraph::new(4);
    /// digraph.add_edge(DirectedEdge::new(0, 1, 0.5));
    /// digraph.add_edge(DirectedEdge::new(0, 2, 0.3));
    /// digraph.add_edge(DirectedEdge::new(0, 3, 0.7));
    ///
    /// let edges: Vec<DirectedEdge> = digraph.adj(0).iter().copied().collect();
    /// assert_eq!(edges.len(), 3);
    /// ```
    pub fn adj(&self, v: usize) -> &[DirectedEdge] {
        self.validate_vertex(v);
        &self.adj[v]
    }

    /// Returns the outdegree of vertex `v`.
    ///
    /// # Arguments
    ///
    /// * `v` - The vertex
    ///
    /// # Returns
    ///
    /// The number of directed edges leaving vertex `v`
    ///
    /// # Panics
    ///
    /// Panics if vertex is not between 0 and V-1.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge};
    ///
    /// let mut digraph = EdgeWeightedDigraph::new(4);
    /// digraph.add_edge(DirectedEdge::new(0, 1, 0.5));
    /// digraph.add_edge(DirectedEdge::new(0, 2, 0.3));
    /// digraph.add_edge(DirectedEdge::new(0, 3, 0.7));
    ///
    /// assert_eq!(digraph.outdegree(0), 3);
    /// assert_eq!(digraph.outdegree(1), 0);
    /// ```
    pub fn outdegree(&self, v: usize) -> usize {
        self.validate_vertex(v);
        self.adj[v].len()
    }

    /// Returns the indegree of vertex `v`.
    ///
    /// # Arguments
    ///
    /// * `v` - The vertex
    ///
    /// # Returns
    ///
    /// The number of directed edges arriving at vertex `v`
    ///
    /// # Panics
    ///
    /// Panics if vertex is not between 0 and V-1.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge};
    ///
    /// let mut digraph = EdgeWeightedDigraph::new(4);
    /// digraph.add_edge(DirectedEdge::new(0, 1, 0.5));
    /// digraph.add_edge(DirectedEdge::new(2, 1, 0.3));
    /// digraph.add_edge(DirectedEdge::new(3, 1, 0.7));
    ///
    /// assert_eq!(digraph.indegree(1), 3);
    /// assert_eq!(digraph.indegree(0), 0);
    /// ```
    pub fn indegree(&self, v: usize) -> usize {
        self.validate_vertex(v);
        self.indegree[v]
    }

    /// Returns all directed edges in the digraph.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge};
    ///
    /// let mut digraph = EdgeWeightedDigraph::new(4);
    /// digraph.add_edge(DirectedEdge::new(0, 1, 0.5));
    /// digraph.add_edge(DirectedEdge::new(1, 2, 0.3));
    /// digraph.add_edge(DirectedEdge::new(2, 3, 0.7));
    ///
    /// let edges = digraph.edges();
    /// assert_eq!(edges.len(), 3);
    /// ```
    pub fn edges(&self) -> Vec<DirectedEdge> {
        let mut edges = Vec::new();
        for v in 0..self.v {
            for &e in &self.adj[v] {
                edges.push(e);
            }
        }
        edges
    }
}

impl fmt::Display for EdgeWeightedDigraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} vertices, {} edges", self.v, self.e)?;
        for v in 0..self.v {
            write!(f, "{}: ", v)?;
            for e in &self.adj[v] {
                write!(f, "{} ", e)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_digraph() {
        let digraph = EdgeWeightedDigraph::new(5);
        assert_eq!(digraph.v(), 5);
        assert_eq!(digraph.e(), 0);
    }

    #[test]
    fn test_add_edge() {
        let mut digraph = EdgeWeightedDigraph::new(5);
        digraph.add_edge(DirectedEdge::new(0, 1, 0.5));
        assert_eq!(digraph.e(), 1);
        assert_eq!(digraph.outdegree(0), 1);
        assert_eq!(digraph.indegree(1), 1);
    }

    #[test]
    fn test_adjacency() {
        let mut digraph = EdgeWeightedDigraph::new(4);
        digraph.add_edge(DirectedEdge::new(0, 1, 0.5));
        digraph.add_edge(DirectedEdge::new(0, 2, 0.3));
        digraph.add_edge(DirectedEdge::new(0, 3, 0.7));

        assert_eq!(digraph.adj(0).len(), 3);
    }

    #[test]
    fn test_outdegree() {
        let mut digraph = EdgeWeightedDigraph::new(4);
        digraph.add_edge(DirectedEdge::new(0, 1, 0.5));
        digraph.add_edge(DirectedEdge::new(0, 2, 0.3));
        digraph.add_edge(DirectedEdge::new(0, 3, 0.7));

        assert_eq!(digraph.outdegree(0), 3);
        assert_eq!(digraph.outdegree(1), 0);
    }

    #[test]
    fn test_indegree() {
        let mut digraph = EdgeWeightedDigraph::new(4);
        digraph.add_edge(DirectedEdge::new(0, 1, 0.5));
        digraph.add_edge(DirectedEdge::new(2, 1, 0.3));
        digraph.add_edge(DirectedEdge::new(3, 1, 0.7));

        assert_eq!(digraph.indegree(1), 3);
        assert_eq!(digraph.indegree(0), 0);
    }

    #[test]
    fn test_edges() {
        let mut digraph = EdgeWeightedDigraph::new(4);
        digraph.add_edge(DirectedEdge::new(0, 1, 0.5));
        digraph.add_edge(DirectedEdge::new(1, 2, 0.3));
        digraph.add_edge(DirectedEdge::new(2, 3, 0.7));

        let edges = digraph.edges();
        assert_eq!(edges.len(), 3);
    }

    #[test]
    fn test_self_loop() {
        let mut digraph = EdgeWeightedDigraph::new(3);
        digraph.add_edge(DirectedEdge::new(0, 0, 0.5));
        digraph.add_edge(DirectedEdge::new(0, 1, 0.3));

        assert_eq!(digraph.e(), 2);
        let edges = digraph.edges();
        assert_eq!(edges.len(), 2);
    }

    #[test]
    fn test_parallel_edges() {
        let mut digraph = EdgeWeightedDigraph::new(2);
        digraph.add_edge(DirectedEdge::new(0, 1, 0.5));
        digraph.add_edge(DirectedEdge::new(0, 1, 0.3));

        assert_eq!(digraph.e(), 2);
        assert_eq!(digraph.outdegree(0), 2);
        assert_eq!(digraph.indegree(1), 2);
    }

    #[test]
    #[should_panic(expected = "vertex 5 is not between 0 and 4")]
    fn test_invalid_vertex() {
        let mut digraph = EdgeWeightedDigraph::new(5);
        digraph.add_edge(DirectedEdge::new(0, 5, 0.5));
    }

    #[test]
    fn test_display() {
        let mut digraph = EdgeWeightedDigraph::new(3);
        digraph.add_edge(DirectedEdge::new(0, 1, 0.5));
        digraph.add_edge(DirectedEdge::new(1, 2, 0.3));

        let output = format!("{}", digraph);
        assert!(output.contains("3 vertices, 2 edges"));
    }
}
