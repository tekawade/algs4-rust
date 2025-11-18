//! Non-recursive cycle detection in directed graphs.
//!
//! This module provides a non-recursive algorithm to detect cycles in directed graphs
//! using an explicit stack.

use crate::digraph::Digraph;

/// Determines whether a directed graph has a directed cycle using a non-recursive approach.
///
/// Uses an explicit stack instead of recursion for the depth-first search.
/// This is useful for very large graphs where stack overflow might be a concern.
///
/// Time complexity: O(V + E)
/// Space complexity: O(V)
///
/// # Examples
///
/// ```
/// use algs4_graphs::{Digraph, DirectedCycleX};
///
/// let mut digraph = Digraph::new(4);
/// digraph.add_edge(0, 1);
/// digraph.add_edge(1, 2);
/// digraph.add_edge(2, 3);
///
/// let cycle = DirectedCycleX::new(&digraph);
/// assert!(!cycle.has_cycle());
/// ```
#[derive(Debug)]
pub struct DirectedCycleX {
    cycle: Option<Vec<usize>>, // directed cycle (or None if no cycle)
}

impl DirectedCycleX {
    /// Determines whether the directed graph has a directed cycle using non-recursive DFS.
    ///
    /// # Arguments
    ///
    /// * `g` - The directed graph
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Digraph, DirectedCycleX};
    ///
    /// let mut digraph = Digraph::new(3);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    /// digraph.add_edge(2, 0);
    ///
    /// let cycle = DirectedCycleX::new(&digraph);
    /// assert!(cycle.has_cycle());
    /// ```
    pub fn new(g: &Digraph) -> Self {
        let mut marked = vec![false; g.v()];
        let mut on_stack = vec![false; g.v()];
        let mut edge_to = vec![0; g.v()];
        let mut cycle = None;

        for s in 0..g.v() {
            if cycle.is_some() {
                break;
            }
            if !marked[s] {
                // Non-recursive DFS
                let mut stack = vec![(s, 0)]; // (vertex, adjacency index)
                on_stack[s] = true;
                marked[s] = true;

                while let Some((v, idx)) = stack.pop() {
                    let adj = g.adj(v);

                    if idx < adj.len() {
                        let w = adj[idx];
                        stack.push((v, idx + 1)); // Continue exploring other neighbors later

                        if !marked[w] {
                            edge_to[w] = v;
                            marked[w] = true;
                            on_stack[w] = true;
                            stack.push((w, 0));
                        } else if on_stack[w] {
                            // Found a cycle
                            let mut c = Vec::new();
                            let mut x = v;
                            while x != w {
                                c.push(x);
                                x = edge_to[x];
                            }
                            c.push(w);
                            c.push(v);
                            c.reverse();
                            cycle = Some(c);
                            break;
                        }
                    } else {
                        on_stack[v] = false;
                    }
                }
            }
        }

        DirectedCycleX { cycle }
    }

    /// Returns true if the directed graph has a cycle.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Digraph, DirectedCycleX};
    ///
    /// let mut digraph = Digraph::new(3);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    ///
    /// let cycle = DirectedCycleX::new(&digraph);
    /// assert!(!cycle.has_cycle());
    /// ```
    pub fn has_cycle(&self) -> bool {
        self.cycle.is_some()
    }

    /// Returns a directed cycle if one exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Digraph, DirectedCycleX};
    ///
    /// let mut digraph = Digraph::new(3);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    /// digraph.add_edge(2, 0);
    ///
    /// let cycle = DirectedCycleX::new(&digraph);
    /// assert!(cycle.cycle().is_some());
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

        let cycle = DirectedCycleX::new(&digraph);
        assert!(!cycle.has_cycle());
    }

    #[test]
    fn test_simple_cycle() {
        let mut digraph = Digraph::new(3);
        digraph.add_edge(0, 1);
        digraph.add_edge(1, 2);
        digraph.add_edge(2, 0);

        let cycle = DirectedCycleX::new(&digraph);
        assert!(cycle.has_cycle());
    }

    #[test]
    fn test_self_loop() {
        let mut digraph = Digraph::new(3);
        digraph.add_edge(0, 0);

        let cycle = DirectedCycleX::new(&digraph);
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

        let cycle = DirectedCycleX::new(&digraph);
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

        let cycle = DirectedCycleX::new(&digraph);
        assert!(cycle.has_cycle());
    }

    #[test]
    fn test_empty_graph() {
        let digraph = Digraph::new(5);
        let cycle = DirectedCycleX::new(&digraph);
        assert!(!cycle.has_cycle());
    }
}
