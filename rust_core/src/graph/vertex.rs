// rust_core/src/graph/vertex.rs

use crate::geom::point::Point;

#[derive(Debug)]
pub struct Vertex {
    pub position: Point,
    pub edge_id: usize,
}
