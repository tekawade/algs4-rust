//! Directed graph implementation using adjacency lists.
//!
//! This implementation uses adjacency lists to represent a directed graph
//! where vertices are labeled 0 through V-1.

use std::fmt;

/// A directed graph, implemented using adjacency lists.
///
/// Supports adding directed edges and iterating over vertices adjacent to a given vertex.
/// Parallel edges and self-loops are permitted.
///
/// # Examples
///
/// ```
/// use algs4_graphs::Digraph;
///
/// let mut digraph = Digraph::new(6);
/// digraph.add_edge(0, 1);
/// digraph.add_edge(0, 2);
/// digraph.add_edge(1, 2);
///
/// assert_eq!(digraph.v(), 6);
/// assert_eq!(digraph.e(), 3);
/// assert_eq!(digraph.outdegree(0), 2);
/// assert_eq!(digraph.indegree(1), 1);
/// ```
#[derive(Debug, Clone)]
pub struct Digraph {
    v: usize,             // number of vertices
    e: usize,             // number of edges
    adj: Vec<Vec<usize>>, // adjacency lists (outgoing edges)
    indegree: Vec<usize>, // indegree of each vertex
}

impl Digraph {
    /// Creates a new directed graph with `v` vertices and no edges.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::Digraph;
    ///
    /// let digraph = Digraph::new(10);
    /// assert_eq!(digraph.v(), 10);
    /// assert_eq!(digraph.e(), 0);
    /// ```
    pub fn new(v: usize) -> Self {
        Digraph {
            v,
            e: 0,
            adj: vec![Vec::new(); v],
            indegree: vec![0; v],
        }
    }

    /// Returns the number of vertices in the digraph.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::Digraph;
    ///
    /// let digraph = Digraph::new(5);
    /// assert_eq!(digraph.v(), 5);
    /// ```
    pub fn v(&self) -> usize {
        self.v
    }

    /// Returns the number of edges in the digraph.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::Digraph;
    ///
    /// let mut digraph = Digraph::new(5);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    /// assert_eq!(digraph.e(), 2);
    /// ```
    pub fn e(&self) -> usize {
        self.e
    }

    /// Validates that vertex v is a valid vertex in the digraph.
    fn validate_vertex(&self, v: usize) {
        if v >= self.v {
            panic!("vertex {} is not between 0 and {}", v, self.v - 1);
        }
    }

    /// Adds a directed edge from vertex `v` to vertex `w`.
    ///
    /// # Arguments
    ///
    /// * `v` - The source vertex
    /// * `w` - The destination vertex
    ///
    /// # Panics
    ///
    /// Panics if either vertex is not between 0 and V-1.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::Digraph;
    ///
    /// let mut digraph = Digraph::new(3);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    /// assert_eq!(digraph.e(), 2);
    /// ```
    pub fn add_edge(&mut self, v: usize, w: usize) {
        self.validate_vertex(v);
        self.validate_vertex(w);
        self.adj[v].push(w);
        self.indegree[w] += 1;
        self.e += 1;
    }

    /// Returns the vertices adjacent from vertex `v` (outgoing edges).
    ///
    /// # Arguments
    ///
    /// * `v` - The vertex
    ///
    /// # Returns
    ///
    /// A slice containing the vertices pointed to by edges from vertex `v`
    ///
    /// # Panics
    ///
    /// Panics if vertex is not between 0 and V-1.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::Digraph;
    ///
    /// let mut digraph = Digraph::new(4);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(0, 2);
    /// digraph.add_edge(0, 3);
    ///
    /// let neighbors: Vec<usize> = digraph.adj(0).iter().copied().collect();
    /// assert_eq!(neighbors.len(), 3);
    /// ```
    pub fn adj(&self, v: usize) -> &[usize] {
        self.validate_vertex(v);
        &self.adj[v]
    }

    /// Returns the outdegree of vertex `v` (number of outgoing edges).
    ///
    /// # Arguments
    ///
    /// * `v` - The vertex
    ///
    /// # Returns
    ///
    /// The number of directed edges from vertex `v`
    ///
    /// # Panics
    ///
    /// Panics if vertex is not between 0 and V-1.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::Digraph;
    ///
    /// let mut digraph = Digraph::new(4);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(0, 2);
    /// digraph.add_edge(0, 3);
    ///
    /// assert_eq!(digraph.outdegree(0), 3);
    /// assert_eq!(digraph.outdegree(1), 0);
    /// ```
    pub fn outdegree(&self, v: usize) -> usize {
        self.validate_vertex(v);
        self.adj[v].len()
    }

