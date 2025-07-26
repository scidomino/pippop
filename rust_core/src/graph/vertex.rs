// rust_core/src/graph/vertex.rs

use crate::geom::point::Point;
use crate::graph::EdgeKey;

#[derive(Debug)]
pub struct Vertex {
    pub position: Point,
    pub edge_key: EdgeKey,
}