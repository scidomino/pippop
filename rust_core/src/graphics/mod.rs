pub mod bubble;
pub mod colors;
pub mod effects;
pub mod geometry;
mod ui;

use self::effects::EffectsManager;
use crate::graph::Graph;
use macroquad::prelude::*;

pub struct Renderer {
    pub font: Font,
    pub effects: EffectsManager,
}

impl Renderer {
    pub fn new() -> Self {
        let font_bytes = include_bytes!("../../assets/sniglet_extrabold.ttf");
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
        // Collect visible bubble geometry
        let mut bubble_render_data = Vec::new();
        for (bkey, bubble) in graph.bubbles.iter() {
            let points = bubble::get_bubble_points(graph, bkey);
            if !points.is_empty() {
                let centroid = geometry::calculate_centroid(&points);
                bubble_render_data.push((&bubble.style, points, centroid));
            }
        }

        // --- Pass 1: World Space (Bubbles & Debug) ---
        set_camera(camera);

        for (style, points, centroid) in &bubble_render_data {
            bubble::draw_bubble_body(style, points, *centroid);
        }

        if cfg!(debug_assertions) {
            for (_, vertex) in graph.vertices.iter() {
                for edge in &vertex.edges {
                    let p = edge.point.position;
                    draw_circle(p.x, p.y, 1.0, colors::YELLOW);
                }
            }
        }

        // --- Pass 2: Screen Space (UI & Effects) ---
        set_default_camera();

        for (style, _, centroid) in &bubble_render_data {
            ui::draw_bubble_label(style, *centroid, camera, &self.font);
        }

        self.effects.draw(camera, &self.font);
    }
}
