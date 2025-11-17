//! Weighted edge for undirected edge-weighted graphs.

use std::cmp::Ordering;
use std::fmt;

/// A weighted edge in an undirected edge-weighted graph.
///
/// Each edge consists of two vertices and a real-valued weight.
///
/// # Examples
///
/// ```
/// use algs4_graphs::Edge;
///
/// let edge = Edge::new(0, 1, 0.5);
/// assert_eq!(edge.weight(), 0.5);
/// assert_eq!(edge.either(), 0);
/// assert_eq!(edge.other(0), 1);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Edge {
    v: usize,
    w: usize,
    weight: f64,
}

impl Edge {
    /// Creates a new weighted edge between vertices `v` and `w` with the given `weight`.
    ///
    /// # Arguments
    ///
    /// * `v` - One vertex
    /// * `w` - The other vertex
    /// * `weight` - The weight of the edge
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::Edge;
    ///
    /// let edge = Edge::new(0, 1, 0.35);
    /// assert_eq!(edge.weight(), 0.35);
    /// ```
    pub fn new(v: usize, w: usize, weight: f64) -> Self {
        Edge { v, w, weight }
    }

    /// Returns the weight of the edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::Edge;
    ///
    /// let edge = Edge::new(0, 1, 0.5);
    /// assert_eq!(edge.weight(), 0.5);
    /// ```
    pub fn weight(&self) -> f64 {
        self.weight
    }

    /// Returns either endpoint of the edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::Edge;
    ///
    /// let edge = Edge::new(3, 7, 0.5);
    /// assert_eq!(edge.either(), 3);
    /// ```
    pub fn either(&self) -> usize {
        self.v
    }

    /// Returns the endpoint of the edge that is different from the given vertex.
    ///
    /// # Arguments
    ///
    /// * `vertex` - One endpoint
    ///
    /// # Returns
    ///
    /// The other endpoint
    ///
    /// # Panics
    ///
    /// Panics if the vertex is not one of the endpoints of the edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::Edge;
    ///
    /// let edge = Edge::new(3, 7, 0.5);
    /// assert_eq!(edge.other(3), 7);
    /// assert_eq!(edge.other(7), 3);
    /// ```
    pub fn other(&self, vertex: usize) -> usize {
        if vertex == self.v {
            self.w
        } else if vertex == self.w {
            self.v
        } else {
            panic!("Illegal endpoint");
        }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl Eq for Edge {}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.partial_cmp(&other.weight).unwrap_or(Ordering::Equal)
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{} {:.5}", self.v, self.w, self.weight)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_edge() {
        let edge = Edge::new(0, 1, 0.5);
        assert_eq!(edge.weight(), 0.5);
        assert_eq!(edge.either(), 0);
        assert_eq!(edge.other(0), 1);
        assert_eq!(edge.other(1), 0);
    }

    #[test]
    fn test_comparison() {
        let e1 = Edge::new(0, 1, 0.3);
        let e2 = Edge::new(2, 3, 0.5);
        let e3 = Edge::new(4, 5, 0.3);

        assert!(e1 < e2);
        assert!(e2 > e1);
        assert_eq!(e1, e3);
    }

    #[test]
    #[should_panic(expected = "Illegal endpoint")]
    fn test_invalid_other() {
        let edge = Edge::new(0, 1, 0.5);
        edge.other(2);  // vertex 2 is not an endpoint
    }

    #[test]
    fn test_display() {
        let edge = Edge::new(3, 7, 0.12345);
        let output = format!("{}", edge);
        assert!(output.contains("3-7"));
        assert!(output.contains("0.12345"));
    }

    #[test]
    fn test_ordering() {
        let mut edges = vec![
            Edge::new(0, 1, 0.5),
            Edge::new(1, 2, 0.3),
            Edge::new(2, 3, 0.7),
        ];

        edges.sort();

        assert_eq!(edges[0].weight(), 0.3);
        assert_eq!(edges[1].weight(), 0.5);
        assert_eq!(edges[2].weight(), 0.7);
    }
}
