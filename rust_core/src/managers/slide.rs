use crate::graph::edge::EdgeKey;
use crate::graph::Graph;
use crate::managers::burst::BurstManager;
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

    pub fn slide_slidable_edges(&mut self, graph: &mut Graph, burst: &BurstManager, dt: f32) {
        self.prune(dt);
        if let Some(edge_key) = self.get_first_slidable(graph) {
            graph.slide(edge_key);
            self.recently_slid.insert(edge_key, TIMEOUT);
            
            burst.burst_all(graph); 
        }
    }

    fn prune(&mut self, dt: f32) {
        self.recently_slid.retain(|_, time_left| {
            *time_left -= dt;
            *time_left > 0.0
        });
    }

    fn get_first_slidable(&self, graph: &Graph) -> Option<EdgeKey> {
        for (vkey, vertex) in graph.vertices.iter() {
            for (offset, edge) in vertex.edges.iter().enumerate() {
                let edge_key = vkey.edge_key(offset as u8);
                
                // Avoid checking duplicate edges (only check one direction)
                if !graph.vertices.contains_key(edge.twin.vertex) {
                    continue;
                }
                
                let twin_vertex = &graph.vertices[edge.twin.vertex];
                
                let start_pos = vertex.point.position;
                let end_pos = twin_vertex.point.position;
                
                let length = start_pos.distance(end_pos);
                
                if length < MIN_LENGTH 
                    && !self.recently_slid.contains_key(&edge_key) 
                    && !self.recently_slid.contains_key(&edge.twin) 
                {
                    return Some(edge_key);
                }
            }
        }
        None
    }
}
