use crate::graph::bubble::{BubbleKey, BubbleStyle};
use crate::graph::Graph;
use macroquad::math::{vec2, Vec2};

const POPPING_TIME: f32 = 0.5;

#[derive(Default)]
pub struct PopManager {
    /// Bubbles currently in the timed "frozen" popping state.
    pub pending_pop: Option<BubbleKey>,
}

impl PopManager {
    pub fn new() -> Self {
        Self { pending_pop: None }
    }

    pub fn is_handling(&self, bkey: BubbleKey) -> bool {
        self.pending_pop == Some(bkey)
    }

    pub fn draw(&self, ctx: &crate::graphics::RenderContext) {
        if let Some(bkey) = self.pending_pop {
            let bubble = &ctx.graph.bubbles[bkey];
            let points = crate::graphics::bubble::get_bubble_points(ctx.graph, bkey);
            if points.is_empty() {
                return;
            }

            if let BubbleStyle::Popping { size, timer, .. } = bubble.style {
                let progress = (timer / 0.5).clamp(0.0, 1.0);
                let morphed_points = self.apply_pop_morph(&points, bubble.centroid, size, progress);
                crate::graphics::bubble::draw_bubble(
                    &bubble.style,
                    &morphed_points,
                    bubble.centroid,
                    ctx.font,
                );
            }
        }
    }

    fn apply_pop_morph(
        &self,
        points: &[Vec2],
        centroid: Vec2,
        size: i32,
        progress: f32,
    ) -> Vec<Vec2> {
        let target_area = 3000.0 * (size as f32).sqrt();
        let radius = 5.0 * (target_area / std::f32::consts::PI).sqrt();

        let first_p = points[0];
        let start_angle = (first_p.y - centroid.y).atan2(first_p.x - centroid.x);

        let n = points.len();
        let morph_ratio = progress.powi(2);
        let inv_morph = 1.0 - morph_ratio;

        points
            .iter()
            .enumerate()
            .map(|(i, &p)| {
                let angle = start_angle - (2.0 * std::f32::consts::PI) * (i as f32 / n as f32);
                let circle_p = centroid + vec2(angle.cos() * radius, angle.sin() * radius);
                p * morph_ratio + circle_p * inv_morph
            })
            .collect()
    }

    /// Checks if any bubble is ready to pop and transitions it to the Popping state.
    pub fn start_pop_if_ready(&mut self, graph: &mut Graph) -> bool {
        if self.pending_pop.is_some() {
            return false;
        }

        if let Some(bkey) =
            graph.bubbles.iter().find_map(
                |(k, b)| {
                    if b.style.is_poppable() {
                        Some(k)
                    } else {
                        None
                    }
                },
            )
        {
            let style = graph.bubbles[bkey].style;
            if let BubbleStyle::Standard { size, color } = style {
                graph.bubbles[bkey].style = BubbleStyle::Popping {
                    size,
                    color,
                    timer: POPPING_TIME,
                };
                self.pending_pop = Some(bkey);
                return true;
            }
        }
        false
    }

    /// Updates timers for popping bubbles and handles transitions.
    pub fn update(&mut self, graph: &mut Graph, dt: f32) -> bool {
        if let Some(bkey) = self.pending_pop {
            if let Some(bubble) = graph.bubbles.get_mut(bkey) {
                if let BubbleStyle::Popping { timer, .. } = &mut bubble.style {
                    *timer -= dt;
                    if *timer <= 0.0 {
                        // "Pop" happened. Style target area is now 0.
                        self.pending_pop = None;
                        return true;
                    }
                }
            } else {
                self.pending_pop = None;
            }
        }
        false
    }
}
