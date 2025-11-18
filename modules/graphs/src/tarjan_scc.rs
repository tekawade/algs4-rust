//! Tarjan's algorithm for computing strongly connected components.
//!
//! This module implements Tarjan's algorithm using a single-pass DFS
//! with low-link values.

use crate::digraph::Digraph;

/// Computes the strongly connected components of a directed graph using
/// Tarjan's algorithm.
///
/// The algorithm uses a single depth-first search that maintains:
/// - id[v]: the DFS order/index when vertex v is visited
/// - low[v]: the lowest id reachable from v
///
/// Time complexity: O(V + E)
/// Space complexity: O(V)
///
/// # Examples
///
/// ```
/// use algs4_graphs::{Digraph, TarjanSCC};
///
/// let mut digraph = Digraph::new(5);
/// digraph.add_edge(0, 1);
/// digraph.add_edge(1, 2);
/// digraph.add_edge(2, 0);
/// digraph.add_edge(3, 4);
///
/// let scc = TarjanSCC::new(&digraph);
/// assert_eq!(scc.count(), 3);
/// ```
#[derive(Debug)]
pub struct TarjanSCC {
    marked: Vec<bool>,     // marked[v] = has v been visited?
    id: Vec<usize>,        // id[v] = id of strong component containing v
    low: Vec<usize>,       // low[v] = low number of v
    pre_order: Vec<usize>, // pre_order[v] = preorder of v
    count: usize,          // number of strongly-connected components
    pre: usize,            // preorder counter
    stack: Vec<usize>,     // stack of vertices
}

impl TarjanSCC {
    /// Computes the strongly connected components of the directed graph.
    ///
    /// # Arguments
    ///
    /// * `g` - The directed graph
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Digraph, TarjanSCC};
    ///
    /// let mut digraph = Digraph::new(5);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    /// digraph.add_edge(2, 0);
    ///
    /// let scc = TarjanSCC::new(&digraph);
    /// assert!(scc.strongly_connected(0, 1));
    /// assert!(scc.strongly_connected(1, 2));
    /// ```
    pub fn new(g: &Digraph) -> Self {
        let mut scc = TarjanSCC {
            marked: vec![false; g.v()],
            id: vec![0; g.v()],
            low: vec![0; g.v()],
            pre_order: vec![0; g.v()],
            count: 0,
            pre: 0,
            stack: Vec::new(),
        };

        for v in 0..g.v() {
            if !scc.marked[v] {
                scc.dfs(g, v);
            }
        }

        scc
    }

    /// Depth-first search using Tarjan's algorithm.
    fn dfs(&mut self, g: &Digraph, v: usize) {
        self.marked[v] = true;
        self.low[v] = self.pre;
        self.pre_order[v] = self.pre;
        self.pre += 1;
        self.stack.push(v);

        for &w in g.adj(v) {
            if !self.marked[w] {
                self.dfs(g, w);
                self.low[v] = self.low[v].min(self.low[w]);
            } else if self.stack.contains(&w) {
                // w is on the stack, so it's in the current SCC
                self.low[v] = self.low[v].min(self.pre_order[w]);
            }
        }

        // Found root of an SCC
        if self.low[v] == self.pre_order[v] {
            loop {
                let w = self.stack.pop().unwrap();
                self.id[w] = self.count;
                if w == v {
                    break;
                }
            }
            self.count += 1;
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
    /// use algs4_graphs::{Digraph, TarjanSCC};
    ///
    /// let mut digraph = Digraph::new(5);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    /// digraph.add_edge(2, 0);
    ///
    /// let scc = TarjanSCC::new(&digraph);
    /// assert_eq!(scc.count(), 3);
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
    /// use algs4_graphs::{Digraph, TarjanSCC};
    ///
    /// let mut digraph = Digraph::new(5);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    /// digraph.add_edge(2, 0);
    ///
    /// let scc = TarjanSCC::new(&digraph);
    /// assert_eq!(scc.id(0), scc.id(1));
    /// assert_eq!(scc.id(1), scc.id(2));
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
    /// use algs4_graphs::{Digraph, TarjanSCC};
    ///
    /// let mut digraph = Digraph::new(5);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    /// digraph.add_edge(2, 0);
    ///
    /// let scc = TarjanSCC::new(&digraph);
    /// assert!(scc.strongly_connected(0, 1));
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

        let scc = TarjanSCC::new(&digraph);
        assert_eq!(scc.count(), 3);
        assert!(scc.strongly_connected(0, 1));
        assert!(scc.strongly_connected(1, 2));
        assert!(scc.strongly_connected(0, 2));
        assert!(!scc.strongly_connected(0, 3));
    }

    #[test]
    fn test_single_vertex_components() {
        let mut digraph = Digraph::new(4);
        digraph.add_edge(0, 1);
        digraph.add_edge(1, 2);
        digraph.add_edge(2, 3);

        let scc = TarjanSCC::new(&digraph);
        assert_eq!(scc.count(), 4);
    }

    #[test]
    fn test_all_one_component() {
        let mut digraph = Digraph::new(3);
        digraph.add_edge(0, 1);
        digraph.add_edge(1, 2);
        digraph.add_edge(2, 0);

        let scc = TarjanSCC::new(&digraph);
        assert_eq!(scc.count(), 1);
        assert!(scc.strongly_connected(0, 1));
        assert!(scc.strongly_connected(1, 2));
    }

    #[test]
    fn test_empty_graph() {
        let digraph = Digraph::new(5);
        let scc = TarjanSCC::new(&digraph);
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

        let scc = TarjanSCC::new(&digraph);
        assert_eq!(scc.count(), 5);
    }

    #[test]
    #[should_panic(expected = "vertex 5 is not between 0 and 4")]
    fn test_invalid_vertex() {
        let digraph = Digraph::new(5);
        let scc = TarjanSCC::new(&digraph);
        scc.id(5);
    }
}
