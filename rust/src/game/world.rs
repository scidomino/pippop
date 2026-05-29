use crate::game::state::{GamePhase, UpdateContext};
use crate::graphics::{bubble, RenderContext};
use crate::physics;
use macroquad::prelude::*;

#[derive(Default)]
pub struct WorldManager {
    pub physics_accumulator: f32,
}

const PHYSICS_TIMESTEP: f32 = 0.016;

impl WorldManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Advances the physics simulation using a fixed timestep.
    pub fn update(&mut self, ctx: &mut UpdateContext) {
        if matches!(ctx.state.phase, GamePhase::Paused(_) | GamePhase::Bursting) {
            return;
        }
        self.physics_accumulator += ctx.dt;
        while self.physics_accumulator >= PHYSICS_TIMESTEP {
            physics::advance_frame(&mut ctx.state.graph);
            self.physics_accumulator -= PHYSICS_TIMESTEP;
        }
    }

    /// Draws all bubbles in the graph that are not intercepted by other managers.
    pub fn draw(&self, ctx: &RenderContext) {
        set_camera(ctx.camera);

        for (bkey, bubble) in &ctx.state.graph.bubbles {
            let points = bubble::get_bubble_points(&ctx.state.graph, bkey);
            bubble::draw_bubble(ctx.resources, &bubble.style, &points, bubble.centroid);
        }
    }
}
