//! Gabow's algorithm for computing strongly connected components.
//!
//! This module implements Gabow's path-based SCC algorithm using a single-pass DFS
//! with two stacks.

use crate::digraph::Digraph;

/// Computes the strongly connected components of a directed graph using
/// Gabow's algorithm.
///
/// The algorithm uses a single depth-first search with two stacks:
/// - path stack: contains vertices in the current DFS path
/// - stack1: contains vertices that might be in the current SCC
///
/// Time complexity: O(V + E)
/// Space complexity: O(V)
///
/// # Examples
///
/// ```
/// use algs4_graphs::{Digraph, GabowSCC};
///
/// let mut digraph = Digraph::new(5);
/// digraph.add_edge(0, 1);
/// digraph.add_edge(1, 2);
/// digraph.add_edge(2, 0);
/// digraph.add_edge(3, 4);
///
/// let scc = GabowSCC::new(&digraph);
/// assert_eq!(scc.count(), 3);
/// ```
#[derive(Debug)]
pub struct GabowSCC {
    marked: Vec<bool>,     // marked[v] = has v been visited?
    id: Vec<usize>,        // id[v] = id of strong component containing v
    pre_order: Vec<usize>, // pre_order[v] = preorder of v
    count: usize,          // number of strongly-connected components
    pre: usize,            // preorder counter
    stack1: Vec<usize>,    // vertices that might be in current SCC
    stack2: Vec<usize>,    // path stack
}

impl GabowSCC {
    /// Computes the strongly connected components of the directed graph.
    ///
    /// # Arguments
    ///
    /// * `g` - The directed graph
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Digraph, GabowSCC};
    ///
    /// let mut digraph = Digraph::new(5);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    /// digraph.add_edge(2, 0);
    ///
    /// let scc = GabowSCC::new(&digraph);
    /// assert!(scc.strongly_connected(0, 1));
    /// assert!(scc.strongly_connected(1, 2));
    /// ```
    pub fn new(g: &Digraph) -> Self {
        let mut scc = GabowSCC {
            marked: vec![false; g.v()],
            id: vec![0; g.v()],
            pre_order: vec![0; g.v()],
            count: 0,
            pre: 0,
            stack1: Vec::new(),
            stack2: Vec::new(),
        };

        for v in 0..g.v() {
            if !scc.marked[v] {
                scc.dfs(g, v);
            }
        }

        scc
    }

    /// Depth-first search using Gabow's algorithm.
    fn dfs(&mut self, g: &Digraph, v: usize) {
        self.marked[v] = true;
        self.pre_order[v] = self.pre;
        self.pre += 1;
        self.stack1.push(v);
        self.stack2.push(v);

        for &w in g.adj(v) {
            if !self.marked[w] {
                self.dfs(g, w);
            } else if self.id[w] == 0 {
                // w is on the stack and not yet assigned to an SCC
                while !self.stack2.is_empty()
                    && self.pre_order[*self.stack2.last().unwrap()] > self.pre_order[w]
                {
                    self.stack2.pop();
                }
            }
        }

        // Found root of an SCC
        if !self.stack2.is_empty() && *self.stack2.last().unwrap() == v {
            self.stack2.pop();
            self.count += 1;
            loop {
                let w = self.stack1.pop().unwrap();
                self.id[w] = self.count;
                if w == v {
                    break;
                }
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
    /// use algs4_graphs::{Digraph, GabowSCC};
    ///
    /// let mut digraph = Digraph::new(5);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    /// digraph.add_edge(2, 0);
    ///
    /// let scc = GabowSCC::new(&digraph);
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
    /// use algs4_graphs::{Digraph, GabowSCC};
    ///
    /// let mut digraph = Digraph::new(5);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    /// digraph.add_edge(2, 0);
    ///
    /// let scc = GabowSCC::new(&digraph);
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
    /// use algs4_graphs::{Digraph, GabowSCC};
    ///
    /// let mut digraph = Digraph::new(5);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    /// digraph.add_edge(2, 0);
    ///
    /// let scc = GabowSCC::new(&digraph);
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

        let scc = GabowSCC::new(&digraph);
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

        let scc = GabowSCC::new(&digraph);
        assert_eq!(scc.count(), 4);
    }

    #[test]
    fn test_all_one_component() {
        let mut digraph = Digraph::new(3);
        digraph.add_edge(0, 1);
        digraph.add_edge(1, 2);
        digraph.add_edge(2, 0);

        let scc = GabowSCC::new(&digraph);
        assert_eq!(scc.count(), 1);
        assert!(scc.strongly_connected(0, 1));
        assert!(scc.strongly_connected(1, 2));
    }

    #[test]
    fn test_empty_graph() {
        let digraph = Digraph::new(5);
        let scc = GabowSCC::new(&digraph);
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

        let scc = GabowSCC::new(&digraph);
        assert_eq!(scc.count(), 5);
    }

    #[test]
    #[should_panic(expected = "vertex 5 is not between 0 and 4")]
    fn test_invalid_vertex() {
        let digraph = Digraph::new(5);
        let scc = GabowSCC::new(&digraph);
        scc.id(5);
    }
}
