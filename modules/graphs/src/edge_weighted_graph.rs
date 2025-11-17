//! Edge-weighted undirected graph implementation.

use crate::edge::Edge;
use std::fmt;

/// An edge-weighted undirected graph, implemented using adjacency lists.
///
/// Supports adding weighted edges and iterating over edges adjacent to a given vertex.
/// Parallel edges and self-loops are permitted.
///
/// # Examples
///
/// ```
/// use algs4_graphs::{EdgeWeightedGraph, Edge};
///
/// let mut graph = EdgeWeightedGraph::new(6);
/// graph.add_edge(Edge::new(0, 1, 0.5));
/// graph.add_edge(Edge::new(0, 2, 0.3));
/// graph.add_edge(Edge::new(1, 2, 0.7));
///
/// assert_eq!(graph.v(), 6);
/// assert_eq!(graph.e(), 3);
/// ```
#[derive(Debug, Clone)]
pub struct EdgeWeightedGraph {
    v: usize,            // number of vertices
    e: usize,            // number of edges
    adj: Vec<Vec<Edge>>, // adjacency lists
}

impl EdgeWeightedGraph {
    /// Creates a new edge-weighted graph with `v` vertices and no edges.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::EdgeWeightedGraph;
    ///
    /// let graph = EdgeWeightedGraph::new(10);
    /// assert_eq!(graph.v(), 10);
    /// assert_eq!(graph.e(), 0);
    /// ```
    pub fn new(v: usize) -> Self {
        EdgeWeightedGraph {
            v,
            e: 0,
            adj: vec![Vec::new(); v],
        }
    }

    /// Returns the number of vertices in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::EdgeWeightedGraph;
    ///
    /// let graph = EdgeWeightedGraph::new(5);
    /// assert_eq!(graph.v(), 5);
    /// ```
    pub fn v(&self) -> usize {
        self.v
    }

    /// Returns the number of edges in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedGraph, Edge};
    ///
    /// let mut graph = EdgeWeightedGraph::new(5);
    /// graph.add_edge(Edge::new(0, 1, 0.5));
    /// graph.add_edge(Edge::new(1, 2, 0.3));
    /// assert_eq!(graph.e(), 2);
    /// ```
    pub fn e(&self) -> usize {
        self.e
    }

    /// Validates that vertex v is a valid vertex in the graph.
    fn validate_vertex(&self, v: usize) {
        if v >= self.v {
            panic!("vertex {} is not between 0 and {}", v, self.v - 1);
        }
    }

    /// Adds the weighted edge to the graph.
    ///
    /// # Arguments
    ///
    /// * `e` - The edge to add
    ///
    /// # Panics
    ///
    /// Panics if either vertex is not between 0 and V-1.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedGraph, Edge};
    ///
    /// let mut graph = EdgeWeightedGraph::new(3);
    /// graph.add_edge(Edge::new(0, 1, 0.5));
    /// graph.add_edge(Edge::new(1, 2, 0.3));
    /// assert_eq!(graph.e(), 2);
    /// ```
    pub fn add_edge(&mut self, e: Edge) {
        let v = e.either();
        let w = e.other(v);
        self.validate_vertex(v);
        self.validate_vertex(w);
        self.adj[v].push(e);
        self.adj[w].push(e);
        self.e += 1;
    }

    /// Returns the edges adjacent to vertex `v`.
    ///
    /// # Arguments
    ///
    /// * `v` - The vertex
    ///
    /// # Returns
    ///
    /// A slice containing the edges incident to vertex `v`
    ///
    /// # Panics
    ///
    /// Panics if vertex is not between 0 and V-1.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedGraph, Edge};
    ///
    /// let mut graph = EdgeWeightedGraph::new(4);
    /// graph.add_edge(Edge::new(0, 1, 0.5));
    /// graph.add_edge(Edge::new(0, 2, 0.3));
    /// graph.add_edge(Edge::new(0, 3, 0.7));
    ///
    /// let edges: Vec<Edge> = graph.adj(0).iter().copied().collect();
    /// assert_eq!(edges.len(), 3);
    /// ```
    pub fn adj(&self, v: usize) -> &[Edge] {
        self.validate_vertex(v);
        &self.adj[v]
    }

