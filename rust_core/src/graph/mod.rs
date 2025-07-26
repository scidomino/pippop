// rust_core/src/graph/mod.rs

pub mod vertex;
pub mod edge;
pub mod bubble;

use crate::geom::point::Point;
use vertex::Vertex;
use edge::Edge;
use bubble::Bubble;

#[derive(Debug, Default)]
pub struct Graph {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
    pub bubbles: Vec<Bubble>,
}

impl Graph {
    /// Creates a new, empty graph.
    pub fn new() -> Self {
        Graph::default()
    }

    /// Creates a simple test graph with two bubbles sharing an edge.
    /// The graph looks like this: V0 -- E0/E1 -- V1
    /// Bubble 0 is "above" the edge, Bubble 1 is "below".
    pub fn new_test_graph() -> Self {
        let mut graph = Graph::new();

        // Bubbles
        graph.bubbles.push(Bubble { first_edge_id: 0, pressure: 1.0 }); // Bubble 0
        graph.bubbles.push(Bubble { first_edge_id: 1, pressure: 2.0 }); // Bubble 1

        // Vertices
        graph.vertices.push(Vertex { position: Point { x: 0.0, y: 0.0 }, edge_id: 0 }); // V0
        graph.vertices.push(Vertex { position: Point { x: 10.0, y: 0.0 }, edge_id: 1 });// V1

        // Edges
        graph.edges.push(Edge { // Edge 0
            start_vertex_id: 0,
            control_point: Point { x: 2.5, y: 5.0 }, // Asymmetric control point
            twin_edge_id: 1,
            next_edge_id: 0, // Points to itself for a simple 1-edge bubble
            bubble_id: 0,
        });
        graph.edges.push(Edge { // Edge 1 (twin of Edge 0)
            start_vertex_id: 1,
            control_point: Point { x: 7.5, y: -2.0 }, // Asymmetric control point
            twin_edge_id: 0,
            next_edge_id: 1, // Points to itself
            bubble_id: 1,
        });

        graph
    }
}
