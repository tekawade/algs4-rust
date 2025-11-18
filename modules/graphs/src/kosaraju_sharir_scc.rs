//! Kosaraju-Sharir algorithm for computing strongly connected components.
//!
//! This module implements the Kosaraju-Sharir algorithm using two-pass DFS.

use crate::depth_first_order::DepthFirstOrder;
use crate::digraph::Digraph;

/// Computes the strongly connected components of a directed graph using
/// the Kosaraju-Sharir algorithm.
///
/// The algorithm uses two depth-first searches:
/// 1. Compute reverse postorder of the reverse graph
/// 2. Run DFS on original graph in reverse postorder
///
/// Time complexity: O(V + E)
/// Space complexity: O(V)
///
/// # Examples
///
/// ```
/// use algs4_graphs::{Digraph, KosarajuSharirSCC};
///
/// let mut digraph = Digraph::new(13);
/// digraph.add_edge(0, 1);
/// digraph.add_edge(0, 5);
/// digraph.add_edge(2, 0);
/// digraph.add_edge(2, 3);
/// digraph.add_edge(3, 2);
/// digraph.add_edge(3, 5);
/// digraph.add_edge(4, 2);
/// digraph.add_edge(4, 3);
/// digraph.add_edge(5, 4);
/// digraph.add_edge(6, 0);
/// digraph.add_edge(6, 4);
/// digraph.add_edge(6, 9);
/// digraph.add_edge(7, 6);
/// digraph.add_edge(7, 8);
/// digraph.add_edge(8, 7);
/// digraph.add_edge(8, 9);
/// digraph.add_edge(9, 10);
/// digraph.add_edge(9, 11);
/// digraph.add_edge(10, 12);
/// digraph.add_edge(11, 4);
/// digraph.add_edge(11, 12);
/// digraph.add_edge(12, 9);
///
/// let scc = KosarajuSharirSCC::new(&digraph);
/// assert_eq!(scc.count(), 5);
/// ```
#[derive(Debug)]
pub struct KosarajuSharirSCC {
    marked: Vec<bool>, // marked[v] = has vertex v been marked?
    id: Vec<usize>,    // id[v] = id of strong component containing v
    count: usize,      // number of strongly-connected components
}

