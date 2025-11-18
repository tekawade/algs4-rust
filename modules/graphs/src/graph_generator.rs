//! Random graph generator.
//!
//! Provides utilities to generate random graphs for testing and benchmarking.

use crate::graph::Graph;
use rand::Rng;

/// Utilities for generating random undirected graphs.
#[derive(Debug)]
pub struct GraphGenerator;

impl GraphGenerator {
    /// Generates a random simple graph with `v` vertices and `e` edges.
    ///
    /// A simple graph has no self-loops and no parallel edges.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices
    /// * `e` - The number of edges
    ///
    /// # Panics
    ///
    /// Panics if `e` is greater than `v * (v - 1) / 2`.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::GraphGenerator;
    ///
    /// let graph = GraphGenerator::simple(10, 15);
    /// assert_eq!(graph.v(), 10);
    /// assert_eq!(graph.e(), 15);
    /// ```
    pub fn simple(v: usize, e: usize) -> Graph {
        if e > v * (v - 1) / 2 {
            panic!("Too many edges");
        }

        let mut rng = rand::thread_rng();
        let mut graph = Graph::new(v);

        while graph.e() < e {
            let v1 = rng.gen_range(0..v);
            let v2 = rng.gen_range(0..v);

            // Avoid self-loops
            if v1 == v2 {
                continue;
            }

            // Check for parallel edges
            if !graph.adj(v1).contains(&v2) {
                graph.add_edge(v1, v2);
            }
        }

        graph
    }

    /// Generates a random simple graph with `v` vertices and edge probability `p`.
    ///
    /// Each possible edge is included independently with probability `p`.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices
    /// * `p` - The probability of each edge (between 0.0 and 1.0)
    ///
    /// # Panics
    ///
    /// Panics if `p` is not between 0.0 and 1.0.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::GraphGenerator;
    ///
    /// let graph = GraphGenerator::simple_prob(10, 0.3);
    /// assert_eq!(graph.v(), 10);
    /// // Number of edges is random, approximately 0.3 * 10 * 9 / 2 = 13.5
    /// ```
    pub fn simple_prob(v: usize, p: f64) -> Graph {
        if !(0.0..=1.0).contains(&p) {
            panic!("Probability must be between 0.0 and 1.0");
        }

        let mut rng = rand::thread_rng();
        let mut graph = Graph::new(v);

        for i in 0..v {
            for j in (i + 1)..v {
                if rng.gen::<f64>() < p {
                    graph.add_edge(i, j);
                }
            }
        }

        graph
    }

    /// Generates a complete graph with `v` vertices.
    ///
    /// A complete graph has an edge between every pair of vertices.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::GraphGenerator;
    ///
    /// let graph = GraphGenerator::complete(5);
    /// assert_eq!(graph.v(), 5);
    /// assert_eq!(graph.e(), 10); // 5 * 4 / 2
    /// ```
    pub fn complete(v: usize) -> Graph {
        Self::simple_prob(v, 1.0)
    }

    /// Generates a complete bipartite graph with `v1` and `v2` vertices in each partition.
    ///
    /// # Arguments
    ///
    /// * `v1` - The number of vertices in the first partition
    /// * `v2` - The number of vertices in the second partition
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::GraphGenerator;
    ///
    /// let graph = GraphGenerator::complete_bipartite(3, 4);
    /// assert_eq!(graph.v(), 7);
    /// assert_eq!(graph.e(), 12); // 3 * 4
    /// ```
    pub fn complete_bipartite(v1: usize, v2: usize) -> Graph {
        let mut graph = Graph::new(v1 + v2);

        for i in 0..v1 {
            for j in 0..v2 {
                graph.add_edge(i, v1 + j);
            }
        }

        graph
    }

    /// Generates a random bipartite graph with `v1` and `v2` vertices and `e` edges.
    ///
    /// # Arguments
    ///
    /// * `v1` - The number of vertices in the first partition
    /// * `v2` - The number of vertices in the second partition
    /// * `e` - The number of edges
    ///
    /// # Panics
    ///
    /// Panics if `e` is greater than `v1 * v2`.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::GraphGenerator;
    ///
    /// let graph = GraphGenerator::bipartite(3, 4, 6);
    /// assert_eq!(graph.v(), 7);
    /// assert_eq!(graph.e(), 6);
    /// ```
    pub fn bipartite(v1: usize, v2: usize, e: usize) -> Graph {
        if e > v1 * v2 {
            panic!("Too many edges");
        }

        let mut rng = rand::thread_rng();
        let mut graph = Graph::new(v1 + v2);

        while graph.e() < e {
            let i = rng.gen_range(0..v1);
            let j = rng.gen_range(0..v2);

            // Check for parallel edges
            if !graph.adj(i).contains(&(v1 + j)) {
                graph.add_edge(i, v1 + j);
            }
        }

        graph
    }

