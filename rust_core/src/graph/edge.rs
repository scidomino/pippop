use super::bubble::BubbleKey;
use super::point::Point;
use super::vertex::Vertex;
use super::vertex::VertexKey;

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

    // Returns the next edge key on the vertex in a clockwise direction
    pub fn next_on_vertex(&self) -> EdgeKey {
        EdgeKey {
            vertex: self.vertex,
            offset: (self.offset + 1) % 3,
        }
    }

    // Returns the previous edge key on the vertex in a clockwise direction
    pub fn prev_on_vertex(&self) -> EdgeKey {
        EdgeKey {
            vertex: self.vertex,
            offset: (self.offset + 2) % 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub point: Point,
    pub twin: EdgeKey,
    pub bubble: BubbleKey,
}

impl Edge {
    pub fn new(point: Point) -> Self {
        Edge {
            point: point,
            twin: EdgeKey::default(),
            bubble: BubbleKey::default(),
        }
    }

    pub fn get_half_area(&self, vertex: &Vertex, twin: &Edge, twin_vertex: &Vertex) -> f32 {
        let s = vertex.point.position;
        let sc = self.point.position;
        let ec = twin.point.position;
        let e = twin_vertex.point.position;

        // Calculate the area of the triangle formed by the points
        return (s.x * (-10.0 * s.y - 6.0 * sc.y - 3.0 * ec.y - e.y)
            + sc.x * (6.0 * s.y - 3.0 * ec.y - 3.0 * e.y))
            / 20.0;
    }
}
