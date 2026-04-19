use macroquad::prelude::*;

const FLATNESS: f32 = 0.5;
const MAX_DEPTH: u32 = 10;

/// Flatten a cubic Bezier curve into a sequence of points.
pub fn flatten_bezier(points: &mut Vec<Vec2>, p1: Vec2, p2: Vec2, p3: Vec2, p4: Vec2) {
    flatten_bezier_recursive(points, p1, p2, p3, p4, 0);
}

fn flatten_bezier_recursive(
    points: &mut Vec<Vec2>,
    p1: Vec2,
    p2: Vec2,
    p3: Vec2,
    p4: Vec2,
    depth: u32,
) {
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
    let mat = Mat2::from_angle(angle);
    points.iter().map(|&p| mat * (p - pivot) + pivot).collect()
}

pub fn tween_points(a: &[Vec2], b: &[Vec2], progress: f32) -> Vec<Vec2> {
    if a.len() == b.len() {
        return a
            .iter()
            .zip(b.iter())
            .map(|(&p1, &p2)| p1.lerp(p2, progress))
            .collect();
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

struct MiterPoint {
    center: Vec2,
    normal: Vec2,
    length: f32,
}

fn calculate_miter(points: &[Vec2], i: usize, width: f32, closed: bool) -> MiterPoint {
    let p = points[i];

    let prev = if i == 0 {
        if closed {
            points[points.len() - 1]
        } else {
            p - (points[1] - p)
        }
    } else {
        points[i - 1]
    };
    let next = if i == points.len() - 1 {
        if closed {
            points[0]
        } else {
            p + (p - points[i - 1])
        }
    } else {
        points[i + 1]
    };

    let mut t1 = (p - prev).normalize_or_zero();
    let mut t2 = (next - p).normalize_or_zero();

    // Fallback for coincident points
    if t1.length_squared() == 0.0 {
        t1 = t2;
    }
    if t2.length_squared() == 0.0 {
        t2 = t1;
    }
    if t1.length_squared() == 0.0 {
        t1 = Vec2::X;
        t2 = Vec2::X;
    }

    let n1 = t1.perp();
    let n2 = t2.perp();

    let mut miter_normal = (n1 + n2).normalize_or_zero();
    let mut dot = miter_normal.dot(n1);

    // Fallback if normals are exactly opposite
    if dot < 0.1 {
        miter_normal = n1;
        dot = 1.0;
    }

    // Limit the miter length to avoid huge spikes at very sharp angles
    let length = (width * 0.5 / dot).min(width * 4.0);

    MiterPoint {
        center: p,
        normal: miter_normal,
        length,
    }
}

/// Generates a ribbon mesh (thick line) from a sequence of points.
pub fn generate_ribbon_mesh(points: &[Vec2], width: f32, color: Color, closed: bool) -> Mesh {
    let mut vertices = Vec::with_capacity(points.len() * 2);
    let mut indices = Vec::with_capacity(points.len() * 6);

    for i in 0..points.len() {
        let miter = calculate_miter(points, i, width, closed);
        let p1 = miter.center + miter.normal * miter.length;
        let p2 = miter.center - miter.normal * miter.length;

        vertices.push(Vertex::new2(vec3(p1.x, p1.y, 0.0), vec2(0.0, 0.0), color));
        vertices.push(Vertex::new2(vec3(p2.x, p2.y, 0.0), vec2(0.0, 0.0), color));

        if i < points.len() - 1 {
            let base = (i * 2) as u16;
            indices.extend_from_slice(&[base, base + 1, base + 2, base + 2, base + 1, base + 3]);
        }
    }

    if closed && points.len() > 1 {
        let base = ((points.len() - 1) * 2) as u16;
        indices.extend_from_slice(&[base, base + 1, 0, 0, base + 1, 1]);
    }

    Mesh {
        vertices,
        indices,
        texture: None,
    }
}

/// Generates a glow mesh (thick line with alpha falloff) from a sequence of points.
pub fn generate_glow_mesh(points: &[Vec2], width: f32, color: Color, closed: bool) -> Mesh {
    let mut vertices = Vec::with_capacity(points.len() * 3);
    let mut indices = Vec::with_capacity(points.len() * 12);

    let inner_color = color;
    let outer_color = Color::new(color.r, color.g, color.b, 0.0);

    for i in 0..points.len() {
        let miter = calculate_miter(points, i, width, closed);
        let p_outer1 = miter.center + miter.normal * miter.length;
        let p_inner = miter.center;
        let p_outer2 = miter.center - miter.normal * miter.length;

        vertices.push(Vertex::new2(
            vec3(p_outer1.x, p_outer1.y, 0.0),
            vec2(0.0, 0.0),
            outer_color,
        ));
        vertices.push(Vertex::new2(
            vec3(p_inner.x, p_inner.y, 0.0),
            vec2(0.0, 0.0),
            inner_color,
        ));
        vertices.push(Vertex::new2(
            vec3(p_outer2.x, p_outer2.y, 0.0),
            vec2(0.0, 0.0),
            outer_color,
        ));

        if i < points.len() - 1 {
            let base = (i * 3) as u16;
            // First strip (outer1 to inner)
            indices.extend_from_slice(&[base, base + 1, base + 3, base + 3, base + 1, base + 4]);
            // Second strip (inner to outer2)
            indices.extend_from_slice(&[
                base + 1,
                base + 2,
                base + 4,
                base + 4,
                base + 2,
                base + 5,
            ]);
        }
    }

    if closed && points.len() > 1 {
        let base = ((points.len() - 1) * 3) as u16;
        indices.extend_from_slice(&[
            base,
            base + 1,
            0,
            0,
            base + 1,
            1,
            base + 1,
            base + 2,
            1,
            1,
            base + 2,
            2,
        ]);
    } else if !closed && points.len() > 1 {
        let cap_segments = 10;

        // Start Cap
        let p_start = points[0];
        let t_start = (points[1] - points[0]).normalize_or_zero();
        let n_start = Vec2::new(-t_start.y, t_start.x);
        let start_angle = n_start.y.atan2(n_start.x);

        let center_idx = 1 as u16;
        let mut prev_idx = 0 as u16; // outer1 at i=0

        for s in 1..cap_segments {
            let a = start_angle + (s as f32 / cap_segments as f32) * std::f32::consts::PI;
            let pos = p_start + Vec2::new(a.cos(), a.sin()) * (width * 0.5);
            let new_idx = vertices.len() as u16;
            vertices.push(Vertex::new2(
                vec3(pos.x, pos.y, 0.0),
                vec2(0.0, 0.0),
                outer_color,
            ));

            indices.extend_from_slice(&[center_idx, prev_idx, new_idx]);
            prev_idx = new_idx;
        }
        let outer2_idx = 2 as u16;
        indices.extend_from_slice(&[center_idx, prev_idx, outer2_idx]);

        // End Cap
        let last_i = points.len() - 1;
        let p_end = points[last_i];
        let t_end = (p_end - points[last_i - 1]).normalize_or_zero();
        let n_end = Vec2::new(-t_end.y, t_end.x);
        let end_angle = n_end.y.atan2(n_end.x);

        let base = (last_i * 3) as u16;
        let center_idx = base + 1;
        let mut prev_idx = base + 0; // outer1

        for s in 1..cap_segments {
            let a = end_angle - (s as f32 / cap_segments as f32) * std::f32::consts::PI;
            let pos = p_end + Vec2::new(a.cos(), a.sin()) * (width * 0.5);
            let new_idx = vertices.len() as u16;
            vertices.push(Vertex::new2(
                vec3(pos.x, pos.y, 0.0),
                vec2(0.0, 0.0),
                outer_color,
            ));

            indices.extend_from_slice(&[center_idx, new_idx, prev_idx]);
            prev_idx = new_idx;
        }
        let outer2_idx = base + 2;
        indices.extend_from_slice(&[center_idx, outer2_idx, prev_idx]);
    }

    Mesh {
        vertices,
        indices,
        texture: None,
    }
}

use crate::graph::bubble::BubbleKey;
use crate::graph::Graph;

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
        let twin_half_cx =
            calculate_half_partial_centroid(e.x, e.y, ec.x, ec.y, sc.x, sc.y, s.x, s.y);
        centroid_x += half_cx - twin_half_cx;

        let half_cy = calculate_half_partial_centroid(s.y, s.x, sc.y, sc.x, ec.y, ec.x, e.y, e.x);
        let twin_half_cy =
            calculate_half_partial_centroid(e.y, e.x, ec.y, ec.x, sc.y, sc.x, s.y, s.x);
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

fn calculate_half_partial_centroid(
    sx: f32,
    sy: f32,
    scx: f32,
    scy: f32,
    ecx: f32,
    ecy: f32,
    ex: f32,
    ey: f32,
) -> f32 {
    (scx * ecx * (45.0 * sy + 27.0 * scy)
        + scx * ex * (12.0 * sy + 18.0 * scy)
        + sx * scx * (105.0 * sy - 45.0 * scy - 45.0 * ecy - 15.0 * ey)
        + sx * ecx * (30.0 * sy)
        + sx * ex * (5.0 * sy + 3.0 * scy)
        + scx * scx * (45.0 * sy - 27.0 * ecy - 18.0 * ey)
        + sx * sx * (-280.0 * sy - 105.0 * scy - 30.0 * ecy - 5.0 * ey))
        / 840.0
}

pub fn triangulate(points: &[Vec2]) -> Vec<(Vec2, Vec2, Vec2)> {
    let mut clean_points: Vec<Vec2> = Vec::with_capacity(points.len());
    for &p in points {
        if clean_points.is_empty() || clean_points.last().unwrap().distance_squared(p) > 0.01 {
            clean_points.push(p);
        }
    }
    if clean_points.len() > 1
        && clean_points[0].distance_squared(*clean_points.last().unwrap()) <= 0.01
    {
        clean_points.pop();
    }

    if clean_points.len() < 3 {
        return vec![];
    }

    let mut indices: Vec<usize> = (0..clean_points.len()).collect();
    let mut triangles = Vec::with_capacity(clean_points.len() - 2);

    // Ensure the polygon is counter-clockwise (CCW)
    let mut area = 0.0;
    for i in 0..clean_points.len() {
        let p1 = clean_points[i];
        let p2 = clean_points[(i + 1) % clean_points.len()];
        area += p1.x * p2.y - p2.x * p1.y;
    }

    if area < 0.0 {
        indices.reverse();
    }

    let mut i = 0;
    let mut attempts = 0;

    // Standard Ear Clipping algorithm
    while indices.len() > 3 && attempts < indices.len() * 2 {
        let n = indices.len();
        let prev_idx = indices[(i + n - 1) % n];
        let curr_idx = indices[i];
        let next_idx = indices[(i + 1) % n];

        let p_prev = clean_points[prev_idx];
        let p_curr = clean_points[curr_idx];
        let p_next = clean_points[next_idx];

        let v1 = p_curr - p_prev;
        let v2 = p_next - p_curr;
        // Cross product logic for CCW polygons
        let cross = v1.perp_dot(v2);

        if cross > 0.0 {
            let mut is_ear = true;
            for j in 0..n {
                let test_idx = indices[j];
                if test_idx == prev_idx || test_idx == curr_idx || test_idx == next_idx {
                    continue;
                }
                if point_in_triangle(clean_points[test_idx], p_prev, p_curr, p_next) {
                    is_ear = false;
                    break;
                }
            }

            if is_ear {
                triangles.push((p_prev, p_curr, p_next));
                indices.remove(i);
                attempts = 0;
                if i >= indices.len() {
                    i = 0;
                }
                continue;
            }
        }

        i = (i + 1) % indices.len();
        attempts += 1;
    }

    if indices.len() == 3 {
        triangles.push((
            clean_points[indices[0]],
            clean_points[indices[1]],
            clean_points[indices[2]],
        ));
    } else {
        // Fallback for self-intersecting or highly degenerate edge cases
        let base = clean_points[indices[0]];
        for j in 1..indices.len() - 1 {
            triangles.push((base, clean_points[indices[j]], clean_points[indices[j + 1]]));
        }
    }

    triangles
}

fn point_in_triangle(p: Vec2, a: Vec2, b: Vec2, c: Vec2) -> bool {
    let v0 = c - a;
    let v1 = b - a;
    let v2 = p - a;

    let dot00 = v0.dot(v0);
    let dot01 = v0.dot(v1);
    let dot02 = v0.dot(v2);
    let dot11 = v1.dot(v1);
    let dot12 = v1.dot(v2);

    let denom = dot00 * dot11 - dot01 * dot01;
    if denom.abs() < 1e-6 {
        return false;
    }
    let inv_denom = 1.0 / denom;
    let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
    let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

    (u >= 0.0) && (v >= 0.0) && (u + v <= 1.0)
}
