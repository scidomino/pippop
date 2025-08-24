use super::vector::GraphVector;
use crate::graph::bubble::BubbleKey;
use crate::graph::edge::Edge;
use crate::graph::point::Coordinate;
use crate::graph::vertex::Vertex;
use crate::graph::Graph;
use slotmap::SecondaryMap;

const PRESSURE_TENSION: f32 = 0.04;

pub fn update_force(graph: &Graph, force: &mut GraphVector) {
    let bubble_to_pressure = get_bubble_to_pressure(graph);
    for (key, vertex) in graph.vertecies.iter() {
        let mut vertex_force = Coordinate::default();

        for edge in vertex.edges.iter() {
            let (twin, twin_vertex) = graph.get_edge_and_vertex(edge.twin);

            let pressure_diff = bubble_to_pressure[twin.bubble] - bubble_to_pressure[edge.bubble];
            let pressure_diff = pressure_diff.clamp(-2.0, 2.0);
            let pressure = pressure_diff * PRESSURE_TENSION;

            vertex_force.x += pressure * vertex_x_force(vertex, edge, twin, twin_vertex);
            vertex_force.y += pressure * vertex_y_force(vertex, edge, twin, twin_vertex);

            force.add_edge(
                edge.twin,
                Coordinate {
                    x: pressure * edge_x_force(vertex, twin, twin_vertex),
                    y: pressure * edge_y_force(vertex, twin, twin_vertex),
                },
            );
        }

        force.add_vertex(key, vertex_force);
    }
}

fn vertex_x_force(vertex: &Vertex, edge: &Edge, twin: &Edge, twin_vertex: &Vertex) -> f32 {
    vertex_force(
        vertex.point.position.y,
        edge.point.position.y,
        twin.point.position.y,
        twin_vertex.point.position.y,
    )
}

fn vertex_y_force(vertex: &Vertex, edge: &Edge, twin: &Edge, twin_vertex: &Vertex) -> f32 {
    vertex_force(
        vertex.point.position.x,
        edge.point.position.x,
        twin.point.position.x,
        twin_vertex.point.position.x,
    )
}

fn edge_x_force(vertex: &Vertex, twin: &Edge, twin_vertex: &Vertex) -> f32 {
    edge_force(
        vertex.point.position.y,
        twin.point.position.y,
        twin_vertex.point.position.y,
    )
}

fn edge_y_force(vertex: &Vertex, twin: &Edge, twin_vertex: &Vertex) -> f32 {
    edge_force(
        vertex.point.position.x,
        twin.point.position.x,
        twin_vertex.point.position.x,
    )
}

fn vertex_force(s: f32, sc: f32, ec: f32, e: f32) -> f32 {
    (-10.0 * s - 6.0 * sc - 3.0 * ec - e) / 20.0
}

fn edge_force(s: f32, ec: f32, e: f32) -> f32 {
    (6.0 * s - 3.0 * ec - 3.0 * e) / 20.0
}

fn get_bubble_to_pressure(graph: &Graph) -> SecondaryMap<BubbleKey, f32> {
    let mut bubble_to_area: SecondaryMap<BubbleKey, f32> = SecondaryMap::new();
    for (key, _) in graph.bubbles.iter() {
        bubble_to_area.insert(key, 0.0);
    }
    for vertex in graph.vertecies.values() {
        for edge in vertex.edges.iter() {
            let (twin, twin_vertex) = graph.get_edge_and_vertex(edge.twin);
            let half_area = get_half_area(vertex, edge, twin_vertex, twin);
            bubble_to_area[edge.bubble] += half_area;
            bubble_to_area[twin.bubble] -= half_area;
        }
    }

    let mut bubble_to_pressure: SecondaryMap<BubbleKey, f32> = SecondaryMap::new();
    for (key, bubble) in graph.bubbles.iter() {
        bubble_to_pressure.insert(key, bubble.get_pressure(bubble_to_area[key]));
    }
    bubble_to_pressure
}

fn get_half_area(vertex: &Vertex, edge: &Edge, twin_vertex: &Vertex, twin: &Edge) -> f32 {
    let s = vertex.point.position;
    let sc = edge.point.position;
    let ec = twin.point.position;
    let e = twin_vertex.point.position;

    // calculate half the area of the bezier curve defined by the points
    return (s.x * (-10.0 * s.y - 6.0 * sc.y - 3.0 * ec.y - e.y)
        + sc.x * (6.0 * s.y - 3.0 * ec.y - 3.0 * e.y))
        / 20.0;
}
