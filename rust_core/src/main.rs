use macroquad::prelude::*;
use rust_core::graph::Graph;
use rust_core::graphics::Renderer;
use rust_core::physics;
use rust_core::managers::spawn::{SpawnManager, RatchetSpawnTimer};
use rust_core::managers::slide::SlideManager;
use rust_core::graphics::colors;

#[macroquad::main("PipPop")]
async fn main() {
    // Seed the random number generator using the current system time
    rand::srand(miniquad::date::now() as u64);

    let mut graph = Graph::new();
    graph.init();

    let mut renderer = Renderer::new();
    let mut last_physics_time = get_time();

    let mut spawn_manager = SpawnManager::new(
        colors::get_group(6),
        Box::new(RatchetSpawnTimer {
            starting_wait: 2.0,
            doubling_time: 120.0,
        }),
    );
    let mut slide_manager = SlideManager::new();

    loop {
        let dt = get_frame_time();
        
        // Physics update (fixed timestep approx 60fps)
        if get_time() - last_physics_time > 0.016 {
            physics::advance_frame(&mut graph);
            last_physics_time = get_time();
        }

        // Managers update
        spawn_manager.update(dt);
        spawn_manager.possibly_spawn(&mut graph);
        slide_manager.slide_slidable_edges(&mut graph, dt);

        // Animation update
        renderer.update(dt);

        clear_background(BLACK);

        // Set up camera to match Android's "Bubble Space" (Y-up)
        let aspect = screen_width() / screen_height();
        let camera = Camera2D {
            target: vec2(0.0, 0.0),
            zoom: vec2(1.0 / 300.0, -aspect / 300.0),
            ..Default::default()
        };

        // Rendering
        renderer.draw(&graph, &camera);

        next_frame().await
    }
}
