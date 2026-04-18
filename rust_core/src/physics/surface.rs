use super::vector::GraphVector;
use crate::graph::edge::Edge;
use crate::graph::point::Coordinate;
use crate::graph::vertex::Vertex;
use crate::graph::Graph;

const SURFACE_TENSION: f32 = 0.3;

// 3 point Legendre Gauss Integrator
const LEGENDRE_GAUSS_POINTS: [(f32, f32); 3] = [
    (0.277_777_8, 0.887_298_35),
    (0.444_444_45, 0.5),
    (0.277_777_8, 0.112_701_66),
];

pub fn update_force(graph: &Graph, force: &mut GraphVector) {
    for (key, vertex) in graph.vertices.iter() {
        let mut vertex_force = Coordinate::default();

        for (offset, edge) in vertex.edges.iter().enumerate() {
            let edge_key = key.edge_key(offset as u8);
            let (twin, twin_vertex) = graph.get_edge_and_vertex(edge.twin);

            vertex_force.x -= SURFACE_TENSION * vertex_x_force(vertex, edge, twin, twin_vertex);
            vertex_force.y -= SURFACE_TENSION * vertex_y_force(vertex, edge, twin, twin_vertex);

            force.add_edge(
                edge_key,
                Coordinate {
                    x: -SURFACE_TENSION * edge_x_force(vertex, edge, twin, twin_vertex),
                    y: -SURFACE_TENSION * edge_y_force(vertex, edge, twin, twin_vertex),
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
    let ax = 3.0 * (ex - 3.0 * ecx + 3.0 * scx - sx);
    let ay = 3.0 * (ey - 3.0 * ecy + 3.0 * scy - sy);
    let bx = 6.0 * (ecx - 2.0 * scx + sx);
    let by = 6.0 * (ecy - 2.0 * scy + sy);
    let cx = 3.0 * (scx - sx);
    let cy = 3.0 * (scy - sy);

    let mut force = 0.0;

    for (w, p) in LEGENDRE_GAUSS_POINTS.iter() {
        let b_x_dp = cx + p * (bx + p * ax);
        let b_y_dp = cy + p * (by + p * ay);

        let hypot = b_x_dp.hypot(b_y_dp);
        if hypot == 0.0 {
            continue;
        }
        let inv_t = 1.0 - p;
        let der_v = -3.0 * inv_t * inv_t;
        force += w * der_v * b_x_dp / hypot;
    }
    force
}

fn edge_force(sx: f32, sy: f32, scx: f32, scy: f32, ecx: f32, ecy: f32, ex: f32, ey: f32) -> f32 {
    let ax = 3.0 * (ex - 3.0 * ecx + 3.0 * scx - sx);
    let ay = 3.0 * (ey - 3.0 * ecy + 3.0 * scy - sy);
    let bx = 6.0 * (ecx - 2.0 * scx + sx);
    let by = 6.0 * (ecy - 2.0 * scy + sy);
    let cx = 3.0 * (scx - sx);
    let cy = 3.0 * (scy - sy);

    let mut force = 0.0;

    for (w, p) in LEGENDRE_GAUSS_POINTS.iter() {
        let b_x_dp = cx + p * (bx + p * ax);
        let b_y_dp = cy + p * (by + p * ay);

        let hypot = b_x_dp.hypot(b_y_dp);
        if hypot == 0.0 {
            continue;
        }
        let inv_t = 1.0 - p;
        let der_sc = 3.0 * inv_t * (1.0 - 3.0 * p);
        force += w * der_sc * b_x_dp / hypot;
    }
    force
}
