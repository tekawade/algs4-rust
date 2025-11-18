//! Bellman-Ford shortest path algorithm for edge-weighted digraphs.

use crate::{DirectedEdge, EdgeWeightedDigraph};
use std::collections::VecDeque;

/// Computes shortest paths from a source vertex to all other vertices in an
/// edge-weighted digraph using the Bellman-Ford algorithm.
///
/// This implementation can handle negative edge weights and detects negative cycles.
/// The constructor takes O(V * E) time in the worst case, where V is the number
/// of vertices and E is the number of edges.
///
/// # Examples
///
/// ```
/// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, BellmanFordSP};
///
/// let mut graph = EdgeWeightedDigraph::new(8);
/// graph.add_edge(DirectedEdge::new(0, 1, 5.0));
/// graph.add_edge(DirectedEdge::new(0, 4, 9.0));
/// graph.add_edge(DirectedEdge::new(0, 7, 8.0));
/// graph.add_edge(DirectedEdge::new(1, 2, 12.0));
/// graph.add_edge(DirectedEdge::new(1, 3, 15.0));
/// graph.add_edge(DirectedEdge::new(1, 7, 4.0));
/// graph.add_edge(DirectedEdge::new(2, 3, 3.0));
/// graph.add_edge(DirectedEdge::new(2, 6, 11.0));
/// graph.add_edge(DirectedEdge::new(3, 6, 9.0));
/// graph.add_edge(DirectedEdge::new(4, 5, 4.0));
/// graph.add_edge(DirectedEdge::new(4, 6, 20.0));
/// graph.add_edge(DirectedEdge::new(4, 7, 5.0));
/// graph.add_edge(DirectedEdge::new(5, 2, 1.0));
/// graph.add_edge(DirectedEdge::new(5, 6, 13.0));
/// graph.add_edge(DirectedEdge::new(7, 5, 6.0));
/// graph.add_edge(DirectedEdge::new(7, 2, 7.0));
///
/// let sp = BellmanFordSP::new(&graph, 0);
///
/// assert!(!sp.has_negative_cycle());
/// assert!(sp.has_path_to(6));
/// assert_eq!(sp.dist_to(6), Some(25.0));
/// ```
#[derive(Debug, Clone)]
pub struct BellmanFordSP {
    dist_to: Vec<f64>,                  // dist_to[v] = distance of shortest s->v path
    edge_to: Vec<Option<DirectedEdge>>, // edge_to[v] = last edge on shortest s->v path
    on_queue: Vec<bool>,                // on_queue[v] = is v currently on the queue?
    cycle: Option<Vec<DirectedEdge>>,   // negative cycle (or None if no such cycle)
    cost: usize,                        // number of calls to relax()
}

