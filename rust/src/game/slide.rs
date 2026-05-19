use crate::game::state::{GamePhase, UpdateContext};
use crate::graph::edge::EdgeKey;
use crate::graph::Graph;
use std::collections::HashMap;

const TIMEOUT: f32 = 0.5;
const MIN_LENGTH: f32 = 10.0;

#[derive(Default)]
pub struct SlideManager {
    recently_slid: HashMap<EdgeKey, f32>,
}

impl SlideManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, ctx: &mut UpdateContext) {
        self.prune(ctx.dt);
        if !matches!(ctx.state.phase, GamePhase::Normal) {
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
            vkey.edge_keys().into_iter().find(|&ekey| {
                let edge = vertex.edge(ekey);
                graph.vertices.get(edge.twin.vertex).is_some_and(|twin| {
                    vertex.point.position.distance(twin.point.position) < MIN_LENGTH
                        && !self.recently_slid.contains_key(&ekey)
                        && !self.recently_slid.contains_key(&edge.twin)
                })
            })
        })
    }
}
