use macroquad::prelude::*;
use crate::graph::point::Coordinate;

pub enum Effect {
    RisingPoints {
        text: String,
        pos: Coordinate,
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
        Self { effects: Vec::new() }
    }

    pub fn add_rising_points(&mut self, text: String, pos: Coordinate) {
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

    pub fn draw(&self, camera: &Camera2D) {
        for effect in &self.effects {
            match effect {
                Effect::RisingPoints { text, pos, timer, max_time } => {
                    let progress = 1.0 - (timer / max_time);
                    
                    // Project world position to screen pixels
                    let screen_pos = camera.world_to_screen(vec2(pos.x, pos.y));
                    let y_offset = progress * 100.0; // Rise by 100 pixels
                    let alpha = *timer / max_time;
                    let color = Color::new(1.0, 1.0, 1.0, alpha);
                    
                    let font_size = 30.0;
                    let text_dims = measure_text(text, None, font_size as u16, 1.0);
                    
                    draw_text(
                        text,
                        screen_pos.x - text_dims.width / 2.0,
                        screen_pos.y - y_offset,
                        font_size,
                        color,
                    );
                }
            }
        }
    }
}
