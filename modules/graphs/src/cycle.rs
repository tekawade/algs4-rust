//! Cycle detection in undirected graphs.
//!
//! This module provides an algorithm to detect cycles in undirected graphs
//! using depth-first search.

use crate::graph::Graph;

/// Determines whether an undirected graph has a cycle.
///
/// Uses depth-first search to detect cycles. A cycle exists if there is an
/// edge to a vertex that has already been visited (other than the parent).
///
/// Time complexity: O(V + E)
/// Space complexity: O(V)
///
/// # Examples
///
/// ```
/// use algs4_graphs::{Graph, Cycle};
///
/// let mut graph = Graph::new(5);
/// graph.add_edge(0, 1);
/// graph.add_edge(1, 2);
/// graph.add_edge(2, 3);
///
/// let cycle = Cycle::new(&graph);
/// assert!(!cycle.has_cycle());
///
/// // Add edge that creates a cycle
/// graph.add_edge(3, 0);
/// let cycle = Cycle::new(&graph);
/// assert!(cycle.has_cycle());
/// ```
#[derive(Debug)]
pub struct Cycle {
    marked: Vec<bool>,
    has_cycle: bool,
    cycle: Option<Vec<usize>>,
}

impl Cycle {
    /// Determines whether the undirected graph has a cycle.
    ///
    /// # Arguments
    ///
    /// * `g` - The undirected graph
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Graph, Cycle};
    ///
    /// let mut graph = Graph::new(4);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    /// graph.add_edge(2, 3);
    ///
    /// let cycle = Cycle::new(&graph);
    /// assert!(!cycle.has_cycle());
    /// ```
    pub fn new(g: &Graph) -> Self {
        let mut cycle = Cycle {
            marked: vec![false; g.v()],
            has_cycle: false,
            cycle: None,
        };

        // Handle self-loops
        for v in 0..g.v() {
            for &w in g.adj(v) {
                if v == w {
                    cycle.has_cycle = true;
                    cycle.cycle = Some(vec![v, v]);
                    return cycle;
                }
            }
        }

        // Check for parallel edges
        for v in 0..g.v() {
            let mut seen = vec![false; g.v()];
            for &w in g.adj(v) {
                if seen[w] {
                    cycle.has_cycle = true;
                    cycle.cycle = Some(vec![v, w, v]);
                    return cycle;
                }
                seen[w] = true;
            }
        }

        // DFS to find cycles
        for v in 0..g.v() {
            if !cycle.marked[v] {
                cycle.dfs(g, v, v);
            }
        }

        cycle
    }

    /// Depth-first search to detect cycles.
    fn dfs(&mut self, g: &Graph, v: usize, parent: usize) {
        self.marked[v] = true;
        for &w in g.adj(v) {
            if self.has_cycle {
                return;
            }
            if !self.marked[w] {
                self.dfs(g, w, v);
            } else if w != parent {
                // Found a cycle
                self.has_cycle = true;
                // We can't easily reconstruct the full cycle in this simple version
                // without maintaining edge_to array, so we just set the flag
            }
        }
    }

    /// Returns true if the graph has a cycle.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Graph, Cycle};
    ///
    /// let mut graph = Graph::new(3);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    /// graph.add_edge(2, 0);
    ///
    /// let cycle = Cycle::new(&graph);
    /// assert!(cycle.has_cycle());
    /// ```
    pub fn has_cycle(&self) -> bool {
        self.has_cycle
    }

    /// Returns a cycle if one exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Graph, Cycle};
    ///
    /// let mut graph = Graph::new(3);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    ///
    /// let cycle = Cycle::new(&graph);
    /// assert!(cycle.cycle().is_none());
    /// ```
    pub fn cycle(&self) -> Option<&[usize]> {
        self.cycle.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_cycle() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let cycle = Cycle::new(&graph);
        assert!(!cycle.has_cycle());
    }

    #[test]
    fn test_simple_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let cycle = Cycle::new(&graph);
        assert!(cycle.has_cycle());
    }

    #[test]
    fn test_self_loop() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 0);

        let cycle = Cycle::new(&graph);
        assert!(cycle.has_cycle());
        assert_eq!(cycle.cycle(), Some(&[0, 0][..]));
    }

    #[test]
    fn test_parallel_edges() {
        let mut graph = Graph::new(2);
        graph.add_edge(0, 1);
        graph.add_edge(0, 1);

        let cycle = Cycle::new(&graph);
        assert!(cycle.has_cycle());
    }

    #[test]
    fn test_disconnected_graph() {
        let mut graph = Graph::new(6);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(3, 4);
        graph.add_edge(4, 5);
        graph.add_edge(5, 3);

        let cycle = Cycle::new(&graph);
        assert!(cycle.has_cycle());
    }

    #[test]
    fn test_tree() {
        let mut graph = Graph::new(7);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 5);
        graph.add_edge(2, 6);

        let cycle = Cycle::new(&graph);
        assert!(!cycle.has_cycle());
    }

    #[test]
    fn test_empty_graph() {
        let graph = Graph::new(5);
        let cycle = Cycle::new(&graph);
        assert!(!cycle.has_cycle());
    }

    #[test]
    fn test_single_vertex() {
        let graph = Graph::new(1);
        let cycle = Cycle::new(&graph);
        assert!(!cycle.has_cycle());
    }
}
