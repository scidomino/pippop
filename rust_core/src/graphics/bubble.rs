use crate::graph::Graph;
use crate::graph::bubble::{Bubble, BubbleStyle};
use crate::graphics::geometry;
use crate::graphics::colors;
use macroquad::prelude::*;

pub fn get_bubble_points(graph: &Graph, bkey: crate::graph::bubble::BubbleKey) -> Vec<Vec2> {
    get_points_for_bubble(graph, &graph.bubbles[bkey])
}

pub fn get_points_for_bubble(graph: &Graph, bubble: &Bubble) -> Vec<Vec2> {
    let mut points = Vec::new();

    for &ekey in &bubble.edges {
        points.extend(get_edge_points(graph, ekey));
    }
    points
}

pub fn get_edge_points(graph: &Graph, ekey: crate::graph::edge::EdgeKey) -> Vec<Vec2> {
    let mut points = Vec::new();
    let (edge, vertex) = graph.get_edge_and_vertex(ekey);
    let (twin_edge, twin_vertex) = graph.get_edge_and_vertex(edge.twin);

    geometry::flatten_bezier(
        &mut points,
        vertex.point.position,
        edge.point.position,
        twin_edge.point.position,
        twin_vertex.point.position,
    );
    // Add the final point which flatten_bezier omits
    points.push(twin_vertex.point.position);
    points
}

pub fn draw_bubble_body(style: &BubbleStyle, points: &[Vec2], _centroid: Vec2) {
    if points.is_empty() {
        return;
    }

    let color = match style {
        BubbleStyle::Standard { color, .. } => *color,
        BubbleStyle::Player => colors::TRANSPARENT_WHITE,
        BubbleStyle::OpenAir | BubbleStyle::Waiting { .. } => return,
    };

    // Draw Fill (Ear Clipping Triangulation)
    // This perfectly handles concave shapes with zero overdraw, eliminating spikes.
    let triangles = geometry::triangulate(points);
    for (p1, p2, p3) in triangles {
        draw_triangle(p1, p2, p3, color);
    }

    // Draw Outline
    let outline_mesh = geometry::generate_ribbon_mesh(points, 1.5, colors::WHITE, true);
    draw_mesh(&outline_mesh);
}
