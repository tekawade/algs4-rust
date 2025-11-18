//! Random digraph generator.
//!
//! Provides utilities to generate random directed graphs for testing and benchmarking.

use crate::digraph::Digraph;
use rand::Rng;

/// Utilities for generating random directed graphs.
pub struct DigraphGenerator;

impl DigraphGenerator {
    /// Generates a random simple digraph with `v` vertices and `e` edges.
    ///
    /// A simple digraph has no self-loops and no parallel edges.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices
    /// * `e` - The number of edges
    ///
    /// # Panics
    ///
    /// Panics if `e` is greater than `v * (v - 1)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::DigraphGenerator;
    ///
    /// let digraph = DigraphGenerator::simple(10, 20);
    /// assert_eq!(digraph.v(), 10);
    /// assert_eq!(digraph.e(), 20);
    /// ```
    pub fn simple(v: usize, e: usize) -> Digraph {
        if e > v * (v - 1) {
            panic!("Too many edges");
        }

        let mut rng = rand::thread_rng();
        let mut digraph = Digraph::new(v);

        while digraph.e() < e {
            let v1 = rng.gen_range(0..v);
            let v2 = rng.gen_range(0..v);

            // Avoid self-loops
            if v1 == v2 {
                continue;
            }

            // Check for parallel edges
            if !digraph.adj(v1).contains(&v2) {
                digraph.add_edge(v1, v2);
            }
        }

        digraph
    }

    /// Generates a random simple digraph with `v` vertices and edge probability `p`.
    ///
    /// Each possible directed edge is included independently with probability `p`.
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
    /// use algs4_graphs::DigraphGenerator;
    ///
    /// let digraph = DigraphGenerator::simple_prob(10, 0.3);
    /// assert_eq!(digraph.v(), 10);
    /// // Number of edges is random, approximately 0.3 * 10 * 9 = 27
    /// ```
    pub fn simple_prob(v: usize, p: f64) -> Digraph {
        if !(0.0..=1.0).contains(&p) {
            panic!("Probability must be between 0.0 and 1.0");
        }

        let mut rng = rand::thread_rng();
        let mut digraph = Digraph::new(v);

        for i in 0..v {
            for j in 0..v {
                if i != j && rng.gen::<f64>() < p {
                    digraph.add_edge(i, j);
                }
            }
        }

        digraph
    }

    /// Generates a complete digraph with `v` vertices.
    ///
    /// A complete digraph has a directed edge from every vertex to every other vertex.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::DigraphGenerator;
    ///
    /// let digraph = DigraphGenerator::complete(5);
    /// assert_eq!(digraph.v(), 5);
    /// assert_eq!(digraph.e(), 20); // 5 * 4
    /// ```
    pub fn complete(v: usize) -> Digraph {
        Self::simple_prob(v, 1.0)
    }

    /// Generates a random DAG (Directed Acyclic Graph) with `v` vertices and `e` edges.
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
    /// use algs4_graphs::DigraphGenerator;
    ///
    /// let dag = DigraphGenerator::dag(10, 15);
    /// assert_eq!(dag.v(), 10);
    /// assert_eq!(dag.e(), 15);
    /// ```
    pub fn dag(v: usize, e: usize) -> Digraph {
        if e > v * (v - 1) / 2 {
            panic!("Too many edges");
        }

        let mut rng = rand::thread_rng();
        let mut digraph = Digraph::new(v);

        // Generate a random permutation to define the topological order
        let mut vertices: Vec<usize> = (0..v).collect();
        for i in 0..v {
            let j = rng.gen_range(i..v);
            vertices.swap(i, j);
        }

        while digraph.e() < e {
            let i = rng.gen_range(0..v);
            let j = rng.gen_range(0..v);

            let v1 = vertices[i];
            let v2 = vertices[j];

            // Only add edge from lower to higher in topological order
            if i < j && !digraph.adj(v1).contains(&v2) {
                digraph.add_edge(v1, v2);
            }
        }

        digraph
    }

