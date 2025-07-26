// rust_core/src/simulation/mod.rs

use slotmap::SlotMap;
use crate::graph::{Graph, VertexKey, EdgeKey};
use crate::geom::point::Point;
use crate::physics;

const SURFACE_TENSION_COEFFICIENT: f64 = 0.1;

pub struct Simulation {
    pub graph: Graph,
    pub vertex_velocities: SlotMap<VertexKey, Point>,
    pub control_point_velocities: SlotMap<EdgeKey, Point>,
}

impl Simulation {
    pub fn new() -> Self {
        Simulation {
            graph: Graph::new(),
            vertex_velocities: SlotMap::with_key(),
            control_point_velocities: SlotMap::with_key(),
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        // 1. Pre-calculate all half-areas since positions may have changed.
        physics::area::update_edge_half_areas(&mut self.graph);

        let mut forces = SlotMap::with_key();

        // 2. Calculate forces for all control points
        for (edge_key, _) in self.graph.edges.iter() {
            let pressure_force = physics::force::calculate_pressure_force(&self.graph, edge_key, false);
            let tension_force = physics::force::calculate_surface_tension_force(&self.graph, edge_key, false);
            
            let total_force = Point {
                x: pressure_force.x + tension_force.x * SURFACE_TENSION_COEFFICIENT,
                y: pressure_force.y + tension_force.y * SURFACE_TENSION_COEFFICIENT,
            };
            forces.insert(total_force);
        }

        // 3. Update velocities and positions for all control points
        for (edge_key, edge) in self.graph.edges.iter_mut() {
            let force = &forces[edge_key];
            let velocity = self.control_point_velocities.get_mut(edge_key).unwrap();

            velocity.x += force.x * delta_time;
            velocity.y += force.y * delta_time;

            edge.control_point.x += velocity.x * delta_time;
            edge.control_point.y += velocity.y * delta_time;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{Vertex, Edge, Bubble};
    use slotmap::Key;

    fn new_test_simulation() -> (Simulation, EdgeKey) {
        let mut sim = Simulation::new();

        let b0_key = sim.graph.bubbles.insert(Bubble { first_edge_key: EdgeKey::null(), pressure: 1.0 });
        let b1_key = sim.graph.bubbles.insert(Bubble { first_edge_key: EdgeKey::null(), pressure: 2.0 });

        let v0_key = sim.graph.vertices.insert(Vertex { position: Point { x: 0.0, y: 0.0 }, edge_key: EdgeKey::null() });
        let v1_key = sim.graph.vertices.insert(Vertex { position: Point { x: 10.0, y: 0.0 }, edge_key: EdgeKey::null() });
        sim.vertex_velocities.insert(Point { x: 0.0, y: 0.0 });
        sim.vertex_velocities.insert(Point { x: 0.0, y: 0.0 });

        let e0_key;
        let e1_key;
        
        e0_key = sim.graph.edges.insert(Edge {
            start_vertex_key: v0_key,
            control_point: Point { x: 2.5, y: 5.0 },
            twin_edge_key: EdgeKey::null(),
            next_edge_key: EdgeKey::null(),
            bubble_key: b0_key,
            half_area: 0.0,
        });
        e1_key = sim.graph.edges.insert(Edge {
            start_vertex_key: v1_key,
            control_point: Point { x: 7.5, y: -2.0 },
            twin_edge_key: e0_key,
            next_edge_key: EdgeKey::null(),
            bubble_key: b1_key,
            half_area: 0.0,
        });
        sim.control_point_velocities.insert(Point { x: 0.0, y: 0.0 });
        sim.control_point_velocities.insert(Point { x: 0.0, y: 0.0 });
        
        sim.graph.edges[e0_key].twin_edge_key = e1_key;
        sim.graph.edges[e0_key].next_edge_key = e0_key;
        sim.graph.edges[e1_key].next_edge_key = e1_key;
        sim.graph.bubbles[b0_key].first_edge_key = e0_key;
        sim.graph.bubbles[b1_key].first_edge_key = e1_key;
        sim.graph.vertices[v0_key].edge_key = e0_key;
        sim.graph.vertices[v1_key].edge_key = e1_key;

        (sim, e0_key)
    }

    #[test]
    fn test_simulation_update() {
        let (mut simulation, edge_key) = new_test_simulation();
        let initial_pos = simulation.graph.edges[edge_key].control_point.clone();
        simulation.update(0.1);
        let final_pos = &simulation.graph.edges[edge_key].control_point;

        assert_ne!(initial_pos.x, final_pos.x);
        assert_ne!(initial_pos.y, final_pos.y);

        let expected_final = Point { x: 2.5034464375945724, y: 5.0261645218626 };
        
        assert!((final_pos.x - expected_final.x).abs() < 1e-9);
        assert!((final_pos.y - expected_final.y).abs() < 1e-9);
    }
}