    /// Returns the indegree of vertex `v` (number of incoming edges).
    ///
    /// # Arguments
    ///
    /// * `v` - The vertex
    ///
    /// # Returns
    ///
    /// The number of directed edges to vertex `v`
    ///
    /// # Panics
    ///
    /// Panics if vertex is not between 0 and V-1.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::Digraph;
    ///
    /// let mut digraph = Digraph::new(4);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(2, 1);
    /// digraph.add_edge(3, 1);
    ///
    /// assert_eq!(digraph.indegree(1), 3);
    /// assert_eq!(digraph.indegree(0), 0);
    /// ```
    pub fn indegree(&self, v: usize) -> usize {
        self.validate_vertex(v);
        self.indegree[v]
    }

    /// Returns the reverse of the digraph.
    ///
    /// The reverse digraph has the same vertices but all edges reversed.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::Digraph;
    ///
    /// let mut digraph = Digraph::new(3);
    /// digraph.add_edge(0, 1);
    /// digraph.add_edge(1, 2);
    ///
    /// let reverse = digraph.reverse();
    /// assert_eq!(reverse.e(), 2);
    /// assert_eq!(reverse.outdegree(1), 1);
    /// assert_eq!(reverse.outdegree(2), 1);
    /// ```
    pub fn reverse(&self) -> Digraph {
        let mut reverse = Digraph::new(self.v);
        for v in 0..self.v {
            for &w in &self.adj[v] {
                reverse.add_edge(w, v);
            }
        }
        reverse
    }

    /// Returns the number of self-loops in the digraph.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::Digraph;
    ///
    /// let mut digraph = Digraph::new(3);
    /// digraph.add_edge(0, 0);  // self-loop
    /// digraph.add_edge(1, 2);
    ///
    /// assert_eq!(digraph.number_of_self_loops(), 1);
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
        count
    }
}

impl fmt::Display for Digraph {
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
    fn test_new_digraph() {
        let digraph = Digraph::new(5);
        assert_eq!(digraph.v(), 5);
        assert_eq!(digraph.e(), 0);
    }

    #[test]
    fn test_add_edge() {
        let mut digraph = Digraph::new(5);
        digraph.add_edge(0, 1);
        assert_eq!(digraph.e(), 1);
        assert_eq!(digraph.outdegree(0), 1);
        assert_eq!(digraph.indegree(1), 1);
    }

    #[test]
    fn test_adjacency() {
        let mut digraph = Digraph::new(4);
        digraph.add_edge(0, 1);
        digraph.add_edge(0, 2);
        digraph.add_edge(0, 3);

        assert_eq!(digraph.adj(0).len(), 3);
        assert!(digraph.adj(0).contains(&1));
        assert!(digraph.adj(0).contains(&2));
        assert!(digraph.adj(0).contains(&3));
    }

    #[test]
    fn test_outdegree() {
        let mut digraph = Digraph::new(4);
        digraph.add_edge(0, 1);
        digraph.add_edge(0, 2);
        digraph.add_edge(0, 3);

        assert_eq!(digraph.outdegree(0), 3);
        assert_eq!(digraph.outdegree(1), 0);
    }

    #[test]
    fn test_indegree() {
        let mut digraph = Digraph::new(4);
        digraph.add_edge(0, 1);
        digraph.add_edge(2, 1);
        digraph.add_edge(3, 1);

        assert_eq!(digraph.indegree(1), 3);
        assert_eq!(digraph.indegree(0), 0);
    }

    #[test]
    fn test_reverse() {
        let mut digraph = Digraph::new(3);
        digraph.add_edge(0, 1);
        digraph.add_edge(1, 2);
        digraph.add_edge(0, 2);

        let reverse = digraph.reverse();
        assert_eq!(reverse.e(), 3);
        assert_eq!(reverse.outdegree(0), 0);
        assert_eq!(reverse.outdegree(1), 1);
        assert_eq!(reverse.outdegree(2), 2);
    }

    #[test]
    fn test_self_loops() {
        let mut digraph = Digraph::new(3);
        digraph.add_edge(0, 0);
        digraph.add_edge(1, 1);
        digraph.add_edge(0, 1);

        assert_eq!(digraph.number_of_self_loops(), 2);
    }

    #[test]
    fn test_parallel_edges() {
        let mut digraph = Digraph::new(2);
        digraph.add_edge(0, 1);
        digraph.add_edge(0, 1); // parallel edge

        assert_eq!(digraph.e(), 2);
        assert_eq!(digraph.outdegree(0), 2);
        assert_eq!(digraph.indegree(1), 2);
    }

    #[test]
    #[should_panic(expected = "vertex 5 is not between 0 and 4")]
    fn test_invalid_vertex() {
        let mut digraph = Digraph::new(5);
        digraph.add_edge(0, 5); // invalid vertex
    }

    #[test]
    fn test_display() {
        let mut digraph = Digraph::new(3);
        digraph.add_edge(0, 1);
        digraph.add_edge(1, 2);

        let output = format!("{}", digraph);
        assert!(output.contains("3 vertices, 2 edges"));
    }
}
