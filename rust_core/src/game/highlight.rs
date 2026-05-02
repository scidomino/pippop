use crate::game::state::{GameState, Interaction, InteractionState};
use crate::graph::bubble::BubbleStyle;
use crate::graph::Graph;
use crate::graphics::colors;
use crate::graphics::geometry;
use macroquad::math::Vec2;
use macroquad::prelude::*;

const TEASER_DELAY: f32 = 4.0;
const TEASER_THROB: f32 = 1.0;

pub struct HighlightManager {
    pub point: Option<Vec2>,
    pub time: f32,
}

impl Default for HighlightManager {
    fn default() -> Self {
        Self::new()
    }
}

impl HighlightManager {
    pub fn new() -> Self {
        Self {
            point: None,
            time: 0.0,
        }
    }

    pub fn draw(&self, ctx: &crate::graphics::RenderContext) {
        let glow_requests = self.get_glow_requests(ctx.graph);
        for (bkey, intensity) in glow_requests {
            let points = crate::graphics::bubble::get_bubble_points(ctx.graph, bkey);
            if !points.is_empty() {
                let width = 20.0 * intensity;
                let glow_mesh = geometry::generate_glow_mesh(&points, width, colors::WHITE, true);
                draw_mesh(&glow_mesh);
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt;
    }

    pub fn interact(&mut self, state: &mut GameState, interaction: Interaction) {
        if state.phase != crate::game::state::GamePhase::Normal {
            self.point = None;
            return;
        }

        if matches!(interaction.state, InteractionState::Pressed) {
            self.point = Some(interaction.position);
            self.time = 0.0;
        } else {
            self.point = None;
        }
    }

    pub fn get_glow_requests(&self, graph: &Graph) -> Vec<(crate::graph::bubble::BubbleKey, f32)> {
        let mut requests = Vec::new();

        if let Some(p) = self.point {
            if let Some(ekey) = graph.get_closest_swap_candidate(p) {
                let bkey = graph.vertices.get_edge(ekey).bubble;
                requests.push((bkey, 1.0));
            }
        } else {
            let cycle_time = self.time % TEASER_DELAY;
            if cycle_time > TEASER_DELAY / 2.0 {
                let ratio = (self.time * std::f32::consts::PI * 2.0 / TEASER_THROB)
                    .sin()
                    .powi(2);

                if let Some(swappable_bkey) = graph.bubbles.get_swappable() {
                    let swappable_bubble = &graph.bubbles[swappable_bkey];
                    for &ekey in &swappable_bubble.edges {
                        let twin_bkey = graph
                            .vertices
                            .get_edge(graph.vertices.get_edge(ekey).twin)
                            .bubble;
                        if !matches!(graph.bubbles[twin_bkey].style, BubbleStyle::OpenAir) {
                            requests.push((twin_bkey, ratio));
                        }
                    }
                }
            }
        }

        requests
    }
}