impl BellmanFordSP {
    /// Computes a shortest-paths tree from the source vertex `s` to every other
    /// vertex in the edge-weighted digraph `graph`.
    ///
    /// # Arguments
    ///
    /// * `graph` - The edge-weighted digraph
    /// * `s` - The source vertex
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, BellmanFordSP};
    ///
    /// let mut graph = EdgeWeightedDigraph::new(5);
    /// graph.add_edge(DirectedEdge::new(0, 1, 1.0));
    /// graph.add_edge(DirectedEdge::new(0, 2, 3.0));
    /// graph.add_edge(DirectedEdge::new(1, 2, -2.0)); // negative weight allowed
    ///
    /// let sp = BellmanFordSP::new(&graph, 0);
    /// assert_eq!(sp.dist_to(2), Some(-1.0));
    /// assert!(!sp.has_negative_cycle());
    /// ```
    pub fn new(graph: &EdgeWeightedDigraph, s: usize) -> Self {
        let v = graph.v();
        let mut dist_to = vec![f64::INFINITY; v];
        let edge_to = vec![None; v];
        let mut on_queue = vec![false; v];
        let cycle = None;
        let cost = 0;

        dist_to[s] = 0.0;

        // Bellman-Ford algorithm using queue-based implementation
        let mut queue = VecDeque::new();
        queue.push_back(s);
        on_queue[s] = true;

        let mut sp = BellmanFordSP {
            dist_to,
            edge_to,
            on_queue,
            cycle,
            cost,
        };

        while !queue.is_empty() && sp.cycle.is_none() {
            let v = queue.pop_front().unwrap();
            sp.on_queue[v] = false;
            sp.relax(graph, v, &mut queue);
        }

        sp
    }

    // Relax vertex v and put other endpoints on queue if changed
    fn relax(&mut self, graph: &EdgeWeightedDigraph, v: usize, queue: &mut VecDeque<usize>) {
        for &e in graph.adj(v) {
            let w = e.to();
            if self.dist_to[w] > self.dist_to[v] + e.weight() {
                self.dist_to[w] = self.dist_to[v] + e.weight();
                self.edge_to[w] = Some(e);
                if !self.on_queue[w] {
                    queue.push_back(w);
                    self.on_queue[w] = true;
                }
            }
            self.cost += 1;
            #[allow(clippy::manual_is_multiple_of)]
            if self.cost % graph.v() == 0 {
                self.find_negative_cycle();
                if self.has_negative_cycle() {
                    return;
                }
            }
        }
    }

    // Find a negative cycle by following edges
    fn find_negative_cycle(&mut self) {
        let v = self.edge_to.len();
        let mut spt = EdgeWeightedDigraph::new(v);

        for i in 0..v {
            if let Some(e) = self.edge_to[i] {
                spt.add_edge(e);
            }
        }

        let finder = EdgeWeightedCycleFinder::new(&spt);
        self.cycle = finder.cycle();
    }

    /// Returns `true` if there is a negative cycle reachable from the source vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, BellmanFordSP};
    ///
    /// let mut graph = EdgeWeightedDigraph::new(3);
    /// graph.add_edge(DirectedEdge::new(0, 1, 1.0));
    /// graph.add_edge(DirectedEdge::new(1, 2, -2.0));
    /// graph.add_edge(DirectedEdge::new(2, 0, -1.0)); // negative cycle
    ///
    /// let sp = BellmanFordSP::new(&graph, 0);
    /// assert!(sp.has_negative_cycle());
    /// ```
    pub fn has_negative_cycle(&self) -> bool {
        self.cycle.is_some()
    }

    /// Returns a negative cycle reachable from the source vertex.
    ///
    /// # Returns
    ///
    /// * `Some(cycle)` - A vector of edges representing the negative cycle
    /// * `None` - If there is no negative cycle
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, BellmanFordSP};
    ///
    /// let mut graph = EdgeWeightedDigraph::new(3);
    /// graph.add_edge(DirectedEdge::new(0, 1, 1.0));
    /// graph.add_edge(DirectedEdge::new(1, 2, -2.0));
    /// graph.add_edge(DirectedEdge::new(2, 0, -1.0)); // negative cycle
    ///
    /// let sp = BellmanFordSP::new(&graph, 0);
    /// assert!(sp.negative_cycle().is_some());
    /// ```
    pub fn negative_cycle(&self) -> Option<Vec<DirectedEdge>> {
        self.cycle.clone()
    }

    /// Returns the length of a shortest path from the source vertex `s` to vertex `v`.
    ///
    /// # Arguments
    ///
    /// * `v` - The destination vertex
    ///
    /// # Returns
    ///
    /// * `Some(dist)` - The distance if there is a path and no negative cycle
    /// * `None` - If there is no path or a negative cycle exists
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, BellmanFordSP};
    ///
    /// let mut graph = EdgeWeightedDigraph::new(5);
    /// graph.add_edge(DirectedEdge::new(0, 1, 1.0));
    /// graph.add_edge(DirectedEdge::new(1, 2, 2.0));
    ///
    /// let sp = BellmanFordSP::new(&graph, 0);
    /// assert_eq!(sp.dist_to(2), Some(3.0));
    /// assert_eq!(sp.dist_to(4), None);
    /// ```
    pub fn dist_to(&self, v: usize) -> Option<f64> {
        if self.has_negative_cycle() {
            return None;
        }
        if self.dist_to[v].is_finite() {
            Some(self.dist_to[v])
        } else {
            None
        }
    }

    /// Returns `true` if there is a path from the source vertex to vertex `v`.
    ///
    /// # Arguments
    ///
    /// * `v` - The destination vertex
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, BellmanFordSP};
    ///
    /// let mut graph = EdgeWeightedDigraph::new(5);
    /// graph.add_edge(DirectedEdge::new(0, 1, 1.0));
    ///
    /// let sp = BellmanFordSP::new(&graph, 0);
    /// assert!(sp.has_path_to(1));
    /// assert!(!sp.has_path_to(4));
    /// ```
    pub fn has_path_to(&self, v: usize) -> bool {
        !self.has_negative_cycle() && self.dist_to[v].is_finite()
    }

    /// Returns a shortest path from the source vertex to vertex `v`.
    ///
    /// # Arguments
    ///
    /// * `v` - The destination vertex
    ///
    /// # Returns
    ///
    /// * `Some(path)` - A vector of edges representing the shortest path
    /// * `None` - If there is no path or a negative cycle exists
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{EdgeWeightedDigraph, DirectedEdge, BellmanFordSP};
    ///
    /// let mut graph = EdgeWeightedDigraph::new(5);
    /// graph.add_edge(DirectedEdge::new(0, 1, 1.0));
    /// graph.add_edge(DirectedEdge::new(1, 2, 2.0));
    ///
    /// let sp = BellmanFordSP::new(&graph, 0);
    /// let path = sp.path_to(2).unwrap();
    /// assert_eq!(path.len(), 2);
    /// ```
    pub fn path_to(&self, v: usize) -> Option<Vec<DirectedEdge>> {
        if !self.has_path_to(v) {
            return None;
        }

        let mut path = Vec::new();
        let mut current = v;
        while let Some(e) = self.edge_to[current] {
            path.push(e);
            current = e.from();
        }
        path.reverse();
        Some(path)
    }
}

