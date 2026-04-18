use crate::graph::Graph;
use crate::graph::bubble::BubbleStyle;
use crate::graphics::geometry;
use crate::graphics::colors;
use macroquad::prelude::*;

pub fn get_bubble_points(graph: &Graph, bkey: crate::graph::bubble::BubbleKey) -> Vec<Vec2> {
    let bubble = &graph.bubbles[bkey];
    let mut points = Vec::new();

    for &ekey in &bubble.edges {
        let (edge, vertex) = graph.get_edge_and_vertex(ekey);
        let (twin_edge, twin_vertex) = graph.get_edge_and_vertex(edge.twin);

        geometry::flatten_bezier(
            &mut points,
            vertex.point.position,
            edge.point.position,
            twin_edge.point.position,
            twin_vertex.point.position,
        );
    }
    points
}

pub fn draw_bubble_body(style: &BubbleStyle, points: &[Vec2], centroid: Vec2) {
    if points.is_empty() {
        return;
    }

    let color = match style {
        BubbleStyle::Standard { color, .. } => *color,
        BubbleStyle::Player => colors::TRANSPARENT_WHITE,
        BubbleStyle::OpenAir => return,
    };

    // Draw Fill (Triangle Fan)
    for i in 0..points.len() {
        let p1 = points[i];
        let p2 = points[(i + 1) % points.len()];
        draw_triangle(centroid, p1, p2, color);
    }

    // Draw Outline
    for i in 0..points.len() {
        let p1 = points[i];
        let p2 = points[(i + 1) % points.len()];
        draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, colors::WHITE);
    }
}
