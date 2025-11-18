//! Cycle detection in edge-weighted directed graphs.
//!
//! This module provides an algorithm to detect cycles in edge-weighted directed graphs
//! using depth-first search.

use crate::directed_edge::DirectedEdge;
use crate::edge_weighted_digraph::EdgeWeightedDigraph;

/// Determines whether an edge-weighted directed graph has a directed cycle.
///
/// Uses depth-first search to detect cycles. Maintains an on-stack array
/// to track vertices currently on the recursion stack.
///
/// Time complexity: O(V + E)
/// Space complexity: O(V)
///
/// # Examples
///
/// ```
/// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, EdgeWeightedDirectedCycle};
///
/// let mut digraph = EdgeWeightedDigraph::new(4);
/// digraph.add_edge(DirectedEdge::new(0, 1, 0.5));
/// digraph.add_edge(DirectedEdge::new(1, 2, 0.3));
/// digraph.add_edge(DirectedEdge::new(2, 3, 0.7));
///
/// let cycle = EdgeWeightedDirectedCycle::new(&digraph);
/// assert!(!cycle.has_cycle());
/// ```
#[derive(Debug)]
pub struct EdgeWeightedDirectedCycle {
    marked: Vec<bool>,                  // marked[v] = has vertex v been marked?
    edge_to: Vec<Option<DirectedEdge>>, // edge_to[v] = previous edge on path to v
    on_stack: Vec<bool>,                // on_stack[v] = is vertex on the stack?
    cycle: Option<Vec<DirectedEdge>>,   // directed cycle (or None if no cycle)
}

impl EdgeWeightedDirectedCycle {
    /// Determines whether the edge-weighted directed graph has a directed cycle.
    ///
    /// # Arguments
    ///
    /// * `g` - The edge-weighted directed graph
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, EdgeWeightedDirectedCycle};
    ///
    /// let mut digraph = EdgeWeightedDigraph::new(3);
    /// digraph.add_edge(DirectedEdge::new(0, 1, 0.5));
    /// digraph.add_edge(DirectedEdge::new(1, 2, 0.3));
    ///
    /// let cycle = EdgeWeightedDirectedCycle::new(&digraph);
    /// assert!(!cycle.has_cycle());
    /// ```
    pub fn new(g: &EdgeWeightedDigraph) -> Self {
        let mut ewdc = EdgeWeightedDirectedCycle {
            marked: vec![false; g.v()],
            edge_to: vec![None; g.v()],
            on_stack: vec![false; g.v()],
            cycle: None,
        };

        for v in 0..g.v() {
            if !ewdc.marked[v] && ewdc.cycle.is_none() {
                ewdc.dfs(g, v);
            }
        }

        ewdc
    }

    /// Depth-first search to detect cycles.
    fn dfs(&mut self, g: &EdgeWeightedDigraph, v: usize) {
        self.on_stack[v] = true;
        self.marked[v] = true;

        for &e in g.adj(v) {
            let w = e.to();

            if self.cycle.is_some() {
                return;
            }

            if !self.marked[w] {
                self.edge_to[w] = Some(e);
                self.dfs(g, w);
            } else if self.on_stack[w] {
                // Found a cycle
                let mut cycle = Vec::new();
                let mut f = e;
                while f.from() != w {
                    cycle.push(f);
                    f = self.edge_to[f.from()].unwrap();
                }
                cycle.push(f);
                cycle.reverse();
                self.cycle = Some(cycle);
                return;
            }
        }

        self.on_stack[v] = false;
    }

    /// Returns true if the edge-weighted directed graph has a cycle.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, EdgeWeightedDirectedCycle};
    ///
    /// let mut digraph = EdgeWeightedDigraph::new(3);
    /// digraph.add_edge(DirectedEdge::new(0, 1, 0.5));
    /// digraph.add_edge(DirectedEdge::new(1, 2, 0.3));
    /// digraph.add_edge(DirectedEdge::new(2, 0, 0.7));
    ///
    /// let cycle = EdgeWeightedDirectedCycle::new(&digraph);
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
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, EdgeWeightedDirectedCycle};
    ///
    /// let mut digraph = EdgeWeightedDigraph::new(3);
    /// digraph.add_edge(DirectedEdge::new(0, 1, 0.5));
    /// digraph.add_edge(DirectedEdge::new(1, 2, 0.3));
    ///
    /// let cycle = EdgeWeightedDirectedCycle::new(&digraph);
    /// assert!(cycle.cycle().is_none());
    /// ```
    pub fn cycle(&self) -> Option<&[DirectedEdge]> {
        self.cycle.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_cycle() {
        let mut digraph = EdgeWeightedDigraph::new(5);
        digraph.add_edge(DirectedEdge::new(0, 1, 0.5));
        digraph.add_edge(DirectedEdge::new(1, 2, 0.3));
        digraph.add_edge(DirectedEdge::new(2, 3, 0.7));
        digraph.add_edge(DirectedEdge::new(3, 4, 0.2));

        let cycle = EdgeWeightedDirectedCycle::new(&digraph);
        assert!(!cycle.has_cycle());
    }

    #[test]
    fn test_simple_cycle() {
        let mut digraph = EdgeWeightedDigraph::new(3);
        digraph.add_edge(DirectedEdge::new(0, 1, 0.5));
        digraph.add_edge(DirectedEdge::new(1, 2, 0.3));
        digraph.add_edge(DirectedEdge::new(2, 0, 0.7));

        let cycle = EdgeWeightedDirectedCycle::new(&digraph);
        assert!(cycle.has_cycle());
        assert!(cycle.cycle().is_some());
    }

    #[test]
    fn test_self_loop() {
        let mut digraph = EdgeWeightedDigraph::new(3);
        digraph.add_edge(DirectedEdge::new(0, 0, 0.5));

        let cycle = EdgeWeightedDirectedCycle::new(&digraph);
        assert!(cycle.has_cycle());
    }

    #[test]
    fn test_dag() {
        let mut digraph = EdgeWeightedDigraph::new(6);
        digraph.add_edge(DirectedEdge::new(0, 1, 0.5));
        digraph.add_edge(DirectedEdge::new(0, 2, 0.3));
        digraph.add_edge(DirectedEdge::new(1, 3, 0.7));
        digraph.add_edge(DirectedEdge::new(2, 3, 0.2));
        digraph.add_edge(DirectedEdge::new(3, 4, 0.8));
        digraph.add_edge(DirectedEdge::new(3, 5, 0.1));

        let cycle = EdgeWeightedDirectedCycle::new(&digraph);
        assert!(!cycle.has_cycle());
    }

    #[test]
    fn test_disconnected_with_cycle() {
        let mut digraph = EdgeWeightedDigraph::new(6);
        digraph.add_edge(DirectedEdge::new(0, 1, 0.5));
        digraph.add_edge(DirectedEdge::new(1, 2, 0.3));
        digraph.add_edge(DirectedEdge::new(3, 4, 0.7));
        digraph.add_edge(DirectedEdge::new(4, 5, 0.2));
        digraph.add_edge(DirectedEdge::new(5, 3, 0.8));

        let cycle = EdgeWeightedDirectedCycle::new(&digraph);
        assert!(cycle.has_cycle());
    }

    #[test]
    fn test_empty_graph() {
        let digraph = EdgeWeightedDigraph::new(5);
        let cycle = EdgeWeightedDirectedCycle::new(&digraph);
        assert!(!cycle.has_cycle());
    }
}
