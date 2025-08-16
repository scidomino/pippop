use super::point::Point;
use super::edge::{Edge, EdgeKey};
use slotmap::new_key_type;

new_key_type! {
    pub struct VertexKey;
}

impl VertexKey {
    pub fn edge_key(self, offset: u8) -> EdgeKey {
        assert!(offset < 3, "Offset must be 0, 1, or 2");
        EdgeKey::new(self, offset)
    }

    pub fn edge_keys(self) -> [EdgeKey; 3] {
        [self.edge_key(0), self.edge_key(1), self.edge_key(2)]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub point: Point,
    pub edges: [Edge; 3],
}

impl Vertex {
    pub fn new(point: Point) -> Self {
        Vertex {
            point,
            edges: [Edge::new(point), Edge::new(point), Edge::new(point)],
        }
    }
}
