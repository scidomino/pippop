pub mod bubble;
pub mod colors;
pub mod effects;
pub mod geometry;
mod ui;

use self::effects::EffectsManager;
use crate::graph::bubble::BubbleStyle;
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
        highlight_manager: &crate::managers::highlight::HighlightManager,
    ) {
        // Collect visible bubble geometry
        let bubble_render_data: Vec<_> = graph
            .bubbles
            .iter()
            .filter_map(|(bkey, bubble)| {
                let mut points = bubble::get_bubble_points(graph, bkey);
                (!points.is_empty()).then(|| {
                    let centroid = geometry::calculate_centroid(graph, bkey);

                    if let BubbleStyle::Popping { size, timer, .. } = bubble.style {
                        let progress = (timer / 0.5).clamp(0.0, 1.0);
                        points = self.apply_pop_morph(&points, centroid, size, progress);
                    }

                    (bkey, &bubble.style, points, centroid)
                })
            })
            .collect();

        // --- Pass 1: World Space (Bubbles & Debug) ---
        set_camera(camera);

        if let Some(swap) = &swap_manager.active_swap {
            let center = graph.vertices.get_edge(swap.edge).point.position;

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

        // Draw Highlights (Touch & Teaser)
        let glow_requests = highlight_manager.get_glow_requests(graph);
        for (bkey, intensity) in glow_requests {
            if let Some((_, _, points, _)) =
                bubble_render_data.iter().find(|(k, _, _, _)| *k == bkey)
            {
                let width = 20.0 * intensity;
                let glow_mesh = geometry::generate_glow_mesh(points, width, colors::WHITE, true);
                draw_mesh(&glow_mesh);
            }
        }

        // Draw Burst Glow
        if let Some(ekey) = burst_manager.active_edge {
            let mut points = Vec::with_capacity(12);
            bubble::push_edge_points(graph, ekey, &mut points);
            let twin_key = graph.vertices.get_edge(ekey).twin;
            points.push(graph.vertices[twin_key.vertex].point.position);

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

    fn draw_swapping_bubbles(
        &self,
        graph: &Graph,
        swap: &crate::managers::swap::ActiveSwap,
        center: Vec2,
    ) {
        let top_points = bubble::get_bubble_points(graph, swap.top_bkey);
        let bottom_points = bubble::get_bubble_points(graph, swap.bottom_bkey);

        if top_points.is_empty() || bottom_points.is_empty() {
            return;
        }

        // Top Bubble Animation
        let rotated_top_start =
            geometry::rotate_points(&top_points, center, swap.progress * std::f32::consts::PI);
        let rotated_top_end = geometry::rotate_points(
            &bottom_points,
            center,
            (swap.progress - 1.0) * std::f32::consts::PI,
        );
        let morphed_top =
            geometry::tween_points(&rotated_top_start, &rotated_top_end, swap.progress);

        // Bottom Bubble Animation
        let rotated_bottom_start =
            geometry::rotate_points(&bottom_points, center, swap.progress * std::f32::consts::PI);
        let rotated_bottom_end = geometry::rotate_points(
            &top_points,
            center,
            (swap.progress - 1.0) * std::f32::consts::PI,
        );
        let morphed_bottom =
            geometry::tween_points(&rotated_bottom_start, &rotated_bottom_end, swap.progress);

        bubble::draw_bubble_body(&swap.top_style, &morphed_top, center);
        bubble::draw_bubble_body(&swap.bottom_style, &morphed_bottom, center);
    }

    fn apply_pop_morph(
        &self,
        points: &[Vec2],
        centroid: Vec2,
        size: i32,
        progress: f32,
    ) -> Vec<Vec2> {
        let target_area = 3000.0 * (size as f32).sqrt();
        let radius = 5.0 * (target_area / std::f32::consts::PI).sqrt();

        let first_p = points[0];
        let start_angle = (first_p.y - centroid.y).atan2(first_p.x - centroid.x);

        let n = points.len();
        let morph_ratio = progress.powi(2);
        let inv_morph = 1.0 - morph_ratio;

        points
            .iter()
            .enumerate()
            .map(|(i, &p)| {
                let angle = start_angle - (2.0 * std::f32::consts::PI) * (i as f32 / n as f32);
                let circle_p = centroid + vec2(angle.cos() * radius, angle.sin() * radius);
                p * morph_ratio + circle_p * inv_morph
            })
            .collect()
    }
}
