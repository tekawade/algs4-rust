//! Undirected graph implementation using adjacency lists.
//!
//! This implementation uses adjacency lists to represent an undirected graph
//! where vertices are labeled 0 through V-1.

use std::fmt;

/// An undirected graph, implemented using adjacency lists.
///
/// Supports adding edges and iterating over vertices adjacent to a given vertex.
/// Parallel edges and self-loops are permitted.
///
/// # Examples
///
/// ```
/// use algs4_graphs::Graph;
///
/// let mut graph = Graph::new(6);
/// graph.add_edge(0, 1);
/// graph.add_edge(0, 2);
/// graph.add_edge(1, 2);
///
/// assert_eq!(graph.v(), 6);
/// assert_eq!(graph.e(), 3);
/// assert_eq!(graph.degree(0), 2);
/// ```
#[derive(Debug, Clone)]
pub struct Graph {
    v: usize,              // number of vertices
    e: usize,              // number of edges
    adj: Vec<Vec<usize>>,  // adjacency lists
}

impl Graph {
    /// Creates a new undirected graph with `v` vertices and no edges.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::Graph;
    ///
    /// let graph = Graph::new(10);
    /// assert_eq!(graph.v(), 10);
    /// assert_eq!(graph.e(), 0);
    /// ```
    pub fn new(v: usize) -> Self {
        Graph {
            v,
            e: 0,
            adj: vec![Vec::new(); v],
        }
    }

    /// Returns the number of vertices in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::Graph;
    ///
    /// let graph = Graph::new(5);
    /// assert_eq!(graph.v(), 5);
    /// ```
    pub fn v(&self) -> usize {
        self.v
    }

    /// Returns the number of edges in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::Graph;
    ///
    /// let mut graph = Graph::new(5);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    /// assert_eq!(graph.e(), 2);
    /// ```
    pub fn e(&self) -> usize {
        self.e
    }

    /// Validates that vertex v is a valid vertex in the graph.
    fn validate_vertex(&self, v: usize) {
        if v >= self.v {
            panic!("vertex {} is not between 0 and {}", v, self.v - 1);
        }
    }

    /// Adds an undirected edge between vertices `v` and `w`.
    ///
    /// # Arguments
    ///
    /// * `v` - One vertex in the edge
    /// * `w` - The other vertex in the edge
    ///
    /// # Panics
    ///
    /// Panics if either vertex is not between 0 and V-1.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::Graph;
    ///
    /// let mut graph = Graph::new(3);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(1, 2);
    /// assert_eq!(graph.e(), 2);
    /// ```
    pub fn add_edge(&mut self, v: usize, w: usize) {
        self.validate_vertex(v);
        self.validate_vertex(w);
        self.e += 1;
        self.adj[v].push(w);
        self.adj[w].push(v);
    }

    /// Returns the vertices adjacent to vertex `v`.
    ///
    /// # Arguments
    ///
    /// * `v` - The vertex
    ///
    /// # Returns
    ///
    /// A slice containing the vertices adjacent to vertex `v`
    ///
    /// # Panics
    ///
    /// Panics if vertex is not between 0 and V-1.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::Graph;
    ///
    /// let mut graph = Graph::new(4);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(0, 2);
    /// graph.add_edge(0, 3);
    ///
    /// let neighbors: Vec<usize> = graph.adj(0).iter().copied().collect();
    /// assert_eq!(neighbors.len(), 3);
    /// ```
    pub fn adj(&self, v: usize) -> &[usize] {
        self.validate_vertex(v);
        &self.adj[v]
    }

    /// Returns the degree of vertex `v`.
    ///
    /// # Arguments
    ///
    /// * `v` - The vertex
    ///
    /// # Returns
    ///
    /// The number of edges incident to vertex `v`
    ///
    /// # Panics
    ///
    /// Panics if vertex is not between 0 and V-1.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::Graph;
    ///
    /// let mut graph = Graph::new(4);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(0, 2);
    /// graph.add_edge(0, 3);
    ///
    /// assert_eq!(graph.degree(0), 3);
    /// assert_eq!(graph.degree(1), 1);
    /// ```
    pub fn degree(&self, v: usize) -> usize {
        self.validate_vertex(v);
        self.adj[v].len()
    }

