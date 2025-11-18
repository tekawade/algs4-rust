//! Flow edge data type for flow networks.
//!
//! This module provides a flow edge with capacity and flow,
//! supporting residual capacity calculations.

use std::fmt;

/// A flow edge in a flow network.
///
/// Each flow edge has a source vertex, a destination vertex, a capacity,
/// and a current flow. The residual capacity is the difference between
/// capacity and flow.
///
/// # Examples
///
/// ```
/// use algs4_graphs::FlowEdge;
///
/// let mut edge = FlowEdge::new(0, 1, 10.0);
/// assert_eq!(edge.capacity(), 10.0);
/// assert_eq!(edge.flow(), 0.0);
/// assert_eq!(edge.residual_capacity_to(1), 10.0);
///
/// edge.add_residual_flow_to(1, 5.0);
/// assert_eq!(edge.flow(), 5.0);
/// assert_eq!(edge.residual_capacity_to(1), 5.0);
/// assert_eq!(edge.residual_capacity_to(0), 5.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct FlowEdge {
    v: usize,      // from vertex
    w: usize,      // to vertex
    capacity: f64, // edge capacity
    flow: f64,     // current flow
}

impl FlowEdge {
    /// Creates a new flow edge from vertex v to vertex w with given capacity.
    ///
    /// # Arguments
    ///
    /// * `v` - The source vertex
    /// * `w` - The destination vertex
    /// * `capacity` - The capacity of the edge
    ///
    /// # Panics
    ///
    /// Panics if capacity is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::FlowEdge;
    ///
    /// let edge = FlowEdge::new(0, 1, 10.0);
    /// assert_eq!(edge.from(), 0);
    /// assert_eq!(edge.to(), 1);
    /// assert_eq!(edge.capacity(), 10.0);
    /// ```
    pub fn new(v: usize, w: usize, capacity: f64) -> Self {
        if capacity < 0.0 {
            panic!("Edge capacity must be non-negative");
        }
        FlowEdge {
            v,
            w,
            capacity,
            flow: 0.0,
        }
    }

    /// Creates a new flow edge with initial flow.
    ///
    /// # Arguments
    ///
    /// * `v` - The source vertex
    /// * `w` - The destination vertex
    /// * `capacity` - The capacity of the edge
    /// * `flow` - The initial flow
    ///
    /// # Panics
    ///
    /// Panics if capacity or flow is negative, or if flow exceeds capacity.
    pub fn with_flow(v: usize, w: usize, capacity: f64, flow: f64) -> Self {
        if capacity < 0.0 {
            panic!("Edge capacity must be non-negative");
        }
        if flow < 0.0 {
            panic!("Edge flow must be non-negative");
        }
        if flow > capacity {
            panic!("Flow exceeds capacity");
        }
        FlowEdge {
            v,
            w,
            capacity,
            flow,
        }
    }

    /// Returns the source vertex of the edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::FlowEdge;
    ///
    /// let edge = FlowEdge::new(0, 1, 10.0);
    /// assert_eq!(edge.from(), 0);
    /// ```
    pub fn from(&self) -> usize {
        self.v
    }

    /// Returns the destination vertex of the edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::FlowEdge;
    ///
    /// let edge = FlowEdge::new(0, 1, 10.0);
    /// assert_eq!(edge.to(), 1);
    /// ```
    pub fn to(&self) -> usize {
        self.w
    }

    /// Returns the capacity of the edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::FlowEdge;
    ///
    /// let edge = FlowEdge::new(0, 1, 10.0);
    /// assert_eq!(edge.capacity(), 10.0);
    /// ```
    pub fn capacity(&self) -> f64 {
        self.capacity
    }

    /// Returns the current flow on the edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::FlowEdge;
    ///
    /// let edge = FlowEdge::new(0, 1, 10.0);
    /// assert_eq!(edge.flow(), 0.0);
    /// ```
    pub fn flow(&self) -> f64 {
        self.flow
    }

    /// Returns the other endpoint of the edge.
    ///
    /// # Arguments
    ///
    /// * `vertex` - One endpoint of the edge
    ///
    /// # Panics
    ///
    /// Panics if vertex is not an endpoint of the edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::FlowEdge;
    ///
    /// let edge = FlowEdge::new(0, 1, 10.0);
    /// assert_eq!(edge.other(0), 1);
    /// assert_eq!(edge.other(1), 0);
    /// ```
    pub fn other(&self, vertex: usize) -> usize {
        if vertex == self.v {
            self.w
        } else if vertex == self.w {
            self.v
        } else {
            panic!("Invalid endpoint");
        }
    }

