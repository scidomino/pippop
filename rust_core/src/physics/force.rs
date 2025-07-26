// rust_core/src/physics/force.rs

use crate::geom::point::Point;
use crate::graph::{Graph, EdgeKey};

// --- Legendre-Gauss Quadrature Constants ---
const LG_W1: f64 = 0.44444444444; // 4/9
const LG_W2: f64 = 0.27777777777; // 5/18
const LG_X1: f64 = 0.5;
const LG_X2: f64 = 0.11270166537;
const LG_X3: f64 = 0.88729833462;

/// Approximates the definite integral of a function from 0 to 1 using a 3-point
/// Legendre-Gauss Quadrature.
fn integrate(f: impl Fn(f64) -> f64) -> f64 {
    LG_W1 * f(LG_X1) + LG_W2 * (f(LG_X2) + f(LG_X3))
}

/// Calculates the integrand for the surface tension force component on a control point (p1).
fn tension_integrand(p: f64, p0: &Point, p1: &Point, p2: &Point, p3: &Point) -> f64 {
    let (sx, sy) = (p0.x, p0.y);
    let (scx, scy) = (p1.x, p1.y);
    let (ecx, ecy) = (p2.x, p2.y);
    let (ex, ey) = (p3.x, p3.y);

    let ax = 3.0 * (ecx - scx) + sx - ex;
    let ay = 3.0 * (ecy - scy) + sy - ey;
    let bx = 2.0 * (ex - 2.0 * ecx + scx);
    let by = 2.0 * (ey - 2.0 * ecy + scy);
    let cx = ecx - ex;
    let cy = ecy - ey;

    let bezier_x_dp = cx + p * (bx + p * ax);
    let bezier_y_dp = cy + p * (by + p * ay);

    let hypot = (bezier_x_dp * bezier_x_dp + bezier_y_dp * bezier_y_dp).sqrt();
    if hypot == 0.0 { return 0.0; }

    let bezier_y_dp_d_scy = p * (6.0 - 9.0 * p);
    bezier_y_dp_d_scy * bezier_x_dp / hypot
}

/// Calculates the surface tension force exerted on a single control point.
pub fn calculate_surface_tension_force(graph: &Graph, edge_key: EdgeKey, is_twin: bool) -> Point {
    let edge = &graph.edges[edge_key];
    let twin = &graph.edges[edge.twin_edge_key];

    let (p0, p1, p2, p3) = if !is_twin {
        (
            &graph.vertices[edge.start_vertex_key].position,
            &edge.control_point,
            &twin.control_point,
            &graph.vertices[twin.start_vertex_key].position,
        )
    } else {
        (
            &graph.vertices[twin.start_vertex_key].position,
            &twin.control_point,
            &edge.control_point,
            &graph.vertices[edge.start_vertex_key].position,
        )
    };

    let force_x = integrate(|p| {
        let p0_swapped = Point { x: p0.y, y: p0.x };
        let p1_swapped = Point { x: p1.y, y: p1.x };
        let p2_swapped = Point { x: p2.y, y: p2.x };
        let p3_swapped = Point { x: p3.y, y: p3.x };
        tension_integrand(p, &p0_swapped, &p1_swapped, &p2_swapped, &p3_swapped)
    });

    let force_y = integrate(|p| tension_integrand(p, p0, p1, p2, p3));

    Point { x: force_x, y: force_y }
}

/// Calculates the partial derivative of a Bezier curve's area.
pub fn get_area_derivative(p0: &Point, p1: &Point, p2: &Point, p3: &Point, control_point_index: usize) -> Point {
    match control_point_index {
        0 => Point { x: (-10.0*p0.y - 6.0*p1.y - 3.0*p2.y - p3.y)/20.0, y: (-10.0*p0.x + 6.0*p1.x + 3.0*p2.x + p3.x)/20.0 },
        1 => Point { x: (6.0*p0.y - 3.0*p2.y - 3.0*p3.y)/20.0, y: (-6.0*p0.x + 3.0*p2.x + 3.0*p3.x)/20.0 },
        2 => Point { x: (3.0*p0.y + 3.0*p1.y - 6.0*p3.y)/20.0, y: (-3.0*p0.x - 3.0*p1.x + 6.0*p3.x)/20.0 },
        3 => Point { x: (p0.y + 3.0*p1.y + 6.0*p2.y)/20.0, y: (-p0.x - 3.0*p1.x - 6.0*p2.x)/20.0 },
        _ => panic!("Invalid control_point_index"),
    }
}

/// Calculates the pressure force exerted on a single control point.
pub fn calculate_pressure_force(graph: &Graph, edge_key: EdgeKey, is_twin: bool) -> Point {
    let edge = &graph.edges[edge_key];
    let twin = &graph.edges[edge.twin_edge_key];
    let pressure_diff = graph.bubbles[edge.bubble_key].pressure - graph.bubbles[twin.bubble_key].pressure;
    let p0 = &graph.vertices[edge.start_vertex_key].position;
    let p1 = &edge.control_point;
    let p2 = &twin.control_point;
    let p3 = &graph.vertices[twin.start_vertex_key].position;
    let control_point_index = if is_twin { 2 } else { 1 };
    let area_derivative = get_area_derivative(p0, p1, p2, p3, control_point_index);
    Point { x: -pressure_diff * area_derivative.x, y: -pressure_diff * area_derivative.y }
}