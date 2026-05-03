use crate::game::state::{GamePhase, UpdateContext};
use crate::graphics::{bubble, RenderContext};
use crate::physics;
use macroquad::prelude::*;
use std::env;

pub struct WorldManager {
    pub physics_accumulator: f32,
    pub debug_control_points: bool,
}

impl WorldManager {
    pub fn new() -> Self {
        let debug_control_points = env::var("DEBUG").is_ok();
        Self {
            physics_accumulator: 0.0,
            debug_control_points,
        }
    }

    /// Advances the physics simulation using a fixed timestep.
    pub fn update(&mut self, ctx: &mut UpdateContext) {
        if ctx.state.phase == GamePhase::Bursting {
            return;
        }
        self.physics_accumulator += ctx.dt;
        while self.physics_accumulator >= 0.016 {
            physics::advance_frame(&mut ctx.state.graph);
            self.physics_accumulator -= 0.016;
        }
    }

    /// Draws all bubbles in the graph that are not intercepted by other managers.
    pub fn draw(&self, ctx: &RenderContext) {
        set_camera(ctx.camera);

        for (bkey, bubble) in &ctx.graph.bubbles {
            let points = bubble::get_bubble_points(ctx.graph, bkey);
            bubble::draw_bubble(&bubble.style, &points, bubble.centroid, ctx.font);
        }

        if self.debug_control_points {
            bubble::draw_debug_points(ctx.graph);

            // Draw FPS in screen space if debug is enabled
            set_default_camera();
            macroquad::text::draw_text(
                &format!("FPS: {:03}", macroquad::time::get_fps()),
                10.0,
                30.0,
                30.0,
                crate::graphics::colors::WHITE,
            );

            // Restore camera
            set_camera(ctx.camera);
        }
    }
}
