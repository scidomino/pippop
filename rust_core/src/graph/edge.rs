use super::bubble::BubbleKey;
use super::point::Point;
use super::vertex::VertexKey;
use macroquad::math::Vec2;

/// A unique identifier for a directed half-edge in the graph.
///
/// Instead of a global edge pool, edges are owned by their originating vertex.
/// The key is a combination of the origin `VertexKey` and an `offset` (0, 1, or 2),
/// representing which of the three outgoing slots this edge occupies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct EdgeKey {
    pub vertex: VertexKey,
    pub offset: u8, // 0, 1, or 2
}

impl EdgeKey {
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

/// A directed half-edge representing one side of a bubble wall.
///
/// In this half-edge data structure, every physical boundary between two
/// vertices is represented by two directed `Edge`s (twins) pointing in opposite
/// directions.
#[derive(Debug, Clone, Copy)]
pub struct Edge {
    /// The physical control point used to curve the edge via Bezier interpolation.
    pub point: Point,
    /// The opposite half-edge pointing back toward this edge's origin vertex.
    pub twin: EdgeKey,
    /// The bubble bounded by this edge. By convention, traversing the edges
    /// of a bubble in a clockwise direction will have the bubble on the interior.
    pub bubble: BubbleKey,
    /// The cached partial area contribution of this directed edge's Bezier curve.
    /// Subtracting the twin's half_area from this yields the net area under the curve segment.
    pub half_area: f32,
    /// The cached partial centroid contribution of this directed edge's Bezier curve.
    /// Summed over all edges of a bubble and divided by total area to compute the true centroid.
    pub centroid_contribution: Vec2,
}

impl Edge {
    pub fn new(point: Point) -> Self {
        Edge {
            point,
            twin: EdgeKey::default(),
            bubble: BubbleKey::default(),
            half_area: 0.0,
            centroid_contribution: Vec2::ZERO,
        }
    }
}
