// rust_core/src/physics/area.rs

use crate::graph::{Graph, BubbleKey};

/// Calculates the value for a single edge's `half_area` field.
fn calculate_single_half_area(graph: &Graph, edge_key: crate::graph::EdgeKey) -> f64 {
    let edge = &graph.edges[edge_key];
    let twin = &graph.edges[edge.twin_edge_key];
    let p0 = &graph.vertices[edge.start_vertex_key].position;
    let p1 = &edge.control_point;
    let p2 = &twin.control_point;
    let p3 = &graph.vertices[twin.start_vertex_key].position;
    
    let term1 = p0.x * (-10.0 * p0.y - 6.0 * p1.y - 3.0 * p2.y - p3.y);
    let term2 = p1.x * (6.0 * p0.y - 3.0 * p2.y - 3.0 * p3.y);
    
    (term1 + term2) / 20.0
}

/// Iterates through all edges in the graph and updates their `half_area` field.
/// This should be called once per frame, before any physics calculations that need area.
pub fn update_edge_half_areas(graph: &mut Graph) {
    let edge_keys: Vec<_> = graph.edges.keys().collect();
    for edge_key in edge_keys {
        let half_area = calculate_single_half_area(graph, edge_key);
        graph.edges[edge_key].half_area = half_area;
    }
}

/// Calculates the total area of a single bubble by summing the pre-calculated area
/// contributions of its boundary edges.
pub fn calculate_bubble_area(graph: &Graph, bubble_key: BubbleKey) -> f64 {
    let bubble = &graph.bubbles[bubble_key];
    let first_edge_key = bubble.first_edge_key;
    let mut current_edge_key = first_edge_key;

    let mut total_area = 0.0;

    loop {
        let edge = &graph.edges[current_edge_key];
        let twin = &graph.edges[edge.twin_edge_key];

        // The area contribution of an edge is its half_area minus its twin's.
        total_area += edge.half_area - twin.half_area;

        current_edge_key = edge.next_edge_key;
        if current_edge_key == first_edge_key {
            break;
        }
    }

    total_area
}