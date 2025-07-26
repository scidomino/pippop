// rust_core/src/simulation/mod.rs

use crate::graph::Graph;
use crate::geom::point::Point;
use crate::physics;

const SURFACE_TENSION_COEFFICIENT: f64 = 0.1; // Arbitrary constant for surface tension

pub struct Simulation {
    pub graph: Graph,
    pub vertex_velocities: Vec<Point>,
    pub control_point_velocities: Vec<Point>,
}

impl Simulation {
    /// Creates a new Simulation from a given Graph.
    /// Initializes all velocities to zero.
    pub fn new(graph: Graph) -> Self {
        let num_vertices = graph.vertices.len();
        let num_edges = graph.edges.len();

        Simulation {
            graph,
            vertex_velocities: vec![Point { x: 0.0, y: 0.0 }; num_vertices],
            control_point_velocities: vec![Point { x: 0.0, y: 0.0 }; num_edges],
        }
    }

    /// Performs one physics step using Euler integration.
    pub fn update(&mut self, delta_time: f64) {
        let mut forces = vec![Point { x: 0.0, y: 0.0 }; self.graph.edges.len()];

        // 1. Calculate forces for all control points
        for edge_id in 0..self.graph.edges.len() {
            let pressure_force = physics::force::calculate_pressure_force(&self.graph, edge_id, false);
            let tension_force = physics::force::calculate_surface_tension_force(&self.graph, edge_id, false);
            
            forces[edge_id].x = pressure_force.x + tension_force.x * SURFACE_TENSION_COEFFICIENT;
            forces[edge_id].y = pressure_force.y + tension_force.y * SURFACE_TENSION_COEFFICIENT;
        }

        // 2. Update velocities and positions for all control points
        for edge_id in 0..self.graph.edges.len() {
            // Update velocity: v_new = v_old + F * dt
            self.control_point_velocities[edge_id].x += forces[edge_id].x * delta_time;
            self.control_point_velocities[edge_id].y += forces[edge_id].y * delta_time;

            // Update position: p_new = p_old + v_new * dt
            self.graph.edges[edge_id].control_point.x += self.control_point_velocities[edge_id].x * delta_time;
            self.graph.edges[edge_id].control_point.y += self.control_point_velocities[edge_id].y * delta_time;
        }
        
        // Note: Vertex forces and movement are not yet implemented.
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_update() {
        let graph = Graph::new_test_graph();
        let mut simulation = Simulation::new(graph);

        let initial_pos = simulation.graph.edges[0].control_point.clone();
        
        simulation.update(0.1); // Step forward by 0.1 seconds

        let final_pos = &simulation.graph.edges[0].control_point;

        // Verify that the control point has moved from its initial position.
        // The exact value depends on the forces, but it should not be zero.
        assert_ne!(initial_pos.x, final_pos.x);
        assert_ne!(initial_pos.y, final_pos.y);

        // Let's do a more specific check based on our known forces.
        // Force on edge 0's control point = pressure_force + tension_force * 0.1
        // pressure_force = (0.3, 2.625)
        // tension_force = (0.446, -0.085)
        // total_force = (0.3 + 0.0446, 2.625 - 0.0085) = (0.3446, 2.6165)
        //
        // dv = F * dt = (0.03446, 0.26165)
        // dp = dv * dt = (0.003446, 0.026165)
        //
        // final_pos = initial_pos + dp
        // initial_pos = (2.5, 5.0)
        // expected_final = (2.503446, 5.026165)
        
        assert!((final_pos.x - 2.503446).abs() < 1e-6);
        assert!((final_pos.y - 5.026165).abs() < 1e-6);
    }
}