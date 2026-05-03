use crate::game::state::{GamePhase, UpdateContext};
use crate::graph::edge::{EdgeKey, Slot};
use crate::graph::Graph;
use std::collections::HashMap;

const TIMEOUT: f32 = 0.5;
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

    pub fn update(&mut self, ctx: &mut UpdateContext) {
        self.prune(ctx.dt);
        if ctx.state.phase != GamePhase::Normal {
            return;
        }
        if let Some(edge_key) = self.get_first_slidable(&ctx.state.graph) {
            ctx.state.graph.slide(edge_key);
            self.recently_slid.insert(edge_key, TIMEOUT);
        }
    }

    fn prune(&mut self, dt: f32) {
        self.recently_slid.retain(|_, time_left| {
            *time_left -= dt;
            *time_left > 0.0
        });
    }

    fn get_first_slidable(&self, graph: &Graph) -> Option<EdgeKey> {
        graph.vertices.iter().find_map(|(vkey, vertex)| {
            Slot::all().into_iter().find_map(|slot| {
                let edge_key = vkey.slot(slot);
                let edge = vertex.edge(edge_key);

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
