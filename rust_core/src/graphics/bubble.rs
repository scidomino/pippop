use crate::graph::bubble::{Bubble, BubbleStyle};
use crate::graph::Graph;
use crate::graphics::colors;
use crate::graphics::geometry;
use macroquad::prelude::*;

pub fn get_bubble_points(graph: &Graph, bkey: crate::graph::bubble::BubbleKey) -> Vec<Vec2> {
    get_points_for_bubble(graph, &graph.bubbles[bkey])
}

pub fn get_points_for_bubble(graph: &Graph, bubble: &Bubble) -> Vec<Vec2> {
    let mut points = Vec::with_capacity(bubble.edges.len() * 12);

    for &ekey in &bubble.edges {
        push_edge_points(graph, ekey, &mut points);
    }
    points
}

pub fn push_edge_points(graph: &Graph, ekey: crate::graph::edge::EdgeKey, points: &mut Vec<Vec2>) {
    graph.vertices.get_bezier(ekey).flatten(points);
}

pub fn draw_bubble_body(style: &BubbleStyle, points: &[Vec2], _centroid: Vec2) {
    if points.is_empty() {
        return;
    }

    let color = match style {
        BubbleStyle::Standard { color, .. } => *color,
        BubbleStyle::Player => colors::TRANSPARENT_WHITE,
        BubbleStyle::Popping { color, timer, .. } => {
            let alpha = (*timer / 0.5).clamp(0.0, 1.0);
            Color::new(color.r, color.g, color.b, alpha)
        }
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
