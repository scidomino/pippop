use crate::game::state::{GameEvent, GamePhase, GameState};
use crate::graph::bubble::{BubbleKey, BubbleStyle};
use crate::graphics::{bubble, RenderContext};
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::Color;
use std::f32::consts::PI;

const POPPING_TIME: f32 = 0.5;

pub struct PendingPop {
    pub bkey: BubbleKey,
    pub style: BubbleStyle,
    pub timer: f32,
}

#[derive(Default)]
pub struct PopManager {
    /// Bubble currently in the timed "frozen" popping state.
    pub pending_pop: Option<PendingPop>,
}

impl PopManager {
    pub fn new() -> Self {
        Self { pending_pop: None }
    }

    pub fn is_handling(&self, bkey: BubbleKey) -> bool {
        matches!(&self.pending_pop, Some(p) if p.bkey == bkey)
    }

    pub fn draw(&self, ctx: &RenderContext) {
        if let Some(pending) = &self.pending_pop {
            let bubble = &ctx.graph.bubbles[pending.bkey];
            let points = bubble::get_points_for_bubble(ctx.graph, bubble);
            if points.is_empty() {
                return;
            }

            if let BubbleStyle::Colored { size, color } = pending.style {
                let progress = (pending.timer / POPPING_TIME).clamp(0.0, 1.0);
                let morphed_points = self.apply_pop_morph(&points, bubble.centroid, size, progress);

                // Create a temporary Colored style with the faded color to use for rendering
                bubble::draw_bubble(
                    &BubbleStyle::Colored {
                        size,
                        color: Color::new(color.r, color.g, color.b, progress),
                    },
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
        let radius = 5.0 * (target_area / PI).sqrt();

        let first_p = points[0];
        let start_angle = (first_p.y - centroid.y).atan2(first_p.x - centroid.x);

        let n = points.len();
        let morph_ratio = progress.powi(2);
        let inv_morph = 1.0 - morph_ratio;

        points
            .iter()
            .enumerate()
            .map(|(i, &p)| {
                let angle = start_angle - (2.0 * PI) * (i as f32 / n as f32);
                let circle_p = centroid + vec2(angle.cos() * radius, angle.sin() * radius);
                p * morph_ratio + circle_p * inv_morph
            })
            .collect()
    }

    /// Updates the popping state. If in Normal phase, checks if a bubble is ready to pop.
    /// If in Popping phase, advances the animation timer.
    pub fn update(&mut self, state: &mut GameState, dt: f32) {
        match state.phase {
            GamePhase::Normal => {
                if self.pending_pop.is_some() {
                    return;
                }

                if let Some(bkey) = state
                    .graph
                    .bubbles
                    .iter()
                    .find(|(_, b)| b.style.is_poppable())
                    .map(|(k, _)| k)
                {
                    let style = state.graph.bubbles[bkey].style;
                    if let BubbleStyle::Colored { size, .. } = style {
                        state.graph.bubbles[bkey].style = BubbleStyle::Invisible { size };
                        self.pending_pop = Some(PendingPop {
                            bkey,
                            style,
                            timer: POPPING_TIME,
                        });
                        state.phase = GamePhase::Popping;
                    }
                }
            }
            GamePhase::Popping => {
                if let Some(pending) = &mut self.pending_pop {
                    if let Some(bubble) = state.graph.bubbles.get_mut(pending.bkey) {
                        pending.timer -= dt;
                        if pending.timer <= 0.0 {
                            // "Pop" happened. Style target area is now 0.
                            bubble.style = BubbleStyle::Invisible { size: 0 };
                            self.pending_pop = None;
                            state.events.push(GameEvent::Pop);
                            state.phase = GamePhase::Normal;
                        }
                    } else {
                        self.pending_pop = None;
                        state.phase = GamePhase::Normal;
                    }
                } else {
                    state.phase = GamePhase::Normal;
                }
            }
            _ => {}
        }
    }
}
