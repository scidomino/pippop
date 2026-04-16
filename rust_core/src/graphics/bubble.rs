use crate::graph::Graph;
use crate::graph::point::Coordinate;
use macroquad::prelude::*;

pub fn get_bubble_points(graph: &Graph, bkey: crate::graph::bubble::BubbleKey, steps: usize) -> Vec<Vec2> {
    let bubble = &graph.bubbles[bkey];
    let mut points = Vec::new();

    for &ekey in &bubble.edges {
        let (edge, vertex) = graph.get_edge_and_vertex(ekey);
        let (twin_edge, twin_vertex) = graph.get_edge_and_vertex(edge.twin);

        for i in 0..steps {
            let t = i as f32 / steps as f32;
            points.push(sample_cubic_bezier(
                vertex.point.position,
                edge.point.position,
                twin_edge.point.position,
                twin_vertex.point.position,
                t,
            ));
        }
    }
    points
}

pub fn sample_cubic_bezier(p0: Coordinate, p1: Coordinate, p2: Coordinate, p3: Coordinate, t: f32) -> Vec2 {
    let inv_t = 1.0 - t;
    let b0 = inv_t * inv_t * inv_t;
    let b1 = 3.0 * t * inv_t * inv_t;
    let b2 = 3.0 * t * t * inv_t;
    let b3 = t * t * t;

    Vec2::new(
        b0 * p0.x + b1 * p1.x + b2 * p2.x + b3 * p3.x,
        b0 * p0.y + b1 * p1.y + b2 * p2.y + b3 * p3.y,
    )
}

pub fn calculate_centroid(points: &[Vec2]) -> Vec2 {
    if points.is_empty() {
        return Vec2::ZERO;
    }
    let sum = points.iter().fold(Vec2::ZERO, |acc, p| acc + *p);
    sum / points.len() as f32
}