    /// Generates a random bipartite graph with edge probability `p`.
    ///
    /// # Arguments
    ///
    /// * `v1` - The number of vertices in the first partition
    /// * `v2` - The number of vertices in the second partition
    /// * `p` - The probability of each edge (between 0.0 and 1.0)
    ///
    /// # Panics
    ///
    /// Panics if `p` is not between 0.0 and 1.0.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::GraphGenerator;
    ///
    /// let graph = GraphGenerator::bipartite_prob(3, 4, 0.5);
    /// assert_eq!(graph.v(), 7);
    /// // Number of edges is random, approximately 0.5 * 3 * 4 = 6
    /// ```
    pub fn bipartite_prob(v1: usize, v2: usize, p: f64) -> Graph {
        if !(0.0..=1.0).contains(&p) {
            panic!("Probability must be between 0.0 and 1.0");
        }

        let mut rng = rand::thread_rng();
        let mut graph = Graph::new(v1 + v2);

        for i in 0..v1 {
            for j in 0..v2 {
                if rng.gen::<f64>() < p {
                    graph.add_edge(i, v1 + j);
                }
            }
        }

        graph
    }

    /// Generates a path graph with `v` vertices.
    ///
    /// A path graph is a tree with two leaves.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::GraphGenerator;
    ///
    /// let graph = GraphGenerator::path(5);
    /// assert_eq!(graph.v(), 5);
    /// assert_eq!(graph.e(), 4);
    /// ```
    pub fn path(v: usize) -> Graph {
        let mut graph = Graph::new(v);

        for i in 0..(v - 1) {
            graph.add_edge(i, i + 1);
        }

        graph
    }

    /// Generates a cycle graph with `v` vertices.
    ///
    /// A cycle graph is a path with an additional edge connecting the endpoints.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::GraphGenerator;
    ///
    /// let graph = GraphGenerator::cycle(5);
    /// assert_eq!(graph.v(), 5);
    /// assert_eq!(graph.e(), 5);
    /// ```
    pub fn cycle(v: usize) -> Graph {
        let mut graph = Self::path(v);
        if v > 1 {
            graph.add_edge(v - 1, 0);
        }
        graph
    }

    /// Generates a binary tree with `v` vertices.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::GraphGenerator;
    ///
    /// let graph = GraphGenerator::binary_tree(7);
    /// assert_eq!(graph.v(), 7);
    /// assert_eq!(graph.e(), 6);
    /// ```
    pub fn binary_tree(v: usize) -> Graph {
        let mut graph = Graph::new(v);

        for i in 1..v {
            graph.add_edge(i, (i - 1) / 2);
        }

        graph
    }

    /// Generates a random tree with `v` vertices.
    ///
    /// A tree is a connected acyclic graph.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::GraphGenerator;
    ///
    /// let graph = GraphGenerator::tree(10);
    /// assert_eq!(graph.v(), 10);
    /// assert_eq!(graph.e(), 9);
    /// ```
    pub fn tree(v: usize) -> Graph {
        let mut rng = rand::thread_rng();
        let mut graph = Graph::new(v);

        // Generate a random permutation
        let mut vertices: Vec<usize> = (0..v).collect();
        for i in 0..v {
            let j = rng.gen_range(i..v);
            vertices.swap(i, j);
        }

        // Connect each vertex to a random previous vertex
        for i in 1..v {
            let j = rng.gen_range(0..i);
            graph.add_edge(vertices[i], vertices[j]);
        }

        graph
    }

    /// Generates a random regular graph with `v` vertices and degree `k`.
    ///
    /// A regular graph is a graph where every vertex has the same degree.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices
    /// * `k` - The degree of each vertex
    ///
    /// # Panics
    ///
    /// Panics if `v * k` is odd or if `k >= v`.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::GraphGenerator;
    ///
    /// let graph = GraphGenerator::regular(10, 3);
    /// assert_eq!(graph.v(), 10);
    /// assert_eq!(graph.e(), 15); // 10 * 3 / 2
    /// for i in 0..10 {
    ///     assert_eq!(graph.degree(i), 3);
    /// }
    /// ```
    pub fn regular(v: usize, k: usize) -> Graph {
        if !(v * k).is_multiple_of(2) {
            panic!("v * k must be even");
        }
        if k >= v {
            panic!("k must be less than v");
        }

        let mut rng = rand::thread_rng();
        let mut graph = Graph::new(v);

        // Create a list of vertices, each appearing k times
        let mut vertices = Vec::new();
        for i in 0..v {
            for _ in 0..k {
                vertices.push(i);
            }
        }

        // Shuffle the list
        for i in 0..vertices.len() {
            let j = rng.gen_range(i..vertices.len());
            vertices.swap(i, j);
        }

        // Match pairs and add edges (avoid self-loops and parallel edges)
        let mut i = 0;
        while i < vertices.len() {
            let v1 = vertices[i];
            let v2 = vertices[i + 1];

            // Avoid self-loops and parallel edges
            if v1 != v2 && !graph.adj(v1).contains(&v2) {
                graph.add_edge(v1, v2);
                i += 2;
            } else {
                // Swap with a random later vertex and try again
                if i + 2 < vertices.len() {
                    let j = rng.gen_range(i + 2..vertices.len());
                    vertices.swap(i + 1, j);
                } else {
                    // If we can't find a valid pairing, start over
                    return Self::regular(v, k);
                }
            }
        }

        graph
    }

