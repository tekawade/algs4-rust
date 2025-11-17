//! Depth-first search paths in an undirected graph.

use crate::graph::Graph;

/// Finds paths from a source vertex to every other vertex in an undirected graph
/// using depth-first search.
///
/// # Examples
///
/// ```
/// use algs4_graphs::{Graph, DepthFirstPaths};
///
/// let mut graph = Graph::new(6);
/// graph.add_edge(0, 1);
/// graph.add_edge(0, 2);
/// graph.add_edge(1, 3);
/// graph.add_edge(2, 3);
/// graph.add_edge(3, 4);
/// graph.add_edge(4, 5);
///
/// let dfs = DepthFirstPaths::new(&graph, 0);
/// assert!(dfs.has_path_to(5));
/// let path = dfs.path_to(5).unwrap();
/// assert!(path.len() >= 3);
/// ```
#[derive(Debug)]
pub struct DepthFirstPaths {
    marked: Vec<bool>,           // marked[v] = true if v is connected to source
    edge_to: Vec<Option<usize>>, // edge_to[v] = previous vertex on path from source to v
    s: usize,                    // source vertex
}

impl DepthFirstPaths {
    /// Computes a path from source vertex `s` to every other vertex in graph `g`
    /// using depth-first search.
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
    /// use algs4_graphs::{Graph, DepthFirstPaths};
    ///
    /// let mut graph = Graph::new(5);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    ///
    /// let dfs = DepthFirstPaths::new(&graph, 0);
    /// assert!(dfs.has_path_to(2));
    /// ```
    pub fn new(g: &Graph, s: usize) -> Self {
        let mut dfs = DepthFirstPaths {
            marked: vec![false; g.v()],
            edge_to: vec![None; g.v()],
            s,
        };
        dfs.validate_vertex(s, g.v());
        dfs.dfs(g, s);
        dfs
    }

    /// Validates that vertex v is a valid vertex.
    fn validate_vertex(&self, v: usize, max: usize) {
        if v >= max {
            panic!("vertex {} is not between 0 and {}", v, max - 1);
        }
    }

    /// Depth-first search from vertex v.
    fn dfs(&mut self, g: &Graph, v: usize) {
        self.marked[v] = true;
        for &w in g.adj(v) {
            if !self.marked[w] {
                self.edge_to[w] = Some(v);
                self.dfs(g, w);
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
    /// use algs4_graphs::{Graph, DepthFirstPaths};
    ///
    /// let mut graph = Graph::new(5);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    ///
    /// let dfs = DepthFirstPaths::new(&graph, 0);
    /// assert!(dfs.has_path_to(2));
    /// assert!(!dfs.has_path_to(3));
    /// ```
    pub fn has_path_to(&self, v: usize) -> bool {
        self.validate_vertex(v, self.marked.len());
        self.marked[v]
    }

    /// Returns a path from the source vertex to vertex `v`, or `None` if no such path.
    ///
    /// # Arguments
    ///
    /// * `v` - The destination vertex
    ///
    /// # Returns
    ///
    /// A vector containing the sequence of vertices on a path from the source
    /// to vertex `v`, or `None` if no such path exists.
    ///
    /// # Panics
    ///
    /// Panics if `v` is not a valid vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Graph, DepthFirstPaths};
    ///
    /// let mut graph = Graph::new(5);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    /// graph.add_edge(2, 3);
    ///
    /// let dfs = DepthFirstPaths::new(&graph, 0);
    /// let path = dfs.path_to(3).unwrap();
    /// assert_eq!(path[0], 0);
    /// assert_eq!(path[path.len() - 1], 3);
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

        let dfs = DepthFirstPaths::new(&graph, 0);
        assert!(dfs.has_path_to(0));
        assert!(dfs.has_path_to(1));
        assert!(dfs.has_path_to(2));
        assert!(!dfs.has_path_to(3));
    }

    #[test]
    fn test_path_to() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);

        let dfs = DepthFirstPaths::new(&graph, 0);
        let path = dfs.path_to(3).unwrap();

        assert_eq!(path[0], 0);
        assert_eq!(path[path.len() - 1], 3);
        assert!(path.len() >= 2);
    }

    #[test]
    fn test_no_path() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(2, 3);

        let dfs = DepthFirstPaths::new(&graph, 0);
        assert!(dfs.path_to(3).is_none());
    }

    #[test]
    fn test_single_vertex() {
        let graph = Graph::new(1);
        let dfs = DepthFirstPaths::new(&graph, 0);

        assert!(dfs.has_path_to(0));
        let path = dfs.path_to(0).unwrap();
        assert_eq!(path, vec![0]);
    }

    #[test]
    #[should_panic(expected = "vertex 5 is not between 0 and 4")]
    fn test_invalid_source() {
        let graph = Graph::new(5);
        DepthFirstPaths::new(&graph, 5);
    }
}
