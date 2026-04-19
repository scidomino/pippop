use macroquad::math::Vec2;
use macroquad::prelude::*;

pub enum Effect {
    RisingPoints {
        text: String,
        pos: Vec2,
        timer: f32, // Remaining time in seconds
        max_time: f32,
    },
}

pub struct EffectsManager {
    effects: Vec<Effect>,
}

impl Default for EffectsManager {
    fn default() -> Self {
        Self::new()
    }
}

impl EffectsManager {
    pub fn new() -> Self {
        Self {
            effects: Vec::new(),
        }
    }

    pub fn add_rising_points(&mut self, text: String, pos: Vec2) {
        self.effects.push(Effect::RisingPoints {
            text,
            pos,
            timer: 1.0,
            max_time: 1.0,
        });
    }

    pub fn update(&mut self, dt: f32) {
        for effect in &mut self.effects {
            match effect {
                Effect::RisingPoints { timer, .. } => {
                    *timer -= dt;
                }
            }
        }
        self.effects.retain(|e| match e {
            Effect::RisingPoints { timer, .. } => *timer > 0.0,
        });
    }

    pub fn draw(&self, camera: &Camera2D, font: &Font) {
        for effect in &self.effects {
            match effect {
                Effect::RisingPoints {
                    text,
                    pos,
                    timer,
                    max_time,
                } => {
                    let progress = 1.0 - (timer / max_time);

                    // Project world position to screen pixels
                    let screen_pos = camera.world_to_screen(*pos);
                    let scale = screen_width() / 1200.0;
                    let y_offset = progress * 100.0 * scale; // Rise by 100 world units
                    let alpha = *timer / max_time;
                    let color = Color::new(1.0, 1.0, 1.0, alpha);

                    let target_pixel_size = 30.0 * scale;
                    let font_size = 64; // High-res rasterization
                    let font_scale = target_pixel_size / font_size as f32;

                    let text_params = TextParams {
                        font: Some(font),
                        font_size,
                        font_scale,
                        color,
                        ..Default::default()
                    };

                    let text_dims = measure_text(text, Some(font), font_size, font_scale);

                    draw_text_ex(
                        text,
                        screen_pos.x - text_dims.width / 2.0,
                        screen_pos.y - y_offset,
                        text_params,
                    );
                }
            }
        }
    }
}