    /// Generates a random tournament digraph with `v` vertices.
    ///
    /// A tournament is a complete digraph where there is exactly one directed edge
    /// between each pair of vertices.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::DigraphGenerator;
    ///
    /// let tournament = DigraphGenerator::tournament(5);
    /// assert_eq!(tournament.v(), 5);
    /// assert_eq!(tournament.e(), 10); // 5 * 4 / 2
    /// ```
    pub fn tournament(v: usize) -> Digraph {
        let mut rng = rand::thread_rng();
        let mut digraph = Digraph::new(v);

        for i in 0..v {
            for j in (i + 1)..v {
                if rng.gen::<bool>() {
                    digraph.add_edge(i, j);
                } else {
                    digraph.add_edge(j, i);
                }
            }
        }

        digraph
    }

    /// Generates a random rooted-in DAG with `v` vertices and `e` edges.
    ///
    /// A rooted-in DAG has a single vertex with no incoming edges (root)
    /// and all other vertices are reachable from it.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices
    /// * `e` - The number of edges
    ///
    /// # Panics
    ///
    /// Panics if `e` is greater than `v * (v - 1) / 2` or if `e` is less than `v - 1`.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::DigraphGenerator;
    ///
    /// let dag = DigraphGenerator::rooted_in_dag(10, 15);
    /// assert_eq!(dag.v(), 10);
    /// assert_eq!(dag.e(), 15);
    /// ```
    pub fn rooted_in_dag(v: usize, e: usize) -> Digraph {
        if e > v * (v - 1) / 2 {
            panic!("Too many edges");
        }
        if e < v - 1 {
            panic!("Too few edges - need at least v-1 edges for connectivity");
        }

        let mut rng = rand::thread_rng();
        let mut digraph = Digraph::new(v);

        // Generate a random permutation to define the topological order
        let mut vertices: Vec<usize> = (0..v).collect();
        for i in 0..v {
            let j = rng.gen_range(i..v);
            vertices.swap(i, j);
        }

        // Ensure connectivity: each vertex has at least one incoming edge from a previous vertex
        for i in 1..v {
            let j = rng.gen_range(0..i);
            digraph.add_edge(vertices[j], vertices[i]);
        }

        // Add remaining edges randomly
        while digraph.e() < e {
            let i = rng.gen_range(0..v);
            let j = rng.gen_range(0..v);

            let v1 = vertices[i];
            let v2 = vertices[j];

            if i < j && !digraph.adj(v1).contains(&v2) {
                digraph.add_edge(v1, v2);
            }
        }

        digraph
    }

    /// Generates a random rooted-out DAG with `v` vertices and `e` edges.
    ///
    /// A rooted-out DAG has a single vertex with no outgoing edges (root)
    /// and all other vertices can reach it.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices
    /// * `e` - The number of edges
    ///
    /// # Panics
    ///
    /// Panics if `e` is greater than `v * (v - 1) / 2` or if `e` is less than `v - 1`.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::DigraphGenerator;
    ///
    /// let dag = DigraphGenerator::rooted_out_dag(10, 15);
    /// assert_eq!(dag.v(), 10);
    /// assert_eq!(dag.e(), 15);
    /// ```
    pub fn rooted_out_dag(v: usize, e: usize) -> Digraph {
        if e > v * (v - 1) / 2 {
            panic!("Too many edges");
        }
        if e < v - 1 {
            panic!("Too few edges - need at least v-1 edges for connectivity");
        }

        let mut rng = rand::thread_rng();
        let mut digraph = Digraph::new(v);

        // Generate a random permutation to define the topological order
        let mut vertices: Vec<usize> = (0..v).collect();
        for i in 0..v {
            let j = rng.gen_range(i..v);
            vertices.swap(i, j);
        }

        // Ensure connectivity: each vertex has at least one outgoing edge to a later vertex
        for i in 0..(v - 1) {
            let j = rng.gen_range((i + 1)..v);
            digraph.add_edge(vertices[i], vertices[j]);
        }

        // Add remaining edges randomly
        while digraph.e() < e {
            let i = rng.gen_range(0..v);
            let j = rng.gen_range(0..v);

            let v1 = vertices[i];
            let v2 = vertices[j];

            if i < j && !digraph.adj(v1).contains(&v2) {
                digraph.add_edge(v1, v2);
            }
        }

        digraph
    }

