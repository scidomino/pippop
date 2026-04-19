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
    for (key, vertex) in graph.vertices.iter() {
        let mut vertex_force = Vec2::ZERO;

        for (offset, edge) in vertex.edges.iter().enumerate() {
            let ekey = key.edge_key(offset as u8);
            let (twin, twin_vertex) = graph.get_edge_and_vertex(edge.twin);

            let s = vertex.point.position;
            let sc = edge.point.position;
            let ec = twin.point.position;
            let e = twin_vertex.point.position;

            vertex_force -= SURFACE_TENSION * vertex_surface_force(s, sc, ec, e);
            force.add_edge(ekey, -SURFACE_TENSION * edge_surface_force(s, sc, ec, e));
        }

        force.add_vertex(key, vertex_force);
    }
}

fn vertex_surface_force(s: Vec2, sc: Vec2, ec: Vec2, e: Vec2) -> Vec2 {
    let a = 3.0 * (e - 3.0 * ec + 3.0 * sc - s);
    let b = 6.0 * (ec - 2.0 * sc + s);
    let c = 3.0 * (sc - s);

    let mut force = Vec2::ZERO;
    for (w, p) in LEGENDRE_GAUSS_POINTS.iter() {
        let vel = c + *p * (b + *p * a);
        let inv_t = 1.0 - p;
        let der_v = -3.0 * inv_t * inv_t;
        force += *w * der_v * vel.normalize_or_zero();
    }
    force
}

fn edge_surface_force(s: Vec2, sc: Vec2, ec: Vec2, e: Vec2) -> Vec2 {
    let a = 3.0 * (e - 3.0 * ec + 3.0 * sc - s);
    let b = 6.0 * (ec - 2.0 * sc + s);
    let c = 3.0 * (sc - s);

    let mut force = Vec2::ZERO;
    for (w, p) in LEGENDRE_GAUSS_POINTS.iter() {
        let vel = c + *p * (b + *p * a);
        let inv_t = 1.0 - p;
        let der_sc = 3.0 * inv_t * (1.0 - 3.0 * p);
        force += *w * der_sc * vel.normalize_or_zero();
    }
    force
}
