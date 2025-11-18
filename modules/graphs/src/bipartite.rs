//! Bipartite detection and two-coloring for undirected graphs.
//!
//! This module determines whether an undirected graph is bipartite and,
//! if so, finds a bipartition (two-coloring) using depth-first search.

use crate::graph::Graph;

/// Determines whether an undirected graph is bipartite.
///
/// A graph is bipartite if its vertices can be divided into two disjoint sets
/// such that every edge connects vertices from different sets. This is equivalent
/// to being two-colorable.
///
/// Uses depth-first search to attempt a two-coloring. If an odd-length cycle
/// is found, the graph is not bipartite.
///
/// Time complexity: O(V + E)
/// Space complexity: O(V)
///
/// # Examples
///
/// ```
/// use algs4_graphs::{Graph, Bipartite};
///
/// let mut graph = Graph::new(4);
/// graph.add_edge(0, 1);
/// graph.add_edge(1, 2);
/// graph.add_edge(2, 3);
/// graph.add_edge(3, 0);
///
/// let bipartite = Bipartite::new(&graph);
/// assert!(bipartite.is_bipartite());
/// ```
#[derive(Debug)]
pub struct Bipartite {
    is_bipartite: bool,        // is the graph bipartite?
    color: Vec<bool>,          // color[v] gives color of vertex v
    marked: Vec<bool>,         // marked[v] = true if v has been visited
    edge_to: Vec<usize>,       // edge_to[v] = last edge on path to v
    cycle: Option<Vec<usize>>, // odd-length cycle
}

impl Bipartite {
    /// Determines whether the undirected graph is bipartite.
    ///
    /// # Arguments
    ///
    /// * `g` - The undirected graph
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Graph, Bipartite};
    ///
    /// let mut graph = Graph::new(3);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    ///
    /// let bipartite = Bipartite::new(&graph);
    /// assert!(bipartite.is_bipartite());
    /// ```
    pub fn new(g: &Graph) -> Self {
        let mut bipartite = Bipartite {
            is_bipartite: true,
            color: vec![false; g.v()],
            marked: vec![false; g.v()],
            edge_to: vec![0; g.v()],
            cycle: None,
        };

        for v in 0..g.v() {
            if !bipartite.marked[v] {
                bipartite.dfs(g, v);
            }
        }

        bipartite
    }

    /// Depth-first search to two-color the graph.
    fn dfs(&mut self, g: &Graph, v: usize) {
        self.marked[v] = true;
        for &w in g.adj(v) {
            if self.cycle.is_some() {
                return;
            }

            if !self.marked[w] {
                self.edge_to[w] = v;
                self.color[w] = !self.color[v];
                self.dfs(g, w);
            } else if self.color[w] == self.color[v] {
                // Found odd-length cycle
                self.is_bipartite = false;
                let mut cycle = Vec::new();
                let mut x = v;
                while x != w {
                    cycle.push(x);
                    x = self.edge_to[x];
                }
                cycle.push(w);
                cycle.push(v);
                self.cycle = Some(cycle);
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

    /// Returns true if the graph is bipartite.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Graph, Bipartite};
    ///
    /// // Bipartite graph (tree)
    /// let mut graph1 = Graph::new(4);
    /// graph1.add_edge(0, 1);
    /// graph1.add_edge(1, 2);
    /// graph1.add_edge(2, 3);
    ///
    /// let bipartite1 = Bipartite::new(&graph1);
    /// assert!(bipartite1.is_bipartite());
    ///
    /// // Not bipartite (triangle)
    /// let mut graph2 = Graph::new(3);
    /// graph2.add_edge(0, 1);
    /// graph2.add_edge(1, 2);
    /// graph2.add_edge(2, 0);
    ///
    /// let bipartite2 = Bipartite::new(&graph2);
    /// assert!(!bipartite2.is_bipartite());
    /// ```
    pub fn is_bipartite(&self) -> bool {
        self.is_bipartite
    }

    /// Returns the color of vertex v (one side of bipartition).
    ///
    /// # Arguments
    ///
    /// * `v` - The vertex
    ///
    /// # Returns
    ///
    /// The color of vertex v (true or false)
    ///
    /// # Panics
    ///
    /// Panics if the graph is not bipartite or if v is not valid.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Graph, Bipartite};
    ///
    /// let mut graph = Graph::new(4);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    /// graph.add_edge(2, 3);
    ///
    /// let bipartite = Bipartite::new(&graph);
    /// assert_ne!(bipartite.color(0), bipartite.color(1));
    /// assert_eq!(bipartite.color(0), bipartite.color(2));
    /// ```
    pub fn color(&self, v: usize) -> bool {
        self.validate_vertex(v);
        if !self.is_bipartite {
            panic!("graph is not bipartite");
        }
        self.color[v]
    }

    /// Returns an odd-length cycle if the graph is not bipartite.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Graph, Bipartite};
    ///
    /// let mut graph = Graph::new(3);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    /// graph.add_edge(2, 0);
    ///
    /// let bipartite = Bipartite::new(&graph);
    /// assert!(!bipartite.is_bipartite());
    /// assert!(bipartite.odd_cycle().is_some());
    /// ```
    pub fn odd_cycle(&self) -> Option<&[usize]> {
        self.cycle.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bipartite_tree() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);

        let bipartite = Bipartite::new(&graph);
        assert!(bipartite.is_bipartite());
        assert_ne!(bipartite.color(0), bipartite.color(1));
        assert_eq!(bipartite.color(0), bipartite.color(2));
    }

    #[test]
    fn test_bipartite_square() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 0);

        let bipartite = Bipartite::new(&graph);
        assert!(bipartite.is_bipartite());
    }

    #[test]
    fn test_not_bipartite_triangle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let bipartite = Bipartite::new(&graph);
        assert!(!bipartite.is_bipartite());
        assert!(bipartite.odd_cycle().is_some());
    }

    #[test]
    fn test_not_bipartite_pentagon() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);
        graph.add_edge(4, 0);

        let bipartite = Bipartite::new(&graph);
        assert!(!bipartite.is_bipartite());
    }

    #[test]
    fn test_empty_graph() {
        let graph = Graph::new(5);
        let bipartite = Bipartite::new(&graph);
        assert!(bipartite.is_bipartite());
    }

    #[test]
    fn test_disconnected_bipartite() {
        let mut graph = Graph::new(6);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(3, 4);
        graph.add_edge(4, 5);

        let bipartite = Bipartite::new(&graph);
        assert!(bipartite.is_bipartite());
    }

    #[test]
    #[should_panic(expected = "graph is not bipartite")]
    fn test_color_on_non_bipartite() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let bipartite = Bipartite::new(&graph);
        bipartite.color(0);
    }
}
