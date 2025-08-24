pub mod pressure;
pub mod surface;
pub mod vector;

use crate::graph::point::{Coordinate, Point};
use crate::graph::vertex::VertexKey;
use crate::graph::Graph;
use crate::physics::vector::GraphVector;
use slotmap::SecondaryMap;

const FRICTION: f32 = 0.9;

pub fn advance_frame(graph: &mut Graph) {
    let mut force = GraphVector::new();

    pressure::update_force(graph, &mut force);
    surface::update_force(graph, &mut force);

    let vertex_accels = solve_vertex(graph, &force);
    let accels = solve_edge(graph, &force, &vertex_accels);

    accelerate(graph, &accels);
}

fn solve_vertex(graph: &Graph, force: &GraphVector) -> SecondaryMap<VertexKey, Coordinate> {
    let mut fbafs = SecondaryMap::new();
    for (key, vertex) in graph.vertecies.iter() {
        let mut a = force.get_vertex(key).clone();
        for edge_key in key.edge_keys() {
            let edge = vertex.edge(edge_key);

            let edge_force = force.get_edge(edge_key);
            let twin_edge_force = force.get_edge(edge.twin);

            a.x += (-4.0 / 3.0) * edge_force.x + (2.0 / 3.0) * twin_edge_force.x;
            a.y += (-4.0 / 3.0) * edge_force.y + (2.0 / 3.0) * twin_edge_force.y;

            fbafs.insert(key, a);
        }
    }

    let mut vertex_accels = SecondaryMap::new();
    for (key, vertex) in graph.vertecies.iter() {
        let mut a = fbafs[key].clone();
        a.x *= 5.4;
        a.y *= 5.4;
        for edge in vertex.edges.iter() {
            let twin_vertex_force = fbafs[edge.twin.vertex];
            a.x -= 0.4 * twin_vertex_force.x;
            a.y -= 0.4 * twin_vertex_force.y;
        }
        vertex_accels.insert(key, a);
    }
    vertex_accels
}

fn solve_edge(
    graph: &Graph,
    force: &GraphVector,
    vertex_accels: &SecondaryMap<VertexKey, Coordinate>,
) -> GraphVector {
    let mut accels = GraphVector::new();
    for (key, vertex) in graph.vertecies.iter() {
        let vertex_accel = vertex_accels[key];
        accels.add_vertex(key, vertex_accel);
        for edge_key in key.edge_keys() {
            let edge = vertex.edge(edge_key);

            let edge_force = force.get_edge(edge_key);
            let twin_edge_force = force.get_edge(edge.twin);
            let twin_vertex_accel = vertex_accels[edge.twin.vertex];

            accels.add_edge(
                edge_key,
                Coordinate {
                    x: (80.0 / 3.0) * edge_force.x
                        - 20.0 * twin_edge_force.x
                        - (4.0 / 3.0) * vertex_accel.x
                        + (2.0 / 3.0) * twin_vertex_accel.x,
                    y: (80.0 / 3.0) * edge_force.y
                        - 20.0 * twin_edge_force.y
                        - (4.0 / 3.0) * vertex_accel.y
                        + (2.0 / 3.0) * twin_vertex_accel.y,
                },
            );
        }
    }
    accels
}

fn accelerate(graph: &mut Graph, accels: &GraphVector) {
    for (key, vertex) in graph.vertecies.iter_mut() {
        accelerate_point(&mut vertex.point, accels.get_vertex(key));
        for edge_key in key.edge_keys() {
            let edge = &mut vertex.edge(edge_key);
            accelerate_point(&mut edge.point, accels.get_edge(edge_key));
        }
    }
}

fn accelerate_point(point: &mut Point, accel: Coordinate) {
    point.velocity.x = FRICTION * point.velocity.x + accel.x;
    point.velocity.y = FRICTION * point.velocity.y + accel.y;

    point.position.x += point.velocity.x;
    point.position.y += point.velocity.y;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advance_frame() {
        let mut graph = Graph::new();
        graph.init();
        for _ in 0..5 {
            advance_frame(&mut graph);
            graph.print_graph();
        }
    }
}