    /// Returns the maximum degree of any vertex in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::Graph;
    ///
    /// let mut graph = Graph::new(4);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(0, 2);
    /// graph.add_edge(0, 3);
    /// graph.add_edge(1, 2);
    ///
    /// assert_eq!(graph.max_degree(), 3);
    /// ```
    pub fn max_degree(&self) -> usize {
        (0..self.v).map(|v| self.degree(v)).max().unwrap_or(0)
    }

    /// Returns the average degree of the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::Graph;
    ///
    /// let mut graph = Graph::new(4);
    /// graph.add_edge(0, 1);
    /// graph.add_edge(0, 2);
    /// graph.add_edge(1, 2);
    ///
    /// assert_eq!(graph.average_degree(), 1.5);
    /// ```
    pub fn average_degree(&self) -> f64 {
        2.0 * self.e as f64 / self.v as f64
    }

    /// Returns the number of self-loops in the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::Graph;
    ///
    /// let mut graph = Graph::new(3);
    /// graph.add_edge(0, 0);  // self-loop
    /// graph.add_edge(1, 2);
    ///
    /// assert_eq!(graph.number_of_self_loops(), 1);
    /// ```
    pub fn number_of_self_loops(&self) -> usize {
        let mut count = 0;
        for v in 0..self.v {
            for &w in &self.adj[v] {
                if v == w {
                    count += 1;
                }
            }
        }
        count / 2  // each self-loop counted twice
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} vertices, {} edges", self.v, self.e)?;
        for v in 0..self.v {
            write!(f, "{}: ", v)?;
            for w in &self.adj[v] {
                write!(f, "{} ", w)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_graph() {
        let graph = Graph::new(5);
        assert_eq!(graph.v(), 5);
        assert_eq!(graph.e(), 0);
    }

    #[test]
    fn test_add_edge() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        assert_eq!(graph.e(), 1);
        assert_eq!(graph.degree(0), 1);
        assert_eq!(graph.degree(1), 1);
    }

    #[test]
    fn test_adjacency() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(0, 3);

        assert_eq!(graph.adj(0).len(), 3);
        assert!(graph.adj(0).contains(&1));
        assert!(graph.adj(0).contains(&2));
        assert!(graph.adj(0).contains(&3));
    }

    #[test]
    fn test_degree() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(0, 3);
        graph.add_edge(1, 2);

        assert_eq!(graph.degree(0), 3);
        assert_eq!(graph.degree(1), 2);
        assert_eq!(graph.degree(2), 2);
        assert_eq!(graph.degree(3), 1);
    }

    #[test]
    fn test_max_degree() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(0, 3);

        assert_eq!(graph.max_degree(), 3);
    }

    #[test]
    fn test_average_degree() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);

        assert_eq!(graph.average_degree(), 1.5);
    }

    #[test]
    fn test_self_loops() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 0);
        graph.add_edge(1, 1);
        graph.add_edge(0, 1);

        assert_eq!(graph.number_of_self_loops(), 2);
    }

    #[test]
    fn test_parallel_edges() {
        let mut graph = Graph::new(2);
        graph.add_edge(0, 1);
        graph.add_edge(0, 1);  // parallel edge

        assert_eq!(graph.e(), 2);
        assert_eq!(graph.degree(0), 2);
        assert_eq!(graph.degree(1), 2);
    }

    #[test]
    #[should_panic(expected = "vertex 5 is not between 0 and 4")]
    fn test_invalid_vertex() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 5);  // invalid vertex
    }

    #[test]
    fn test_display() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let output = format!("{}", graph);
        assert!(output.contains("3 vertices, 2 edges"));
    }
}