// Helper struct to find cycles in edge-weighted digraphs
struct EdgeWeightedCycleFinder {
    marked: Vec<bool>,
    on_stack: Vec<bool>,
    edge_to: Vec<Option<DirectedEdge>>,
    cycle: Option<Vec<DirectedEdge>>,
}

impl EdgeWeightedCycleFinder {
    fn new(graph: &EdgeWeightedDigraph) -> Self {
        let v = graph.v();
        let mut finder = EdgeWeightedCycleFinder {
            marked: vec![false; v],
            on_stack: vec![false; v],
            edge_to: vec![None; v],
            cycle: None,
        };

        for s in 0..v {
            if !finder.marked[s] {
                finder.dfs(graph, s);
            }
        }

        finder
    }

    fn dfs(&mut self, graph: &EdgeWeightedDigraph, v: usize) {
        self.on_stack[v] = true;
        self.marked[v] = true;

        for &e in graph.adj(v) {
            let w = e.to();

            // Short circuit if cycle already found
            if self.cycle.is_some() {
                return;
            }

            if !self.marked[w] {
                self.edge_to[w] = Some(e);
                self.dfs(graph, w);
            } else if self.on_stack[w] {
                // Found a cycle
                let mut cycle = Vec::new();
                let mut f = e;
                loop {
                    cycle.push(f);
                    if f.from() == w {
                        break;
                    }
                    f = self.edge_to[f.from()].unwrap();
                }
                self.cycle = Some(cycle);
                return;
            }
        }

        self.on_stack[v] = false;
    }

