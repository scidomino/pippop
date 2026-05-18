use crate::game::state::{GamePhase, ScoreEvent, SoundEvent, UpdateContext};
use crate::graph::bubble::{BubbleKey, BubbleStyle};
use crate::graph::edge::EdgeKey;
use crate::graph::Graph;
use crate::graphics::{bubble, colors, geometry, RenderContext};
use macroquad::prelude::*;

const FREEZE_DURATION: f32 = 0.5;

#[derive(Default)]
pub struct BurstManager {
    /// The edge currently being "frozen" for the burst animation.
    /// When present this edge will belong to graph.focus_bubble.
    pub active_edge: Option<EdgeKey>,
    pub timer: f32,
}

impl BurstManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn draw(&self, ctx: &RenderContext) {
        set_camera(ctx.camera);

        if let Some(ekey) = self.active_edge {
            let mut points = Vec::with_capacity(12);
            bubble::push_edge_points(&ctx.state.graph, ekey, &mut points);
            let twin_key = &ctx.state.graph.vertices.get_edge(ekey).twin;
            points.push(ctx.state.graph.vertices[twin_key.vertex].point.position);

            let progress = 1.0 - (self.timer / FREEZE_DURATION).clamp(0.0, 1.0);
            let width = 40.0 * progress;
            let glow_mesh = geometry::generate_edge_glow_mesh(&points, width, colors::WHITE);
            draw_mesh(&glow_mesh);
        }
    }

    pub fn update(&mut self, ctx: &mut UpdateContext) {
        if !matches!(ctx.state.phase, GamePhase::Bursting) {
            return;
        }

        if self.active_edge.is_none() {
            if self.find_and_set_next_burstable(&ctx.state.graph, ctx.state.focus_bubble) {
                // Initial burst setup successful
            } else {
                ctx.state.phase = GamePhase::Normal;
                ctx.state.focus_bubble = None;
                return;
            }
        }

        if let Some(ekey) = self.active_edge {
            self.timer -= ctx.dt;
            if self.timer <= 0.0 {
                let position = ctx.state.graph.vertices.get_edge(ekey).points[0];
                self.burst(&mut ctx.state.graph, ekey);
                ctx.state.sound_events.push(SoundEvent::Burst);
                ctx.state.score_events.push(ScoreEvent::Burst { position });

                if self.find_and_set_next_burstable(&ctx.state.graph, ctx.state.focus_bubble) {
                    // Continue bursting
                } else {
                    self.active_edge = None;
                    ctx.state.focus_bubble = None;
                    ctx.state.phase = GamePhase::Normal;
                }
            }
        }
    }

    /// Performs the topological merge of two bubbles across a shared edge.
    /// The bubble associated with `ekey` survives the merge.
    pub fn burst(&mut self, graph: &mut Graph, ekey: EdgeKey) {
        if !self.is_burstable(graph, ekey) {
            return;
        }

        graph.remove_edge(ekey);

        // Every wall burst grants the player an extra swap
        if let Some(swappable_bkey) = graph.bubbles.get_swappable() {
            if let BubbleStyle::Swappable { swaps_left, .. } =
                &mut graph.bubbles[swappable_bkey].style
            {
                if *swaps_left < 5 {
                    *swaps_left += 1;
                }
            }
        }
    }

    /// Sets up the manager to process a burst with a freeze delay using the focus bubble.
    pub fn find_and_set_next_burstable(
        &mut self,
        graph: &Graph,
        focus_bubble: Option<BubbleKey>,
    ) -> bool {
        if let Some(bkey) = focus_bubble {
            if let Some(ekey) = self.find_burstable_edge_in_bubble(graph, bkey) {
                self.active_edge = Some(ekey);
                self.timer = FREEZE_DURATION;
                return true;
            }
        }
        false
    }

    fn find_burstable_edge_in_bubble(&self, graph: &Graph, bkey: BubbleKey) -> Option<EdgeKey> {
        graph
            .bubbles
            .get(bkey)?
            .edges
            .iter()
            .find(|&&ekey| self.is_burstable(graph, ekey))
            .copied()
    }

    pub fn is_burstable(&self, graph: &Graph, ekey: EdgeKey) -> bool {
        let edge = graph.vertices.get_edge(ekey);
        let twin = graph.vertices.get_edge(edge.twin);

        if edge.bubble == twin.bubble {
            return false;
        }

        let s1 = &graph.bubbles[edge.bubble].style;
        let s2 = &graph.bubbles[twin.bubble].style;

        match (s1, s2) {
            (BubbleStyle::Colored { color: c1, .. }, BubbleStyle::Colored { color: c2, .. }) => {
                c1 == c2
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::state::GameState;
    use crate::graphics::colors;

    #[test]
    fn test_burst_increments_swaps() {
        let graph = Graph::new(
            BubbleStyle::swappable(5),
            BubbleStyle::colored(colors::TURQUOISE),
        );
        let mut state = GameState::new(graph);

        let vkey = state.graph.vertices.keys().next().unwrap();
        state
            .graph
            .spawn(vkey, BubbleStyle::colored(colors::TURQUOISE));

        let mut burst_manager = BurstManager::new();
        // Find the colored bubble that isn't the swappable
        let bkey = state
            .graph
            .bubbles
            .iter()
            .find(|(_, b)| matches!(b.style, BubbleStyle::Colored { .. }))
            .map(|(k, _)| k)
            .expect("Should have a colored bubble");

        state.focus_bubble = Some(bkey);
        assert!(
            burst_manager.find_and_set_next_burstable(&state.graph, state.focus_bubble),
            "Should find a burstable edge in focus bubble"
        );

        let ekey = burst_manager.active_edge.unwrap();
        burst_manager.burst(&mut state.graph, ekey);

        let swappable_bkey = state.graph.bubbles.get_swappable().unwrap();
        if let BubbleStyle::Swappable { swaps_left, .. } = state.graph.bubbles[swappable_bkey].style
        {
            assert_eq!(swaps_left, 5);
        } else {
            panic!("Expected Swappable style");
        }
    }
}
