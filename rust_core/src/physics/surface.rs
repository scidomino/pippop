use super::vector::GraphVector;
use crate::graph::Graph;
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
            let bezier = graph.get_bezier(ekey);

            vertex_force -= SURFACE_TENSION * vertex_surface_force(&bezier);
            force.add_edge(ekey, -SURFACE_TENSION * edge_surface_force(&bezier));
        }

        force.add_vertex(key, vertex_force);
    }
}

fn vertex_surface_force(bez: &crate::graphics::geometry::Bezier) -> Vec2 {
    let a = 3.0 * (bez.e - 3.0 * bez.ec + 3.0 * bez.sc - bez.s);
    let b = 6.0 * (bez.ec - 2.0 * bez.sc + bez.s);
    let c = 3.0 * (bez.sc - bez.s);

    let mut force = Vec2::ZERO;
    for (w, p) in LEGENDRE_GAUSS_POINTS.iter() {
        let vel = c + *p * (b + *p * a);
        let inv_t = 1.0 - p;
        let der_v = -3.0 * inv_t * inv_t;
        force += *w * der_v * vel.normalize_or_zero();
    }
    force
}

fn edge_surface_force(bez: &crate::graphics::geometry::Bezier) -> Vec2 {
    let a = 3.0 * (bez.e - 3.0 * bez.ec + 3.0 * bez.sc - bez.s);
    let b = 6.0 * (bez.ec - 2.0 * bez.sc + bez.s);
    let c = 3.0 * (bez.sc - bez.s);

    let mut force = Vec2::ZERO;
    for (w, p) in LEGENDRE_GAUSS_POINTS.iter() {
        let vel = c + *p * (b + *p * a);
        let inv_t = 1.0 - p;
        let der_sc = 3.0 * inv_t * (1.0 - 3.0 * p);
        force += *w * der_sc * vel.normalize_or_zero();
    }
    force
}
