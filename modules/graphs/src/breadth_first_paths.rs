//! Breadth-first search paths in an undirected graph.

use crate::graph::Graph;
use std::collections::VecDeque;

/// Finds shortest paths from a source vertex to every other vertex in an
/// undirected graph using breadth-first search.
///
/// # Examples
///
/// ```
/// use algs4_graphs::{Graph, BreadthFirstPaths};
///
/// let mut graph = Graph::new(6);
/// graph.add_edge(0, 1);
/// graph.add_edge(0, 2);
/// graph.add_edge(1, 3);
/// graph.add_edge(2, 3);
/// graph.add_edge(3, 4);
/// graph.add_edge(4, 5);
///
/// let bfs = BreadthFirstPaths::new(&graph, 0);
/// assert!(bfs.has_path_to(5));
/// assert_eq!(bfs.dist_to(5).unwrap(), 4);
/// ```
#[derive(Debug)]
pub struct BreadthFirstPaths {
    marked: Vec<bool>,           // marked[v] = true if v is connected to source
    edge_to: Vec<Option<usize>>, // edge_to[v] = previous vertex on shortest path from source to v
    dist_to: Vec<Option<usize>>, // dist_to[v] = number of edges on shortest path from source to v
    s: usize,                    // source vertex
}

impl BreadthFirstPaths {
    /// Computes the shortest path from source vertex `s` to every other vertex
    /// in graph `g` using breadth-first search.
    ///
    /// # Arguments
    ///
    /// * `g` - The undirected graph
    /// * `s` - The source vertex
    ///
    /// # Panics
    ///
    /// Panics if `s` is not a valid vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Graph, BreadthFirstPaths};
    ///
    /// let mut graph = Graph::new(5);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    ///
    /// let bfs = BreadthFirstPaths::new(&graph, 0);
    /// assert!(bfs.has_path_to(2));
    /// assert_eq!(bfs.dist_to(2).unwrap(), 2);
    /// ```
    pub fn new(g: &Graph, s: usize) -> Self {
        let mut bfs = BreadthFirstPaths {
            marked: vec![false; g.v()],
            edge_to: vec![None; g.v()],
            dist_to: vec![None; g.v()],
            s,
        };
        bfs.validate_vertex(s, g.v());
        bfs.bfs(g, s);
        bfs
    }

    /// Validates that vertex v is a valid vertex.
    fn validate_vertex(&self, v: usize, max: usize) {
        if v >= max {
            panic!("vertex {} is not between 0 and {}", v, max - 1);
        }
    }

    /// Breadth-first search from vertex s.
    fn bfs(&mut self, g: &Graph, s: usize) {
        let mut queue = VecDeque::new();
        self.marked[s] = true;
        self.dist_to[s] = Some(0);
        queue.push_back(s);

        while let Some(v) = queue.pop_front() {
            for &w in g.adj(v) {
                if !self.marked[w] {
                    self.edge_to[w] = Some(v);
                    self.dist_to[w] = Some(self.dist_to[v].unwrap() + 1);
                    self.marked[w] = true;
                    queue.push_back(w);
                }
            }
        }
    }

    /// Returns true if there is a path from the source vertex to vertex `v`.
    ///
    /// # Arguments
    ///
    /// * `v` - The destination vertex
    ///
    /// # Panics
    ///
    /// Panics if `v` is not a valid vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Graph, BreadthFirstPaths};
    ///
    /// let mut graph = Graph::new(5);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    ///
    /// let bfs = BreadthFirstPaths::new(&graph, 0);
    /// assert!(bfs.has_path_to(2));
    /// assert!(!bfs.has_path_to(3));
    /// ```
    pub fn has_path_to(&self, v: usize) -> bool {
        self.validate_vertex(v, self.marked.len());
        self.marked[v]
    }

