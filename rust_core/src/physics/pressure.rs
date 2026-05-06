use super::vector::GraphVector;
use crate::graph::bubble::BubbleKey;
use crate::graph::Graph;
use crate::graphics::geometry::Bezier;
use macroquad::math::Vec2;
use slotmap::SecondaryMap;

const PRESSURE_TENSION: f32 = 0.04;

/// Computes and applies pressure forces to all vertices and control points in the graph.
///
/// Pressure forces act to expand or contract a bubble to reach its `target_area`.
/// The force is applied outwardly along the normal of the boundary edges.
pub fn update_force(graph: &Graph, force: &mut GraphVector) {
    let bubble_to_pressure = get_bubble_to_pressure(graph);
    for key in graph.vertices.keys() {
        let mut vertex_force = Vec2::ZERO;

        for ekey in key.edge_keys() {
            let edge = graph.vertices.get_edge(ekey);
            let bezier = graph.vertices.get_bezier(ekey);
            let twin_bubble = graph.vertices.get_edge(edge.twin).bubble;

            let pressure_diff = (bubble_to_pressure[edge.bubble] - bubble_to_pressure[twin_bubble])
                .clamp(-2.0, 2.0);
            let pressure = pressure_diff * PRESSURE_TENSION;

            vertex_force += pressure * vertex_pressure_force(&bezier);
            force.add_edge(ekey, pressure * edge_pressure_force(&bezier));
        }

        force.add_vertex(key, vertex_force);
    }
}

/// Calculates the pressure force applied to the start vertex of a Bezier curve.
///
/// This formula is derived analytically from the gradient of the curve's area
/// integral with respect to the start point (`b.s`).
fn vertex_pressure_force(b: &Bezier) -> Vec2 {
    (10.0 * b.s + 6.0 * b.sc + 3.0 * b.ec + b.e).perp() / 20.0
}

/// Calculates the pressure force applied to the first control point of a Bezier curve.
///
/// This formula is derived analytically from the gradient of the curve's area
/// integral with respect to the first control point (`b.sc`).
fn edge_pressure_force(b: &Bezier) -> Vec2 {
    (-6.0 * b.s + 3.0 * b.ec + 3.0 * b.e).perp() / 20.0
}

fn get_bubble_to_pressure(graph: &Graph) -> SecondaryMap<BubbleKey, f32> {
    graph
        .bubbles
        .iter()
        .map(|(key, bubble)| (key, bubble.get_pressure()))
        .collect()
}
