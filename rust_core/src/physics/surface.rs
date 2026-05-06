use super::vector::GraphVector;
use crate::graph::Graph;
use crate::graphics::geometry::Bezier;
use macroquad::math::Vec2;

const SURFACE_TENSION: f32 = 0.3;

/// Weights and abscissae for 3-point Gauss-Legendre quadrature.
/// Used to numerically integrate the arc length of Bezier curves over the domain [0, 1].
const LEGENDRE_GAUSS_POINTS: [(f32, f32); 3] = [
    (0.277_777_8, 0.887_298_35),
    (0.444_444_45, 0.5),
    (0.277_777_8, 0.112_701_66),
];

/// Computes and applies surface tension forces to all vertices and control points.
///
/// Surface tension seeks to minimize the arc length of the bubble boundaries.
/// By taking the derivative of the arc length integral with respect to each
/// control point, we find the force vector that optimally shrinks the curve.
pub fn update_force(graph: &Graph, force: &mut GraphVector) {
    for key in graph.vertices.keys() {
        let mut vertex_force = Vec2::ZERO;

        for ekey in key.edge_keys() {
            let bezier = graph.vertices.get_bezier(ekey);

            vertex_force -= SURFACE_TENSION * vertex_surface_force(&bezier);
            force.add_edge(ekey, -SURFACE_TENSION * edge_surface_force(&bezier));
        }

        force.add_vertex(key, vertex_force);
    }
}

/// Evaluates the surface tension force integral for a specific control point.
///
/// This uses 3-point Gauss-Legendre quadrature to numerically integrate the
/// gradient of the arc length. `derivative` is the partial derivative of the
/// cubic Bezier polynomial with respect to the target control point.
fn integrate_surface_force(bez: &Bezier, derivative: impl Fn(f32) -> f32) -> Vec2 {
    let a = 3.0 * (bez.e - 3.0 * bez.ec + 3.0 * bez.sc - bez.s);
    let b = 6.0 * (bez.ec - 2.0 * bez.sc + bez.s);
    let c = 3.0 * (bez.sc - bez.s);

    LEGENDRE_GAUSS_POINTS
        .iter()
        .map(|&(w, p)| {
            let vel = c + p * (b + p * a);
            w * derivative(p) * vel.normalize_or_zero()
        })
        .sum()
}

/// Computes the surface tension force acting on the start vertex (`bez.s`).
fn vertex_surface_force(bez: &Bezier) -> Vec2 {
    integrate_surface_force(bez, |p| -3.0 * (1.0 - p).powi(2))
}

/// Computes the surface tension force acting on the first control point (`bez.sc`).
fn edge_surface_force(bez: &Bezier) -> Vec2 {
    integrate_surface_force(bez, |p| 3.0 * (1.0 - p) * (1.0 - 3.0 * p))
}
