use crate::graph::Graph;
use crate::physics::vector::GraphVector;
use macroquad::math::{vec2, Vec2};

const GRAVITY_STRENGTH: f32 = 0.001;
const GRAVITY_TARGET: Vec2 = vec2(0.0, 0.0);

pub fn update_force(graph: &Graph, force: &mut GraphVector) {
    let oa_key = graph.get_open_air();
    let bubble_center = graph.bubbles[oa_key].centroid;
    let gravity_vec = (GRAVITY_TARGET - bubble_center) * GRAVITY_STRENGTH;

    for vkey in graph.vertices.keys() {
        for ekey in vkey.edge_keys() {
            force.add_vertex(vkey, gravity_vec);
            force.add_edge(ekey, gravity_vec);
        }
    }
}