    /// Returns the residual capacity of the edge in the direction to vertex.
    ///
    /// If vertex is the destination, returns capacity - flow.
    /// If vertex is the source, returns flow (backward edge).
    ///
    /// # Arguments
    ///
    /// * `vertex` - The vertex to which we want the residual capacity
    ///
    /// # Panics
    ///
    /// Panics if vertex is not an endpoint of the edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::FlowEdge;
    ///
    /// let mut edge = FlowEdge::new(0, 1, 10.0);
    /// edge.add_residual_flow_to(1, 3.0);
    ///
    /// assert_eq!(edge.residual_capacity_to(1), 7.0);
    /// assert_eq!(edge.residual_capacity_to(0), 3.0);
    /// ```
    pub fn residual_capacity_to(&self, vertex: usize) -> f64 {
        if vertex == self.w {
            self.capacity - self.flow // forward edge
        } else if vertex == self.v {
            self.flow // backward edge
        } else {
            panic!("Invalid endpoint");
        }
    }

    /// Adds flow in the direction to vertex.
    ///
    /// If vertex is the destination, increases flow.
    /// If vertex is the source, decreases flow (backward edge).
    ///
    /// # Arguments
    ///
    /// * `vertex` - The vertex to which we add flow
    /// * `delta` - The amount of flow to add
    ///
    /// # Panics
    ///
    /// Panics if vertex is not an endpoint, if delta is negative,
    /// or if delta exceeds residual capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_graphs::FlowEdge;
    ///
    /// let mut edge = FlowEdge::new(0, 1, 10.0);
    /// edge.add_residual_flow_to(1, 5.0);
    /// assert_eq!(edge.flow(), 5.0);
    /// ```
    pub fn add_residual_flow_to(&mut self, vertex: usize, delta: f64) {
        if delta < 0.0 {
            panic!("Delta must be non-negative");
        }

        if vertex == self.w {
            self.flow += delta; // forward edge
        } else if vertex == self.v {
            self.flow -= delta; // backward edge
        } else {
            panic!("Invalid endpoint");
        }

        // Check for numerical precision issues
        const EPSILON: f64 = 1e-10;
        if self.flow < -EPSILON {
            panic!("Flow is negative");
        }
        if self.flow > self.capacity + EPSILON {
            panic!("Flow exceeds capacity");
        }

        // Round to zero or capacity if very close
        if self.flow < EPSILON {
            self.flow = 0.0;
        }
        if self.flow > self.capacity - EPSILON {
            self.flow = self.capacity;
        }
    }
}

impl fmt::Display for FlowEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}->{} {:.2}/{:.2}",
            self.v, self.w, self.flow, self.capacity
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_flow_edge() {
        let edge = FlowEdge::new(0, 1, 10.0);
        assert_eq!(edge.from(), 0);
        assert_eq!(edge.to(), 1);
        assert_eq!(edge.capacity(), 10.0);
        assert_eq!(edge.flow(), 0.0);
    }

    #[test]
    fn test_residual_capacity() {
        let edge = FlowEdge::new(0, 1, 10.0);
        assert_eq!(edge.residual_capacity_to(1), 10.0);
        assert_eq!(edge.residual_capacity_to(0), 0.0);
    }

    #[test]
    fn test_add_flow() {
        let mut edge = FlowEdge::new(0, 1, 10.0);
        edge.add_residual_flow_to(1, 5.0);
        assert_eq!(edge.flow(), 5.0);
        assert_eq!(edge.residual_capacity_to(1), 5.0);
        assert_eq!(edge.residual_capacity_to(0), 5.0);
    }

    #[test]
    fn test_backward_flow() {
        let mut edge = FlowEdge::new(0, 1, 10.0);
        edge.add_residual_flow_to(1, 8.0);
        edge.add_residual_flow_to(0, 3.0);
        assert_eq!(edge.flow(), 5.0);
    }

    #[test]
    fn test_other() {
        let edge = FlowEdge::new(0, 1, 10.0);
        assert_eq!(edge.other(0), 1);
        assert_eq!(edge.other(1), 0);
    }

    #[test]
    #[should_panic(expected = "Edge capacity must be non-negative")]
    fn test_negative_capacity() {
        FlowEdge::new(0, 1, -5.0);
    }

    #[test]
    #[should_panic(expected = "Delta must be non-negative")]
    fn test_negative_delta() {
        let mut edge = FlowEdge::new(0, 1, 10.0);
        edge.add_residual_flow_to(1, -5.0);
    }
}
