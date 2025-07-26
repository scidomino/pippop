// rust_core/src/graph/edge.rs

use crate::geom::point::Point;

#[derive(Debug)]
pub struct Edge {
    /// The ID of the Vertex where this edge starts.
    pub start_vertex_id: usize,
    /// The control point associated with the start of this edge.
    /// The full Bezier curve is defined by:
    /// p0: start_vertex.position
    /// p1: this.control_point
    /// p2: twin_edge.control_point
    /// p3: twin_edge.start_vertex.position
    pub control_point: Point,
    
    // Topology
    pub twin_edge_id: usize,
    pub next_edge_id: usize,
    pub bubble_id: usize,
}
