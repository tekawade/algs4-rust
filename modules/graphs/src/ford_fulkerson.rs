//! Ford-Fulkerson maximum flow algorithm.
//!
//! This module implements the Ford-Fulkerson algorithm for computing
//! maximum flow in a flow network.

use crate::flow_network::FlowNetwork;
use std::collections::VecDeque;

/// Type alias for edge_to array storing (from_vertex, to_vertex, edge_index).
type EdgeTo = Vec<Option<(usize, usize, usize)>>;

/// Computes a maximum flow in a flow network using the Ford-Fulkerson algorithm.
///
/// Uses breadth-first search to find augmenting paths (Edmonds-Karp algorithm).
/// The maximum flow equals the capacity of the minimum s-t cut.
///
/// Time complexity: O(VE^2)
/// Space complexity: O(V)
///
/// # Examples
///
/// ```
/// use algs4_graphs::{FlowNetwork, FlowEdge, FordFulkerson};
///
/// let mut network = FlowNetwork::new(6);
/// network.add_edge(FlowEdge::new(0, 1, 2.0));
/// network.add_edge(FlowEdge::new(0, 2, 3.0));
/// network.add_edge(FlowEdge::new(1, 3, 3.0));
/// network.add_edge(FlowEdge::new(2, 3, 1.0));
/// network.add_edge(FlowEdge::new(2, 4, 1.0));
/// network.add_edge(FlowEdge::new(3, 5, 2.0));
/// network.add_edge(FlowEdge::new(4, 5, 3.0));
///
/// let max_flow = FordFulkerson::new(&mut network, 0, 5);
/// assert_eq!(max_flow.value(), 3.0);
/// ```
#[derive(Debug)]
pub struct FordFulkerson {
    marked: Vec<bool>, // marked[v] = true if v is reachable from s in residual network
    value: f64,        // current value of max flow
}

impl FordFulkerson {
    /// Computes the maximum flow in the flow network from source s to sink t.
    ///
    /// # Arguments
    ///
    /// * `network` - The flow network (modified in place)
    /// * `s` - The source vertex
    /// * `t` - The sink vertex
    ///
    /// # Panics
    ///
    /// Panics if s or t are not valid vertices or if s equals t.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{FlowNetwork, FlowEdge, FordFulkerson};
    ///
    /// let mut network = FlowNetwork::new(4);
    /// network.add_edge(FlowEdge::new(0, 1, 10.0));
    /// network.add_edge(FlowEdge::new(0, 2, 10.0));
    /// network.add_edge(FlowEdge::new(1, 3, 10.0));
    /// network.add_edge(FlowEdge::new(2, 3, 10.0));
    ///
    /// let max_flow = FordFulkerson::new(&mut network, 0, 3);
    /// assert_eq!(max_flow.value(), 20.0);
    /// ```
    pub fn new(network: &mut FlowNetwork, s: usize, t: usize) -> Self {
        Self::validate_vertex(network, s);
        Self::validate_vertex(network, t);
        if s == t {
            panic!("Source equals sink");
        }

        let mut value = 0.0;

        // While there exists an augmenting path, use it
        loop {
            let (has_path, edge_to) = Self::has_augmenting_path(network, s, t);
            if !has_path {
                break;
            }

            // Compute bottleneck capacity
            let mut bottle = f64::INFINITY;
            let mut v = t;
            while v != s {
                let (from_v, to_v, idx) = edge_to[v].unwrap();
                let edges = network.adj(from_v);
                let e = edges[idx];
                bottle = bottle.min(e.residual_capacity_to(to_v));
                v = from_v;
            }

            // Augment flow
            v = t;
            while v != s {
                let (from_v, to_v, idx) = edge_to[v].unwrap();
                network.adj_mut(from_v)[idx].add_residual_flow_to(to_v, bottle);
                // Also update the copy in the other vertex's adjacency list
                let e_from = network.adj(from_v)[idx].from();
                let e_to = network.adj(from_v)[idx].to();
                for i in 0..network.adj(to_v).len() {
                    let edge = network.adj(to_v)[i];
                    if edge.from() == e_from && edge.to() == e_to {
                        network.adj_mut(to_v)[i].add_residual_flow_to(to_v, bottle);
                        break;
                    }
                }
                v = from_v;
            }

            value += bottle;
        }

        let marked = Self::compute_min_cut(network, s);
        FordFulkerson { marked, value }
    }

    /// Validates that vertex v is a valid vertex.
    fn validate_vertex(network: &FlowNetwork, v: usize) {
        if v >= network.v() {
            panic!("vertex {} is not between 0 and {}", v, network.v() - 1);
        }
    }

