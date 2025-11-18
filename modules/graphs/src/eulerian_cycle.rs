//! Eulerian cycle detection in undirected graphs.
//!
//! This module finds an Eulerian cycle in an undirected graph if one exists.
//! An Eulerian cycle is a cycle that uses every edge exactly once.

use crate::graph::Graph;
use std::collections::VecDeque;

/// Finds an Eulerian cycle in an undirected graph.
///
/// An Eulerian cycle exists if and only if every vertex has even degree
/// and all vertices with nonzero degree belong to a single connected component.
///
/// Uses Hierholzer's algorithm.
///
/// Time complexity: O(E)
/// Space complexity: O(E + V)
///
/// # Examples
///
/// ```
/// use algs4_graphs::{Graph, EulerianCycle};
///
/// let mut graph = Graph::new(4);
/// graph.add_edge(0, 1);
/// graph.add_edge(1, 2);
/// graph.add_edge(2, 3);
/// graph.add_edge(3, 0);
///
/// let euler = EulerianCycle::new(&graph);
/// assert!(euler.has_eulerian_cycle());
/// ```
#[derive(Debug)]
pub struct EulerianCycle {
    cycle: Option<Vec<usize>>, // Eulerian cycle (or None if none exists)
}

impl EulerianCycle {
    /// Computes an Eulerian cycle in the undirected graph if one exists.
    ///
    /// # Arguments
    ///
    /// * `g` - The undirected graph
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Graph, EulerianCycle};
    ///
    /// let mut graph = Graph::new(3);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    /// graph.add_edge(2, 0);
    ///
    /// let euler = EulerianCycle::new(&graph);
    /// assert!(euler.has_eulerian_cycle());
    /// ```
    pub fn new(g: &Graph) -> Self {
        // Check if any vertex has odd degree
        for v in 0..g.v() {
            if !g.degree(v).is_multiple_of(2) {
                return EulerianCycle { cycle: None };
            }
        }

        // Check that all edges are in the same connected component
        let non_isolated = Self::non_isolated_vertex(g);
        if non_isolated.is_none() {
            return EulerianCycle {
                cycle: Some(Vec::new()),
            };
        }

        // Use Hierholzer's algorithm to find Eulerian cycle
        let s = non_isolated.unwrap();
        let mut adj = vec![VecDeque::new(); g.v()];
        for (v, adj_list) in adj.iter_mut().enumerate().take(g.v()) {
            for &w in g.adj(v) {
                adj_list.push_back(w);
            }
        }

        let mut stack = vec![s];
        let mut cycle = VecDeque::new();

        while let Some(&v) = stack.last() {
            if adj[v].is_empty() {
                cycle.push_front(v);
                stack.pop();
            } else if let Some(w) = adj[v].pop_front() {
                stack.push(w);
                // Remove reverse edge
                if let Some(pos) = adj[w].iter().position(|&x| x == v) {
                    adj[w].remove(pos);
                }
            }
        }

        // Check if all edges have been used
        if cycle.len() == g.e() + 1 {
            EulerianCycle {
                cycle: Some(cycle.into_iter().collect()),
            }
        } else {
            EulerianCycle { cycle: None }
        }
    }

    /// Returns a vertex with nonzero degree, or None if no such vertex exists.
    fn non_isolated_vertex(g: &Graph) -> Option<usize> {
        (0..g.v()).find(|&v| g.degree(v) > 0)
    }

    /// Returns true if the graph has an Eulerian cycle.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Graph, EulerianCycle};
    ///
    /// let mut graph = Graph::new(4);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    /// graph.add_edge(2, 3);
    /// graph.add_edge(3, 0);
    ///
    /// let euler = EulerianCycle::new(&graph);
    /// assert!(euler.has_eulerian_cycle());
    /// ```
    pub fn has_eulerian_cycle(&self) -> bool {
        self.cycle.is_some()
    }

    /// Returns the Eulerian cycle if one exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Graph, EulerianCycle};
    ///
    /// let mut graph = Graph::new(3);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    /// graph.add_edge(2, 0);
    ///
    /// let euler = EulerianCycle::new(&graph);
    /// assert!(euler.cycle().is_some());
    /// ```
    pub fn cycle(&self) -> Option<&[usize]> {
        self.cycle.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_cycle() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 0);

        let euler = EulerianCycle::new(&graph);
        assert!(euler.has_eulerian_cycle());
        let cycle = euler.cycle().unwrap();
        assert_eq!(cycle.len(), 5); // 4 edges + 1
    }

    #[test]
    fn test_triangle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let euler = EulerianCycle::new(&graph);
        assert!(euler.has_eulerian_cycle());
    }

    #[test]
    fn test_no_cycle_odd_degree() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let euler = EulerianCycle::new(&graph);
        assert!(!euler.has_eulerian_cycle());
    }

    #[test]
    fn test_empty_graph() {
        let graph = Graph::new(5);
        let euler = EulerianCycle::new(&graph);
        assert!(euler.has_eulerian_cycle());
        assert_eq!(euler.cycle().unwrap().len(), 0);
    }

    #[test]
    fn test_complex_eulerian() {
        // Create a graph where all vertices have even degree
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);
        graph.add_edge(4, 0);
        graph.add_edge(0, 2);
        graph.add_edge(2, 4);
        graph.add_edge(1, 3); // Add edge to make all degrees even
                              // Final degrees: 0:3, 1:3, 2:4, 3:3, 4:3 - still odd!
                              // Let's add one more edge
        graph.add_edge(1, 4);
        // Now degrees: 0:3, 1:4, 2:4, 3:3, 4:4 - still have odd!
        // Add edge 0-3 to balance
        graph.add_edge(0, 3);
        // Now degrees: 0:4, 1:4, 2:4, 3:4, 4:4 - all even!

        let euler = EulerianCycle::new(&graph);
        assert!(euler.has_eulerian_cycle());
    }
}
