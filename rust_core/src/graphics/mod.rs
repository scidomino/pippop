pub mod bubble;
pub mod colors;
pub mod effects;

use self::effects::EffectsManager;
use crate::graph::Graph;
use crate::style::BubbleStyle;
use macroquad::prelude::*;

pub struct Renderer {
    pub font: Font,
    pub effects: EffectsManager,
}

impl Renderer {
    pub fn new() -> Self {
        let font_bytes =
            include_bytes!("../../../android/app/src/main/res/font/sniglet_extrabold.ttf");
        let font = load_ttf_font_from_bytes(font_bytes).unwrap();
        Self {
            font,
            effects: EffectsManager::new(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.effects.update(dt);
    }

    pub fn draw(&self, graph: &Graph, camera: &Camera2D) {
        // --- Pass 1: World Space (Bubbles & UI) ---
        set_camera(camera);

        for (bkey, bubble) in graph.bubbles.iter() {
            let points = bubble::get_bubble_points(graph, bkey);
            if points.is_empty() {
                continue;
            }
            let centroid = bubble::calculate_centroid(&points);

            self.draw_bubble(&bubble.style, &points, centroid, camera);
        }

        // --- Pass 2: Screen Space (Effects) ---
        set_default_camera();
        self.effects.draw(camera, &self.font);
    }

    fn draw_bubble(&self, style: &BubbleStyle, points: &[Vec2], centroid: Vec2, camera: &Camera2D) {
        if points.is_empty() {
            return;
        }

        match style {
            BubbleStyle::Standard { color, .. } => {
                // Draw Fill (Triangle Fan)
                for i in 0..points.len() {
                    let p1 = points[i];
                    let p2 = points[(i + 1) % points.len()];
                    draw_triangle(centroid, p1, p2, *color);
                }
                // Draw Outline
                for i in 0..points.len() {
                    let p1 = points[i];
                    let p2 = points[(i + 1) % points.len()];
                    draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, colors::WHITE);
                }
            }
            BubbleStyle::Player => {
                // Draw Player (Transparent white for now)
                for i in 0..points.len() {
                    let p1 = points[i];
                    let p2 = points[(i + 1) % points.len()];
                    draw_triangle(centroid, p1, p2, colors::TRANSPARENT_WHITE);
                }
                // Draw Outline
                for i in 0..points.len() {
                    let p1 = points[i];
                    let p2 = points[(i + 1) % points.len()];
                    draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, colors::WHITE);
                }
            }
            BubbleStyle::OpenAir => {}
        }

        // --- Screen Space (UI) Rendering ---
        let label = match style {
            BubbleStyle::Standard { size, .. } => format!("{size}"),
            BubbleStyle::Player => "P".to_string(),
            BubbleStyle::OpenAir => return,
        };

        // Temporarily switch to default camera for screen-space text
        set_default_camera();

        let screen_pos = camera.world_to_screen(centroid);
        let scale = screen_width() / 1200.0;
        let target_pixel_size = 40.0 * scale; // 40 pixel height for logical size of 20

        let font_size = 64; // High-res rasterization
        let font_scale = target_pixel_size / font_size as f32;

        let text_params = TextParams {
            font: Some(&self.font),
            font_size,
            font_scale,
            color: colors::WHITE,
            ..Default::default()
        };

        let text_dims = measure_text(&label, Some(&self.font), font_size, font_scale);

        draw_text_ex(
            &label,
            screen_pos.x - text_dims.width / 2.0,
            screen_pos.y + text_dims.height / 2.0,
            text_params,
        );

        // Switch back to world camera
        set_camera(camera);
    }
}