    /// Generates a random rooted-in tree with `v` vertices.
    ///
    /// A rooted-in tree is a DAG where vertex 0 is the root with no incoming edges
    /// and every other vertex has exactly one incoming edge.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::DigraphGenerator;
    ///
    /// let tree = DigraphGenerator::rooted_in_tree(10);
    /// assert_eq!(tree.v(), 10);
    /// assert_eq!(tree.e(), 9);
    /// ```
    pub fn rooted_in_tree(v: usize) -> Digraph {
        let mut rng = rand::thread_rng();
        let mut digraph = Digraph::new(v);

        // Generate a random permutation
        let mut vertices: Vec<usize> = (0..v).collect();
        for i in 0..v {
            let j = rng.gen_range(i..v);
            vertices.swap(i, j);
        }

        // Connect each vertex to a random previous vertex
        for i in 1..v {
            let j = rng.gen_range(0..i);
            digraph.add_edge(vertices[j], vertices[i]);
        }

        digraph
    }

    /// Generates a random rooted-out tree with `v` vertices.
    ///
    /// A rooted-out tree is a DAG where vertex v-1 is the root with no outgoing edges
    /// and every other vertex has exactly one outgoing edge.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::DigraphGenerator;
    ///
    /// let tree = DigraphGenerator::rooted_out_tree(10);
    /// assert_eq!(tree.v(), 10);
    /// assert_eq!(tree.e(), 9);
    /// ```
    pub fn rooted_out_tree(v: usize) -> Digraph {
        let mut rng = rand::thread_rng();
        let mut digraph = Digraph::new(v);

        // Generate a random permutation
        let mut vertices: Vec<usize> = (0..v).collect();
        for i in 0..v {
            let j = rng.gen_range(i..v);
            vertices.swap(i, j);
        }

        // Connect each vertex to a random later vertex
        for i in 0..(v - 1) {
            let j = rng.gen_range((i + 1)..v);
            digraph.add_edge(vertices[i], vertices[j]);
        }

        digraph
    }

    /// Generates a random path digraph with `v` vertices.
    ///
    /// A path digraph is a directed path from vertex 0 to vertex v-1.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::DigraphGenerator;
    ///
    /// let path = DigraphGenerator::path(5);
    /// assert_eq!(path.v(), 5);
    /// assert_eq!(path.e(), 4);
    /// ```
    pub fn path(v: usize) -> Digraph {
        let mut digraph = Digraph::new(v);

        for i in 0..(v - 1) {
            digraph.add_edge(i, i + 1);
        }

        digraph
    }

    /// Generates a random cycle digraph with `v` vertices.
    ///
    /// A cycle digraph is a directed cycle.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::DigraphGenerator;
    ///
    /// let cycle = DigraphGenerator::cycle(5);
    /// assert_eq!(cycle.v(), 5);
    /// assert_eq!(cycle.e(), 5);
    /// ```
    pub fn cycle(v: usize) -> Digraph {
        let mut digraph = Self::path(v);
        if v > 1 {
            digraph.add_edge(v - 1, 0);
        }
        digraph
    }

    /// Generates a binary tree digraph with `v` vertices.
    ///
    /// Edges are directed from parent to children.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::DigraphGenerator;
    ///
    /// let tree = DigraphGenerator::binary_tree(7);
    /// assert_eq!(tree.v(), 7);
    /// assert_eq!(tree.e(), 6);
    /// ```
    pub fn binary_tree(v: usize) -> Digraph {
        let mut digraph = Digraph::new(v);

        for i in 1..v {
            digraph.add_edge((i - 1) / 2, i);
        }

        digraph
    }

