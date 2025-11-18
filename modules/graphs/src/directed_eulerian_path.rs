//! Eulerian path detection in directed graphs.
//!
//! This module finds an Eulerian path in a directed graph if one exists.
//! An Eulerian path is a path that uses every edge exactly once.

use crate::digraph::Digraph;
use std::collections::VecDeque;

/// Finds an Eulerian path in a directed graph.
///
/// An Eulerian path exists if and only if:
/// - At most one vertex has outdegree - indegree = 1
/// - At most one vertex has indegree - outdegree = 1
/// - Every other vertex has equal indegree and outdegree
/// - All vertices with nonzero degree belong to a single connected component
///
/// Uses Hierholzer's algorithm.
///
/// Time complexity: O(E)
/// Space complexity: O(E + V)
///
/// # Examples
///
/// ```
/// use algs4_graphs::{Digraph, DirectedEulerianPath};
///
/// let mut digraph = Digraph::new(4);
/// digraph.add_edge(0, 1);
/// digraph.add_edge(1, 2);
/// digraph.add_edge(2, 3);
///
/// let euler = DirectedEulerianPath::new(&digraph);
/// assert!(euler.has_eulerian_path());
/// ```
#[derive(Debug)]
pub struct DirectedEulerianPath {
    path: Option<Vec<usize>>, // Eulerian path (or None if none exists)
}

impl DirectedEulerianPath {
    /// Computes an Eulerian path in the directed graph if one exists.
    ///
    /// # Arguments
    ///
    /// * `g` - The directed graph
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Digraph, DirectedEulerianPath};
    ///
    /// let mut digraph = Digraph::new(3);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    ///
    /// let euler = DirectedEulerianPath::new(&digraph);
    /// assert!(euler.has_eulerian_path());
    /// ```
    pub fn new(g: &Digraph) -> Self {
        let mut deficit = 0; // outdegree - indegree
        let mut surplus = 0; // indegree - outdegree
        let mut start = 0;

        for v in 0..g.v() {
            let out_deg = g.outdegree(v) as i32;
            let in_deg = g.indegree(v) as i32;
            let diff = out_deg - in_deg;

            if diff > 0 {
                deficit += diff;
                start = v;
            } else if diff < 0 {
                surplus -= diff;
            }
        }

        // Eulerian path exists only if deficit and surplus are at most 1
        if deficit > 1 || surplus > 1 {
            return DirectedEulerianPath { path: None };
        }

        // Check for isolated vertices
        let non_isolated = Self::non_isolated_vertex(g);
        if non_isolated.is_none() {
            return DirectedEulerianPath {
                path: Some(Vec::new()),
            };
        }

        // Use start vertex if deficit > 0, otherwise use any non-isolated vertex
        if deficit == 0 {
            start = non_isolated.unwrap();
        }

        // Use Hierholzer's algorithm
        let mut adj = vec![VecDeque::new(); g.v()];
        for (v, adj_list) in adj.iter_mut().enumerate().take(g.v()) {
            for &w in g.adj(v) {
                adj_list.push_back(w);
            }
        }

        let mut stack = vec![start];
        let mut path = VecDeque::new();

        while let Some(&v) = stack.last() {
            if adj[v].is_empty() {
                path.push_front(v);
                stack.pop();
            } else if let Some(w) = adj[v].pop_front() {
                stack.push(w);
            }
        }

        // Check if all edges have been used
        if path.len() == g.e() + 1 {
            DirectedEulerianPath {
                path: Some(path.into_iter().collect()),
            }
        } else {
            DirectedEulerianPath { path: None }
        }
    }

    /// Returns a vertex with nonzero outdegree, or None if no such vertex exists.
    fn non_isolated_vertex(g: &Digraph) -> Option<usize> {
        (0..g.v()).find(|&v| g.outdegree(v) > 0)
    }

    /// Returns true if the digraph has an Eulerian path.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Digraph, DirectedEulerianPath};
    ///
    /// let mut digraph = Digraph::new(4);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    /// digraph.add_edge(2, 3);
    ///
    /// let euler = DirectedEulerianPath::new(&digraph);
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
    /// use algs4_graphs::{Digraph, DirectedEulerianPath};
    ///
    /// let mut digraph = Digraph::new(3);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    ///
    /// let euler = DirectedEulerianPath::new(&digraph);
    /// assert!(euler.path().is_some());
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
        let mut digraph = Digraph::new(4);
        digraph.add_edge(0, 1);
        digraph.add_edge(1, 2);
        digraph.add_edge(2, 3);

        let euler = DirectedEulerianPath::new(&digraph);
        assert!(euler.has_eulerian_path());
    }

    #[test]
    fn test_cycle_has_path() {
        let mut digraph = Digraph::new(4);
        digraph.add_edge(0, 1);
        digraph.add_edge(1, 2);
        digraph.add_edge(2, 3);
        digraph.add_edge(3, 0);

        let euler = DirectedEulerianPath::new(&digraph);
        assert!(euler.has_eulerian_path());
    }

    #[test]
    fn test_no_path_too_unbalanced() {
        let mut digraph = Digraph::new(4);
        digraph.add_edge(0, 1);
        digraph.add_edge(0, 2);
        digraph.add_edge(0, 3);

        let euler = DirectedEulerianPath::new(&digraph);
        assert!(!euler.has_eulerian_path());
    }

    #[test]
    fn test_empty_graph() {
        let digraph = Digraph::new(5);
        let euler = DirectedEulerianPath::new(&digraph);
        assert!(euler.has_eulerian_path());
    }

    #[test]
    fn test_single_edge() {
        let mut digraph = Digraph::new(2);
        digraph.add_edge(0, 1);

        let euler = DirectedEulerianPath::new(&digraph);
        assert!(euler.has_eulerian_path());
    }
}
