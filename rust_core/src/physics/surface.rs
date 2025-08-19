use super::vector::GraphVector;
use crate::graph::edge::Edge;
use crate::graph::point::Coordinate;
use crate::graph::vertex::Vertex;
use crate::graph::RelationManager;

const SURFACE_TENSION: f32 = 0.3;

// 3 point Legendre Gauss Integrator
const LEGENDRE_GAUSS_POINTS: [(f32, f32); 3] = [
    (0.27777777777, 0.88729833462),
    (0.44444444444, 0.5),
    (0.27777777777, 0.11270166537),
];

pub fn update_force(relation_manager: &RelationManager, force: &mut GraphVector) {
    for (key, vertex) in relation_manager.vertecies.iter() {
        let mut vertex_force = Coordinate::default();

        for edge in vertex.edges.iter() {
            let (twin, twin_vertex) = relation_manager.get_edge_and_vertex(edge.twin);

            vertex_force.x += SURFACE_TENSION * vertex_x_force(vertex, edge, twin, twin_vertex);
            vertex_force.y += SURFACE_TENSION * vertex_y_force(vertex, edge, twin, twin_vertex);

            force.add_edge(
                edge.twin,
                Coordinate {
                    x: SURFACE_TENSION * edge_x_force(vertex, edge, twin, twin_vertex),
                    y: SURFACE_TENSION * edge_y_force(vertex, edge, twin, twin_vertex),
                },
            );
        }

        force.add_vertex(key, vertex_force);
    }
}

fn vertex_x_force(vertex: &Vertex, edge: &Edge, twin: &Edge, twin_vertex: &Vertex) -> f32 {
    vertex_force(
        vertex.point.position.x,
        vertex.point.position.y,
        edge.point.position.x,
        edge.point.position.y,
        twin.point.position.x,
        twin.point.position.y,
        twin_vertex.point.position.x,
        twin_vertex.point.position.y,
    )
}

fn vertex_y_force(vertex: &Vertex, edge: &Edge, twin: &Edge, twin_vertex: &Vertex) -> f32 {
    vertex_force(
        vertex.point.position.y,
        vertex.point.position.x,
        edge.point.position.y,
        edge.point.position.x,
        twin.point.position.y,
        twin.point.position.x,
        twin_vertex.point.position.y,
        twin_vertex.point.position.x,
    )
}

fn edge_x_force(vertex: &Vertex, edge: &Edge, twin: &Edge, twin_vertex: &Vertex) -> f32 {
    edge_force(
        vertex.point.position.x,
        vertex.point.position.y,
        edge.point.position.x,
        edge.point.position.y,
        twin.point.position.x,
        twin.point.position.y,
        twin_vertex.point.position.x,
        twin_vertex.point.position.y,
    )
}

fn edge_y_force(vertex: &Vertex, edge: &Edge, twin: &Edge, twin_vertex: &Vertex) -> f32 {
    edge_force(
        vertex.point.position.y,
        vertex.point.position.x,
        edge.point.position.y,
        edge.point.position.x,
        twin.point.position.y,
        twin.point.position.x,
        twin_vertex.point.position.y,
        twin_vertex.point.position.x,
    )
}

fn vertex_force(sx: f32, sy: f32, scx: f32, scy: f32, ecx: f32, ecy: f32, ex: f32, ey: f32) -> f32 {
    let ax = 3.0 * (ecx - scx) + sx - ex;
    let ay = 3.0 * (ecy - scy) + sy - ey;
    let bx = 2.0 * (ex - 2.0 * ecx + scx);
    let by = 2.0 * (ey - 2.0 * ecy + scy);
    let cx = ecx - ex;
    let cy = ecy - ey;

    let mut force = 0.0;

    for (w, p) in LEGENDRE_GAUSS_POINTS.iter() {
        let b_x_dp = cx + p * (bx + p * ax);
        let b_y_dp = cy + p * (by + p * ay);

        let hypot = b_x_dp.hypot(b_y_dp);
        if hypot == 0.0 {
            continue;
        }
        let b_y_dp_s_x = 3.0 * p * p; // the only difference from edge force
        force += w * b_y_dp_s_x * b_x_dp / hypot;
    }
    force
}

fn edge_force(sx: f32, sy: f32, scx: f32, scy: f32, ecx: f32, ecy: f32, ex: f32, ey: f32) -> f32 {
    let ax = 3.0 * (ecx - scx) + sx - ex;
    let ay = 3.0 * (ecy - scy) + sy - ey;
    let bx = 2.0 * (ex - 2.0 * ecx + scx);
    let by = 2.0 * (ey - 2.0 * ecy + scy);
    let cx = ecx - ex;
    let cy = ecy - ey;

    let mut force = 0.0;

    for (w, p) in LEGENDRE_GAUSS_POINTS.iter() {
        let b_x_dp = cx + p * (bx + p * ax);
        let b_y_dp = cy + p * (by + p * ay);

        let hypot = b_x_dp.hypot(b_y_dp);
        if hypot == 0.0 {
            continue;
        }
        let b_y_dp_sc_x = p * (6.0 - 9.0 * p); // the only difference from vertex force
        force += w * b_y_dp_sc_x * b_x_dp / hypot;
    }
    force
}
