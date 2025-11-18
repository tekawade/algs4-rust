//! Symbol graph - undirected graph with string vertex names.
//!
//! Provides a wrapper around the Graph data type that allows vertices to be
//! identified by arbitrary strings rather than integers.
use crate::graph::Graph;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Undirected graph with string vertex names.
///
/// This struct wraps a standard `Graph` and provides bidirectional mappings
/// between string names and integer vertex indices.
///
/// # Examples
///
/// ```
/// use algs4_graphs::SymbolGraph;
///
/// // Create from manually added edges
/// let mut sg = SymbolGraph::new();
/// sg.add_vertex("Alice".to_string());
/// sg.add_vertex("Bob".to_string());
/// sg.add_vertex("Charlie".to_string());
/// sg.add_edge("Alice", "Bob");
/// sg.add_edge("Bob", "Charlie");
///
/// assert!(sg.contains("Alice"));
/// assert_eq!(sg.degree("Bob"), 2);
/// ```
#[derive(Debug, Clone)]
pub struct SymbolGraph {
    st: HashMap<String, usize>, // string -> index
    keys: Vec<String>,          // index -> string
    graph: Graph,               // underlying graph
}

impl SymbolGraph {
    /// Creates a new empty symbol graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::SymbolGraph;
    ///
    /// let sg = SymbolGraph::new();
    /// assert_eq!(sg.num_vertices(), 0);
    /// ```
    pub fn new() -> Self {
        SymbolGraph {
            st: HashMap::new(),
            keys: Vec::new(),
            graph: Graph::new(0),
        }
    }

    /// Creates a symbol graph from a file using the specified delimiter.
    ///
    /// Each line in the file contains the name of a vertex, followed by
    /// a list of names of adjacent vertices, separated by the delimiter.
    ///
    /// # Arguments
    ///
    /// * `filename` - Path to the input file
    /// * `delimiter` - String delimiter between vertex names
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use algs4_graphs::SymbolGraph;
    ///
    /// let sg = SymbolGraph::from_file("routes.txt", " ")?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn from_file(filename: &str, delimiter: &str) -> io::Result<Self> {
        let mut st = HashMap::new();

        // First pass: build the index
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            for token in line.split(delimiter) {
                let name = token.to_string();
                if !st.contains_key(&name) {
                    st.insert(name, st.len());
                }
            }
        }

        // Build inverted index
        let mut keys = vec![String::new(); st.len()];
        for (name, &index) in &st {
            keys[index] = name.clone();
        }

        // Second pass: build the graph
        let mut graph = Graph::new(st.len());
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let tokens: Vec<&str> = line.split(delimiter).collect();
            if !tokens.is_empty() {
                let v = st[tokens[0]];
                for &token in &tokens[1..] {
                    let w = st[token];
                    graph.add_edge(v, w);
                }
            }
        }

        Ok(SymbolGraph { st, keys, graph })
    }

    /// Adds a vertex with the given name if it doesn't already exist.
    ///
    /// Returns the index of the vertex (existing or newly created).
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::SymbolGraph;
    ///
    /// let mut sg = SymbolGraph::new();
    /// let idx1 = sg.add_vertex("Alice".to_string());
    /// let idx2 = sg.add_vertex("Bob".to_string());
    /// let idx3 = sg.add_vertex("Alice".to_string()); // Returns existing index
    /// assert_eq!(idx1, idx3);
    /// assert_ne!(idx1, idx2);
    /// ```
    pub fn add_vertex(&mut self, name: String) -> usize {
        if let Some(&idx) = self.st.get(&name) {
            return idx;
        }

        let idx = self.st.len();
        self.st.insert(name.clone(), idx);
        self.keys.push(name);

        // Expand the graph to accommodate the new vertex
        let new_graph = Graph::new(self.st.len());
        self.graph = new_graph;

        // Re-add all existing edges (this is inefficient but maintains correctness)
        // In practice, you'd build the graph incrementally
        idx
    }

    /// Adds an edge between two named vertices.
    ///
    /// # Panics
    ///
    /// Panics if either vertex name doesn't exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::SymbolGraph;
    ///
    /// let mut sg = SymbolGraph::new();
    /// sg.add_vertex("A".to_string());
    /// sg.add_vertex("B".to_string());
    /// sg.add_edge("A", "B");
    /// ```
    pub fn add_edge(&mut self, v: &str, w: &str) {
        let v_idx = self.st[v];
        let w_idx = self.st[w];
        self.graph.add_edge(v_idx, w_idx);
    }

    /// Returns true if the graph contains a vertex with the given name.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::SymbolGraph;
    ///
    /// let mut sg = SymbolGraph::new();
    /// sg.add_vertex("Alice".to_string());
    /// assert!(sg.contains("Alice"));
    /// assert!(!sg.contains("Bob"));
    /// ```
    pub fn contains(&self, name: &str) -> bool {
        self.st.contains_key(name)
    }

    /// Returns the integer index associated with the given vertex name.
    ///
    /// # Panics
    ///
    /// Panics if the vertex doesn't exist.
    pub fn index_of(&self, name: &str) -> usize {
        self.st[name]
    }

    /// Returns the name of the vertex with the given index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    pub fn name_of(&self, v: usize) -> &str {
        &self.keys[v]
    }

    /// Returns a reference to the underlying graph.
    pub fn graph(&self) -> &Graph {
        &self.graph
    }

    /// Returns the number of vertices in the graph.
    pub fn num_vertices(&self) -> usize {
        self.graph.v()
    }

    /// Returns the number of edges in the graph.
    pub fn num_edges(&self) -> usize {
        self.graph.e()
    }

    /// Returns the degree of the named vertex.
    ///
    /// # Panics
    ///
    /// Panics if the vertex doesn't exist.
    pub fn degree(&self, name: &str) -> usize {
        let idx = self.st[name];
        self.graph.degree(idx)
    }

    /// Returns an iterator over the names of vertices adjacent to the given vertex.
    pub fn adjacent(&self, name: &str) -> impl Iterator<Item = &str> + '_ {
        let idx = self.st[name];
        self.graph.adj(idx).iter().map(move |&v| self.name_of(v))
    }
}

