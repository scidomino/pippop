pub mod bubble;
pub mod colors;
pub mod geometry;

use crate::graph::Graph;
use crate::resources::Resources;
use macroquad::prelude::*;

pub struct Renderer {
    pub font: Font,
}

pub struct RenderContext<'a> {
    pub graph: &'a Graph,
    pub font: &'a Font,
}

impl Renderer {
    pub fn new(resources: &Resources) -> Self {
        Self {
            font: resources.font.clone(),
        }
    }

    pub fn draw(
        &self,
        graph: &Graph,
        camera: &Camera2D,
        controller: &crate::managers::game::GameController,
    ) {
        let ctx = RenderContext {
            graph,
            font: &self.font,
        };

        // --- Pass 1: World Space (Bubbles, Managers, UI) ---
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
        controller.pop_manager.draw(&ctx);
        controller.swap_manager.draw(&ctx);
        controller.burst_manager.draw(&ctx);
        controller.highlight_manager.draw(&ctx);

        // --- Pass 2: Screen Space (UI) ---
        set_default_camera();
        controller.spawn_manager.draw(&ctx);
    }
}
