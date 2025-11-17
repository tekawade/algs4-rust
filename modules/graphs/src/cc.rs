//! Connected components in an undirected graph.

use crate::graph::Graph;

/// Computes the connected components of an undirected graph using depth-first search.
///
/// The component identifier of a connected component is one of the vertices in
/// the component (it doesn't matter which one).
///
/// # Examples
///
/// ```
/// use algs4_graphs::{Graph, CC};
///
/// let mut graph = Graph::new(7);
/// graph.add_edge(0, 1);
/// graph.add_edge(1, 2);
/// graph.add_edge(3, 4);
/// graph.add_edge(5, 6);
///
/// let cc = CC::new(&graph);
/// assert_eq!(cc.count(), 3);
/// assert!(cc.connected(0, 1));
/// assert!(cc.connected(1, 2));
/// assert!(!cc.connected(0, 3));
/// ```
#[derive(Debug)]
pub struct CC {
    marked: Vec<bool>, // marked[v] = has vertex v been marked?
    id: Vec<usize>,    // id[v] = component identifier for v
    size: Vec<usize>,  // size[id] = number of vertices in component id
    count: usize,      // number of connected components
}

impl CC {
    /// Computes the connected components of the undirected graph `g`.
    ///
    /// # Arguments
    ///
    /// * `g` - The undirected graph
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Graph, CC};
    ///
    /// let mut graph = Graph::new(5);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(2, 3);
    ///
    /// let cc = CC::new(&graph);
    /// assert_eq!(cc.count(), 3);  // {0,1}, {2,3}, {4}
    /// ```
    pub fn new(g: &Graph) -> Self {
        let mut cc = CC {
            marked: vec![false; g.v()],
            id: vec![0; g.v()],
            size: vec![0; g.v()],
            count: 0,
        };

        for v in 0..g.v() {
            if !cc.marked[v] {
                cc.dfs(g, v);
                cc.count += 1;
            }
        }

        cc
    }

    /// Depth-first search for connected component containing vertex v.
    fn dfs(&mut self, g: &Graph, v: usize) {
        self.marked[v] = true;
        self.id[v] = self.count;
        self.size[self.count] += 1;
        for &w in g.adj(v) {
            if !self.marked[w] {
                self.dfs(g, w);
            }
        }
    }

    /// Validates that vertex v is a valid vertex.
    fn validate_vertex(&self, v: usize) {
        if v >= self.marked.len() {
            panic!(
                "vertex {} is not between 0 and {}",
                v,
                self.marked.len() - 1
            );
        }
    }

    /// Returns the component identifier for the connected component containing vertex `v`.
    ///
    /// # Arguments
    ///
    /// * `v` - The vertex
    ///
    /// # Returns
    ///
    /// The component identifier for the connected component containing vertex `v`
    ///
    /// # Panics
    ///
    /// Panics if `v` is not a valid vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Graph, CC};
    ///
    /// let mut graph = Graph::new(5);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    ///
    /// let cc = CC::new(&graph);
    /// assert_eq!(cc.id(0), cc.id(1));
    /// assert_eq!(cc.id(0), cc.id(2));
    /// assert_ne!(cc.id(0), cc.id(3));
    /// ```
    pub fn id(&self, v: usize) -> usize {
        self.validate_vertex(v);
        self.id[v]
    }

    /// Returns the number of vertices in the connected component containing vertex `v`.
    ///
    /// # Arguments
    ///
    /// * `v` - The vertex
    ///
    /// # Returns
    ///
    /// The number of vertices in the connected component containing vertex `v`
    ///
    /// # Panics
    ///
    /// Panics if `v` is not a valid vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Graph, CC};
    ///
    /// let mut graph = Graph::new(5);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    ///
    /// let cc = CC::new(&graph);
    /// assert_eq!(cc.size(0), 3);
    /// assert_eq!(cc.size(3), 1);
    /// ```
    pub fn size(&self, v: usize) -> usize {
        self.validate_vertex(v);
        self.size[self.id[v]]
    }

    /// Returns the number of connected components in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Graph, CC};
    ///
    /// let mut graph = Graph::new(5);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(2, 3);
    ///
    /// let cc = CC::new(&graph);
    /// assert_eq!(cc.count(), 3);
    /// ```
    pub fn count(&self) -> usize {
        self.count
    }

    /// Returns true if vertices `v` and `w` are in the same connected component.
    ///
    /// # Arguments
    ///
    /// * `v` - One vertex
    /// * `w` - The other vertex
    ///
    /// # Returns
    ///
    /// `true` if vertices `v` and `w` are in the same connected component; `false` otherwise
    ///
    /// # Panics
    ///
    /// Panics if either vertex is not valid.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::{Graph, CC};
    ///
    /// let mut graph = Graph::new(5);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    /// graph.add_edge(3, 4);
    ///
    /// let cc = CC::new(&graph);
    /// assert!(cc.connected(0, 1));
    /// assert!(cc.connected(1, 2));
    /// assert!(cc.connected(3, 4));
    /// assert!(!cc.connected(0, 3));
    /// ```
    pub fn connected(&self, v: usize, w: usize) -> bool {
        self.validate_vertex(v);
        self.validate_vertex(w);
        self.id(v) == self.id(w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_component() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let cc = CC::new(&graph);
        assert_eq!(cc.count(), 1);
        assert!(cc.connected(0, 4));
        assert_eq!(cc.size(0), 5);
    }

    #[test]
    fn test_multiple_components() {
        let mut graph = Graph::new(7);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(3, 4);
        graph.add_edge(5, 6);

        let cc = CC::new(&graph);
        assert_eq!(cc.count(), 3);
        assert!(cc.connected(0, 1));
        assert!(cc.connected(1, 2));
        assert!(cc.connected(3, 4));
        assert!(cc.connected(5, 6));
        assert!(!cc.connected(0, 3));
        assert!(!cc.connected(0, 5));
    }

    #[test]
    fn test_component_size() {
        let mut graph = Graph::new(7);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(3, 4);

        let cc = CC::new(&graph);
        assert_eq!(cc.size(0), 3);
        assert_eq!(cc.size(1), 3);
        assert_eq!(cc.size(2), 3);
        assert_eq!(cc.size(3), 2);
        assert_eq!(cc.size(4), 2);
        assert_eq!(cc.size(5), 1);
        assert_eq!(cc.size(6), 1);
    }

    #[test]
    fn test_empty_graph() {
        let graph = Graph::new(5);
        let cc = CC::new(&graph);
        assert_eq!(cc.count(), 5);
        for i in 0..5 {
            assert_eq!(cc.size(i), 1);
        }
    }

    #[test]
    fn test_fully_connected() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(0, 3);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 3);

        let cc = CC::new(&graph);
        assert_eq!(cc.count(), 1);
        assert_eq!(cc.size(0), 4);
        for i in 0..4 {
            for j in 0..4 {
                assert!(cc.connected(i, j));
            }
        }
    }

    #[test]
    #[should_panic(expected = "vertex 5 is not between 0 and 4")]
    fn test_invalid_vertex() {
        let graph = Graph::new(5);
        let cc = CC::new(&graph);
        cc.id(5);
    }
}
