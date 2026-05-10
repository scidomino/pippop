use crate::graph::bubble::{Bubble, BubbleKey, BubbleStyle};
use crate::graph::edge::EdgeKey;
use crate::graph::Graph;
use crate::graphics::colors;
use crate::graphics::geometry;
use macroquad::prelude::*;

pub fn get_bubble_points(graph: &Graph, bkey: BubbleKey) -> Vec<Vec2> {
    get_points_for_bubble(graph, &graph.bubbles[bkey])
}

pub fn get_points_for_bubble(graph: &Graph, bubble: &Bubble) -> Vec<Vec2> {
    bubble
        .edges
        .iter()
        .flat_map(|&ekey| graph.vertices.get_edge(ekey).points.iter().copied())
        .collect()
}

pub fn push_edge_points(graph: &Graph, ekey: EdgeKey, points: &mut Vec<Vec2>) {
    points.extend_from_slice(&graph.vertices.get_edge(ekey).points);
}

pub fn draw_bubble(style: &BubbleStyle, points: &[Vec2], centroid: Vec2, font: &Font) {
    if points.is_empty() {
        return;
    }

    let color = match style {
        BubbleStyle::Colored { color, .. } => *color,
        BubbleStyle::Swappable { .. } => colors::TRANSPARENT,
        BubbleStyle::OpenAir | BubbleStyle::Invisible { .. } => return,
    };

    // Draw Fill (Ear Clipping Triangulation)
    // This perfectly handles concave shapes with zero overdraw, eliminating spikes.
    for &(p1, p2, p3) in geometry::triangulate(points).iter() {
        draw_triangle(p1, p2, p3, color);
    }

    // Draw Outline
    draw_mesh(&geometry::generate_ribbon_mesh(
        points,
        1.5,
        colors::WHITE,
        true,
    ));

    // Draw Label
    let label = match style {
        BubbleStyle::Swappable { swaps_left, .. } => format!("{swaps_left}"),
        _ => return,
    };

    let text_dims = measure_text(&label, Some(font), 64, 0.4);

    draw_text_ex(
        &label,
        centroid.x - text_dims.width / 2.0,
        centroid.y + text_dims.height / 2.0,
        TextParams {
            font: Some(font),
            font_size: 64,
            font_scale: 0.5,
            color: colors::WHITE,
            ..Default::default()
        },
    );
}

pub fn draw_debug_points(graph: &Graph) {
    for (_, vertex) in &graph.vertices {
        for edge in &vertex.edges {
            // Draw edge control point
            draw_circle(
                edge.point.position.x,
                edge.point.position.y,
                3.0,
                colors::YELLOW,
            );

            // Draw line from vertex to control point
            draw_line(
                vertex.point.position.x,
                vertex.point.position.y,
                edge.point.position.x,
                edge.point.position.y,
                0.5,
                Color::new(1.0, 1.0, 1.0, 0.3),
            );
        }
    }
}
