// rust_core/src/physics.rs

use crate::geom::point::Point;
use crate::graph::Graph;

/// This is a faithful Rust translation of the `calculateHalfArea` method from the original
/// Java code in `Edge.java`.
pub fn calculate_half_area(s: &Point, sc: &Point, ec: &Point, e: &Point) -> f64 {
    let term1 = s.x * (-10.0 * s.y - 6.0 * sc.y - 3.0 * ec.y - e.y);
    let term2 = sc.x * (6.0 * s.y - 3.0 * ec.y - 3.0 * e.y);
    
    (term1 + term2) / 20.0
}

/// Calculates the total area of a single bubble by summing the area contributions of its
/// boundary edges. This replicates the logic from the original Java implementation.
pub fn calculate_bubble_area(graph: &Graph, bubble_id: usize) -> f64 {
    let bubble = &graph.bubbles[bubble_id];
    let first_edge_id = bubble.first_edge_id;
    let mut current_edge_id = first_edge_id;

    let mut total_area = 0.0;

    loop {
        let edge = &graph.edges[current_edge_id];
        let twin = &graph.edges[edge.twin_edge_id];

        // Get the 4 control points for the full Bezier curve of the current edge
        let p0 = &graph.vertices[edge.start_vertex_id].position;
        let p1 = &edge.control_point;
        let p2 = &twin.control_point;
        let p3 = &graph.vertices[twin.start_vertex_id].position;

        // The area of an edge is `halfArea - twin.halfArea`
        let half_area = calculate_half_area(p0, p1, p2, p3);

        // The twin's halfArea calculation uses the same 4 points, but in reverse order
        // for its own `calculateHalfArea` call.
        let twin_half_area = calculate_half_area(p3, p2, p1, p0);
        
        total_area += half_area - twin_half_area;

        current_edge_id = edge.next_edge_id;
        if current_edge_id == first_edge_id {
            break;
        }
    }

    total_area
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::point::Point;
    use crate::graph::Graph;

    #[test]
    fn test_half_area_calculation() {
        let p0 = Point { x: 0.0, y: 0.0 };
        let p1 = Point { x: 0.25, y: 1.0 };
        let p2 = Point { x: 0.75, y: 1.0 };
        let p3 = Point { x: 1.0, y: 0.0 };

        let expected_value = -0.0375;
        let actual_value = calculate_half_area(&p0, &p1, &p2, &p3);

        assert_eq!(actual_value, expected_value);
    }

    #[test]
    fn test_another_case() {
        let s = Point { x: 1.0, y: 2.0 };
        let sc = Point { x: 3.0, y: 4.0 };
        let ec = Point { x: 5.0, y: 6.0 };
        let e = Point { x: 7.0, y: 8.0 };

        let expected_value = -8.0;
        let actual_value = calculate_half_area(&s, &sc, &ec, &e);

        assert_eq!(actual_value, expected_value);
    }

    #[test]
    fn test_bubble_area() {
        let graph = Graph::new_test_graph();
        
        // --- Calculate area for Bubble 0 ---
        // The boundary is Edge 0. Its curve is defined by V0, C0, C1, V1
        // p0=(0,0), p1=(2.5, 5), p2=(7.5, -2), p3=(10,0)
        
        // Area of the full curve, from the PDF formula:
        // x0=0 -> 0
        // x1=2.5 -> 2.5 * (6*0 - 3*-2 - 3*0) = 2.5 * 6 = 15
        // x2=7.5 -> 7.5 * (3*0 + 3*5 - 6*0) = 7.5 * 15 = 112.5
        // x3=10 -> 10 * (1*0 + 3*5 + 6*-2 + 10*0) = 10 * (15 - 12) = 30
        // Total = (15 + 112.5 + 30) / 20 = 157.5 / 20 = 7.875.
        let expected_area0 = 7.875;
        let area0 = calculate_bubble_area(&graph, 0);
        assert!((area0 - expected_area0).abs() < 1e-9);

        // The area for bubble 1 uses the same edge, but in reverse.
        // The area should be the negative of bubble 0's area.
        let area1 = calculate_bubble_area(&graph, 1);
        assert!((area1 + expected_area0).abs() < 1e-9);
    }
}