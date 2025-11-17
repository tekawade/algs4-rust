//! Depth-first search paths in a directed graph.

use crate::digraph::Digraph;

/// Finds paths from a source vertex to every other vertex in a directed graph
/// using depth-first search.
///
/// # Examples
///
/// ```
/// use algs4_graphs::{Digraph, DepthFirstDirectedPaths};
///
/// let mut digraph = Digraph::new(6);
/// digraph.add_edge(0, 1);
/// digraph.add_edge(0, 2);
/// digraph.add_edge(1, 3);
/// digraph.add_edge(2, 3);
/// digraph.add_edge(3, 4);
/// digraph.add_edge(4, 5);
///
/// let dfs = DepthFirstDirectedPaths::new(&digraph, 0);
/// assert!(dfs.has_path_to(5));
/// let path = dfs.path_to(5).unwrap();
/// assert!(path.len() >= 3);
/// ```
#[derive(Debug)]
pub struct DepthFirstDirectedPaths {
    marked: Vec<bool>,           // marked[v] = true if v is reachable from source
    edge_to: Vec<Option<usize>>, // edge_to[v] = previous vertex on path from source to v
    s: usize,                    // source vertex
}

impl DepthFirstDirectedPaths {
    /// Computes a path from source vertex `s` to every other vertex in digraph `g`
    /// using depth-first search.
    ///
    /// # Arguments
    ///
    /// * `g` - The directed graph
    /// * `s` - The source vertex
    ///
    /// # Panics
    ///
    /// Panics if `s` is not a valid vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Digraph, DepthFirstDirectedPaths};
    ///
    /// let mut digraph = Digraph::new(5);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    ///
    /// let dfs = DepthFirstDirectedPaths::new(&digraph, 0);
    /// assert!(dfs.has_path_to(2));
    /// ```
    pub fn new(g: &Digraph, s: usize) -> Self {
        let mut dfs = DepthFirstDirectedPaths {
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
    fn dfs(&mut self, g: &Digraph, v: usize) {
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
    /// use algs4_graphs::{Digraph, DepthFirstDirectedPaths};
    ///
    /// let mut digraph = Digraph::new(5);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    ///
    /// let dfs = DepthFirstDirectedPaths::new(&digraph, 0);
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
    /// use algs4_graphs::{Digraph, DepthFirstDirectedPaths};
    ///
    /// let mut digraph = Digraph::new(5);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    /// digraph.add_edge(2, 3);
    ///
    /// let dfs = DepthFirstDirectedPaths::new(&digraph, 0);
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
        let mut digraph = Digraph::new(5);
        digraph.add_edge(0, 1);
        digraph.add_edge(1, 2);

        let dfs = DepthFirstDirectedPaths::new(&digraph, 0);
        assert!(dfs.has_path_to(0));
        assert!(dfs.has_path_to(1));
        assert!(dfs.has_path_to(2));
        assert!(!dfs.has_path_to(3));
    }

    #[test]
    fn test_path_to() {
        let mut digraph = Digraph::new(5);
        digraph.add_edge(0, 1);
        digraph.add_edge(1, 2);
        digraph.add_edge(2, 3);

        let dfs = DepthFirstDirectedPaths::new(&digraph, 0);
        let path = dfs.path_to(3).unwrap();

        assert_eq!(path[0], 0);
        assert_eq!(path[path.len() - 1], 3);
        assert!(path.len() >= 2);
    }

    #[test]
    fn test_no_path() {
        let mut digraph = Digraph::new(5);
        digraph.add_edge(0, 1);
        digraph.add_edge(3, 2);

        let dfs = DepthFirstDirectedPaths::new(&digraph, 0);
        assert!(dfs.path_to(3).is_none());
    }

    #[test]
    fn test_directed_path() {
        let mut digraph = Digraph::new(3);
        digraph.add_edge(0, 1);
        digraph.add_edge(2, 1); // edge from 2 to 1, not 1 to 2

        let dfs = DepthFirstDirectedPaths::new(&digraph, 0);
        assert!(dfs.has_path_to(1));
        assert!(!dfs.has_path_to(2)); // no path from 0 to 2

        let dfs2 = DepthFirstDirectedPaths::new(&digraph, 2);
        assert!(dfs2.has_path_to(1));
        assert!(!dfs2.has_path_to(0));
    }

    #[test]
    fn test_single_vertex() {
        let digraph = Digraph::new(1);
        let dfs = DepthFirstDirectedPaths::new(&digraph, 0);

        assert!(dfs.has_path_to(0));
        let path = dfs.path_to(0).unwrap();
        assert_eq!(path, vec![0]);
    }

    #[test]
    #[should_panic(expected = "vertex 5 is not between 0 and 4")]
    fn test_invalid_source() {
        let digraph = Digraph::new(5);
        DepthFirstDirectedPaths::new(&digraph, 5);
    }
}
