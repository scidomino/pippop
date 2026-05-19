use crate::game::state::{GamePhase, InteractContext};
use crate::graphics::{colors, RenderContext};
use macroquad::prelude::*;

#[derive(Default)]
pub struct PauseManager;

impl PauseManager {
    pub fn new() -> Self {
        Self
    }

    pub fn interact(&self, ctx: &mut InteractContext) {
        if ctx.interaction.keys_pressed.contains(&KeyCode::Space)
            || ctx.interaction.keys_pressed.contains(&KeyCode::P)
        {
            self.toggle_pause(ctx);
        }
    }

    fn toggle_pause(&self, ctx: &mut InteractContext) {
        ctx.state.phase = match &ctx.state.phase {
            GamePhase::Paused(inner) => *inner.clone(),
            GamePhase::GameOver => GamePhase::GameOver,
            old => GamePhase::Paused(Box::new(old.clone())),
        };
    }

    pub fn draw(&self, ctx: &RenderContext) {
        if !matches!(&ctx.state.phase, GamePhase::Paused(_)) {
            return;
        }

        // --- Screen Space (UI) ---
        set_default_camera();

        let screen_width = screen_width();
        let screen_height = screen_height();

        // Dim the background
        draw_rectangle(
            0.0,
            0.0,
            screen_width,
            screen_height,
            Color::new(0.0, 0.0, 0.0, 0.5),
        );

        let screen_center_x = screen_width / 2.0;
        let screen_center_y = screen_height / 2.0;

        let text = "PAUSA";
        let font_size: u16 = 64;
        let font_scale = 1.5;
        let dims = measure_text(text, Some(&ctx.resources.font), font_size, font_scale);

        let x = (screen_center_x - dims.width / 2.0).floor();
        let y = screen_center_y.floor();

        // Shadow
        draw_text_ex(
            text,
            x - 4.0,
            y + 4.0,
            TextParams {
                font: Some(&ctx.resources.font),
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
                font: Some(&ctx.resources.font),
                font_size,
                font_scale,
                color: colors::TURQUOISE,
                ..Default::default()
            },
        );

        let sub_text = "Premi SPAZIO per riprendere";
        let sub_size: u16 = 32;
        let sub_dims = measure_text(sub_text, Some(&ctx.resources.font), sub_size, 1.0);
        draw_text_ex(
            sub_text,
            (screen_center_x - sub_dims.width / 2.0).floor(),
            (screen_center_y + 80.0).floor(),
            TextParams {
                font: Some(&ctx.resources.font),
                font_size: sub_size,
                color: colors::WHITE,
                ..Default::default()
            },
        );
    }
}
