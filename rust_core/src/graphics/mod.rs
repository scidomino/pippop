pub mod bubble;
pub mod colors;
pub mod effects;
pub mod geometry;

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
        controller: &crate::managers::game::GameController,
    ) {
        // --- Pass 1: World Space (Bubbles, Managers, UI & Effects) ---
        set_camera(camera);

        for (bkey, bubble) in &graph.bubbles {
            if controller.pop_manager.is_handling(bkey) {
                continue;
            }
            if controller.swap_manager.is_handling(bkey) {
                continue;
            }

            let points = bubble::get_bubble_points(graph, bkey);
            bubble::draw_bubble(&bubble.style, &points, bubble.centroid, &self.font);
        }

        // Delegate specialized rendering to managers
        controller.pop_manager.draw(graph, &self.font);
        controller.swap_manager.draw(graph, &self.font);
        controller.burst_manager.draw(graph);
        controller.highlight_manager.draw(graph);

        self.effects.draw(&self.font);

        // --- Pass 2: Screen Space (UI) ---
        set_default_camera();
        controller.spawn_manager.draw();
    }
}
