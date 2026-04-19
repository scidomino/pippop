use macroquad::math::Vec2;

const FLATNESS: f32 = 0.5;
const MAX_DEPTH: u32 = 10;

/// Flatten a cubic Bezier curve into a sequence of points.
pub fn flatten_bezier(points: &mut Vec<Vec2>, p1: Vec2, p2: Vec2, p3: Vec2, p4: Vec2) {
    flatten_bezier_recursive(points, p1, p2, p3, p4, 0);
}

fn flatten_bezier_recursive(points: &mut Vec<Vec2>, p1: Vec2, p2: Vec2, p3: Vec2, p4: Vec2, depth: u32) {
    let dx = p4.x - p1.x;
    let dy = p4.y - p1.y;
    let d2 = ((p2.x - p4.x) * dy - (p2.y - p4.y) * dx).abs();
    let d3 = ((p3.x - p4.x) * dy - (p3.y - p4.y) * dx).abs();

    if depth >= MAX_DEPTH || (d2 + d3) * (d2 + d3) <= FLATNESS * (dx * dx + dy * dy) {
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

    flatten_bezier_recursive(points, p1, p12, p123, p1234, depth + 1);
    flatten_bezier_recursive(points, p1234, p234, p34, p4, depth + 1);
}

pub fn rotate_points(points: &[Vec2], pivot: Vec2, angle: f32) -> Vec<Vec2> {
    let (sin, cos) = angle.sin_cos();
    points.iter().map(|p| {
        let dx = p.x - pivot.x;
        let dy = p.y - pivot.y;
        Vec2::new(
            cos * dx - sin * dy + pivot.x,
            sin * dx + cos * dy + pivot.y,
        )
    }).collect()
}

pub fn tween_points(a: &[Vec2], b: &[Vec2], progress: f32) -> Vec<Vec2> {
    if a.len() == b.len() {
        return a.iter().zip(b.iter()).map(|(&p1, &p2)| p1.lerp(p2, progress)).collect();
    }

    // If lengths differ, we scale the indices as in the Android implementation
    let (big, small, morph) = if a.len() > b.len() {
        (a, b, progress)
    } else {
        (b, a, 1.0 - progress)
    };

    let big_count = big.len();
    let small_count = small.len();
    let ratio = big_count as f32 / small_count as f32;
    let mut out = Vec::with_capacity(big_count);

    for i in 0..big_count {
        let small_idx = (i as f32 / ratio) as usize;
        let p1 = big[i];
        let p2 = small[small_idx.min(small_count - 1)];
        out.push(p1.lerp(p2, morph));
    }
    
    out
}

use crate::graph::Graph;
use crate::graph::bubble::BubbleKey;

pub fn calculate_centroid(graph: &Graph, bkey: BubbleKey) -> Vec2 {
    let bubble = &graph.bubbles[bkey];
    if bubble.edges.is_empty() {
        return Vec2::ZERO;
    }

    let mut area = 0.0;
    let mut centroid_x = 0.0;
    let mut centroid_y = 0.0;

    for &ekey in &bubble.edges {
        let (edge, vertex) = graph.get_edge_and_vertex(ekey);
        let (twin, twin_vertex) = graph.get_edge_and_vertex(edge.twin);

        let s = vertex.point.position;
        let sc = edge.point.position;
        let ec = twin.point.position;
        let e = twin_vertex.point.position;

        let half_area = (s.x * (-10.0 * s.y - 6.0 * sc.y - 3.0 * ec.y - e.y)
            + sc.x * (6.0 * s.y - 3.0 * ec.y - 3.0 * e.y))
            / 20.0;
            
        let twin_half_area = (e.x * (-10.0 * e.y - 6.0 * ec.y - 3.0 * sc.y - s.y)
            + ec.x * (6.0 * e.y - 3.0 * sc.y - 3.0 * s.y))
            / 20.0;

        area += half_area - twin_half_area;

        let half_cx = calculate_half_partial_centroid(s.x, s.y, sc.x, sc.y, ec.x, ec.y, e.x, e.y);
        let twin_half_cx = calculate_half_partial_centroid(e.x, e.y, ec.x, ec.y, sc.x, sc.y, s.x, s.y);
        centroid_x += half_cx - twin_half_cx;

        let half_cy = calculate_half_partial_centroid(s.y, s.x, sc.y, sc.x, ec.y, ec.x, e.y, e.x);
        let twin_half_cy = calculate_half_partial_centroid(e.y, e.x, ec.y, ec.x, sc.y, sc.x, s.y, s.x);
        centroid_y += half_cy - twin_half_cy;
    }

    if area.abs() < 1e-6 {
        let (first_edge, first_vertex) = graph.get_edge_and_vertex(bubble.edges[0]);
        let (first_twin, first_twin_vertex) = graph.get_edge_and_vertex(first_edge.twin);
        
        let s = first_vertex.point.position;
        let e = first_twin_vertex.point.position;
        let sc = first_edge.point.position;
        let ec = first_twin.point.position;
        
        return Vec2::new(
            ((s.x + e.x) + 3.0 * (sc.x + ec.x)) / 8.0,
            ((s.y + e.y) + 3.0 * (sc.y + ec.y)) / 8.0,
        );
    }

    Vec2::new(centroid_x / area as f32, -centroid_y / area as f32)
}

fn calculate_half_partial_centroid(sx: f32, sy: f32, scx: f32, scy: f32, ecx: f32, ecy: f32, ex: f32, ey: f32) -> f32 {
    (scx * ecx * (45.0 * sy + 27.0 * scy)
        + scx * ex * (12.0 * sy + 18.0 * scy)
        + sx * scx * (105.0 * sy - 45.0 * scy - 45.0 * ecy - 15.0 * ey)
        + sx * ecx * (30.0 * sy)
        + sx * ex * (5.0 * sy + 3.0 * scy)
        + scx * scx * (45.0 * sy - 27.0 * ecy - 18.0 * ey)
        + sx * sx * (-280.0 * sy - 105.0 * scy - 30.0 * ecy - 5.0 * ey))
        / 840.0
}
