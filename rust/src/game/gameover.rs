use crate::game::state::{GamePhase, InteractContext, InteractionState, UpdateContext};
use crate::graphics::{colors, RenderContext};
use macroquad::prelude::*;

#[derive(Default)]
pub struct GameOverManager {
    pub timer: f32,
}

impl GameOverManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, ctx: &mut UpdateContext) {
        if ctx.state.phase != GamePhase::GameOver {
            // Check for game over condition: swappable bubble has 0 swaps
            // and we are in Normal phase (not in the middle of animations)
            if ctx.state.phase == GamePhase::Normal {
                if let Some(swappable_bkey) = ctx.state.graph.bubbles.get_swappable() {
                    if let crate::graph::bubble::BubbleStyle::Swappable { swaps_left, .. } =
                        ctx.state.graph.bubbles[swappable_bkey].style
                    {
                        if swaps_left <= 0 {
                            ctx.state.phase = GamePhase::GameOver;
                            self.timer = 0.0;
                        }
                    }
                }
            }
            return;
        }

        self.timer += ctx.dt;
    }

    pub fn interact(&mut self, ctx: &mut InteractContext) -> bool {
        if ctx.state.phase != GamePhase::GameOver {
            return false;
        }
        // Return true if we should go back to title screen
        if self.timer > 2.0 && matches!(ctx.interaction.state, InteractionState::Released) {
            return true;
        }
        false
    }

    pub fn draw(&self, ctx: &RenderContext) {
        if !matches!(ctx.phase, GamePhase::GameOver) {
            return;
        }

        // --- Screen Space (UI) ---
        set_default_camera();

        let screen_width = screen_width();
        let screen_height = screen_height();

        // Dim the background
        let alpha = (self.timer * 2.0).min(0.8);
        draw_rectangle(
            0.0,
            0.0,
            screen_width,
            screen_height,
            Color::new(0.0, 0.0, 0.0, alpha),
        );

        if self.timer < 0.5 {
            return;
        }

        let screen_center_x = screen_width / 2.0;
        let screen_center_y = screen_height / 2.0;

        // Draw "Game Over"
        let line1 = "Game";
        let line2 = "Over";
        let font_size: u16 = 64;
        let font_scale = 2.0;
        let dims1 = measure_text(line1, Some(ctx.font), font_size, font_scale);
        let dims2 = measure_text(line2, Some(ctx.font), font_size, font_scale);

        let x1 = (screen_center_x - dims1.width / 2.0).floor();
        let y1 = (screen_center_y - 50.0).floor();
        let x2 = (screen_center_x - dims2.width / 2.0).floor();
        let y2 = (screen_center_y + 70.0).floor();

        for (text, x, y) in [(line1, x1, y1), (line2, x2, y2)] {
            // Shadow
            draw_text_ex(
                text,
                x - 4.0,
                y + 4.0,
                TextParams {
                    font: Some(ctx.font),
                    font_size,
                    font_scale,
                    color: colors::WHITE,
                    ..Default::default()
                },
            );
            // Foreground
            draw_text_ex(
                text,
                x,
                y,
                TextParams {
                    font: Some(ctx.font),
                    font_size,
                    font_scale,
                    color: colors::TURQUOISE,
                    ..Default::default()
                },
            );
        }

        if self.timer > 2.0 {
            let sub_text = "Tocca per ricominciare";
            let sub_size: u16 = 32;
            let sub_dims = measure_text(sub_text, Some(ctx.font), sub_size, 1.0);
            draw_text_ex(
                sub_text,
                (screen_center_x - sub_dims.width / 2.0).floor(),
                (screen_center_y + 160.0).floor(),
                TextParams {
                    font: Some(ctx.font),
                    font_size: sub_size,
                    color: colors::WHITE,
                    ..Default::default()
                },
            );
        }
    }
}