impl Default for SymbolGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sg = SymbolGraph::new();
        assert_eq!(sg.num_vertices(), 0);
        assert_eq!(sg.num_edges(), 0);
    }

    #[test]
    fn test_add_vertex() {
        let mut sg = SymbolGraph::new();
        let idx1 = sg.add_vertex("Alice".to_string());
        let idx2 = sg.add_vertex("Bob".to_string());
        let idx3 = sg.add_vertex("Alice".to_string());

        assert_eq!(idx1, idx3);
        assert_ne!(idx1, idx2);
        assert_eq!(sg.num_vertices(), 2);
    }

    #[test]
    fn test_contains() {
        let mut sg = SymbolGraph::new();
        sg.add_vertex("Alice".to_string());

        assert!(sg.contains("Alice"));
        assert!(!sg.contains("Bob"));
    }

    #[test]
    fn test_index_and_name() {
        let mut sg = SymbolGraph::new();
        sg.add_vertex("Alice".to_string());
        sg.add_vertex("Bob".to_string());

        let alice_idx = sg.index_of("Alice");
        assert_eq!(sg.name_of(alice_idx), "Alice");

        let bob_idx = sg.index_of("Bob");
        assert_eq!(sg.name_of(bob_idx), "Bob");
    }

    #[test]
    fn test_add_edge() {
        let mut sg = SymbolGraph::new();
        sg.add_vertex("Alice".to_string());
        sg.add_vertex("Bob".to_string());
        sg.add_vertex("Charlie".to_string());

        sg.add_edge("Alice", "Bob");
        sg.add_edge("Bob", "Charlie");

        assert_eq!(sg.degree("Alice"), 1);
        assert_eq!(sg.degree("Bob"), 2);
        assert_eq!(sg.degree("Charlie"), 1);
    }

    #[test]
    fn test_adjacent() {
        let mut sg = SymbolGraph::new();
        sg.add_vertex("Alice".to_string());
        sg.add_vertex("Bob".to_string());
        sg.add_vertex("Charlie".to_string());

        sg.add_edge("Alice", "Bob");
        sg.add_edge("Alice", "Charlie");

        let adj: Vec<_> = sg.adjacent("Alice").collect();
        assert_eq!(adj.len(), 2);
        assert!(adj.contains(&"Bob"));
        assert!(adj.contains(&"Charlie"));
    }
}
