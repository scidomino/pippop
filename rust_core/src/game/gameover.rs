use crate::game::state::{GamePhase, GameState, Interaction, InteractionState};
use crate::graphics::{colors, RenderContext};
use macroquad::prelude::*;

pub struct GameOverManager {
    pub timer: f32,
}

impl GameOverManager {
    pub fn new() -> Self {
        Self { timer: 0.0 }
    }

    pub fn update(&mut self, state: &mut GameState, dt: f32) {
        if state.phase != GamePhase::GameOver {
            // Check for game over condition: swappable bubble has 0 swaps
            // and we are in Normal phase (not in the middle of animations)
            if state.phase == GamePhase::Normal {
                if let Some(swappable_bkey) = state.graph.bubbles.get_swappable() {
                    if let crate::graph::bubble::BubbleStyle::Swappable { swaps_left, .. } =
                        state.graph.bubbles[swappable_bkey].style
                    {
                        if swaps_left <= 0 {
                            state.phase = GamePhase::GameOver;
                            self.timer = 0.0;
                        }
                    }
                }
            }
            return;
        }

        self.timer += dt;
    }

    pub fn interact(&mut self, state: &mut GameState, interaction: Interaction) -> bool {
        if state.phase != GamePhase::GameOver {
            return false;
        }
        // Return true if we should go back to title screen
        if self.timer > 2.0 && matches!(interaction.state, InteractionState::Released) {
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

        // Draw "Gioco Concluso" (Game Over)
        let text = "Gioco Concluso";
        let font_size = 100;
        let dims = measure_text(text, Some(ctx.font), font_size, 1.0);

        // Shadow
        draw_text_ex(
            text,
            screen_center_x - dims.width / 2.0 - 4.0,
            screen_center_y - 40.0 + 4.0,
            TextParams {
                font: Some(ctx.font),
                font_size,
                color: colors::WHITE,
                ..Default::default()
            },
        );
        // Foreground
        draw_text_ex(
            text,
            screen_center_x - dims.width / 2.0,
            screen_center_y - 40.0,
            TextParams {
                font: Some(ctx.font),
                font_size,
                color: colors::TURQUOISE,
                ..Default::default()
            },
        );

        if self.timer > 2.0 {
            let sub_text = "Tocca per ricominciare";
            let sub_size = 30;
            let sub_dims = measure_text(sub_text, Some(ctx.font), sub_size, 1.0);
            draw_text_ex(
                sub_text,
                screen_center_x - sub_dims.width / 2.0,
                screen_center_y + 40.0,
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
