use crate::graph::Graph;
use crate::physics::vector::GraphVector;
use macroquad::math::{vec2, Vec2};

const GRAVITY_STRENGTH: f32 = 0.001;
const EXPANSION_STRENGTH: f32 = 0.0001;
const GRAVITY_TARGET: Vec2 = vec2(0.0, 0.0);

/// Updates the centering and expansion forces.
///
/// 1. Gravity: A global force that pulls the entire cluster toward the origin.
/// 2. Expansion: A radial, constant-magnitude force that pushes individual points
///    away from the origin to maintain cluster convexity and prevent overlapping.
pub fn update_force(graph: &Graph, force: &mut GraphVector) {
    let oa_key = graph.get_open_air();
    let bubble_center = graph.bubbles[oa_key].centroid;

    // This vector pulls the entire cluster toward the origin
    let cluster_pull = (GRAVITY_TARGET - bubble_center) * GRAVITY_STRENGTH;

    for (vkey, vertex) in &graph.vertices {
        // Individual expansion: push point away from the origin to prevent collapsing/thinning
        let v_pos = vertex.point.position;
        let v_expansion = (v_pos - GRAVITY_TARGET).normalize_or_zero() * EXPANSION_STRENGTH;
        force.add_vertex(vkey, cluster_pull + v_expansion);

        for ekey in vkey.edge_keys() {
            let edge = vertex.edge(ekey);

            let e_pos = edge.point.position;
            let e_expansion = (e_pos - GRAVITY_TARGET).normalize_or_zero() * EXPANSION_STRENGTH;
            force.add_edge(ekey, cluster_pull + e_expansion);
        }
    }
}
