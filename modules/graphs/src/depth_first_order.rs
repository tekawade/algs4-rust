//! Depth-first search vertex orderings for directed graphs.
//!
//! This module computes the pre-order, post-order, and reverse post-order
//! of vertices in a directed graph using depth-first search.

use crate::digraph::Digraph;
use std::collections::VecDeque;

/// Computes depth-first search orderings for a directed graph.
///
/// The preorder is the order in which vertices are first visited.
/// The postorder is the order in which vertices are last visited (after all adjacent vertices).
/// The reverse postorder is the reverse of the postorder (useful for topological sort).
///
/// Time complexity: O(V + E)
/// Space complexity: O(V)
///
/// # Examples
///
/// ```
/// use algs4_graphs::{Digraph, DepthFirstOrder};
///
/// let mut digraph = Digraph::new(6);
/// digraph.add_edge(0, 1);
/// digraph.add_edge(0, 2);
/// digraph.add_edge(1, 3);
/// digraph.add_edge(2, 3);
/// digraph.add_edge(3, 4);
/// digraph.add_edge(3, 5);
///
/// let dfo = DepthFirstOrder::new(&digraph);
/// let rpo: Vec<usize> = dfo.reverse_post().iter().copied().collect();
/// // Reverse postorder gives a topological ordering for DAGs
/// ```
#[derive(Debug)]
pub struct DepthFirstOrder {
    marked: Vec<bool>,          // marked[v] = has v been marked in dfs?
    pre: Vec<usize>,            // pre[v] = preorder number of v
    post: Vec<usize>,           // post[v] = postorder number of v
    preorder: VecDeque<usize>,  // vertices in preorder
    postorder: VecDeque<usize>, // vertices in postorder
    pre_counter: usize,         // counter for preorder numbering
    post_counter: usize,        // counter for postorder numbering
}

