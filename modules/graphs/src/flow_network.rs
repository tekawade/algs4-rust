//! Flow network data structure.
//!
//! This module provides a flow network for maximum flow algorithms.

use crate::flow_edge::FlowEdge;
use std::fmt;

/// A flow network.
///
/// A flow network is a directed graph where each edge has a capacity and flow.
/// Supports adding flow edges and iterating over edges incident to a vertex.
///
/// # Examples
///
/// ```
/// use algs4_graphs::{FlowNetwork, FlowEdge};
///
/// let mut network = FlowNetwork::new(4);
/// network.add_edge(FlowEdge::new(0, 1, 10.0));
/// network.add_edge(FlowEdge::new(1, 2, 5.0));
/// network.add_edge(FlowEdge::new(2, 3, 8.0));
///
/// assert_eq!(network.v(), 4);
/// assert_eq!(network.e(), 3);
/// ```
#[derive(Debug, Clone)]
pub struct FlowNetwork {
    v: usize,                // number of vertices
    e: usize,                // number of edges
    adj: Vec<Vec<FlowEdge>>, // adjacency lists (includes both forward and backward)
}

impl FlowNetwork {
    /// Creates a new flow network with `v` vertices and no edges.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::FlowNetwork;
    ///
    /// let network = FlowNetwork::new(10);
    /// assert_eq!(network.v(), 10);
    /// assert_eq!(network.e(), 0);
    /// ```
    pub fn new(v: usize) -> Self {
        FlowNetwork {
            v,
            e: 0,
            adj: vec![Vec::new(); v],
        }
    }

    /// Returns the number of vertices in the network.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::FlowNetwork;
    ///
    /// let network = FlowNetwork::new(5);
    /// assert_eq!(network.v(), 5);
    /// ```
    pub fn v(&self) -> usize {
        self.v
    }

    /// Returns the number of edges in the network.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{FlowNetwork, FlowEdge};
    ///
    /// let mut network = FlowNetwork::new(5);
    /// network.add_edge(FlowEdge::new(0, 1, 10.0));
    /// assert_eq!(network.e(), 1);
    /// ```
    pub fn e(&self) -> usize {
        self.e
    }

    /// Validates that vertex v is a valid vertex in the network.
    fn validate_vertex(&self, v: usize) {
        if v >= self.v {
            panic!("vertex {} is not between 0 and {}", v, self.v - 1);
        }
    }

    /// Adds a flow edge to the network.
    ///
    /// The edge is added to the adjacency lists of both endpoints.
    ///
    /// # Arguments
    ///
    /// * `e` - The flow edge to add
    ///
    /// # Panics
    ///
    /// Panics if either vertex is not between 0 and V-1.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{FlowNetwork, FlowEdge};
    ///
    /// let mut network = FlowNetwork::new(3);
    /// network.add_edge(FlowEdge::new(0, 1, 10.0));
    /// network.add_edge(FlowEdge::new(1, 2, 5.0));
    /// assert_eq!(network.e(), 2);
    /// ```
    pub fn add_edge(&mut self, e: FlowEdge) {
        let v = e.from();
        let w = e.to();
        self.validate_vertex(v);
        self.validate_vertex(w);
        self.adj[v].push(e);
        self.adj[w].push(e);
        self.e += 1;
    }

    /// Returns the flow edges incident to vertex `v`.
    ///
    /// This includes both edges leaving v and edges entering v.
    ///
    /// # Arguments
    ///
    /// * `v` - The vertex
    ///
    /// # Returns
    ///
    /// A slice containing the flow edges incident to vertex `v`
    ///
    /// # Panics
    ///
    /// Panics if vertex is not between 0 and V-1.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{FlowNetwork, FlowEdge};
    ///
    /// let mut network = FlowNetwork::new(4);
    /// network.add_edge(FlowEdge::new(0, 1, 10.0));
    /// network.add_edge(FlowEdge::new(0, 2, 5.0));
    ///
    /// let edges = network.adj(0);
    /// assert_eq!(edges.len(), 2);
    /// ```
    pub fn adj(&self, v: usize) -> &[FlowEdge] {
        self.validate_vertex(v);
        &self.adj[v]
    }

    /// Returns a mutable reference to the flow edges incident to vertex `v`.
    ///
    /// # Arguments
    ///
    /// * `v` - The vertex
    ///
    /// # Panics
    ///
    /// Panics if vertex is not between 0 and V-1.
    pub fn adj_mut(&mut self, v: usize) -> &mut [FlowEdge] {
        self.validate_vertex(v);
        &mut self.adj[v]
    }

    /// Returns all flow edges in the network.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{FlowNetwork, FlowEdge};
    ///
    /// let mut network = FlowNetwork::new(4);
    /// network.add_edge(FlowEdge::new(0, 1, 10.0));
    /// network.add_edge(FlowEdge::new(1, 2, 5.0));
    ///
    /// let edges = network.edges();
    /// assert_eq!(edges.len(), 2);
    /// ```
    pub fn edges(&self) -> Vec<FlowEdge> {
        let mut edges = Vec::new();
        for v in 0..self.v {
            for &e in &self.adj[v] {
                if e.from() == v {
                    edges.push(e);
                }
            }
        }
        edges
    }
}

impl fmt::Display for FlowNetwork {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} vertices, {} edges", self.v, self.e)?;
        for v in 0..self.v {
            write!(f, "{}: ", v)?;
            for e in &self.adj[v] {
                if e.from() == v {
                    write!(f, "{} ", e)?;
                }
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
    fn test_new_network() {
        let network = FlowNetwork::new(5);
        assert_eq!(network.v(), 5);
        assert_eq!(network.e(), 0);
    }

    #[test]
    fn test_add_edge() {
        let mut network = FlowNetwork::new(5);
        network.add_edge(FlowEdge::new(0, 1, 10.0));
        assert_eq!(network.e(), 1);
    }

    #[test]
    fn test_adjacency() {
        let mut network = FlowNetwork::new(4);
        network.add_edge(FlowEdge::new(0, 1, 10.0));
        network.add_edge(FlowEdge::new(0, 2, 5.0));

        let edges = network.adj(0);
        assert_eq!(edges.len(), 2);
    }

    #[test]
    fn test_edges() {
        let mut network = FlowNetwork::new(4);
        network.add_edge(FlowEdge::new(0, 1, 10.0));
        network.add_edge(FlowEdge::new(1, 2, 5.0));
        network.add_edge(FlowEdge::new(2, 3, 8.0));

        let edges = network.edges();
        assert_eq!(edges.len(), 3);
    }

    #[test]
    #[should_panic(expected = "vertex 5 is not between 0 and 4")]
    fn test_invalid_vertex() {
        let mut network = FlowNetwork::new(5);
        network.add_edge(FlowEdge::new(0, 5, 10.0));
    }

    #[test]
    fn test_display() {
        let mut network = FlowNetwork::new(3);
        network.add_edge(FlowEdge::new(0, 1, 10.0));
        network.add_edge(FlowEdge::new(1, 2, 5.0));

        let output = format!("{}", network);
        assert!(output.contains("3 vertices, 2 edges"));
    }
}
