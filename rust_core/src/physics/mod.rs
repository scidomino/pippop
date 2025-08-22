pub mod pressure;
pub mod surface;
pub mod vector;

use crate::graph::point::Coordinate;
use crate::graph::vertex::VertexKey;
use crate::graph::Graph;
use crate::physics::vector::GraphVector;
use slotmap::SecondaryMap;

pub fn update_force(graph: &Graph) {
    let mut force = GraphVector::new();

    pressure::update_force(graph, &mut force);
    surface::update_force(graph, &mut force);

    let mut vert_accel = solve_vertex(graph, &force);
}

fn solve_vertex(graph: &Graph, force: &GraphVector) -> SecondaryMap<VertexKey, Coordinate> {
    let mut fbaf = SecondaryMap::new();
    for (key, vertex) in graph.vertecies.iter() {
        let mut a = force.get_vertex(key).clone();
        for edge_key in key.edge_keys() {
            let edge = vertex.edge(edge_key);

            let edge_force = force.get_edge(edge_key);
            let twin_edge_force = force.get_edge(edge.twin);

            a.x += (-4.0 / 3.0) * edge_force.x + (2.0 / 3.0) * twin_edge_force.x;
            a.y += (-4.0 / 3.0) * edge_force.y + (2.0 / 3.0) * twin_edge_force.y;

            fbaf.insert(key, a);
        }
    }

    let mut accel = SecondaryMap::new();
    for (key, vertex) in graph.vertecies.iter() {
        let mut a = fbaf[key].clone();
        a.x *= 5.4;
        a.y *= 5.4;
        for edge in vertex.edges.iter() {
            let twin_vertex_force = fbaf[edge.twin.vertex]
            a.x -= 0.4 * twin_vertex_force.x;
            a.y -= 0.4 * twin_vertex_force.y;
        }
        accel.insert(key, a);
    }
    accel
}
