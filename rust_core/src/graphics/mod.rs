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

    pub fn draw(
        &self,
        graph: &Graph,
        camera: &Camera2D,
        swap_manager: &crate::managers::swap::SwapManager,
        burst_manager: &crate::managers::burst::BurstManager,
    ) {
        // Collect visible bubble geometry
        let mut bubble_render_data = Vec::new();
        for (bkey, bubble) in graph.bubbles.iter() {
            let points = bubble::get_bubble_points(graph, bkey);
            if !points.is_empty() {
                let centroid = geometry::calculate_centroid(graph, bkey);
                bubble_render_data.push((bkey, &bubble.style, points, centroid));
            }
        }

        // --- Pass 1: World Space (Bubbles & Debug) ---
        set_camera(camera);

        if let Some(swap) = &swap_manager.active_swap {
            let center = graph.get_edge(swap.edge).point.position;

            for (bkey, style, points, centroid) in &bubble_render_data {
                if *bkey == swap.top_bkey || *bkey == swap.bottom_bkey {
                    continue;
                }
                bubble::draw_bubble_body(style, points, *centroid);
            }

            // Draw the two swapping bubbles with morphing
            self.draw_swapping_bubbles(graph, swap, center);
        } else {
            for (_, style, points, centroid) in &bubble_render_data {
                bubble::draw_bubble_body(style, points, *centroid);
            }
        }

        // Draw Burst Glow
        if let Some(ekey) = burst_manager.active_edge {
            let points = bubble::get_edge_points(graph, ekey);
            let progress = 1.0 - (burst_manager.timer / 0.5).clamp(0.0, 1.0);
            let width = 40.0 * progress;
            let glow_mesh = geometry::generate_glow_mesh(&points, width, colors::WHITE, false);
            draw_mesh(&glow_mesh);
        }

        // --- Pass 2: Screen Space (UI & Effects) ---
        set_default_camera();

        for (_, style, _, centroid) in &bubble_render_data {
            ui::draw_bubble_label(style, *centroid, camera, &self.font);
        }

        self.effects.draw(camera, &self.font);
    }

    fn draw_swapping_bubbles(&self, graph: &Graph, swap: &crate::managers::swap::ActiveSwap, center: Vec2) {
        let top_points = bubble::get_bubble_points(graph, swap.top_bkey);
        let bottom_points = bubble::get_bubble_points(graph, swap.bottom_bkey);

        if top_points.is_empty() || bottom_points.is_empty() {
            return;
        }

        // Top Bubble Animation
        let rotated_top_start = geometry::rotate_points(&top_points, center, swap.progress * std::f32::consts::PI);
        let rotated_top_end = geometry::rotate_points(&bottom_points, center, (swap.progress - 1.0) * std::f32::consts::PI);
        let morphed_top = geometry::tween_points(&rotated_top_start, &rotated_top_end, swap.progress);
        
        // Bottom Bubble Animation
        let rotated_bottom_start = geometry::rotate_points(&bottom_points, center, swap.progress * std::f32::consts::PI);
        let rotated_bottom_end = geometry::rotate_points(&top_points, center, (swap.progress - 1.0) * std::f32::consts::PI);
        let morphed_bottom = geometry::tween_points(&rotated_bottom_start, &rotated_bottom_end, swap.progress);

        bubble::draw_bubble_body(&swap.top_style, &morphed_top, center);
        bubble::draw_bubble_body(&swap.bottom_style, &morphed_bottom, center);
    }
}
