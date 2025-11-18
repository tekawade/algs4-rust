//! Eulerian cycle detection in directed graphs.
//!
//! This module finds an Eulerian cycle in a directed graph if one exists.
//! An Eulerian cycle is a cycle that uses every edge exactly once.

use crate::digraph::Digraph;
use std::collections::VecDeque;

/// Finds an Eulerian cycle in a directed graph.
///
/// An Eulerian cycle exists if and only if every vertex has equal indegree
/// and outdegree, and all vertices with nonzero degree belong to a single
/// strongly connected component.
///
/// Uses Hierholzer's algorithm.
///
/// Time complexity: O(E)
/// Space complexity: O(E + V)
///
/// # Examples
///
/// ```
/// use algs4_graphs::{Digraph, DirectedEulerianCycle};
///
/// let mut digraph = Digraph::new(4);
/// digraph.add_edge(0, 1);
/// digraph.add_edge(1, 2);
/// digraph.add_edge(2, 3);
/// digraph.add_edge(3, 0);
///
/// let euler = DirectedEulerianCycle::new(&digraph);
/// assert!(euler.has_eulerian_cycle());
/// ```
#[derive(Debug)]
pub struct DirectedEulerianCycle {
    cycle: Option<Vec<usize>>, // Eulerian cycle (or None if none exists)
}

impl DirectedEulerianCycle {
    /// Computes an Eulerian cycle in the directed graph if one exists.
    ///
    /// # Arguments
    ///
    /// * `g` - The directed graph
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Digraph, DirectedEulerianCycle};
    ///
    /// let mut digraph = Digraph::new(3);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    /// digraph.add_edge(2, 0);
    ///
    /// let euler = DirectedEulerianCycle::new(&digraph);
    /// assert!(euler.has_eulerian_cycle());
    /// ```
    pub fn new(g: &Digraph) -> Self {
        // Check if indegree equals outdegree for all vertices
        for v in 0..g.v() {
            if g.indegree(v) != g.outdegree(v) {
                return DirectedEulerianCycle { cycle: None };
            }
        }

        // Check for isolated vertices
        let non_isolated = Self::non_isolated_vertex(g);
        if non_isolated.is_none() {
            return DirectedEulerianCycle {
                cycle: Some(Vec::new()),
            };
        }

        // Use Hierholzer's algorithm
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
            }
        }

        // Check if all edges have been used
        if cycle.len() == g.e() + 1 {
            DirectedEulerianCycle {
                cycle: Some(cycle.into_iter().collect()),
            }
        } else {
            DirectedEulerianCycle { cycle: None }
        }
    }

    /// Returns a vertex with nonzero outdegree, or None if no such vertex exists.
    fn non_isolated_vertex(g: &Digraph) -> Option<usize> {
        (0..g.v()).find(|&v| g.outdegree(v) > 0)
    }

    /// Returns true if the digraph has an Eulerian cycle.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Digraph, DirectedEulerianCycle};
    ///
    /// let mut digraph = Digraph::new(4);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    /// digraph.add_edge(2, 3);
    /// digraph.add_edge(3, 0);
    ///
    /// let euler = DirectedEulerianCycle::new(&digraph);
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
    /// use algs4_graphs::{Digraph, DirectedEulerianCycle};
    ///
    /// let mut digraph = Digraph::new(3);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    /// digraph.add_edge(2, 0);
    ///
    /// let euler = DirectedEulerianCycle::new(&digraph);
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
        let mut digraph = Digraph::new(4);
        digraph.add_edge(0, 1);
        digraph.add_edge(1, 2);
        digraph.add_edge(2, 3);
        digraph.add_edge(3, 0);

        let euler = DirectedEulerianCycle::new(&digraph);
        assert!(euler.has_eulerian_cycle());
    }

    #[test]
    fn test_triangle() {
        let mut digraph = Digraph::new(3);
        digraph.add_edge(0, 1);
        digraph.add_edge(1, 2);
        digraph.add_edge(2, 0);

        let euler = DirectedEulerianCycle::new(&digraph);
        assert!(euler.has_eulerian_cycle());
    }

    #[test]
    fn test_no_cycle_unbalanced() {
        let mut digraph = Digraph::new(3);
        digraph.add_edge(0, 1);
        digraph.add_edge(1, 2);

        let euler = DirectedEulerianCycle::new(&digraph);
        assert!(!euler.has_eulerian_cycle());
    }

    #[test]
    fn test_empty_graph() {
        let digraph = Digraph::new(5);
        let euler = DirectedEulerianCycle::new(&digraph);
        assert!(euler.has_eulerian_cycle());
    }

    #[test]
    fn test_complex_eulerian() {
        let mut digraph = Digraph::new(5);
        digraph.add_edge(0, 1);
        digraph.add_edge(1, 2);
        digraph.add_edge(2, 0);
        digraph.add_edge(2, 3);
        digraph.add_edge(3, 4);
        digraph.add_edge(4, 2);

        let euler = DirectedEulerianCycle::new(&digraph);
        assert!(euler.has_eulerian_cycle());
    }
}