    /// Returns the degree of vertex `v`.
    ///
    /// # Arguments
    ///
    /// * `v` - The vertex
    ///
    /// # Returns
    ///
    /// The number of edges incident to vertex `v`
    ///
    /// # Panics
    ///
    /// Panics if vertex is not between 0 and V-1.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedGraph, Edge};
    ///
    /// let mut graph = EdgeWeightedGraph::new(4);
    /// graph.add_edge(Edge::new(0, 1, 0.5));
    /// graph.add_edge(Edge::new(0, 2, 0.3));
    /// graph.add_edge(Edge::new(0, 3, 0.7));
    ///
    /// assert_eq!(graph.degree(0), 3);
    /// assert_eq!(graph.degree(1), 1);
    /// ```
    pub fn degree(&self, v: usize) -> usize {
        self.validate_vertex(v);
        self.adj[v].len()
    }

    /// Returns all edges in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedGraph, Edge};
    ///
    /// let mut graph = EdgeWeightedGraph::new(4);
    /// graph.add_edge(Edge::new(0, 1, 0.5));
    /// graph.add_edge(Edge::new(1, 2, 0.3));
    /// graph.add_edge(Edge::new(2, 3, 0.7));
    ///
    /// let edges = graph.edges();
    /// assert_eq!(edges.len(), 3);
    /// ```
    pub fn edges(&self) -> Vec<Edge> {
        let mut edges = Vec::new();
        for v in 0..self.v {
            let mut self_loops = 0;
            for &e in &self.adj[v] {
                let w = e.other(v);
                if w > v {
                    edges.push(e);
                } else if w == v {
                    // only add one copy of each self-loop
                    if self_loops % 2 == 0 {
                        edges.push(e);
                    }
                    self_loops += 1;
                }
            }
        }
        edges
    }
}

impl fmt::Display for EdgeWeightedGraph {
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
    fn test_new_graph() {
        let graph = EdgeWeightedGraph::new(5);
        assert_eq!(graph.v(), 5);
        assert_eq!(graph.e(), 0);
    }

    #[test]
    fn test_add_edge() {
        let mut graph = EdgeWeightedGraph::new(5);
        graph.add_edge(Edge::new(0, 1, 0.5));
        assert_eq!(graph.e(), 1);
        assert_eq!(graph.degree(0), 1);
        assert_eq!(graph.degree(1), 1);
    }

    #[test]
    fn test_adjacency() {
        let mut graph = EdgeWeightedGraph::new(4);
        graph.add_edge(Edge::new(0, 1, 0.5));
        graph.add_edge(Edge::new(0, 2, 0.3));
        graph.add_edge(Edge::new(0, 3, 0.7));

        assert_eq!(graph.adj(0).len(), 3);
    }

    #[test]
    fn test_degree() {
        let mut graph = EdgeWeightedGraph::new(4);
        graph.add_edge(Edge::new(0, 1, 0.5));
        graph.add_edge(Edge::new(0, 2, 0.3));
        graph.add_edge(Edge::new(0, 3, 0.7));
        graph.add_edge(Edge::new(1, 2, 0.2));

        assert_eq!(graph.degree(0), 3);
        assert_eq!(graph.degree(1), 2);
        assert_eq!(graph.degree(2), 2);
        assert_eq!(graph.degree(3), 1);
    }

    #[test]
    fn test_edges() {
        let mut graph = EdgeWeightedGraph::new(4);
        graph.add_edge(Edge::new(0, 1, 0.5));
        graph.add_edge(Edge::new(1, 2, 0.3));
        graph.add_edge(Edge::new(2, 3, 0.7));

        let edges = graph.edges();
        assert_eq!(edges.len(), 3);
    }

    #[test]
    fn test_self_loop() {
        let mut graph = EdgeWeightedGraph::new(3);
        graph.add_edge(Edge::new(0, 0, 0.5));
        graph.add_edge(Edge::new(0, 1, 0.3));

        assert_eq!(graph.e(), 2);
        let edges = graph.edges();
        assert_eq!(edges.len(), 2);
    }

    #[test]
    fn test_parallel_edges() {
        let mut graph = EdgeWeightedGraph::new(2);
        graph.add_edge(Edge::new(0, 1, 0.5));
        graph.add_edge(Edge::new(0, 1, 0.3));

        assert_eq!(graph.e(), 2);
        assert_eq!(graph.degree(0), 2);
        assert_eq!(graph.degree(1), 2);
    }

    #[test]
    #[should_panic(expected = "vertex 5 is not between 0 and 4")]
    fn test_invalid_vertex() {
        let mut graph = EdgeWeightedGraph::new(5);
        graph.add_edge(Edge::new(0, 5, 0.5));
    }

    #[test]
    fn test_display() {
        let mut graph = EdgeWeightedGraph::new(3);
        graph.add_edge(Edge::new(0, 1, 0.5));
        graph.add_edge(Edge::new(1, 2, 0.3));

        let output = format!("{}", graph);
        assert!(output.contains("3 vertices, 2 edges"));
    }
}