    /// Generates a strong digraph with `v` vertices and `e` edges.
    ///
    /// A strong digraph is strongly connected: there is a directed path from
    /// every vertex to every other vertex.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices
    /// * `e` - The number of edges
    ///
    /// # Panics
    ///
    /// Panics if `e` is less than `v` or greater than `v * (v - 1)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::DigraphGenerator;
    ///
    /// let digraph = DigraphGenerator::strong(10, 20);
    /// assert_eq!(digraph.v(), 10);
    /// assert_eq!(digraph.e(), 20);
    /// ```
    pub fn strong(v: usize, e: usize) -> Digraph {
        if e < v {
            panic!("Too few edges - need at least v edges for strong connectivity");
        }
        if e > v * (v - 1) {
            panic!("Too many edges");
        }

        let mut rng = rand::thread_rng();

        // Start with a directed cycle to ensure strong connectivity
        let mut digraph = Self::cycle(v);

        // Add remaining edges randomly
        while digraph.e() < e {
            let v1 = rng.gen_range(0..v);
            let v2 = rng.gen_range(0..v);

            if v1 != v2 && !digraph.adj(v1).contains(&v2) {
                digraph.add_edge(v1, v2);
            }
        }

        digraph
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let digraph = DigraphGenerator::simple(10, 20);
        assert_eq!(digraph.v(), 10);
        assert_eq!(digraph.e(), 20);
    }

    #[test]
    #[should_panic(expected = "Too many edges")]
    fn test_simple_too_many_edges() {
        DigraphGenerator::simple(10, 100);
    }

    #[test]
    fn test_simple_prob() {
        let digraph = DigraphGenerator::simple_prob(10, 0.5);
        assert_eq!(digraph.v(), 10);
    }

    #[test]
    fn test_complete() {
        let digraph = DigraphGenerator::complete(5);
        assert_eq!(digraph.v(), 5);
        assert_eq!(digraph.e(), 20);
    }

    #[test]
    fn test_dag() {
        let dag = DigraphGenerator::dag(10, 15);
        assert_eq!(dag.v(), 10);
        assert_eq!(dag.e(), 15);
    }

    #[test]
    fn test_tournament() {
        let tournament = DigraphGenerator::tournament(5);
        assert_eq!(tournament.v(), 5);
        assert_eq!(tournament.e(), 10);
    }

    #[test]
    fn test_rooted_in_dag() {
        let dag = DigraphGenerator::rooted_in_dag(10, 15);
        assert_eq!(dag.v(), 10);
        assert_eq!(dag.e(), 15);
    }

    #[test]
    fn test_rooted_out_dag() {
        let dag = DigraphGenerator::rooted_out_dag(10, 15);
        assert_eq!(dag.v(), 10);
        assert_eq!(dag.e(), 15);
    }

    #[test]
    fn test_rooted_in_tree() {
        let tree = DigraphGenerator::rooted_in_tree(10);
        assert_eq!(tree.v(), 10);
        assert_eq!(tree.e(), 9);
    }

    #[test]
    fn test_rooted_out_tree() {
        let tree = DigraphGenerator::rooted_out_tree(10);
        assert_eq!(tree.v(), 10);
        assert_eq!(tree.e(), 9);
    }

    #[test]
    fn test_path() {
        let path = DigraphGenerator::path(5);
        assert_eq!(path.v(), 5);
        assert_eq!(path.e(), 4);
    }

    #[test]
    fn test_cycle() {
        let cycle = DigraphGenerator::cycle(5);
        assert_eq!(cycle.v(), 5);
        assert_eq!(cycle.e(), 5);
    }

    #[test]
    fn test_binary_tree() {
        let tree = DigraphGenerator::binary_tree(7);
        assert_eq!(tree.v(), 7);
        assert_eq!(tree.e(), 6);
    }

    #[test]
    fn test_strong() {
        let digraph = DigraphGenerator::strong(10, 20);
        assert_eq!(digraph.v(), 10);
        assert_eq!(digraph.e(), 20);
    }
}