impl DepthFirstOrder {
    /// Computes the depth-first search orderings for the directed graph.
    ///
    /// # Arguments
    ///
    /// * `g` - The directed graph
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Digraph, DepthFirstOrder};
    ///
    /// let mut digraph = Digraph::new(5);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    /// digraph.add_edge(2, 3);
    /// digraph.add_edge(3, 4);
    ///
    /// let dfo = DepthFirstOrder::new(&digraph);
    /// assert_eq!(dfo.pre(0), 0); // First vertex visited
    /// ```
    pub fn new(g: &Digraph) -> Self {
        let mut dfo = DepthFirstOrder {
            marked: vec![false; g.v()],
            pre: vec![0; g.v()],
            post: vec![0; g.v()],
            preorder: VecDeque::new(),
            postorder: VecDeque::new(),
            pre_counter: 0,
            post_counter: 0,
        };

        for v in 0..g.v() {
            if !dfo.marked[v] {
                dfo.dfs(g, v);
            }
        }

        dfo
    }

    /// Depth-first search.
    fn dfs(&mut self, g: &Digraph, v: usize) {
        self.marked[v] = true;
        self.pre[v] = self.pre_counter;
        self.pre_counter += 1;
        self.preorder.push_back(v);

        for &w in g.adj(v) {
            if !self.marked[w] {
                self.dfs(g, w);
            }
        }

        self.postorder.push_back(v);
        self.post[v] = self.post_counter;
        self.post_counter += 1;
    }

    /// Validates that vertex v is a valid vertex.
    fn validate_vertex(&self, v: usize) {
        let n = self.marked.len();
        if v >= n {
            panic!("vertex {} is not between 0 and {}", v, n - 1);
        }
    }

    /// Returns the preorder number of vertex v.
    ///
    /// # Arguments
    ///
    /// * `v` - The vertex
    ///
    /// # Returns
    ///
    /// The preorder number of vertex v
    ///
    /// # Panics
    ///
    /// Panics if v is not a valid vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Digraph, DepthFirstOrder};
    ///
    /// let mut digraph = Digraph::new(3);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    ///
    /// let dfo = DepthFirstOrder::new(&digraph);
    /// assert_eq!(dfo.pre(0), 0);
    /// ```
    pub fn pre(&self, v: usize) -> usize {
        self.validate_vertex(v);
        self.pre[v]
    }

    /// Returns the postorder number of vertex v.
    ///
    /// # Arguments
    ///
    /// * `v` - The vertex
    ///
    /// # Returns
    ///
    /// The postorder number of vertex v
    ///
    /// # Panics
    ///
    /// Panics if v is not a valid vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Digraph, DepthFirstOrder};
    ///
    /// let mut digraph = Digraph::new(3);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    ///
    /// let dfo = DepthFirstOrder::new(&digraph);
    /// assert_eq!(dfo.post(2), 0);
    /// ```
    pub fn post(&self, v: usize) -> usize {
        self.validate_vertex(v);
        self.post[v]
    }

    /// Returns the vertices in preorder.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Digraph, DepthFirstOrder};
    ///
    /// let mut digraph = Digraph::new(3);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    ///
    /// let dfo = DepthFirstOrder::new(&digraph);
    /// let pre: Vec<usize> = dfo.preorder().iter().copied().collect();
    /// assert_eq!(pre, vec![0, 1, 2]);
    /// ```
    pub fn preorder(&self) -> &VecDeque<usize> {
        &self.preorder
    }

    /// Returns the vertices in postorder.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Digraph, DepthFirstOrder};
    ///
    /// let mut digraph = Digraph::new(3);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    ///
    /// let dfo = DepthFirstOrder::new(&digraph);
    /// let post: Vec<usize> = dfo.postorder().iter().copied().collect();
    /// assert_eq!(post, vec![2, 1, 0]);
    /// ```
    pub fn postorder(&self) -> &VecDeque<usize> {
        &self.postorder
    }

    /// Returns the vertices in reverse postorder.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Digraph, DepthFirstOrder};
    ///
    /// let mut digraph = Digraph::new(3);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    ///
    /// let dfo = DepthFirstOrder::new(&digraph);
    /// let rpo: Vec<usize> = dfo.reverse_post().iter().copied().collect();
    /// assert_eq!(rpo, vec![0, 1, 2]);
    /// ```
    pub fn reverse_post(&self) -> Vec<usize> {
        self.postorder.iter().copied().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_dag() {
        let mut digraph = Digraph::new(6);
        digraph.add_edge(0, 1);
        digraph.add_edge(0, 2);
        digraph.add_edge(1, 3);
        digraph.add_edge(2, 3);
        digraph.add_edge(3, 4);
        digraph.add_edge(3, 5);

        let dfo = DepthFirstOrder::new(&digraph);

        // Verify preorder
        assert_eq!(dfo.pre(0), 0);

        // Verify that all vertices are visited
        assert_eq!(dfo.preorder().len(), 6);
        assert_eq!(dfo.postorder().len(), 6);
    }

    #[test]
    fn test_linear_dag() {
        let mut digraph = Digraph::new(4);
        digraph.add_edge(0, 1);
        digraph.add_edge(1, 2);
        digraph.add_edge(2, 3);

        let dfo = DepthFirstOrder::new(&digraph);
        let pre: Vec<usize> = dfo.preorder().iter().copied().collect();
        let post: Vec<usize> = dfo.postorder().iter().copied().collect();
        let rpo = dfo.reverse_post();

        assert_eq!(pre, vec![0, 1, 2, 3]);
        assert_eq!(post, vec![3, 2, 1, 0]);
        assert_eq!(rpo, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_disconnected_graph() {
        let mut digraph = Digraph::new(5);
        digraph.add_edge(0, 1);
        digraph.add_edge(2, 3);

        let dfo = DepthFirstOrder::new(&digraph);

        // All vertices should be visited
        assert_eq!(dfo.preorder().len(), 5);
        assert_eq!(dfo.postorder().len(), 5);
    }

    #[test]
    fn test_empty_graph() {
        let digraph = Digraph::new(3);
        let dfo = DepthFirstOrder::new(&digraph);

        assert_eq!(dfo.preorder().len(), 3);
        assert_eq!(dfo.postorder().len(), 3);
    }

    #[test]
    #[should_panic(expected = "vertex 6 is not between 0 and 5")]
    fn test_invalid_vertex() {
        let digraph = Digraph::new(6);
        let dfo = DepthFirstOrder::new(&digraph);
        dfo.pre(6);
    }
}
