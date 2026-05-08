use macroquad::prelude::*;
use std::f32::consts::PI;

const FLATNESS: f32 = 0.5;
const MAX_DEPTH: u32 = 10;

pub struct Bezier {
    pub x: Vec4,
    pub y: Vec4,
}

impl Bezier {
    pub fn from_points(s: Vec2, sc: Vec2, ec: Vec2, e: Vec2) -> Self {
        Self {
            x: Vec4::new(s.x, sc.x, ec.x, e.x),
            y: Vec4::new(s.y, sc.y, ec.y, e.y),
        }
    }

    /// Calculates area contribution from the start vertex and first control point only.
    /// In the half-edge graph, (edge.half_area - twin_edge.half_area) completes
    /// the total area integral for the Bezier curve.
    pub fn half_area(&self) -> f32 {
        const AREA_ROW0: Vec4 = Vec4::new(-10.0, -6.0, -3.0, -1.0);
        const AREA_ROW1: Vec4 = Vec4::new(6.0, 0.0, -3.0, -3.0);
        (self.x.x * self.y.dot(AREA_ROW0) + self.x.y * self.y.dot(AREA_ROW1)) / 20.0
    }

    pub fn centroid_contribution(&self) -> Vec2 {
        let x_rev = Vec4::new(self.x.w, self.x.z, self.x.y, self.x.x);
        let y_rev = Vec4::new(self.y.w, self.y.z, self.y.y, self.y.x);

        let c_start = Self::calculate_half_partial_centroid(self.x, self.y);
        let c_end = Self::calculate_half_partial_centroid(x_rev, y_rev);
        let c = c_start - c_end;
        Vec2::new(c.x, -c.y)
    }

    pub fn flatten(&self, points: &mut Vec<Vec2>) {
        let s = vec2(self.x.x, self.y.x);
        let sc = vec2(self.x.y, self.y.y);
        let ec = vec2(self.x.z, self.y.z);
        let e = vec2(self.x.w, self.y.w);
        Self::flatten_bezier_recursive(points, s, sc, ec, e, 0);
    }

    fn flatten_bezier_recursive(
        points: &mut Vec<Vec2>,
        p1: Vec2,
        p2: Vec2,
        p3: Vec2,
        p4: Vec2,
        depth: u32,
    ) {
        let d = p4 - p1;
        let d2 = (p2 - p4).perp_dot(d).abs();
        let d3 = (p3 - p4).perp_dot(d).abs();

        if depth >= MAX_DEPTH || (d2 + d3) * (d2 + d3) <= FLATNESS * d.length_squared() {
            points.push(p1);
            return;
        }

        let p12 = p1.midpoint(p2);
        let p23 = p2.midpoint(p3);
        let p34 = p3.midpoint(p4);
        let p123 = p12.midpoint(p23);
        let p234 = p23.midpoint(p34);
        let p1234 = p123.midpoint(p234);

        Self::flatten_bezier_recursive(points, p1, p12, p123, p1234, depth + 1);
        Self::flatten_bezier_recursive(points, p1234, p234, p34, p4, depth + 1);
    }

    fn calculate_half_partial_centroid(x: Vec4, y: Vec4) -> Vec2 {
        const M0: Mat4 = Mat4::from_cols(
            Vec4::new(-280.0, -105.0, -30.0, -5.0),
            Vec4::new(105.0, -45.0, -45.0, -15.0),
            Vec4::new(30.0, 0.0, 0.0, 0.0),
            Vec4::new(5.0, 3.0, 0.0, 0.0),
        );

        const M1: Mat4 = Mat4::from_cols(
            Vec4::new(0.0, 0.0, 0.0, 0.0),
            Vec4::new(45.0, 0.0, -27.0, -18.0),
            Vec4::new(45.0, 27.0, 0.0, 0.0),
            Vec4::new(12.0, 18.0, 0.0, 0.0),
        );

        let v_x = (M0 * x) * x.x + (M1 * x) * x.y;
        let v_y = (M0 * y) * y.x + (M1 * y) * y.y;

        vec2(v_x.dot(y), v_y.dot(x)) / 840.0
    }
}

pub fn tween_points(a: &[Vec2], b: &[Vec2], progress: f32) -> Vec<Vec2> {
    if a.len() == b.len() {
        return a
            .iter()
            .zip(b)
            .map(|(&p1, &p2)| p1.lerp(p2, progress))
            .collect();
    }

    let (big, small, morph) = if a.len() > b.len() {
        (a, b, progress)
    } else {
        (b, a, 1.0 - progress)
    };

    let ratio = (small.len() - 1) as f32 / (big.len() - 1) as f32;
    big.iter()
        .enumerate()
        .map(|(i, &p1)| {
            let f = i as f32 * ratio;
            let i_low = f.floor() as usize;
            let i_high = (i_low + 1).min(small.len() - 1);
            let p2 = small[i_low].lerp(small[i_high], f.fract());
            p1.lerp(p2, morph)
        })
        .collect()
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

        let center_idx = 1_u16;
        let mut prev_idx = 0_u16; // outer1 at i=0

        for s in 1..cap_segments {
            let a = start_angle + (s as f32 / cap_segments as f32) * PI;
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
        let outer2_idx = 2_u16;
        indices.extend_from_slice(&[center_idx, prev_idx, outer2_idx]);

        // End Cap
        let last_i = points.len() - 1;
        let p_end = points[last_i];
        let t_end = (p_end - points[last_i - 1]).normalize_or_zero();
        let n_end = Vec2::new(-t_end.y, t_end.x);
        let end_angle = n_end.y.atan2(n_end.x);

        let base = (last_i * 3) as u16;
        let center_idx = base + 1;
        let mut prev_idx = base; // outer1

        for s in 1..cap_segments {
            let a = end_angle - (s as f32 / cap_segments as f32) * PI;
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
        area += p1.perp_dot(p2);
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
            let is_ear = !indices.iter().any(|&test_idx| {
                test_idx != prev_idx
                    && test_idx != curr_idx
                    && test_idx != next_idx
                    && point_in_triangle(clean_points[test_idx], p_prev, p_curr, p_next)
            });

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
    let d1 = (p - b).perp_dot(a - b);
    let d2 = (p - c).perp_dot(b - c);
    let d3 = (p - a).perp_dot(c - a);

    let has_neg = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0);
    let has_pos = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);

    !(has_neg && has_pos)
}
