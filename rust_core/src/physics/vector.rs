use crate::graph::edge::EdgeKey;
use crate::graph::vertex::VertexKey;
use macroquad::math::Vec2;
use slotmap::SecondaryMap;

#[derive(Debug, Clone, Default)]
struct Value {
    vertex: Vec2,
    edges: [Vec2; 3],
}

#[derive(Debug, Clone, Default)]
pub struct GraphVector {
    vertex_to_value: SecondaryMap<VertexKey, Value>,
}

impl GraphVector {
    pub fn new() -> Self {
        Self {
            vertex_to_value: SecondaryMap::new(),
        }
    }

    pub fn get_vertex(&self, key: VertexKey) -> Vec2 {
        self.vertex_to_value
            .get(key)
            .map_or_else(Vec2::default, |p| p.vertex)
    }

    pub fn get_edge(&self, key: EdgeKey) -> Vec2 {
        self.vertex_to_value
            .get(key.vertex)
            .map_or_else(Vec2::default, |p| p.edges[key.offset as usize])
    }

    fn get_mut(&mut self, key: VertexKey) -> &mut Value {
        self.vertex_to_value.entry(key).unwrap().or_default()
    }

    pub fn add_vertex(&mut self, key: VertexKey, value: Vec2) {
        self.get_mut(key).vertex += value;
    }

    pub fn add_edge(&mut self, key: EdgeKey, value: Vec2) {
        self.get_mut(key.vertex).edges[key.offset as usize] += value;
    }

    pub fn clear(&mut self) {
        self.vertex_to_value.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::point::Point;
    use crate::graph::vertex::{Vertex, VertexKey};
    use slotmap::SlotMap;

    #[test]
    fn test_new() {
        let gv = GraphVector::new();
        assert!(gv.vertex_to_value.is_empty());
    }

    #[test]
    fn test_add_and_get_vertex() {
        let mut sm: SlotMap<VertexKey, Vertex> = SlotMap::with_key();
        let v1 = sm.insert(Vertex::new(Point::default()));
        let mut gv = GraphVector::new();
        gv.add_vertex(v1, Vec2::new(1.0, 2.0));

        let result = gv.get_vertex(v1);

        assert_eq!(result, Vec2::new(1.0, 2.0));
    }

    #[test]
    fn test_add_and_get_edge() {
        let mut sm: SlotMap<VertexKey, Vertex> = SlotMap::with_key();
        let v1 = sm.insert(Vertex::new(Point::default()));
        let e1 = v1.edge_key(0);
        let mut gv = GraphVector::new();
        gv.add_edge(e1, Vec2::new(3.0, 4.0));

        let result = gv.get_edge(e1);

        assert_eq!(result, Vec2::new(3.0, 4.0));
    }

    #[test]
    fn test_get_unadded_returns_default() {
        let gv = GraphVector::new();
        let mut sm: SlotMap<VertexKey, Vertex> = SlotMap::with_key();
        let v1 = sm.insert(Vertex::new(Point::default()));
        let e1 = v1.edge_key(0);

        let vertex_result = gv.get_vertex(v1);

        assert_eq!(vertex_result, Vec2::ZERO);

        let edge_result = gv.get_edge(e1);
        assert_eq!(edge_result, Vec2::ZERO);
    }

    #[test]
    fn test_add_multiple_forces() {
        let mut gv = GraphVector::new();
        let mut sm: SlotMap<VertexKey, Vertex> = SlotMap::with_key();
        let v1 = sm.insert(Vertex::new(Point::default()));
        let e1 = v1.edge_key(0);

        gv.add_vertex(v1, Vec2::new(1.0, 2.0));
        gv.add_vertex(v1, Vec2::new(1.5, 2.5));

        let vertex_result = gv.get_vertex(v1);

        assert_eq!(vertex_result, Vec2::new(2.5, 4.5));

        gv.add_edge(e1, Vec2::new(3.0, 4.0));
        gv.add_edge(e1, Vec2::new(3.5, 4.5));

        let edge_result = gv.get_edge(e1);

        assert_eq!(edge_result, Vec2::new(6.5, 8.5));
    }

    #[test]
    fn test_clear() {
        let mut gv = GraphVector::new();
        let mut sm: SlotMap<VertexKey, Vertex> = SlotMap::with_key();
        let v1 = sm.insert(Vertex::new(Point::default()));
        let e1 = v1.edge_key(0);

        gv.add_vertex(v1, Vec2::new(1.0, 2.0));
        gv.add_edge(e1, Vec2::new(3.0, 4.0));

        gv.clear();

        assert!(gv.vertex_to_value.is_empty());

        let vertex_result = gv.get_vertex(v1);

        assert_eq!(vertex_result, Vec2::ZERO);
    }
}
