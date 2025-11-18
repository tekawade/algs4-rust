///! Symbol digraph - directed graph with string vertex names.
///!
///! Provides a wrapper around the Digraph data type that allows vertices to be
///! identified by arbitrary strings rather than integers.
use crate::digraph::Digraph;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Directed graph with string vertex names.
///
/// This struct wraps a standard `Digraph` and provides bidirectional mappings
/// between string names and integer vertex indices.
///
/// # Examples
///
/// ```
/// use algs4_graphs::SymbolDigraph;
///
/// // Create from manually added edges
/// let mut sg = SymbolDigraph::new();
/// sg.add_vertex("A".to_string());
/// sg.add_vertex("B".to_string());
/// sg.add_vertex("C".to_string());
/// sg.add_edge("A", "B");
/// sg.add_edge("B", "C");
/// sg.add_edge("C", "A"); // Creates a cycle
///
/// assert!(sg.contains("A"));
/// assert_eq!(sg.outdegree("B"), 1);
/// assert_eq!(sg.indegree("C"), 1);
/// ```
#[derive(Debug, Clone)]
pub struct SymbolDigraph {
    st: HashMap<String, usize>, // string -> index
    keys: Vec<String>,          // index -> string
    digraph: Digraph,           // underlying digraph
}

impl SymbolDigraph {
    /// Creates a new empty symbol digraph.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::SymbolDigraph;
    ///
    /// let sg = SymbolDigraph::new();
    /// assert_eq!(sg.num_vertices(), 0);
    /// ```
    pub fn new() -> Self {
        SymbolDigraph {
            st: HashMap::new(),
            keys: Vec::new(),
            digraph: Digraph::new(0),
        }
    }

    /// Creates a symbol digraph from a file using the specified delimiter.
    ///
    /// Each line in the file contains the name of a vertex, followed by
    /// a list of names of vertices pointed to by that vertex, separated by the delimiter.
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
    /// use algs4_graphs::SymbolDigraph;
    ///
    /// let sg = SymbolDigraph::from_file("links.txt", " ")?;
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

        // Second pass: build the digraph
        let mut digraph = Digraph::new(st.len());
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let tokens: Vec<&str> = line.split(delimiter).collect();
            if !tokens.is_empty() {
                let v = st[tokens[0]];
                for &token in &tokens[1..] {
                    let w = st[token];
                    digraph.add_edge(v, w);
                }
            }
        }

        Ok(SymbolDigraph { st, keys, digraph })
    }

    /// Adds a vertex with the given name if it doesn't already exist.
    ///
    /// Returns the index of the vertex (existing or newly created).
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::SymbolDigraph;
    ///
    /// let mut sg = SymbolDigraph::new();
    /// let idx1 = sg.add_vertex("A".to_string());
    /// let idx2 = sg.add_vertex("B".to_string());
    /// let idx3 = sg.add_vertex("A".to_string()); // Returns existing index
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

        // Expand the digraph to accommodate the new vertex
        let new_digraph = Digraph::new(self.st.len());
        self.digraph = new_digraph;

        idx
    }

    /// Adds a directed edge from vertex v to vertex w.
    ///
    /// # Panics
    ///
    /// Panics if either vertex name doesn't exist.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::SymbolDigraph;
    ///
    /// let mut sg = SymbolDigraph::new();
    /// sg.add_vertex("A".to_string());
    /// sg.add_vertex("B".to_string());
    /// sg.add_edge("A", "B"); // A -> B
    /// ```
    pub fn add_edge(&mut self, v: &str, w: &str) {
        let v_idx = self.st[v];
        let w_idx = self.st[w];
        self.digraph.add_edge(v_idx, w_idx);
    }

    /// Returns true if the digraph contains a vertex with the given name.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::SymbolDigraph;
    ///
    /// let mut sg = SymbolDigraph::new();
    /// sg.add_vertex("A".to_string());
    /// assert!(sg.contains("A"));
    /// assert!(!sg.contains("B"));
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

    /// Returns a reference to the underlying digraph.
    pub fn digraph(&self) -> &Digraph {
        &self.digraph
    }

    /// Returns the number of vertices in the digraph.
    pub fn num_vertices(&self) -> usize {
        self.digraph.v()
    }

    /// Returns the number of edges in the digraph.
    pub fn num_edges(&self) -> usize {
        self.digraph.e()
    }

    /// Returns the outdegree of the named vertex.
    ///
    /// # Panics
    ///
    /// Panics if the vertex doesn't exist.
    pub fn outdegree(&self, name: &str) -> usize {
        let idx = self.st[name];
        self.digraph.outdegree(idx)
    }

    /// Returns the indegree of the named vertex.
    ///
    /// # Panics
    ///
    /// Panics if the vertex doesn't exist.
    pub fn indegree(&self, name: &str) -> usize {
        let idx = self.st[name];
        self.digraph.indegree(idx)
    }

    /// Returns an iterator over the names of vertices adjacent to the given vertex.
    pub fn adjacent(&self, name: &str) -> impl Iterator<Item = &str> + '_ {
        let idx = self.st[name];
        self.digraph.adj(idx).iter().map(move |&v| self.name_of(v))
    }
}

