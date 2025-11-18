//! Cycle detection in directed graphs.
//!
//! This module provides an algorithm to detect cycles in directed graphs
//! using depth-first search.

use crate::digraph::Digraph;

/// Determines whether a directed graph has a directed cycle.
///
/// Uses depth-first search to detect cycles. Maintains an on-stack array
/// to track vertices currently on the recursion stack. A cycle exists if
/// we encounter an edge to a vertex that is currently on the stack.
///
/// Time complexity: O(V + E)
/// Space complexity: O(V)
///
/// # Examples
///
/// ```
/// use algs4_graphs::{Digraph, DirectedCycle};
///
/// let mut digraph = Digraph::new(4);
/// digraph.add_edge(0, 1);
/// digraph.add_edge(1, 2);
/// digraph.add_edge(2, 3);
///
/// let cycle = DirectedCycle::new(&digraph);
/// assert!(!cycle.has_cycle());
///
/// // Add edge that creates a cycle
/// digraph.add_edge(3, 0);
/// let cycle = DirectedCycle::new(&digraph);
/// assert!(cycle.has_cycle());
/// ```
#[derive(Debug)]
pub struct DirectedCycle {
    marked: Vec<bool>,         // marked[v] = has vertex v been marked?
    edge_to: Vec<usize>,       // edge_to[v] = previous vertex on path to v
    on_stack: Vec<bool>,       // on_stack[v] = is vertex on the stack?
    cycle: Option<Vec<usize>>, // directed cycle (or None if no cycle)
}

impl DirectedCycle {
    /// Determines whether the directed graph has a directed cycle.
    ///
    /// # Arguments
    ///
    /// * `g` - The directed graph
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Digraph, DirectedCycle};
    ///
    /// let mut digraph = Digraph::new(5);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    /// digraph.add_edge(2, 3);
    /// digraph.add_edge(3, 4);
    ///
    /// let cycle = DirectedCycle::new(&digraph);
    /// assert!(!cycle.has_cycle());
    /// ```
    pub fn new(g: &Digraph) -> Self {
        let mut dc = DirectedCycle {
            marked: vec![false; g.v()],
            edge_to: vec![0; g.v()],
            on_stack: vec![false; g.v()],
            cycle: None,
        };

        for v in 0..g.v() {
            if !dc.marked[v] && dc.cycle.is_none() {
                dc.dfs(g, v);
            }
        }

        dc
    }

    /// Depth-first search to detect cycles.
    fn dfs(&mut self, g: &Digraph, v: usize) {
        self.on_stack[v] = true;
        self.marked[v] = true;

        for &w in g.adj(v) {
            if self.cycle.is_some() {
                return;
            }

            if !self.marked[w] {
                self.edge_to[w] = v;
                self.dfs(g, w);
            } else if self.on_stack[w] {
                // Found a cycle
                let mut cycle = Vec::new();
                let mut x = v;
                while x != w {
                    cycle.push(x);
                    x = self.edge_to[x];
                }
                cycle.push(w);
                cycle.push(v);
                cycle.reverse();
                self.cycle = Some(cycle);
                return;
            }
        }

        self.on_stack[v] = false;
    }

    /// Returns true if the directed graph has a cycle.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Digraph, DirectedCycle};
    ///
    /// let mut digraph = Digraph::new(3);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    /// digraph.add_edge(2, 0);
    ///
    /// let cycle = DirectedCycle::new(&digraph);
    /// assert!(cycle.has_cycle());
    /// ```
    pub fn has_cycle(&self) -> bool {
        self.cycle.is_some()
    }

    /// Returns a directed cycle if one exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Digraph, DirectedCycle};
    ///
    /// let mut digraph = Digraph::new(3);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    /// digraph.add_edge(2, 0);
    ///
    /// let cycle = DirectedCycle::new(&digraph);
    /// assert!(cycle.cycle().is_some());
    /// let c = cycle.cycle().unwrap();
    /// assert_eq!(c.len(), 4); // cycle includes repeated vertex
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
        let mut digraph = Digraph::new(5);
        digraph.add_edge(0, 1);
        digraph.add_edge(1, 2);
        digraph.add_edge(2, 3);
        digraph.add_edge(3, 4);

        let cycle = DirectedCycle::new(&digraph);
        assert!(!cycle.has_cycle());
    }

    #[test]
    fn test_simple_cycle() {
        let mut digraph = Digraph::new(3);
        digraph.add_edge(0, 1);
        digraph.add_edge(1, 2);
        digraph.add_edge(2, 0);

        let cycle = DirectedCycle::new(&digraph);
        assert!(cycle.has_cycle());
        assert!(cycle.cycle().is_some());
    }

    #[test]
    fn test_self_loop() {
        let mut digraph = Digraph::new(3);
        digraph.add_edge(0, 0);

        let cycle = DirectedCycle::new(&digraph);
        assert!(cycle.has_cycle());
    }

    #[test]
    fn test_dag() {
        let mut digraph = Digraph::new(6);
        digraph.add_edge(0, 1);
        digraph.add_edge(0, 2);
        digraph.add_edge(1, 3);
        digraph.add_edge(2, 3);
        digraph.add_edge(3, 4);
        digraph.add_edge(3, 5);

        let cycle = DirectedCycle::new(&digraph);
        assert!(!cycle.has_cycle());
    }

    #[test]
    fn test_disconnected_with_cycle() {
        let mut digraph = Digraph::new(6);
        digraph.add_edge(0, 1);
        digraph.add_edge(1, 2);
        digraph.add_edge(3, 4);
        digraph.add_edge(4, 5);
        digraph.add_edge(5, 3);

        let cycle = DirectedCycle::new(&digraph);
        assert!(cycle.has_cycle());
    }

    #[test]
    fn test_empty_graph() {
        let digraph = Digraph::new(5);
        let cycle = DirectedCycle::new(&digraph);
        assert!(!cycle.has_cycle());
    }

    #[test]
    fn test_single_vertex() {
        let digraph = Digraph::new(1);
        let cycle = DirectedCycle::new(&digraph);
        assert!(!cycle.has_cycle());
    }

    #[test]
    fn test_complex_cycle() {
        let mut digraph = Digraph::new(7);
        digraph.add_edge(0, 1);
        digraph.add_edge(1, 2);
        digraph.add_edge(2, 3);
        digraph.add_edge(3, 4);
        digraph.add_edge(4, 1); // Creates cycle 1 -> 2 -> 3 -> 4 -> 1
        digraph.add_edge(0, 5);
        digraph.add_edge(5, 6);

        let cycle = DirectedCycle::new(&digraph);
        assert!(cycle.has_cycle());
    }
}
