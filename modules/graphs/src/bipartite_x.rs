//! Non-recursive bipartite detection for undirected graphs.
//!
//! This module determines whether an undirected graph is bipartite using
//! breadth-first search instead of recursion.

use crate::graph::Graph;
use std::collections::VecDeque;

/// Determines whether an undirected graph is bipartite using non-recursive BFS.
///
/// Uses breadth-first search to attempt a two-coloring. This is more robust
/// than the recursive DFS version for very large graphs.
///
/// Time complexity: O(V + E)
/// Space complexity: O(V)
///
/// # Examples
///
/// ```
/// use algs4_graphs::{Graph, BipartiteX};
///
/// let mut graph = Graph::new(4);
/// graph.add_edge(0, 1);
/// graph.add_edge(1, 2);
/// graph.add_edge(2, 3);
/// graph.add_edge(3, 0);
///
/// let bipartite = BipartiteX::new(&graph);
/// assert!(bipartite.is_bipartite());
/// ```
#[derive(Debug)]
pub struct BipartiteX {
    is_bipartite: bool,        // is the graph bipartite?
    color: Vec<bool>,          // color[v] gives color of vertex v
    marked: Vec<bool>,         // marked[v] = true if v has been visited
    edge_to: Vec<usize>,       // edge_to[v] = last edge on path to v
    cycle: Option<Vec<usize>>, // odd-length cycle
}

impl BipartiteX {
    /// Determines whether the undirected graph is bipartite using BFS.
    ///
    /// # Arguments
    ///
    /// * `g` - The undirected graph
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Graph, BipartiteX};
    ///
    /// let mut graph = Graph::new(3);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    ///
    /// let bipartite = BipartiteX::new(&graph);
    /// assert!(bipartite.is_bipartite());
    /// ```
    pub fn new(g: &Graph) -> Self {
        let mut bipartite = BipartiteX {
            is_bipartite: true,
            color: vec![false; g.v()],
            marked: vec![false; g.v()],
            edge_to: vec![0; g.v()],
            cycle: None,
        };

        for v in 0..g.v() {
            if !bipartite.marked[v] {
                bipartite.bfs(g, v);
            }
        }

        bipartite
    }

    /// Breadth-first search to two-color the graph.
    fn bfs(&mut self, g: &Graph, s: usize) {
        let mut queue = VecDeque::new();
        self.color[s] = false;
        self.marked[s] = true;
        queue.push_back(s);

        while let Some(v) = queue.pop_front() {
            for &w in g.adj(v) {
                if !self.marked[w] {
                    self.marked[w] = true;
                    self.edge_to[w] = v;
                    self.color[w] = !self.color[v];
                    queue.push_back(w);
                } else if self.color[w] == self.color[v] {
                    // Found odd-length cycle
                    self.is_bipartite = false;

                    // Trace back the cycle
                    let mut cycle = Vec::new();
                    let mut x = v;
                    let mut y = w;

                    // Find common ancestor
                    let mut stack1 = vec![x];
                    let mut stack2 = vec![y];

                    while x != y {
                        if x != s {
                            x = self.edge_to[x];
                            stack1.push(x);
                        }
                        if y != s {
                            y = self.edge_to[y];
                            stack2.push(y);
                        }

                        // Handle case where paths meet at different points
                        if stack1.len() > stack2.len() && x == s {
                            break;
                        }
                        if stack2.len() > stack1.len() && y == s {
                            break;
                        }
                    }

                    // Build cycle
                    for &vertex in stack1.iter().rev() {
                        cycle.push(vertex);
                        if vertex == x {
                            break;
                        }
                    }
                    for i in 0..stack2.len() {
                        if stack2[i] == x {
                            for j in (0..=i).rev() {
                                if j < i {
                                    cycle.push(stack2[j]);
                                }
                            }
                            break;
                        }
                    }

                    self.cycle = Some(cycle);
                    return;
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

    /// Returns true if the graph is bipartite.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Graph, BipartiteX};
    ///
    /// let mut graph = Graph::new(3);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    ///
    /// let bipartite = BipartiteX::new(&graph);
    /// assert!(bipartite.is_bipartite());
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
    /// # Panics
    ///
    /// Panics if the graph is not bipartite or if v is not valid.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Graph, BipartiteX};
    ///
    /// let mut graph = Graph::new(4);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    /// graph.add_edge(2, 3);
    ///
    /// let bipartite = BipartiteX::new(&graph);
    /// assert_ne!(bipartite.color(0), bipartite.color(1));
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
    /// use algs4_graphs::{Graph, BipartiteX};
    ///
    /// let mut graph = Graph::new(3);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    /// graph.add_edge(2, 0);
    ///
    /// let bipartite = BipartiteX::new(&graph);
    /// assert!(!bipartite.is_bipartite());
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

        let bipartite = BipartiteX::new(&graph);
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

        let bipartite = BipartiteX::new(&graph);
        assert!(bipartite.is_bipartite());
    }

    #[test]
    fn test_not_bipartite_triangle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let bipartite = BipartiteX::new(&graph);
        assert!(!bipartite.is_bipartite());
    }

    #[test]
    fn test_not_bipartite_pentagon() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);
        graph.add_edge(4, 0);

        let bipartite = BipartiteX::new(&graph);
        assert!(!bipartite.is_bipartite());
    }

    #[test]
    fn test_empty_graph() {
        let graph = Graph::new(5);
        let bipartite = BipartiteX::new(&graph);
        assert!(bipartite.is_bipartite());
    }

    #[test]
    fn test_disconnected_bipartite() {
        let mut graph = Graph::new(6);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(3, 4);
        graph.add_edge(4, 5);

        let bipartite = BipartiteX::new(&graph);
        assert!(bipartite.is_bipartite());
    }
}