    /// Generates a wheel graph with `v` vertices.
    ///
    /// A wheel graph is a cycle with an additional central hub vertex
    /// connected to all other vertices.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices (must be at least 2)
    ///
    /// # Panics
    ///
    /// Panics if `v` is less than 2.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::GraphGenerator;
    ///
    /// let graph = GraphGenerator::wheel(6);
    /// assert_eq!(graph.v(), 6);
    /// assert_eq!(graph.e(), 10); // 5 spokes + 5 rim edges
    /// ```
    pub fn wheel(v: usize) -> Graph {
        if v < 2 {
            panic!("v must be at least 2");
        }

        let mut graph = Graph::new(v);

        // Create a cycle on vertices 1..v
        for i in 1..(v - 1) {
            graph.add_edge(i, i + 1);
        }
        if v > 2 {
            graph.add_edge(v - 1, 1);
        }

        // Connect the hub (vertex 0) to all other vertices
        for i in 1..v {
            graph.add_edge(0, i);
        }

        graph
    }

    /// Generates a star graph with `v` vertices.
    ///
    /// A star graph has one central hub vertex connected to all other vertices.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::GraphGenerator;
    ///
    /// let graph = GraphGenerator::star(6);
    /// assert_eq!(graph.v(), 6);
    /// assert_eq!(graph.e(), 5);
    /// assert_eq!(graph.degree(0), 5);
    /// ```
    pub fn star(v: usize) -> Graph {
        let mut graph = Graph::new(v);

        for i in 1..v {
            graph.add_edge(0, i);
        }

        graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let graph = GraphGenerator::simple(10, 15);
        assert_eq!(graph.v(), 10);
        assert_eq!(graph.e(), 15);
    }

    #[test]
    #[should_panic(expected = "Too many edges")]
    fn test_simple_too_many_edges() {
        GraphGenerator::simple(10, 100);
    }

    #[test]
    fn test_simple_prob() {
        let graph = GraphGenerator::simple_prob(10, 0.5);
        assert_eq!(graph.v(), 10);
    }

    #[test]
    fn test_complete() {
        let graph = GraphGenerator::complete(5);
        assert_eq!(graph.v(), 5);
        assert_eq!(graph.e(), 10);
    }

    #[test]
    fn test_complete_bipartite() {
        let graph = GraphGenerator::complete_bipartite(3, 4);
        assert_eq!(graph.v(), 7);
        assert_eq!(graph.e(), 12);
    }

    #[test]
    fn test_bipartite() {
        let graph = GraphGenerator::bipartite(3, 4, 6);
        assert_eq!(graph.v(), 7);
        assert_eq!(graph.e(), 6);
    }

    #[test]
    fn test_path() {
        let graph = GraphGenerator::path(5);
        assert_eq!(graph.v(), 5);
        assert_eq!(graph.e(), 4);
    }

    #[test]
    fn test_cycle() {
        let graph = GraphGenerator::cycle(5);
        assert_eq!(graph.v(), 5);
        assert_eq!(graph.e(), 5);
    }

    #[test]
    fn test_binary_tree() {
        let graph = GraphGenerator::binary_tree(7);
        assert_eq!(graph.v(), 7);
        assert_eq!(graph.e(), 6);
    }

    #[test]
    fn test_tree() {
        let graph = GraphGenerator::tree(10);
        assert_eq!(graph.v(), 10);
        assert_eq!(graph.e(), 9);
    }

    #[test]
    fn test_regular() {
        let graph = GraphGenerator::regular(10, 3);
        assert_eq!(graph.v(), 10);
        assert_eq!(graph.e(), 15);
        for i in 0..10 {
            assert_eq!(graph.degree(i), 3);
        }
    }

    #[test]
    fn test_wheel() {
        let graph = GraphGenerator::wheel(6);
        assert_eq!(graph.v(), 6);
        assert_eq!(graph.e(), 10);
    }

    #[test]
    fn test_star() {
        let graph = GraphGenerator::star(6);
        assert_eq!(graph.v(), 6);
        assert_eq!(graph.e(), 5);
        assert_eq!(graph.degree(0), 5);
    }
}
