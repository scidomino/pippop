// rust_core/src/graph/bubble.rs

#[derive(Debug)]
pub struct Bubble {
    /// The ID of the first edge in the cycle forming this bubble's boundary.
    pub first_edge_id: usize,
    pub pressure: f64,
}