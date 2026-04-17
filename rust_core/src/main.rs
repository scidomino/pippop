use macroquad::prelude::*;
use rust_core::graph::Graph;
use rust_core::physics;
use rust_core::graphics::Renderer;

#[macroquad::main("PipPop")]
async fn main() {
    let mut graph = Graph::new();
    graph.init();

    let mut renderer = Renderer::new();
    let mut last_physics_time = get_time();
    let mut last_spawn_time = get_time();

    loop {
        let dt = get_frame_time();

        // Physics update (fixed timestep approx 60fps)
        if get_time() - last_physics_time > 0.016 {
            physics::advance_frame(&mut graph);
            last_physics_time = get_time();
        }

        // Periodic spawn every 3 seconds
        if get_time() - last_spawn_time > 3.0 {
            let open_air_vertices = graph.get_open_air_vertices();
            if !open_air_vertices.is_empty() {
                let vkey = open_air_vertices[macroquad::rand::gen_range(0, open_air_vertices.len())];
                let color = rust_core::graphics::colors::random_game_color();
                graph.spawn(vkey, rust_core::graph::bubble::BubbleStyle::Standard {
                    size: 1,
                    max_size: 5,
                    color,
                });
            }
            last_spawn_time = get_time();
        }

        // Animation update
        renderer.update(dt);

        clear_background(BLACK);

        // Set up camera to match Android's "Bubble Space" (Y-up)
        let aspect = screen_width() / screen_height();
        let camera = Camera2D {
            target: vec2(0.0, 0.0),
            zoom: vec2(1.0 / 600.0, -aspect / 600.0),
            ..Default::default()
        };

        // Rendering
        renderer.draw(&graph, &camera);

        // Example: Add a rising point on mouse click
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            // Convert screen to world space
            let world_pos = camera.screen_to_world(vec2(mouse_pos.0, mouse_pos.1));
            renderer.effects.add_rising_points(
                "+100".to_string(), 
                rust_core::graph::point::Coordinate::new(world_pos.x, world_pos.y)
            );
        }

        next_frame().await
    }
}