    fn cycle(&self) -> Option<Vec<DirectedEdge>> {
        self.cycle.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_graph() -> EdgeWeightedDigraph {
        let mut graph = EdgeWeightedDigraph::new(8);
        graph.add_edge(DirectedEdge::new(0, 1, 5.0));
        graph.add_edge(DirectedEdge::new(0, 4, 9.0));
        graph.add_edge(DirectedEdge::new(0, 7, 8.0));
        graph.add_edge(DirectedEdge::new(1, 2, 12.0));
        graph.add_edge(DirectedEdge::new(1, 3, 15.0));
        graph.add_edge(DirectedEdge::new(1, 7, 4.0));
        graph.add_edge(DirectedEdge::new(2, 3, 3.0));
        graph.add_edge(DirectedEdge::new(2, 6, 11.0));
        graph.add_edge(DirectedEdge::new(3, 6, 9.0));
        graph.add_edge(DirectedEdge::new(4, 5, 4.0));
        graph.add_edge(DirectedEdge::new(4, 6, 20.0));
        graph.add_edge(DirectedEdge::new(4, 7, 5.0));
        graph.add_edge(DirectedEdge::new(5, 2, 1.0));
        graph.add_edge(DirectedEdge::new(5, 6, 13.0));
        graph.add_edge(DirectedEdge::new(7, 5, 6.0));
        graph.add_edge(DirectedEdge::new(7, 2, 7.0));
        graph
    }

    #[test]
    fn test_bellman_ford_basic() {
        let graph = create_test_graph();
        let sp = BellmanFordSP::new(&graph, 0);

        assert!(!sp.has_negative_cycle());
        assert_eq!(sp.dist_to(0), Some(0.0));
        assert_eq!(sp.dist_to(1), Some(5.0));
        assert_eq!(sp.dist_to(2), Some(14.0));
        assert_eq!(sp.dist_to(3), Some(17.0));
        assert_eq!(sp.dist_to(4), Some(9.0));
        assert_eq!(sp.dist_to(5), Some(13.0));
        assert_eq!(sp.dist_to(6), Some(25.0));
        assert_eq!(sp.dist_to(7), Some(8.0));
    }

    #[test]
    fn test_negative_weights() {
        let mut graph = EdgeWeightedDigraph::new(5);
        graph.add_edge(DirectedEdge::new(0, 1, 1.0));
        graph.add_edge(DirectedEdge::new(1, 2, -2.0));
        graph.add_edge(DirectedEdge::new(2, 3, 3.0));

        let sp = BellmanFordSP::new(&graph, 0);
        assert!(!sp.has_negative_cycle());
        assert_eq!(sp.dist_to(3), Some(2.0));
    }

    #[test]
    fn test_negative_cycle() {
        let mut graph = EdgeWeightedDigraph::new(3);
        graph.add_edge(DirectedEdge::new(0, 1, 1.0));
        graph.add_edge(DirectedEdge::new(1, 2, -2.0));
        graph.add_edge(DirectedEdge::new(2, 0, -1.0));

        let sp = BellmanFordSP::new(&graph, 0);
        assert!(sp.has_negative_cycle());
        assert!(sp.negative_cycle().is_some());
    }

    #[test]
    fn test_has_path_to() {
        let graph = create_test_graph();
        let sp = BellmanFordSP::new(&graph, 0);

        for v in 0..8 {
            assert!(sp.has_path_to(v));
        }
    }

    #[test]
    fn test_path_to() {
        let graph = create_test_graph();
        let sp = BellmanFordSP::new(&graph, 0);

        let path = sp.path_to(6).unwrap();
        let mut total_weight = 0.0;
        for e in &path {
            total_weight += e.weight();
        }
        assert_eq!(total_weight, 25.0);
    }

    #[test]
    fn test_disconnected_graph() {
        let mut graph = EdgeWeightedDigraph::new(5);
        graph.add_edge(DirectedEdge::new(0, 1, 1.0));
        graph.add_edge(DirectedEdge::new(3, 4, 1.0));

        let sp = BellmanFordSP::new(&graph, 0);
        assert!(sp.has_path_to(1));
        assert!(!sp.has_path_to(3));
        assert_eq!(sp.dist_to(3), None);
        assert_eq!(sp.path_to(3), None);
    }

    #[test]
    fn test_empty_graph() {
        let graph = EdgeWeightedDigraph::new(5);
        let sp = BellmanFordSP::new(&graph, 0);

        assert_eq!(sp.dist_to(0), Some(0.0));
        for v in 1..5 {
            assert_eq!(sp.dist_to(v), None);
            assert!(!sp.has_path_to(v));
        }
    }
}
