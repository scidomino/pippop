use macroquad::prelude::*;
use rust_core::graph::Graph;
use rust_core::physics;
use rust_core::graph::point::Coordinate;

#[macroquad::main("PipPop")]
async fn main() {
    let mut graph = Graph::new();
    graph.init();

    let mut last_physics_time = get_time();

    loop {
        // Physics update
        if get_time() - last_physics_time > 0.016 {
            physics::advance_frame(&mut graph);
            last_physics_time = get_time();
        }

        clear_background(BLACK);

        // Set up camera to match Android's "Bubble Space" (Y-up)
        let aspect = screen_width() / screen_height();
        let camera = Camera2D {
            target: vec2(0.0, 0.0),
            zoom: vec2(1.0 / 600.0, -aspect / 600.0),
            ..Default::default()
        };
        set_camera(&camera);

        // Rendering
        for (_bkey, bubble) in graph.bubbles.iter() {
            if bubble.open_air {
                continue;
            }

            for &ekey in &bubble.edges {
                let (edge, vertex) = graph.get_edge_and_vertex(ekey);
                let (twin_edge, twin_vertex) = graph.get_edge_and_vertex(edge.twin);

                draw_cubic_bezier(
                    vertex.point.position,
                    edge.point.position,
                    twin_edge.point.position,
                    twin_vertex.point.position,
                    2.0,
                    SKYBLUE,
                );
            }
        }

        // Reset camera for any UI (not that we have any yet)
        set_default_camera();

        next_frame().await
    }
}

fn draw_cubic_bezier(p0: Coordinate, p1: Coordinate, p2: Coordinate, p3: Coordinate, thickness: f32, color: Color) {
    let mut last_p = Vec2::new(p0.x, p0.y);
    let steps = 20;
    for i in 1..=steps {
        let t = i as f32 / steps as f32;
        let next_p = sample_cubic_bezier(p0, p1, p2, p3, t);
        draw_line(last_p.x, last_p.y, next_p.x, next_p.y, thickness, color);
        last_p = next_p;
    }
}

fn sample_cubic_bezier(p0: Coordinate, p1: Coordinate, p2: Coordinate, p3: Coordinate, t: f32) -> Vec2 {
    let inv_t = 1.0 - t;
    let b0 = inv_t * inv_t * inv_t;
    let b1 = 3.0 * t * inv_t * inv_t;
    let b2 = 3.0 * t * t * inv_t;
    let b3 = t * t * t;

    Vec2::new(
        b0 * p0.x + b1 * p1.x + b2 * p2.x + b3 * p3.x,
        b0 * p0.y + b1 * p1.y + b2 * p2.y + b3 * p3.y,
    )
}