    /// Finds an augmenting path using BFS and returns true if one exists,
    /// along with the edge_to array for backtracking.
    fn has_augmenting_path(network: &FlowNetwork, s: usize, t: usize) -> (bool, EdgeTo) {
        let mut marked = vec![false; network.v()];
        let mut edge_to: EdgeTo = vec![None; network.v()];

        let mut queue = VecDeque::new();
        queue.push_back(s);
        marked[s] = true;

        while let Some(v) = queue.pop_front() {
            let edges = network.adj(v);
            for (i, &e) in edges.iter().enumerate() {
                let w = e.other(v);

                // Is there a path from s to w in the residual network?
                if e.residual_capacity_to(w) > 0.0 && !marked[w] {
                    edge_to[w] = Some((v, w, i));
                    marked[w] = true;
                    if w == t {
                        return (true, edge_to);
                    }
                    queue.push_back(w);
                }
            }
        }

        (false, edge_to)
    }

    /// Computes the minimum cut (vertices reachable from s in residual network).
    fn compute_min_cut(network: &FlowNetwork, s: usize) -> Vec<bool> {
        let mut marked = vec![false; network.v()];
        let mut queue = VecDeque::new();
        queue.push_back(s);
        marked[s] = true;

        while let Some(v) = queue.pop_front() {
            for &e in network.adj(v) {
                let w = e.other(v);
                if e.residual_capacity_to(w) > 0.0 && !marked[w] {
                    marked[w] = true;
                    queue.push_back(w);
                }
            }
        }

        marked
    }

    /// Returns the value of the maximum flow.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{FlowNetwork, FlowEdge, FordFulkerson};
    ///
    /// let mut network = FlowNetwork::new(4);
    /// network.add_edge(FlowEdge::new(0, 1, 10.0));
    /// network.add_edge(FlowEdge::new(1, 3, 10.0));
    ///
    /// let max_flow = FordFulkerson::new(&mut network, 0, 3);
    /// assert_eq!(max_flow.value(), 10.0);
    /// ```
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Returns true if vertex v is on the source side of the min cut.
    ///
    /// # Arguments
    ///
    /// * `v` - The vertex
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{FlowNetwork, FlowEdge, FordFulkerson};
    ///
    /// let mut network = FlowNetwork::new(4);
    /// network.add_edge(FlowEdge::new(0, 1, 10.0));
    /// network.add_edge(FlowEdge::new(1, 3, 10.0));
    ///
    /// let max_flow = FordFulkerson::new(&mut network, 0, 3);
    /// assert!(max_flow.in_cut(0));
    /// assert!(!max_flow.in_cut(3));
    /// ```
    pub fn in_cut(&self, v: usize) -> bool {
        if v >= self.marked.len() {
            panic!(
                "vertex {} is not between 0 and {}",
                v,
                self.marked.len() - 1
            );
        }
        self.marked[v]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flow_edge::FlowEdge;

    #[test]
    fn test_simple_flow() {
        let mut network = FlowNetwork::new(4);
        network.add_edge(FlowEdge::new(0, 1, 10.0));
        network.add_edge(FlowEdge::new(0, 2, 10.0));
        network.add_edge(FlowEdge::new(1, 3, 10.0));
        network.add_edge(FlowEdge::new(2, 3, 10.0));

        let max_flow = FordFulkerson::new(&mut network, 0, 3);
        assert_eq!(max_flow.value(), 20.0);
    }

    #[test]
    fn test_bottleneck() {
        let mut network = FlowNetwork::new(4);
        network.add_edge(FlowEdge::new(0, 1, 10.0));
        network.add_edge(FlowEdge::new(1, 2, 5.0)); // bottleneck
        network.add_edge(FlowEdge::new(2, 3, 10.0));

        let max_flow = FordFulkerson::new(&mut network, 0, 3);
        assert_eq!(max_flow.value(), 5.0);
    }

    #[test]
    fn test_multiple_paths() {
        let mut network = FlowNetwork::new(6);
        network.add_edge(FlowEdge::new(0, 1, 2.0));
        network.add_edge(FlowEdge::new(0, 2, 3.0));
        network.add_edge(FlowEdge::new(1, 3, 3.0));
        network.add_edge(FlowEdge::new(2, 3, 1.0));
        network.add_edge(FlowEdge::new(2, 4, 1.0));
        network.add_edge(FlowEdge::new(3, 5, 2.0));
        network.add_edge(FlowEdge::new(4, 5, 3.0));

        let max_flow = FordFulkerson::new(&mut network, 0, 5);
        // Max flow is 3.0: path 0->1->3->5 (flow 2) + path 0->2->4->5 (flow 1)
        assert_eq!(max_flow.value(), 3.0);
    }

    #[test]
    fn test_in_cut() {
        let mut network = FlowNetwork::new(4);
        network.add_edge(FlowEdge::new(0, 1, 10.0));
        network.add_edge(FlowEdge::new(1, 3, 5.0));

        let max_flow = FordFulkerson::new(&mut network, 0, 3);
        assert!(max_flow.in_cut(0));
        assert!(max_flow.in_cut(1));
        assert!(!max_flow.in_cut(3));
    }

    #[test]
    #[should_panic(expected = "Source equals sink")]
    fn test_same_source_sink() {
        let mut network = FlowNetwork::new(4);
        FordFulkerson::new(&mut network, 0, 0);
    }
}
