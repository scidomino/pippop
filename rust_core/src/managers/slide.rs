use crate::graph::edge::EdgeKey;
use crate::graph::Graph;
use std::collections::HashMap;

const TIMEOUT: f32 = 1.0; // 1 second
const MIN_LENGTH: f32 = 10.0;

pub struct SlideManager {
    recently_slid: HashMap<EdgeKey, f32>,
}

impl Default for SlideManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SlideManager {
    pub fn new() -> Self {
        Self {
            recently_slid: HashMap::new(),
        }
    }

    pub fn slide_slidable_edges(&mut self, graph: &mut Graph, dt: f32) -> bool {
        self.prune(dt);
        if let Some(edge_key) = self.get_first_slidable(graph) {
            graph.slide(edge_key);
            self.recently_slid.insert(edge_key, TIMEOUT);
            return true;
        }
        false
    }

    fn prune(&mut self, dt: f32) {
        self.recently_slid.retain(|_, time_left| {
            *time_left -= dt;
            *time_left > 0.0
        });
    }

    fn get_first_slidable(&self, graph: &Graph) -> Option<EdgeKey> {
        graph.vertices.iter().find_map(|(vkey, vertex)| {
            vertex.edges.iter().enumerate().find_map(|(offset, edge)| {
                let edge_key = vkey.edge_key(offset as u8);

                graph
                    .vertices
                    .get(edge.twin.vertex)
                    .and_then(|twin_vertex| {
                        let length = vertex.point.position.distance(twin_vertex.point.position);
                        (length < MIN_LENGTH
                            && !self.recently_slid.contains_key(&edge_key)
                            && !self.recently_slid.contains_key(&edge.twin))
                        .then_some(edge_key)
                    })
            })
        })
    }
}
