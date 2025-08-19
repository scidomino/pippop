use crate::graph::edge::{EdgeKey, EdgeMap};
use crate::graph::point::Coordinate;
use crate::graph::vertex::VertexKey;
use slotmap::SecondaryMap;

#[derive(Debug, Clone)]
pub struct Force {
    vertex_to_force: SecondaryMap<VertexKey, Coordinate>,
    edge_to_force: EdgeMap<Coordinate>,
}

impl Force {
    pub fn new() -> Self {
        Self {
            vertex_to_force: SecondaryMap::new(),
            edge_to_force: EdgeMap::new(),
        }
    }

    pub fn add_vertex_force(&mut self, key: VertexKey, force: Coordinate) {
        if let Some(vertex_force) = self.vertex_to_force.get_mut(key) {
            vertex_force.x += force.x;
            vertex_force.y += force.y;
        } else {
            self.vertex_to_force.insert(key, force);
        }
    }

    pub fn add_edge_force(&mut self, key: EdgeKey, force: Coordinate) {
        if let Some(edge_force) = self.edge_to_force.get_mut(key) {
            edge_force.x += force.x;
            edge_force.y += force.y;
        } else {
            self.edge_to_force.insert(key, force);
        }
    }

    pub fn clear(&mut self) {
        self.vertex_to_force.clear();
        self.edge_to_force.clear();
    }
}
