use crate::game::state::{GamePhase, InteractContext, InteractionState, SoundEvent, UpdateContext};
use crate::graphics::{colors, RenderContext};
use macroquad::audio::{play_sound, play_sound_once, PlaySoundParams};
use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;

#[derive(Default)]
pub struct SoundManager;

impl SoundManager {
    pub fn new() -> Self {
        Self
    }

    pub fn interact(&self, ctx: &mut InteractContext) {
        if matches!(ctx.interaction.state, InteractionState::Released) {
            let (mx, my) = mouse_position();
            let x_min = 20.0;
            let y_min = 20.0;
            let x_max = 60.0;
            let y_max = 60.0;
            if mx >= x_min && mx <= x_max && my >= y_min && my <= y_max {
                ctx.state.toggle_mute();
            }
        }
    }

    pub fn update(&self, ctx: &mut UpdateContext) {
        if matches!(ctx.state.phase, GamePhase::Paused(_)) || ctx.state.muted {
            ctx.state.sound_events.clear();
            return;
        }
        if let Some(resources) = ctx.resources {
            for event in ctx.state.sound_events.drain(..) {
                match event {
                    SoundEvent::Pop => play_sound_once(&resources.pop_sound),
                    SoundEvent::Spawn => play_sound_once(&resources.spawn_sound),
                    SoundEvent::Burst => play_sound_once(&resources.burst_sound),
                    SoundEvent::Swap => {
                        play_sound(
                            resources
                                .splash_sounds
                                .choose()
                                .expect("has at least one splash noise"),
                            PlaySoundParams {
                                looped: false,
                                volume: 0.4,
                            },
                        );
                    }
                }
            }
        } else {
            ctx.state.sound_events.clear();
        }
    }

    pub fn draw(&self, ctx: &RenderContext) {
        // --- Screen Space (UI) ---
        set_default_camera();

        let x = 20.0;
        let y = 20.0;
        let w = 40.0;
        let h = 40.0;

        let (mx, my) = mouse_position();
        let is_hover = mx >= x && mx <= x + w && my >= y && my <= y + h;

        // Draw hover highlight circle
        if is_hover {
            draw_circle(
                x + w / 2.0,
                y + h / 2.0,
                22.0,
                Color::new(1.0, 1.0, 1.0, 0.15),
            );
        }

        let color = colors::WHITE;

        // Speaker body: rectangle on the left
        draw_rectangle(x + 5.0, y + 13.0, 8.0, 14.0, color);

        // Speaker cone: trapezoid on the right
        // We draw it as two triangles sharing a diagonal
        draw_triangle(
            vec2(x + 13.0, y + 13.0),
            vec2(x + 25.0, y + 7.0),
            vec2(x + 25.0, y + 33.0),
            color,
        );
        draw_triangle(
            vec2(x + 13.0, y + 13.0),
            vec2(x + 25.0, y + 33.0),
            vec2(x + 13.0, y + 27.0),
            color,
        );

        if ctx.state.muted {
            // Draw red cross
            let line_color = colors::RED;
            let thickness = 3.0;
            draw_line(x + 6.0, y + 6.0, x + 34.0, y + 34.0, thickness, line_color);
            draw_line(x + 34.0, y + 6.0, x + 6.0, y + 34.0, thickness, line_color);
        } else {
            // Draw sound wave lines: simple curves/lines
            let wave_color = colors::WHITE;
            let thickness = 2.0;

            // Inner wave
            draw_line(
                x + 29.0,
                y + 16.0,
                x + 32.0,
                y + 20.0,
                thickness,
                wave_color,
            );
            draw_line(
                x + 32.0,
                y + 20.0,
                x + 29.0,
                y + 24.0,
                thickness,
                wave_color,
            );

            // Outer wave
            draw_line(
                x + 33.0,
                y + 12.0,
                x + 37.0,
                y + 20.0,
                thickness,
                wave_color,
            );
            draw_line(
                x + 37.0,
                y + 20.0,
                x + 33.0,
                y + 28.0,
                thickness,
                wave_color,
            );
        }
    }
}
