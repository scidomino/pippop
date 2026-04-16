pub mod bubble;
pub mod colors;
pub mod effects;

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
        // --- Pass 1: World Space (Bubbles & UI) ---
        set_camera(camera);
        
        for (bkey, bubble) in graph.bubbles.iter() {
            if bubble.style.is_open_air() {
                continue;
            }

            let points = bubble::get_bubble_points(graph, bkey);
            if points.is_empty() {
                continue;
            }

            let centroid = bubble::calculate_centroid(&points);

            // Delegate all rendering (World + UI) to the style implementation
            bubble.style.render(&points, centroid, camera);
        }

        // --- Pass 2: Screen Space (Effects) ---
        set_default_camera();
        self.effects.draw(camera);
    }
}