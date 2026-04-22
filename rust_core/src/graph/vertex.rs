use super::edge::{Edge, EdgeKey};
use super::point::Point;
use slotmap::{new_key_type, SlotMap};
use std::ops::{Deref, DerefMut};

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

/// A junction where exactly three bubble walls (edges) meet.
///
/// Following Plateau's Laws, the physics simulation drives the points
/// to maintain 120-degree angles between the three outgoing half-edges.
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    /// The physical location and velocity of this junction.
    pub point: Point,
    /// The three outgoing half-edges originating from this vertex, indexed clockwise.
    pub edges: [Edge; 3],
}

impl Vertex {
    pub fn new(point: Point) -> Self {
        Vertex {
            point,
            edges: [Edge::new(point); 3],
        }
    }

    pub fn edge(&self, key: EdgeKey) -> Edge {
        self.edges[key.offset as usize]
    }
}

pub struct VertexSet {
    pub inner: SlotMap<VertexKey, Vertex>,
}

impl VertexSet {
    pub fn new() -> Self {
        Self {
            inner: SlotMap::with_key(),
        }
    }

    pub fn get_edge(&self, key: EdgeKey) -> &Edge {
        &self.inner[key.vertex].edges[key.offset as usize]
    }

    pub fn get_edge_mut(&mut self, key: EdgeKey) -> &mut Edge {
        &mut self.inner[key.vertex].edges[key.offset as usize]
    }

    pub fn get_edge_and_vertex(&self, key: EdgeKey) -> (&Edge, &Vertex) {
        let vertex = &self.inner[key.vertex];
        (&vertex.edges[key.offset as usize], vertex)
    }

    pub fn get_bezier(&self, ekey: EdgeKey) -> crate::graphics::geometry::Bezier {
        let (edge, vertex) = self.get_edge_and_vertex(ekey);
        let (twin, twin_vertex) = self.get_edge_and_vertex(edge.twin);
        crate::graphics::geometry::Bezier::from_points(
            vertex.point.position,
            edge.point.position,
            twin.point.position,
            twin_vertex.point.position,
        )
    }

    // next edge on the same bubble in clockwise order
    pub fn next_on_bubble(&self, key: EdgeKey) -> EdgeKey {
        self.get_edge(key).twin.prev_on_vertex()
    }
}

impl Deref for VertexSet {
    type Target = SlotMap<VertexKey, Vertex>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for VertexSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a> IntoIterator for &'a VertexSet {
    type Item = (VertexKey, &'a Vertex);
    type IntoIter = slotmap::basic::Iter<'a, VertexKey, Vertex>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'a> IntoIterator for &'a mut VertexSet {
    type Item = (VertexKey, &'a mut Vertex);
    type IntoIter = slotmap::basic::IterMut<'a, VertexKey, Vertex>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}
