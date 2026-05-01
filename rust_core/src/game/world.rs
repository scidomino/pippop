use crate::game::state::GameState;
use crate::graph::Graph;
use crate::graphics::{bubble, RenderContext};
use crate::physics;

pub struct WorldManager {
    pub physics_accumulator: f32,
}

impl WorldManager {
    pub fn new() -> Self {
        Self {
            physics_accumulator: 0.0,
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
    }
}
