// rust_core/src/graph/edge.rs

use crate::geom::point::Point;
use crate::graph::{VertexKey, EdgeKey, BubbleKey};

#[derive(Debug)]
pub struct Edge {
    pub start_vertex_key: VertexKey,
    pub control_point: Point,
    
    // Topology
    pub twin_edge_key: EdgeKey,
    pub next_edge_key: EdgeKey,
    pub bubble_key: BubbleKey,

    // Pre-calculated physics values
    pub half_area: f64,
}
