use crate::graph::bubble::{Bubble, BubbleKey, BubbleStyle};
use crate::graph::edge::EdgeKey;
use crate::graph::Graph;
use crate::graphics::colors;
use crate::graphics::geometry;
use crate::resources::Resources;
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

pub fn build_bubble_mesh(points: &[Vec2], color: Color) -> Mesh {
    // Calculate Bounding Box for UV mapping
    let mut min_p = vec2(f32::MAX, f32::MAX);
    let mut max_p = vec2(f32::MIN, f32::MIN);
    for &p in points {
        min_p = min_p.min(p);
        max_p = max_p.max(p);
    }
    let size = (max_p - min_p).max(vec2(1.0, 1.0));

    // Build Mesh (Ear Clipping Triangulation)
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    for (i, &(p1, p2, p3)) in geometry::triangulate(points).iter().enumerate() {
        let base = (i * 3) as u16;

        let uv1 = (p1 - min_p) / size;
        let uv2 = (p2 - min_p) / size;
        let uv3 = (p3 - min_p) / size;

        vertices.push(Vertex::new2(vec3(p1.x, p1.y, 0.0), uv1, color));
        vertices.push(Vertex::new2(vec3(p2.x, p2.y, 0.0), uv2, color));
        vertices.push(Vertex::new2(vec3(p3.x, p3.y, 0.0), uv3, color));
        indices.extend_from_slice(&[base, base + 1, base + 2]);
    }

    Mesh {
        vertices,
        indices,
        texture: None,
    }
}

pub fn draw_bubble(resources: &Resources, style: &BubbleStyle, points: &[Vec2], centroid: Vec2) {
    if points.is_empty() {
        return;
    }

    match style {
        BubbleStyle::Colored { color, .. } => {
            // Draw Body with Material
            gl_use_material(&resources.bubble_material);
            draw_mesh(&build_bubble_mesh(points, *color));
            gl_use_default_material();
        }
        BubbleStyle::Swappable { swaps_left, .. } => {
            // Draw Body as Solid Color
            draw_mesh(&build_bubble_mesh(points, colors::DARK_GRAY));

            // Draw Swaps left
            let text = format!("{swaps_left}");
            let text_dims = measure_text(&text, Some(&resources.font), 32, 1.0);
            draw_text_ex(
                &text,
                centroid.x - text_dims.width / 2.0,
                centroid.y + text_dims.height / 2.0,
                TextParams {
                    font: Some(&resources.font),
                    font_size: 32,
                    font_scale: 1.0,
                    color: colors::WHITE,
                    ..Default::default()
                },
            );
        }
        BubbleStyle::OpenAir | BubbleStyle::Invisible { .. } => (),
    }
}

pub fn draw_ctrl_points(graph: &Graph) {
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
