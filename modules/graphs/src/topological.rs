//! Topological sort of a directed acyclic graph (DAG).
//!
//! This module computes a topological order of vertices in a DAG.
//! A topological order is an ordering of vertices such that for every
//! directed edge (v, w), vertex v comes before vertex w in the ordering.

use crate::depth_first_order::DepthFirstOrder;
use crate::digraph::Digraph;
use crate::directed_cycle::DirectedCycle;

/// Computes a topological ordering of the vertices in a directed acyclic graph (DAG).
///
/// Uses depth-first search and reverse postorder. First checks if the graph
/// has a cycle using DirectedCycle. If there is no cycle, the reverse postorder
/// gives a topological ordering.
///
/// Time complexity: O(V + E)
/// Space complexity: O(V)
///
/// # Examples
///
/// ```
/// use algs4_graphs::{Digraph, Topological};
///
/// let mut digraph = Digraph::new(6);
/// digraph.add_edge(0, 1);
/// digraph.add_edge(0, 2);
/// digraph.add_edge(1, 3);
/// digraph.add_edge(2, 3);
/// digraph.add_edge(3, 4);
/// digraph.add_edge(3, 5);
///
/// let topo = Topological::new(&digraph);
/// assert!(topo.has_order());
/// let order = topo.order().unwrap();
/// assert_eq!(order[0], 0); // 0 comes first
/// ```
#[derive(Debug)]
pub struct Topological {
    order: Option<Vec<usize>>, // topological order (or None if graph has cycle)
}

impl Topological {
    /// Determines whether the directed graph has a topological order and,
    /// if so, finds such a topological order.
    ///
    /// # Arguments
    ///
    /// * `g` - The directed graph
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Digraph, Topological};
    ///
    /// let mut digraph = Digraph::new(4);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    /// digraph.add_edge(2, 3);
    ///
    /// let topo = Topological::new(&digraph);
    /// assert!(topo.has_order());
    /// ```
    pub fn new(g: &Digraph) -> Self {
        let cycle = DirectedCycle::new(g);
        if !cycle.has_cycle() {
            let dfs = DepthFirstOrder::new(g);
            Topological {
                order: Some(dfs.reverse_post()),
            }
        } else {
            Topological { order: None }
        }
    }

    /// Returns a topological order if the directed graph has one.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Digraph, Topological};
    ///
    /// let mut digraph = Digraph::new(4);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    /// digraph.add_edge(2, 3);
    ///
    /// let topo = Topological::new(&digraph);
    /// let order = topo.order().unwrap();
    /// assert_eq!(order, vec![0, 1, 2, 3]);
    /// ```
    pub fn order(&self) -> Option<&[usize]> {
        self.order.as_deref()
    }

    /// Returns true if the directed graph has a topological order.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Digraph, Topological};
    ///
    /// // DAG - has topological order
    /// let mut dag = Digraph::new(3);
    /// dag.add_edge(0, 1);
    /// dag.add_edge(1, 2);
    ///
    /// let topo = Topological::new(&dag);
    /// assert!(topo.has_order());
    ///
    /// // Graph with cycle - no topological order
    /// let mut cyclic = Digraph::new(3);
    /// cyclic.add_edge(0, 1);
    /// cyclic.add_edge(1, 2);
    /// cyclic.add_edge(2, 0);
    ///
    /// let topo = Topological::new(&cyclic);
    /// assert!(!topo.has_order());
    /// ```
    pub fn has_order(&self) -> bool {
        self.order.is_some()
    }

    /// Returns the rank of vertex v in the topological order.
    ///
    /// Returns None if the graph is not a DAG or if v is not valid.
    ///
    /// # Arguments
    ///
    /// * `v` - The vertex
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Digraph, Topological};
    ///
    /// let mut digraph = Digraph::new(4);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    /// digraph.add_edge(2, 3);
    ///
    /// let topo = Topological::new(&digraph);
    /// assert_eq!(topo.rank(0), Some(0));
    /// assert_eq!(topo.rank(3), Some(3));
    /// ```
    pub fn rank(&self, v: usize) -> Option<usize> {
        if let Some(order) = &self.order {
            order.iter().position(|&x| x == v)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_dag() {
        let mut digraph = Digraph::new(6);
        digraph.add_edge(0, 1);
        digraph.add_edge(0, 2);
        digraph.add_edge(1, 3);
        digraph.add_edge(2, 3);
        digraph.add_edge(3, 4);
        digraph.add_edge(3, 5);

        let topo = Topological::new(&digraph);
        assert!(topo.has_order());
        let order = topo.order().unwrap();

        // Verify that all edges go from earlier to later in the ordering
        for v in 0..digraph.v() {
            let v_pos = order.iter().position(|&x| x == v).unwrap();
            for &w in digraph.adj(v) {
                let w_pos = order.iter().position(|&x| x == w).unwrap();
                assert!(v_pos < w_pos);
            }
        }
    }

    #[test]
    fn test_linear_dag() {
        let mut digraph = Digraph::new(4);
        digraph.add_edge(0, 1);
        digraph.add_edge(1, 2);
        digraph.add_edge(2, 3);

        let topo = Topological::new(&digraph);
        assert!(topo.has_order());
        let order = topo.order().unwrap();
        assert_eq!(order, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_graph_with_cycle() {
        let mut digraph = Digraph::new(3);
        digraph.add_edge(0, 1);
        digraph.add_edge(1, 2);
        digraph.add_edge(2, 0);

        let topo = Topological::new(&digraph);
        assert!(!topo.has_order());
        assert!(topo.order().is_none());
    }

    #[test]
    fn test_empty_graph() {
        let digraph = Digraph::new(5);
        let topo = Topological::new(&digraph);
        assert!(topo.has_order());
        assert_eq!(topo.order().unwrap().len(), 5);
    }

    #[test]
    fn test_rank() {
        let mut digraph = Digraph::new(4);
        digraph.add_edge(0, 1);
        digraph.add_edge(1, 2);
        digraph.add_edge(2, 3);

        let topo = Topological::new(&digraph);
        assert_eq!(topo.rank(0), Some(0));
        assert_eq!(topo.rank(1), Some(1));
        assert_eq!(topo.rank(2), Some(2));
        assert_eq!(topo.rank(3), Some(3));
    }

    #[test]
    fn test_disconnected_dag() {
        let mut digraph = Digraph::new(5);
        digraph.add_edge(0, 1);
        digraph.add_edge(2, 3);

        let topo = Topological::new(&digraph);
        assert!(topo.has_order());
        let order = topo.order().unwrap();
        assert_eq!(order.len(), 5);
    }
}
