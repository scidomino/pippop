pub mod bubble;
pub mod effects;
pub mod styles;

use macroquad::prelude::*;
use crate::graph::Graph;
use self::effects::EffectsManager;

pub struct Renderer {
    pub effects: EffectsManager,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            effects: EffectsManager::new(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.effects.update(dt);
    }

    pub fn draw(&self, graph: &Graph, camera: &Camera2D) {
        // --- Pass 1: World Space (Bubbles) ---
        set_camera(camera);
        
        for (bkey, bubble) in graph.bubbles.iter() {
            if bubble.open_air {
                continue;
            }

            let points = bubble::get_bubble_points(graph, bkey, 10);
            if points.is_empty() {
                continue;
            }

            let style = styles::get_bubble_style(bubble.size, bubble.open_air);
            let centroid = bubble::calculate_centroid(&points);

            // 1. Draw Fill (Triangle Fan)
            for i in 0..points.len() {
                let p1 = points[i];
                let p2 = points[(i + 1) % points.len()];
                draw_triangle(centroid, p1, p2, style.fill_color);
            }

            // 2. Draw Outline
            for i in 0..points.len() {
                let p1 = points[i];
                let p2 = points[(i + 1) % points.len()];
                draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, style.outline_color);
            }
        }

        // --- Pass 2: Screen Space (UI & Effects) ---
        set_default_camera();

        for (bkey, bubble) in graph.bubbles.iter() {
            if bubble.open_air {
                continue;
            }

            let points = bubble::get_bubble_points(graph, bkey, 10);
            if points.is_empty() {
                continue;
            }
            
            let centroid = bubble::calculate_centroid(&points);
            let screen_pos = camera.world_to_screen(centroid);
            let style = styles::get_bubble_style(bubble.size, bubble.open_air);

            let label = format!("{}", bubble.size as i32);
            let font_size = 20.0;
            let text_dims = measure_text(&label, None, font_size as u16, 1.0);
            
            draw_text(
                &label,
                screen_pos.x - text_dims.width / 2.0,
                screen_pos.y + text_dims.height / 2.0,
                font_size,
                style.label_color,
            );
        }

        self.effects.draw(camera);
    }
}
