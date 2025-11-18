//! Eulerian path detection in undirected graphs.
//!
//! This module finds an Eulerian path in an undirected graph if one exists.
//! An Eulerian path is a path that uses every edge exactly once.

use crate::graph::Graph;
use std::collections::VecDeque;

/// Finds an Eulerian path in an undirected graph.
///
/// An Eulerian path exists if and only if the graph has exactly 0 or 2
/// vertices with odd degree, and all vertices with nonzero degree belong
/// to a single connected component.
///
/// Uses Hierholzer's algorithm.
///
/// Time complexity: O(E)
/// Space complexity: O(E + V)
///
/// # Examples
///
/// ```
/// use algs4_graphs::{Graph, EulerianPath};
///
/// let mut graph = Graph::new(4);
/// graph.add_edge(0, 1);
/// graph.add_edge(1, 2);
/// graph.add_edge(2, 3);
///
/// let euler = EulerianPath::new(&graph);
/// assert!(euler.has_eulerian_path());
/// ```
#[derive(Debug)]
pub struct EulerianPath {
    path: Option<Vec<usize>>, // Eulerian path (or None if none exists)
}

impl EulerianPath {
    /// Computes an Eulerian path in the undirected graph if one exists.
    ///
    /// # Arguments
    ///
    /// * `g` - The undirected graph
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Graph, EulerianPath};
    ///
    /// let mut graph = Graph::new(3);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    ///
    /// let euler = EulerianPath::new(&graph);
    /// assert!(euler.has_eulerian_path());
    /// ```
    pub fn new(g: &Graph) -> Self {
        // Find vertices with odd degree
        let mut odd_count = 0;
        let mut odd_vertex = 0;
        for v in 0..g.v() {
            if !g.degree(v).is_multiple_of(2) {
                odd_count += 1;
                odd_vertex = v;
            }
        }

        // Eulerian path exists only if there are 0 or 2 vertices with odd degree
        if odd_count != 0 && odd_count != 2 {
            return EulerianPath { path: None };
        }

        // Check for isolated vertices
        let non_isolated = Self::non_isolated_vertex(g);
        if non_isolated.is_none() {
            return EulerianPath {
                path: Some(Vec::new()),
            };
        }

        // Use Hierholzer's algorithm
        let s = if odd_count == 2 {
            odd_vertex
        } else {
            non_isolated.unwrap()
        };

        let mut adj = vec![VecDeque::new(); g.v()];
        for (v, adj_list) in adj.iter_mut().enumerate().take(g.v()) {
            for &w in g.adj(v) {
                adj_list.push_back(w);
            }
        }

        let mut stack = vec![s];
        let mut path = VecDeque::new();

        while let Some(&v) = stack.last() {
            if adj[v].is_empty() {
                path.push_front(v);
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
        if path.len() == g.e() + 1 {
            EulerianPath {
                path: Some(path.into_iter().collect()),
            }
        } else {
            EulerianPath { path: None }
        }
    }

    /// Returns a vertex with nonzero degree, or None if no such vertex exists.
    fn non_isolated_vertex(g: &Graph) -> Option<usize> {
        (0..g.v()).find(|&v| g.degree(v) > 0)
    }

    /// Returns true if the graph has an Eulerian path.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Graph, EulerianPath};
    ///
    /// let mut graph = Graph::new(4);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    /// graph.add_edge(2, 3);
    ///
    /// let euler = EulerianPath::new(&graph);
    /// assert!(euler.has_eulerian_path());
    /// ```
    pub fn has_eulerian_path(&self) -> bool {
        self.path.is_some()
    }

    /// Returns the Eulerian path if one exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Graph, EulerianPath};
    ///
    /// let mut graph = Graph::new(3);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    ///
    /// let euler = EulerianPath::new(&graph);
    /// assert!(euler.path().is_some());
    /// let path = euler.path().unwrap();
    /// assert_eq!(path.len(), 3); // 2 edges + 1
    /// ```
    pub fn path(&self) -> Option<&[usize]> {
        self.path.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_path() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);

        let euler = EulerianPath::new(&graph);
        assert!(euler.has_eulerian_path());
        let path = euler.path().unwrap();
        assert_eq!(path.len(), 4);
    }

    #[test]
    fn test_cycle_has_path() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 0);

        let euler = EulerianPath::new(&graph);
        assert!(euler.has_eulerian_path()); // cycle is also a path
    }

    #[test]
    fn test_no_path_too_many_odd() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(0, 3);

        let euler = EulerianPath::new(&graph);
        assert!(!euler.has_eulerian_path());
    }

    #[test]
    fn test_empty_graph() {
        let graph = Graph::new(5);
        let euler = EulerianPath::new(&graph);
        assert!(euler.has_eulerian_path());
        assert_eq!(euler.path().unwrap().len(), 0);
    }

    #[test]
    fn test_single_edge() {
        let mut graph = Graph::new(2);
        graph.add_edge(0, 1);

        let euler = EulerianPath::new(&graph);
        assert!(euler.has_eulerian_path());
    }
}
