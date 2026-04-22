pub mod gravity;
pub mod pressure;
pub mod surface;
pub mod vector;

use crate::graph::point::Point;
use crate::graph::vertex::VertexKey;
use crate::graph::Graph;
use crate::physics::vector::GraphVector;
use macroquad::math::Vec2;
use slotmap::SecondaryMap;

const FRICTION: f32 = 0.9;

pub fn advance_frame(graph: &mut Graph) {
    let mut force = GraphVector::new();

    pressure::update_force(graph, &mut force);
    surface::update_force(graph, &mut force);
    gravity::update_force(graph, &mut force);

    let vertex_accels = solve_vertex(graph, &force);
    let accels = solve_edge(graph, &force, &vertex_accels);

    accelerate(graph, &accels);
    graph.update_cache();
}

fn solve_vertex(graph: &Graph, force: &GraphVector) -> SecondaryMap<VertexKey, Vec2> {
    let mut fbafs = SecondaryMap::new();
    for (key, vertex) in graph.vertices.iter() {
        let mut a = force.get_vertex(key);
        for edge_key in key.edge_keys() {
            let edge = vertex.edge(edge_key);
            a -= (4.0 / 3.0) * force.get_edge(edge_key) - (2.0 / 3.0) * force.get_edge(edge.twin);
        }
        fbafs.insert(key, a);
    }

    let mut vertex_accels = SecondaryMap::new();
    for (key, vertex) in graph.vertices.iter() {
        let mut a = fbafs[key] * 5.1;
        for edge in vertex.edges.iter() {
            a -= 0.4 * fbafs[edge.twin.vertex];
        }
        vertex_accels.insert(key, a);
    }
    vertex_accels
}

fn solve_edge(
    graph: &Graph,
    force: &GraphVector,
    vertex_accels: &SecondaryMap<VertexKey, Vec2>,
) -> GraphVector {
    let mut accels = GraphVector::new();
    for (key, vertex) in graph.vertices.iter() {
        let vertex_accel = vertex_accels[key];
        accels.add_vertex(key, vertex_accel);
        for ekey in key.edge_keys() {
            let edge = vertex.edge(ekey);
            let twin_vertex_accel = vertex_accels[edge.twin.vertex];

            accels.add_edge(
                ekey,
                (80.0 / 3.0) * force.get_edge(ekey)
                    - 20.0 * force.get_edge(edge.twin)
                    - (4.0 / 3.0) * vertex_accel
                    + (2.0 / 3.0) * twin_vertex_accel,
            );
        }
    }
    accels
}

fn accelerate(graph: &mut Graph, accels: &GraphVector) {
    for (key, vertex) in graph.vertices.iter_mut() {
        accelerate_point(&mut vertex.point, accels.get_vertex(key));
        for (offset, edge) in vertex.edges.iter_mut().enumerate() {
            let ekey = key.edge_key(offset as u8);
            accelerate_point(&mut edge.point, accels.get_edge(ekey));
        }
    }
}

fn accelerate_point(point: &mut Point, accel: Vec2) {
    point.velocity = point.velocity * FRICTION + accel;
    point.position += point.velocity;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advance_frame() {
        let mut graph = Graph::new();
        graph.init(
            crate::graph::bubble::BubbleStyle::Standard {
                size: 1,
                color: crate::graphics::colors::TURQUOISE,
            },
            crate::graph::bubble::BubbleStyle::Standard {
                size: 1,
                color: crate::graphics::colors::ROSE,
            },
        );
        for _ in 0..5 {
            advance_frame(&mut graph);
            println!("{}", graph.dump_state());
        }
    }
}
