use crate::game::state::{GamePhase, InteractContext, InteractionState, UpdateContext};
use crate::graph::bubble::{BubbleKey, BubbleStyle};
use crate::graph::Graph;
use crate::graphics::{bubble, colors, geometry, RenderContext};
use macroquad::prelude::*;
use std::f32::consts::PI;

const TEASER_DELAY: f32 = 4.0;
const TEASER_THROB: f32 = 1.0;

#[derive(Default)]
pub struct HighlightManager {
    pub manual_highlight: Option<BubbleKey>,
    pub time: f32,
}

impl HighlightManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn draw(&self, ctx: &RenderContext) {
        if !matches!(&ctx.state.phase, GamePhase::Normal) {
            return;
        }
        set_camera(ctx.camera);

        let glow_requests = self.get_glow_requests(&ctx.state.graph);
        for (bkey, intensity) in glow_requests {
            let points = bubble::get_bubble_points(&ctx.state.graph, bkey);
            if !points.is_empty() {
                let centroid = ctx.state.graph.bubbles[bkey].centroid;
                let width = 20.0 * intensity;
                let glow_mesh =
                    geometry::generate_bubble_glow_mesh(&points, centroid, width, colors::WHITE);
                draw_mesh(&glow_mesh);
            }
        }
    }

    pub fn update(&mut self, ctx: &mut UpdateContext) {
        self.time += ctx.dt;
    }

    pub fn interact(&mut self, ctx: &mut InteractContext) {
        if ctx.state.phase != GamePhase::Normal {
            self.manual_highlight = None;
            return;
        }

        match ctx.interaction.state {
            InteractionState::Pressed => {
                if let Some(ekey) = ctx
                    .state
                    .graph
                    .get_closest_swap_candidate(ctx.interaction.position)
                {
                    let bkey = ctx.state.graph.vertices.get_edge(ekey).bubble;
                    self.manual_highlight = Some(bkey);
                    self.time = 0.0;
                } else {
                    self.manual_highlight = None;
                }
            }
            InteractionState::Hover => {
                if let Some(bkey) = ctx
                    .state
                    .graph
                    .get_swap_candidate_at_point(ctx.interaction.position)
                {
                    self.manual_highlight = Some(bkey);
                    self.time = 0.0;
                } else {
                    self.manual_highlight = None;
                }
            }
            _ => {
                self.manual_highlight = None;
            }
        }
    }

    pub fn get_glow_requests(&self, graph: &Graph) -> Vec<(BubbleKey, f32)> {
        let mut requests = Vec::new();

        if let Some(bkey) = self.manual_highlight {
            if graph.bubbles.contains_key(bkey) {
                requests.push((bkey, 1.0));
            }
        } else {
            let cycle_time = self.time % TEASER_DELAY;
            if cycle_time > TEASER_DELAY / 2.0 {
                let ratio = (self.time * PI * 2.0 / TEASER_THROB).sin().powi(2);

                if let Some(swappable_bkey) = graph.bubbles.get_swappable() {
                    requests.extend(
                        graph.bubbles[swappable_bkey]
                            .edges
                            .iter()
                            .map(|&ekey| {
                                graph
                                    .vertices
                                    .get_edge(graph.vertices.get_edge(ekey).twin)
                                    .bubble
                            })
                            .filter(|&twin_bkey| {
                                !matches!(graph.bubbles[twin_bkey].style, BubbleStyle::OpenAir)
                            })
                            .map(|twin_bkey| (twin_bkey, ratio)),
                    );
                }
            }
        }

        requests
    }
}
