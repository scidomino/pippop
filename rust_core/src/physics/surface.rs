use super::vector::GraphVector;
use crate::graph::Graph;
use crate::graphics::geometry::Bezier;
use macroquad::math::Vec2;

const SURFACE_TENSION: f32 = 0.3;

// 3 point Legendre Gauss Integrator
const LEGENDRE_GAUSS_POINTS: [(f32, f32); 3] = [
    (0.277_777_8, 0.887_298_35),
    (0.444_444_45, 0.5),
    (0.277_777_8, 0.112_701_66),
];

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

fn vertex_surface_force(bez: &Bezier) -> Vec2 {
    integrate_surface_force(bez, |p| -3.0 * (1.0 - p).powi(2))
}

fn edge_surface_force(bez: &Bezier) -> Vec2 {
    integrate_surface_force(bez, |p| 3.0 * (1.0 - p) * (1.0 - 3.0 * p))
}