    /// Returns the number of edges in a shortest path from the source vertex
    /// to vertex `v`, or `None` if no such path.
    ///
    /// # Arguments
    ///
    /// * `v` - The destination vertex
    ///
    /// # Panics
    ///
    /// Panics if `v` is not a valid vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Graph, BreadthFirstPaths};
    ///
    /// let mut graph = Graph::new(5);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    /// graph.add_edge(2, 3);
    ///
    /// let bfs = BreadthFirstPaths::new(&graph, 0);
    /// assert_eq!(bfs.dist_to(3).unwrap(), 3);
    /// assert_eq!(bfs.dist_to(4), None);
    /// ```
    pub fn dist_to(&self, v: usize) -> Option<usize> {
        self.validate_vertex(v, self.marked.len());
        self.dist_to[v]
    }

    /// Returns a shortest path from the source vertex to vertex `v`,
    /// or `None` if no such path.
    ///
    /// # Arguments
    ///
    /// * `v` - The destination vertex
    ///
    /// # Returns
    ///
    /// A vector containing the sequence of vertices on a shortest path
    /// from the source to vertex `v`, or `None` if no such path exists.
    ///
    /// # Panics
    ///
    /// Panics if `v` is not a valid vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Graph, BreadthFirstPaths};
    ///
    /// let mut graph = Graph::new(5);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    /// graph.add_edge(2, 3);
    ///
    /// let bfs = BreadthFirstPaths::new(&graph, 0);
    /// let path = bfs.path_to(3).unwrap();
    /// assert_eq!(path, vec![0, 1, 2, 3]);
    /// ```
    pub fn path_to(&self, v: usize) -> Option<Vec<usize>> {
        self.validate_vertex(v, self.marked.len());
        if !self.has_path_to(v) {
            return None;
        }

        let mut path = Vec::new();
        let mut x = v;
        while x != self.s {
            path.push(x);
            x = self.edge_to[x].unwrap();
        }
        path.push(self.s);
        path.reverse();
        Some(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let bfs = BreadthFirstPaths::new(&graph, 0);
        assert!(bfs.has_path_to(0));
        assert!(bfs.has_path_to(1));
        assert!(bfs.has_path_to(2));
        assert!(!bfs.has_path_to(3));
    }

    #[test]
    fn test_shortest_path() {
        let mut graph = Graph::new(6);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);
        graph.add_edge(4, 5);

        let bfs = BreadthFirstPaths::new(&graph, 0);

        // Shortest path from 0 to 5 should be 0->1->3->4->5 or 0->2->3->4->5 (length 4)
        assert_eq!(bfs.dist_to(5).unwrap(), 4);

        let path = bfs.path_to(5).unwrap();
        assert_eq!(path[0], 0);
        assert_eq!(path[path.len() - 1], 5);
        assert_eq!(path.len(), 5);
    }

    #[test]
    fn test_dist_to() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);

        let bfs = BreadthFirstPaths::new(&graph, 0);
        assert_eq!(bfs.dist_to(0).unwrap(), 0);
        assert_eq!(bfs.dist_to(1).unwrap(), 1);
        assert_eq!(bfs.dist_to(2).unwrap(), 2);
        assert_eq!(bfs.dist_to(3).unwrap(), 3);
        assert_eq!(bfs.dist_to(4), None);
    }

    #[test]
    fn test_no_path() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(2, 3);

        let bfs = BreadthFirstPaths::new(&graph, 0);
        assert!(bfs.path_to(3).is_none());
        assert_eq!(bfs.dist_to(3), None);
    }

    #[test]
    fn test_single_vertex() {
        let graph = Graph::new(1);
        let bfs = BreadthFirstPaths::new(&graph, 0);

        assert!(bfs.has_path_to(0));
        assert_eq!(bfs.dist_to(0).unwrap(), 0);
        let path = bfs.path_to(0).unwrap();
        assert_eq!(path, vec![0]);
    }

    #[test]
    #[should_panic(expected = "vertex 5 is not between 0 and 4")]
    fn test_invalid_source() {
        let graph = Graph::new(5);
        BreadthFirstPaths::new(&graph, 5);
    }
}
