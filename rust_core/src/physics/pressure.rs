use super::vector::GraphVector;
use crate::graph::bubble::BubbleKey;
use crate::graph::Graph;
use macroquad::math::Vec2;
use slotmap::SecondaryMap;

const PRESSURE_TENSION: f32 = 0.04;

pub fn update_force(graph: &Graph, force: &mut GraphVector) {
    let bubble_to_pressure = get_bubble_to_pressure(graph);
    for key in graph.vertices.keys() {
        let mut vertex_force = Vec2::ZERO;

        for ekey in key.edge_keys() {
            let edge = graph.get_edge(ekey);
            let bezier = graph.get_bezier(ekey);
            let twin_bubble = graph.get_edge(edge.twin).bubble;

            let pressure_diff = (bubble_to_pressure[edge.bubble] - bubble_to_pressure[twin_bubble])
                .clamp(-2.0, 2.0);
            let pressure = pressure_diff * PRESSURE_TENSION;

            vertex_force += pressure * vertex_pressure_force(&bezier);
            force.add_edge(ekey, pressure * edge_pressure_force(&bezier));
        }

        force.add_vertex(key, vertex_force);
    }
}

fn vertex_pressure_force(b: &crate::graphics::geometry::Bezier) -> Vec2 {
    Vec2::new(
        (-10.0 * b.s.y - 6.0 * b.sc.y - 3.0 * b.ec.y - b.e.y) / 20.0,
        (-10.0 * b.s.x + 6.0 * b.sc.x + 3.0 * b.ec.x + b.e.x) / 20.0,
    )
}

fn edge_pressure_force(b: &crate::graphics::geometry::Bezier) -> Vec2 {
    Vec2::new(
        (6.0 * b.s.y - 3.0 * b.ec.y - 3.0 * b.e.y) / 20.0,
        (-6.0 * b.s.x + 3.0 * b.ec.x + 3.0 * b.e.x) / 20.0,
    )
}

fn get_bubble_to_pressure(graph: &Graph) -> SecondaryMap<BubbleKey, f32> {
    let mut bubble_to_area: SecondaryMap<BubbleKey, f32> =
        graph.bubbles.keys().map(|k| (k, 0.0)).collect();

    for vkey in graph.vertices.keys() {
        for ekey in vkey.edge_keys() {
            let edge = graph.get_edge(ekey);
            let bezier = graph.get_bezier(ekey);
            let twin_bubble = graph.get_edge(edge.twin).bubble;

            let half_area = bezier.half_area_contribution();
            bubble_to_area[edge.bubble] += half_area;
            bubble_to_area[twin_bubble] -= half_area;
        }
    }

    graph
        .bubbles
        .iter()
        .map(|(key, bubble)| (key, bubble.get_pressure(bubble_to_area[key])))
        .collect()
}
