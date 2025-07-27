// rust_core/src/simulation/mod.rs

use slotmap::{SlotMap, Key};
use crate::graph::{Graph, Vertex, Edge, Bubble, VertexKey, EdgeKey, BubbleKey, bubble::BubbleKind};
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

    pub fn spawn(&mut self, vertex_key: VertexKey, kind: BubbleKind) -> BubbleKey {
        let [e1_key, e2_key, e3_key] = self.get_vertex_neighbors(vertex_key);
        
        let ne1_key = self.split_edge(e1_key);
        let ne2_key = self.split_edge(e2_key);
        let ne3_key = self.split_edge(e3_key);

        self.graph.edges[ne1_key].next_edge_key = ne3_key;
        self.graph.edges[ne2_key].next_edge_key = ne1_key;
        self.graph.edges[ne3_key].next_edge_key = ne2_key;

        let new_bubble_key = self.graph.bubbles.insert(Bubble {
            first_edge_key: ne1_key,
            pressure: 1.5, // Average pressure for now
            kind,
        });

        self.graph.edges[ne1_key].bubble_key = new_bubble_key;
        self.graph.edges[ne2_key].bubble_key = new_bubble_key;
        self.graph.edges[ne3_key].bubble_key = new_bubble_key;

        self.graph.vertices.remove(vertex_key);
        self.vertex_velocities.remove(vertex_key);

        new_bubble_key
    }

    fn get_vertex_neighbors(&self, vertex_key: VertexKey) -> [EdgeKey; 3] {
        let e0_key = self.graph.vertices[vertex_key].edge_key;
        let e1_key = self.graph.edges[e0_key].next_edge_key;
        let e2_key = self.graph.edges[e1_key].next_edge_key;
        [e0_key, e1_key, e2_key]
    }

    fn split_edge(&mut self, edge_key: EdgeKey) -> EdgeKey {
        let (p0, p1, p2, p3, original_next_key, original_twin_next_key, bubble_key, twin_bubble_key, twin_key, end_vertex_key) = {
            let edge = &self.graph.edges[edge_key];
            let twin = &self.graph.edges[edge.twin_edge_key];
            (
                self.graph.vertices[edge.start_vertex_key].position,
                edge.control_point,
                twin.control_point,
                self.graph.vertices[twin.start_vertex_key].position,
                edge.next_edge_key,
                twin.next_edge_key,
                edge.bubble_key,
                twin.bubble_key,
                edge.twin_edge_key,
                twin.start_vertex_key,
            )
        };

        let center_ctrl = p1.midpoint(&p2);
        let new_p1 = p0.midpoint(&p1);
        let new_p2 = p3.midpoint(&p2);
        let ctrl_12 = new_p1.midpoint(&center_ctrl);
        let ctrl_21 = new_p2.midpoint(&center_ctrl);
        let center_pos = ctrl_12.midpoint(&ctrl_21);

        let new_vertex_key = self.graph.vertices.insert(Vertex {
            position: center_pos,
            edge_key: EdgeKey::null(),
        });
        self.vertex_velocities.insert(Point { x: 0.0, y: 0.0 });

        let new_edge = Edge {
            start_vertex_key: new_vertex_key,
            control_point: ctrl_21,
            twin_edge_key: EdgeKey::null(),
            next_edge_key: original_next_key,
            bubble_key,
            half_area: 0.0,
        };
        let new_twin_edge = Edge {
            start_vertex_key: end_vertex_key,
            control_point: new_p2,
            twin_edge_key: EdgeKey::null(),
            next_edge_key: original_twin_next_key,
            bubble_key: twin_bubble_key,
            half_area: 0.0,
        };
        
        let new_edge_key = self.graph.edges.insert(new_edge);
        let new_twin_key = self.graph.edges.insert(new_twin_edge);
        
        self.graph.edges[new_edge_key].twin_edge_key = new_twin_key;
        self.graph.edges[new_twin_key].twin_edge_key = new_edge_key;
        
        self.control_point_velocities.insert(Point { x: 0.0, y: 0.0 });
        self.control_point_velocities.insert(Point { x: 0.0, y: 0.0 });

        let original_edge = &mut self.graph.edges[edge_key];
        original_edge.next_edge_key = new_edge_key;
        original_edge.control_point = new_p1;
        
        let twin_edge = &mut self.graph.edges[twin_key];
        twin_edge.next_edge_key = new_twin_key;
        twin_edge.start_vertex_key = new_vertex_key;
        twin_edge.control_point = ctrl_12;

        self.graph.vertices[new_vertex_key].edge_key = new_edge_key;

        new_edge_key
    }

    pub fn update(&mut self, delta_time: f64) {
        physics::area::update_edge_half_areas(&mut self.graph);

        let mut forces = SlotMap::with_key();

        for (edge_key, _) in self.graph.edges.iter() {
            let pressure_force = physics::force::calculate_pressure_force(&self.graph, edge_key, false);
            let tension_force = physics::force::calculate_surface_tension_force(&self.graph, edge_key, false);
            
            let total_force = Point {
                x: pressure_force.x + tension_force.x * SURFACE_TENSION_COEFFICIENT,
                y: pressure_force.y + tension_force.y * SURFACE_TENSION_COEFFICIENT,
            };
            forces.insert(total_force);
        }

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
    use crate::graph::{Bubble, Edge, Vertex};
    use slotmap::Key;

    fn new_test_simulation() -> (Simulation, VertexKey, [EdgeKey; 3]) {
        let mut sim = Simulation::new();

        let b0 = sim.graph.bubbles.insert(Bubble {
            first_edge_key: EdgeKey::null(),
            pressure: 10.0,
            kind: BubbleKind::Game { color: Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }, size: 1 },
        });
        let b1 = sim.graph.bubbles.insert(Bubble {
            first_edge_key: EdgeKey::null(),
            pressure: 1.5,
            kind: BubbleKind::Player,
        });
        let b2 = sim.graph.bubbles.insert(Bubble {
            first_edge_key: EdgeKey::null(),
            pressure: 2.0,
            kind: BubbleKind::Game { color: Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 }, size: 2 },
        });

        let v_center = sim.graph.vertices.insert(Vertex { position: Point { x: 0.0, y: 0.0 }, edge_key: EdgeKey::null() });
        sim.vertex_velocities.insert(Point { x: 0.0, y: 0.0 });
        
        let v0 = sim.graph.vertices.insert(Vertex { position: Point { x: 12.0, y: 2.0 }, edge_key: EdgeKey::null() });
        let v1 = sim.graph.vertices.insert(Vertex { position: Point { x: -5.0, y: 8.66 }, edge_key: EdgeKey::null() });
        let v2 = sim.graph.vertices.insert(Vertex { position: Point { x: -5.0, y: -8.66 }, edge_key: EdgeKey::null() });
        sim.vertex_velocities.insert(Point { x: 0.0, y: 0.0 });
        sim.vertex_velocities.insert(Point { x: 0.0, y: 0.0 });
        sim.vertex_velocities.insert(Point { x: 0.0, y: 0.0 });

        let e0 = sim.graph.edges.insert(Edge { start_vertex_key: v_center, control_point: Point{x: 5.0, y: 0.0}, twin_edge_key: EdgeKey::null(), next_edge_key: EdgeKey::null(), bubble_key: b0, half_area: 0.0 });
        let e0_twin = sim.graph.edges.insert(Edge { start_vertex_key: v0, control_point: Point{x: 5.0, y: 0.0}, twin_edge_key: e0, next_edge_key: EdgeKey::null(), bubble_key: b2, half_area: 0.0 });
        sim.graph.edges[e0].twin_edge_key = e0_twin;

        let e1 = sim.graph.edges.insert(Edge { start_vertex_key: v_center, control_point: Point{x: -2.5, y: 4.33}, twin_edge_key: EdgeKey::null(), next_edge_key: EdgeKey::null(), bubble_key: b1, half_area: 0.0 });
        let e1_twin = sim.graph.edges.insert(Edge { start_vertex_key: v1, control_point: Point{x: -2.5, y: 4.33}, twin_edge_key: e1, next_edge_key: EdgeKey::null(), bubble_key: b0, half_area: 0.0 });
        sim.graph.edges[e1].twin_edge_key = e1_twin;

        let e2 = sim.graph.edges.insert(Edge { start_vertex_key: v_center, control_point: Point{x: -2.5, y: -4.33}, twin_edge_key: EdgeKey::null(), next_edge_key: EdgeKey::null(), bubble_key: b2, half_area: 0.0 });
        let e2_twin = sim.graph.edges.insert(Edge { start_vertex_key: v2, control_point: Point{x: -2.5, y: -4.33}, twin_edge_key: e2, next_edge_key: EdgeKey::null(), bubble_key: b1, half_area: 0.0 });
        sim.graph.edges[e2].twin_edge_key = e2_twin;

        sim.graph.edges[e0].next_edge_key = e1;
        sim.graph.edges[e1].next_edge_key = e2;
        sim.graph.edges[e2].next_edge_key = e0;
        
        sim.graph.vertices[v_center].edge_key = e0;

        for _ in 0..6 {
            sim.control_point_velocities.insert(Point { x: 0.0, y: 0.0 });
        }

        (sim, v_center, [e0, e1, e2])
    }

    #[test]
    fn test_get_vertex_neighbors() {
        let (simulation, v_center, ref expected_edges) = new_test_simulation();
        let neighbors = simulation.get_vertex_neighbors(v_center);
        
        assert_eq!(neighbors[0], expected_edges[0]);
        assert_eq!(neighbors[1], expected_edges[1]);
        assert_eq!(neighbors[2], expected_edges[2]);
    }

    #[test]
    fn test_simulation_update() {
        let (mut simulation, _, edges) = new_test_simulation();
        let edge_key = edges[0];
        let initial_pos = simulation.graph.edges[edge_key].control_point.clone();
        simulation.update(0.1);
        let final_pos = &simulation.graph.edges[edge_key].control_point;

        assert_ne!(initial_pos.x, final_pos.x);
        assert_ne!(initial_pos.y, final_pos.y);
    }

    #[test]
    fn test_split_edge() {
        let (mut simulation, _, edges) = new_test_simulation();
        let edge_key_to_split = edges[0];
        
        let initial_vertex_count = simulation.graph.vertices.len();
        let initial_edge_count = simulation.graph.edges.len();

        let new_edge_key = simulation.split_edge(edge_key_to_split);

        assert_eq!(simulation.graph.vertices.len(), initial_vertex_count + 1);
        assert_eq!(simulation.graph.edges.len(), initial_edge_count + 2);
        
        let original_edge = &simulation.graph.edges[edge_key_to_split];
        let new_edge = &simulation.graph.edges[new_edge_key];
        
        assert_eq!(original_edge.next_edge_key, new_edge_key);
        
        let new_vertex_key = new_edge.start_vertex_key;
        assert!(simulation.graph.vertices.contains_key(new_vertex_key));
        
        let twin_key = original_edge.twin_edge_key;
        let twin_edge = &simulation.graph.edges[twin_key];
        assert_eq!(twin_edge.start_vertex_key, new_vertex_key);

        let new_twin_key = new_edge.twin_edge_key;
        let new_twin_edge = &simulation.graph.edges[new_twin_key];
        assert_eq!(new_twin_edge.twin_edge_key, new_edge_key);
    }

    #[test]
    fn test_spawn() {
        let (mut simulation, v_center, _) = new_test_simulation();
        let initial_bubble_count = simulation.graph.bubbles.len();
        let initial_vertex_count = simulation.graph.vertices.len();
        let initial_edge_count = simulation.graph.edges.len();

        let new_bubble_key = simulation.spawn(v_center, BubbleKind::Game {
            color: Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 },
            size: 1,
        });

        assert_eq!(simulation.graph.bubbles.len(), initial_bubble_count + 1);
        assert_eq!(simulation.graph.vertices.len(), initial_vertex_count - 1 + 3);
        assert_eq!(simulation.graph.edges.len(), initial_edge_count + 6);

        let new_bubble = &simulation.graph.bubbles[new_bubble_key];
        let e1_key = new_bubble.first_edge_key;
        let e2_key = simulation.graph.edges[e1_key].next_edge_key;
        let e3_key = simulation.graph.edges[e2_key].next_edge_key;

        assert_eq!(simulation.graph.edges[e3_key].next_edge_key, e1_key);

        assert_eq!(simulation.graph.edges[e1_key].bubble_key, new_bubble_key);
        assert_eq!(simulation.graph.edges[e2_key].bubble_key, new_bubble_key);
        assert_eq!(simulation.graph.edges[e3_key].bubble_key, new_bubble_key);
    }
}