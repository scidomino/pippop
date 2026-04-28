use crate::graph::bubble::{BubbleKey, BubbleStyle};
use crate::graph::edge::EdgeKey;
use crate::graph::Graph;
use crate::graphics::bubble;
use crate::graphics::colors;
use crate::graphics::geometry;
use macroquad::prelude::*;

const FREEZE_DURATION: f32 = 0.5;

pub struct BurstManager {
    /// The edge currently being "frozen" for the burst animation.
    pub active_edge: Option<EdgeKey>,
    pub timer: f32,
    pub focus_bubble: Option<BubbleKey>,
}

impl BurstManager {
    pub fn new(_threshold: usize) -> Self {
        Self {
            active_edge: None,
            timer: 0.0,
            focus_bubble: None,
        }
    }

    pub fn set_focus_bubble(&mut self, bkey: BubbleKey) {
        self.focus_bubble = Some(bkey);
    }

    pub fn draw(&self, ctx: &crate::graphics::RenderContext) {
        if let Some(ekey) = self.active_edge {
            let mut points = Vec::with_capacity(12);
            bubble::push_edge_points(ctx.graph, ekey, &mut points);
            let twin_key = ctx.graph.vertices.get_edge(ekey).twin;
            points.push(ctx.graph.vertices[twin_key.vertex].point.position);

            let progress = 1.0 - (self.timer / FREEZE_DURATION).clamp(0.0, 1.0);
            let width = 40.0 * progress;
            let glow_mesh = geometry::generate_glow_mesh(&points, width, colors::WHITE, false);
            draw_mesh(&glow_mesh);
        }
    }

    pub fn update(&mut self, dt: f32) -> bool {
        if self.active_edge.is_some() {
            self.timer -= dt;
            if self.timer <= 0.0 {
                return true; // Signal that the active edge is ready to be burst
            }
        }
        false
    }

    /// Performs the topological merge of two bubbles across a shared edge.
    /// Ensures that the focus bubble survives the merge.
    pub fn burst(&mut self, graph: &mut Graph, mut ekey: EdgeKey) {
        if !self.is_burstable(graph, ekey) {
            return;
        }

        // In graph.remove_edge(ekey), ekey.bubble survives (b_top).
        // If our focus bubble is the twin, we must swap to ensure it survives.
        if let Some(focus) = self.focus_bubble {
            let twin_key = graph.vertices.get_edge(ekey).twin;
            if graph.vertices.get_edge(twin_key).bubble == focus {
                ekey = twin_key;
            }
        }

        graph.remove_edge(ekey);

        // Every wall burst grants the player an extra swap
        if let Some(player_bkey) = graph.bubbles.get_player_bubble() {
            if let BubbleStyle::Player { swaps_left } = &mut graph.bubbles[player_bkey].style {
                *swaps_left += 1;
            }
        }
    }

    /// Sets up the manager to process a burst with a freeze delay using the focus bubble.
    pub fn find_and_set_next_burstable(&mut self, graph: &Graph) -> bool {
        if let Some(bkey) = self.focus_bubble {
            if let Some(ekey) = self.find_burstable_edge_in_bubble(graph, bkey) {
                self.active_edge = Some(ekey);
                self.timer = FREEZE_DURATION;
                return true;
            }
        }
        false
    }

    fn find_burstable_edge_in_bubble(&self, graph: &Graph, bkey: BubbleKey) -> Option<EdgeKey> {
        let bubble = graph.bubbles.get(bkey)?;
        for &ekey in &bubble.edges {
            if self.is_burstable(graph, ekey) {
                return Some(ekey);
            }
        }
        None
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
            (BubbleStyle::Standard { color: c1, .. }, BubbleStyle::Standard { color: c2, .. }) => {
                c1 == c2
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::bubble::BubbleStyle;
    use crate::graphics::colors;

    #[test]
    fn test_burst_increments_player_swaps() {
        let mut graph = Graph::new(
            BubbleStyle::Player { swaps_left: 5 },
            BubbleStyle::Standard {
                size: 1,
                color: colors::TURQUOISE,
            },
        );

        let vkey = graph.vertices.keys().next().unwrap();
        graph.spawn(
            vkey,
            BubbleStyle::Standard {
                size: 1,
                color: colors::TURQUOISE,
            },
        );

        let mut burst_manager = BurstManager::new(1);
        // Find the standard bubble that isn't the player
        let bkey = graph
            .bubbles
            .iter()
            .find(|(_, b)| matches!(b.style, BubbleStyle::Standard { .. }))
            .map(|(k, _)| k)
            .expect("Should have a standard bubble");

        burst_manager.set_focus_bubble(bkey);
        assert!(
            burst_manager.find_and_set_next_burstable(&graph),
            "Should find a burstable edge in focus bubble"
        );

        let ekey = burst_manager.active_edge.unwrap();
        burst_manager.burst(&mut graph, ekey);

        let player_bkey = graph.bubbles.get_player_bubble().unwrap();
        if let BubbleStyle::Player { swaps_left } = graph.bubbles[player_bkey].style {
            assert_eq!(swaps_left, 6);
        } else {
            panic!("Expected Player style");
        }
    }
}
