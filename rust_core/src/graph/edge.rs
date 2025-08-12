use super::vertex::VertexKey;
use super::bubble::BubbleKey;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeKey {
    pub vertex: VertexKey,
    pub offset: u8, // 0, 1, or 2
}
impl EdgeKey {
    pub fn default() -> Self {
        EdgeKey {
            vertex: VertexKey::default(),
            offset: 0,
        }
    }

    pub fn new(vertex: VertexKey, offset: u8) -> Self {
        assert!(offset < 3, "Offset must be 0, 1, or 2");
        EdgeKey { vertex, offset }
    }

    pub fn next_on_vertex(&self) -> EdgeKey {
        EdgeKey {
            vertex: self.vertex,
            offset: (self.offset + 1) % 3,
        }
    }

    pub fn prev_on_vertex(&self) -> EdgeKey {
        EdgeKey {
            vertex: self.vertex,
            offset: (self.offset + 2) % 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub twin: EdgeKey,
    pub bubble: BubbleKey,
}

impl Edge {
    pub fn new() -> Self {
        Edge {
            twin: EdgeKey::default(),
            bubble: BubbleKey::default(),
        }
    }
}