impl Default for SymbolDigraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sg = SymbolDigraph::new();
        assert_eq!(sg.num_vertices(), 0);
        assert_eq!(sg.num_edges(), 0);
    }

    #[test]
    fn test_add_vertex() {
        let mut sg = SymbolDigraph::new();
        let idx1 = sg.add_vertex("A".to_string());
        let idx2 = sg.add_vertex("B".to_string());
        let idx3 = sg.add_vertex("A".to_string());

        assert_eq!(idx1, idx3);
        assert_ne!(idx1, idx2);
        assert_eq!(sg.num_vertices(), 2);
    }

    #[test]
    fn test_contains() {
        let mut sg = SymbolDigraph::new();
        sg.add_vertex("A".to_string());

        assert!(sg.contains("A"));
        assert!(!sg.contains("B"));
    }

    #[test]
    fn test_index_and_name() {
        let mut sg = SymbolDigraph::new();
        sg.add_vertex("A".to_string());
        sg.add_vertex("B".to_string());

        let a_idx = sg.index_of("A");
        assert_eq!(sg.name_of(a_idx), "A");

        let b_idx = sg.index_of("B");
        assert_eq!(sg.name_of(b_idx), "B");
    }

    #[test]
    fn test_add_edge_directed() {
        let mut sg = SymbolDigraph::new();
        sg.add_vertex("A".to_string());
        sg.add_vertex("B".to_string());
        sg.add_vertex("C".to_string());

        sg.add_edge("A", "B"); // A -> B
        sg.add_edge("B", "C"); // B -> C

        assert_eq!(sg.outdegree("A"), 1);
        assert_eq!(sg.indegree("A"), 0);

        assert_eq!(sg.outdegree("B"), 1);
        assert_eq!(sg.indegree("B"), 1);

        assert_eq!(sg.outdegree("C"), 0);
        assert_eq!(sg.indegree("C"), 1);
    }

    #[test]
    fn test_adjacent() {
        let mut sg = SymbolDigraph::new();
        sg.add_vertex("A".to_string());
        sg.add_vertex("B".to_string());
        sg.add_vertex("C".to_string());

        sg.add_edge("A", "B");
        sg.add_edge("A", "C");

        let adj: Vec<_> = sg.adjacent("A").collect();
        assert_eq!(adj.len(), 2);
        assert!(adj.contains(&"B"));
        assert!(adj.contains(&"C"));

        let adj_b: Vec<_> = sg.adjacent("B").collect();
        assert_eq!(adj_b.len(), 0); // B has no outgoing edges
    }

    #[test]
    fn test_cycle() {
        let mut sg = SymbolDigraph::new();
        sg.add_vertex("A".to_string());
        sg.add_vertex("B".to_string());
        sg.add_vertex("C".to_string());

        sg.add_edge("A", "B");
        sg.add_edge("B", "C");
        sg.add_edge("C", "A"); // Creates a cycle

        assert_eq!(sg.outdegree("A"), 1);
        assert_eq!(sg.indegree("A"), 1);
    }
}
