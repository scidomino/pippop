use crate::game::state::GameState;
use crate::graph::Graph;
use crate::graphics::{bubble, RenderContext};
use crate::physics;

pub struct WorldManager {
    pub physics_accumulator: f32,
    pub debug_control_points: bool,
}

impl WorldManager {
    pub fn new() -> Self {
        let debug_control_points = std::env::var("PIPPOP_DEBUG_CONTROL_POINTS").is_ok();
        Self {
            physics_accumulator: 0.0,
            debug_control_points,
        }
    }

    /// Advances the physics simulation using a fixed timestep.
    pub fn update(&mut self, state: &mut GameState, dt: f32) {
        self.physics_accumulator += dt;
        while self.physics_accumulator >= 0.016 {
            physics::advance_frame(&mut state.graph);
            self.physics_accumulator -= 0.016;
        }
    }

    /// Draws all bubbles in the graph that are not intercepted by other managers.
    pub fn draw(&self, graph: &Graph, ctx: &RenderContext) {
        for (bkey, bubble) in &graph.bubbles {
            let points = bubble::get_bubble_points(graph, bkey);
            bubble::draw_bubble(&bubble.style, &points, bubble.centroid, ctx.font);
        }

        if self.debug_control_points {
            bubble::draw_debug_points(graph);
        }
    }
}
