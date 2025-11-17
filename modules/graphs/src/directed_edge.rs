//! Weighted edge for directed edge-weighted graphs.

use std::cmp::Ordering;
use std::fmt;

/// A weighted directed edge in an edge-weighted digraph.
///
/// Each directed edge consists of two vertices and a real-valued weight.
/// The edge is directed from one vertex (the "from" vertex) to another
/// (the "to" vertex).
///
/// # Examples
///
/// ```
/// use algs4_graphs::DirectedEdge;
///
/// let edge = DirectedEdge::new(0, 1, 0.5);
/// assert_eq!(edge.from(), 0);
/// assert_eq!(edge.to(), 1);
/// assert_eq!(edge.weight(), 0.5);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct DirectedEdge {
    v: usize,    // from vertex
    w: usize,    // to vertex
    weight: f64, // edge weight
}

impl DirectedEdge {
    /// Creates a new weighted directed edge from vertex `v` to vertex `w`
    /// with the given `weight`.
    ///
    /// # Arguments
    ///
    /// * `v` - The tail vertex
    /// * `w` - The head vertex
    /// * `weight` - The weight of the edge
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::DirectedEdge;
    ///
    /// let edge = DirectedEdge::new(0, 1, 0.35);
    /// assert_eq!(edge.from(), 0);
    /// assert_eq!(edge.to(), 1);
    /// assert_eq!(edge.weight(), 0.35);
    /// ```
    pub fn new(v: usize, w: usize, weight: f64) -> Self {
        DirectedEdge { v, w, weight }
    }

    /// Returns the tail vertex of the directed edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::DirectedEdge;
    ///
    /// let edge = DirectedEdge::new(3, 7, 0.5);
    /// assert_eq!(edge.from(), 3);
    /// ```
    pub fn from(&self) -> usize {
        self.v
    }

    /// Returns the head vertex of the directed edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::DirectedEdge;
    ///
    /// let edge = DirectedEdge::new(3, 7, 0.5);
    /// assert_eq!(edge.to(), 7);
    /// ```
    pub fn to(&self) -> usize {
        self.w
    }

    /// Returns the weight of the directed edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::DirectedEdge;
    ///
    /// let edge = DirectedEdge::new(0, 1, 0.5);
    /// assert_eq!(edge.weight(), 0.5);
    /// ```
    pub fn weight(&self) -> f64 {
        self.weight
    }
}

impl PartialEq for DirectedEdge {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl Eq for DirectedEdge {}

impl PartialOrd for DirectedEdge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DirectedEdge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight
            .partial_cmp(&other.weight)
            .unwrap_or(Ordering::Equal)
    }
}

impl fmt::Display for DirectedEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}->{} {:.5}", self.v, self.w, self.weight)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_directed_edge() {
        let edge = DirectedEdge::new(0, 1, 0.5);
        assert_eq!(edge.from(), 0);
        assert_eq!(edge.to(), 1);
        assert_eq!(edge.weight(), 0.5);
    }

    #[test]
    fn test_comparison() {
        let e1 = DirectedEdge::new(0, 1, 0.3);
        let e2 = DirectedEdge::new(2, 3, 0.5);
        let e3 = DirectedEdge::new(4, 5, 0.3);

        assert!(e1 < e2);
        assert!(e2 > e1);
        assert_eq!(e1, e3);
    }

    #[test]
    fn test_display() {
        let edge = DirectedEdge::new(3, 7, 0.12345);
        let output = format!("{}", edge);
        assert!(output.contains("3->7"));
        assert!(output.contains("0.12345"));
    }

    #[test]
    fn test_ordering() {
        let mut edges = [
            DirectedEdge::new(0, 1, 0.5),
            DirectedEdge::new(1, 2, 0.3),
            DirectedEdge::new(2, 3, 0.7),
        ];

        edges.sort();

        assert_eq!(edges[0].weight(), 0.3);
        assert_eq!(edges[1].weight(), 0.5);
        assert_eq!(edges[2].weight(), 0.7);
    }

    #[test]
    fn test_negative_weight() {
        let edge = DirectedEdge::new(0, 1, -0.5);
        assert_eq!(edge.weight(), -0.5);
    }
}