impl KosarajuSharirSCC {
    /// Computes the strongly connected components of the directed graph.
    ///
    /// # Arguments
    ///
    /// * `g` - The directed graph
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Digraph, KosarajuSharirSCC};
    ///
    /// let mut digraph = Digraph::new(5);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    /// digraph.add_edge(2, 0);
    /// digraph.add_edge(3, 4);
    ///
    /// let scc = KosarajuSharirSCC::new(&digraph);
    /// assert_eq!(scc.count(), 3); // {0,1,2}, {3}, {4}
    /// ```
    pub fn new(g: &Digraph) -> Self {
        let mut scc = KosarajuSharirSCC {
            marked: vec![false; g.v()],
            id: vec![0; g.v()],
            count: 0,
        };

        // Compute reverse postorder of reverse graph
        let dfs = DepthFirstOrder::new(&g.reverse());

        // Run DFS on G in reverse postorder to find SCCs
        for &v in &dfs.reverse_post() {
            if !scc.marked[v] {
                scc.dfs(g, v);
                scc.count += 1;
            }
        }

        scc
    }

    /// Depth-first search.
    fn dfs(&mut self, g: &Digraph, v: usize) {
        self.marked[v] = true;
        self.id[v] = self.count;
        for &w in g.adj(v) {
            if !self.marked[w] {
                self.dfs(g, w);
            }
        }
    }

    /// Validates that vertex v is a valid vertex.
    fn validate_vertex(&self, v: usize) {
        let n = self.marked.len();
        if v >= n {
            panic!("vertex {} is not between 0 and {}", v, n - 1);
        }
    }

    /// Returns the number of strongly connected components.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Digraph, KosarajuSharirSCC};
    ///
    /// let mut digraph = Digraph::new(5);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    /// digraph.add_edge(2, 0);
    ///
    /// let scc = KosarajuSharirSCC::new(&digraph);
    /// assert_eq!(scc.count(), 3); // {0,1,2}, {3}, {4}
    /// ```
    pub fn count(&self) -> usize {
        self.count
    }

    /// Returns the component id of the strongly connected component containing vertex v.
    ///
    /// # Arguments
    ///
    /// * `v` - The vertex
    ///
    /// # Panics
    ///
    /// Panics if v is not a valid vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Digraph, KosarajuSharirSCC};
    ///
    /// let mut digraph = Digraph::new(5);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    /// digraph.add_edge(2, 0);
    ///
    /// let scc = KosarajuSharirSCC::new(&digraph);
    /// assert_eq!(scc.id(0), scc.id(1));
    /// assert_eq!(scc.id(1), scc.id(2));
    /// assert_ne!(scc.id(0), scc.id(3));
    /// ```
    pub fn id(&self, v: usize) -> usize {
        self.validate_vertex(v);
        self.id[v]
    }

    /// Returns true if vertices v and w are in the same strongly connected component.
    ///
    /// # Arguments
    ///
    /// * `v` - One vertex
    /// * `w` - The other vertex
    ///
    /// # Panics
    ///
    /// Panics if either vertex is not valid.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Digraph, KosarajuSharirSCC};
    ///
    /// let mut digraph = Digraph::new(5);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    /// digraph.add_edge(2, 0);
    ///
    /// let scc = KosarajuSharirSCC::new(&digraph);
    /// assert!(scc.strongly_connected(0, 1));
    /// assert!(scc.strongly_connected(1, 2));
    /// assert!(!scc.strongly_connected(0, 3));
    /// ```
    pub fn strongly_connected(&self, v: usize, w: usize) -> bool {
        self.validate_vertex(v);
        self.validate_vertex(w);
        self.id[v] == self.id[w]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_scc() {
        let mut digraph = Digraph::new(5);
        digraph.add_edge(0, 1);
        digraph.add_edge(1, 2);
        digraph.add_edge(2, 0);
        digraph.add_edge(3, 4);

        let scc = KosarajuSharirSCC::new(&digraph);
        assert_eq!(scc.count(), 3);
        assert!(scc.strongly_connected(0, 1));
        assert!(scc.strongly_connected(1, 2));
        assert!(scc.strongly_connected(0, 2));
        assert!(!scc.strongly_connected(0, 3));
        assert!(!scc.strongly_connected(3, 4));
    }

    #[test]
    fn test_single_vertex_components() {
        let mut digraph = Digraph::new(4);
        digraph.add_edge(0, 1);
        digraph.add_edge(1, 2);
        digraph.add_edge(2, 3);

        let scc = KosarajuSharirSCC::new(&digraph);
        assert_eq!(scc.count(), 4);
    }

    #[test]
    fn test_all_one_component() {
        let mut digraph = Digraph::new(3);
        digraph.add_edge(0, 1);
        digraph.add_edge(1, 2);
        digraph.add_edge(2, 0);

        let scc = KosarajuSharirSCC::new(&digraph);
        assert_eq!(scc.count(), 1);
        assert!(scc.strongly_connected(0, 1));
        assert!(scc.strongly_connected(1, 2));
        assert!(scc.strongly_connected(0, 2));
    }

    #[test]
    fn test_empty_graph() {
        let digraph = Digraph::new(5);
        let scc = KosarajuSharirSCC::new(&digraph);
        assert_eq!(scc.count(), 5);
    }

    #[test]
    fn test_complex_scc() {
        let mut digraph = Digraph::new(13);
        digraph.add_edge(0, 1);
        digraph.add_edge(0, 5);
        digraph.add_edge(2, 0);
        digraph.add_edge(2, 3);
        digraph.add_edge(3, 2);
        digraph.add_edge(3, 5);
        digraph.add_edge(4, 2);
        digraph.add_edge(4, 3);
        digraph.add_edge(5, 4);
        digraph.add_edge(6, 0);
        digraph.add_edge(6, 4);
        digraph.add_edge(6, 9);
        digraph.add_edge(7, 6);
        digraph.add_edge(7, 8);
        digraph.add_edge(8, 7);
        digraph.add_edge(8, 9);
        digraph.add_edge(9, 10);
        digraph.add_edge(9, 11);
        digraph.add_edge(10, 12);
        digraph.add_edge(11, 4);
        digraph.add_edge(11, 12);
        digraph.add_edge(12, 9);

        let scc = KosarajuSharirSCC::new(&digraph);
        assert_eq!(scc.count(), 5);
    }

    #[test]
    #[should_panic(expected = "vertex 5 is not between 0 and 4")]
    fn test_invalid_vertex() {
        let digraph = Digraph::new(5);
        let scc = KosarajuSharirSCC::new(&digraph);
        scc.id(5);
    }
}
