use super::edge::{Edge, EdgeKey, Slot};
use super::point::Point;
use slotmap::{new_key_type, SlotMap};
use std::ops::{Deref, DerefMut};

new_key_type! {
    pub struct VertexKey;
}

impl VertexKey {
    pub fn slot(self, slot: super::edge::Slot) -> EdgeKey {
        EdgeKey::new(self, slot)
    }

    pub fn edge_keys(self) -> [EdgeKey; 3] {
        [self.slot(Slot::A), self.slot(Slot::B), self.slot(Slot::C)]
    }
}

/// A junction where exactly three bubble walls (edges) meet.
///
/// Following Plateau's Laws, the physics simulation drives the points
/// to maintain 120-degree angles between the three outgoing half-edges.
#[derive(Debug, Clone)]
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
            edges: [Edge::new(point), Edge::new(point), Edge::new(point)],
        }
    }

    pub fn edge(&self, key: EdgeKey) -> &Edge {
        &self.edges[key.slot]
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
        &self.inner[key.vertex].edges[key.slot]
    }

    pub fn get_edge_mut(&mut self, key: EdgeKey) -> &mut Edge {
        &mut self.inner[key.vertex].edges[key.slot]
    }

    pub fn get_edge_and_vertex(&self, key: EdgeKey) -> (&Edge, &Vertex) {
        let vertex = &self.inner[key.vertex];
        (&vertex.edges[key.slot], vertex)
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
