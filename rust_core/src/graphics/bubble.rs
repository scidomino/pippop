use crate::graph::Graph;
use macroquad::prelude::*;

const FLATNESS: f32 = 0.5;

pub fn get_bubble_points(graph: &Graph, bkey: crate::graph::bubble::BubbleKey) -> Vec<Vec2> {
    let bubble = &graph.bubbles[bkey];
    let mut points = Vec::new();

    for &ekey in &bubble.edges {
        let (edge, vertex) = graph.get_edge_and_vertex(ekey);
        let (twin_edge, twin_vertex) = graph.get_edge_and_vertex(edge.twin);

        flatten_bezier(
            &mut points,
            vec2(vertex.point.position.x, vertex.point.position.y),
            vec2(edge.point.position.x, edge.point.position.y),
            vec2(twin_edge.point.position.x, twin_edge.point.position.y),
            vec2(twin_vertex.point.position.x, twin_vertex.point.position.y),
        );
    }
    points
}

fn flatten_bezier(points: &mut Vec<Vec2>, p1: Vec2, p2: Vec2, p3: Vec2, p4: Vec2) {
    let dx = p4.x - p1.x;
    let dy = p4.y - p1.y;
    let d2 = ((p2.x - p4.x) * dy - (p2.y - p4.y) * dx).abs();
    let d3 = ((p3.x - p4.x) * dy - (p3.y - p4.y) * dx).abs();

    if (d2 + d3) * (d2 + d3) < FLATNESS * (dx * dx + dy * dy) {
        points.push(p1);
        return;
    }

    // Split in two by De Casteljau's Algorithm
    let p12 = (p1 + p2) / 2.0;
    let p23 = (p2 + p3) / 2.0;
    let p34 = (p3 + p4) / 2.0;
    let p123 = (p12 + p23) / 2.0;
    let p234 = (p23 + p34) / 2.0;
    let p1234 = (p123 + p234) / 2.0;

    flatten_bezier(points, p1, p12, p123, p1234);
    flatten_bezier(points, p1234, p234, p34, p4);
}

pub fn calculate_centroid(points: &[Vec2]) -> Vec2 {
    if points.is_empty() {
        return Vec2::ZERO;
    }
    let sum = points.iter().fold(Vec2::ZERO, |acc, p| acc + *p);
    sum / points.len() as f32
}
