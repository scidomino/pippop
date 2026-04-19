use macroquad::prelude::*;
use rust_core::graph::Graph;
use rust_core::graphics::Renderer;
use rust_core::physics;
use rust_core::managers::spawn::{SpawnManager, RatchetSpawnTimer};
use rust_core::managers::slide::SlideManager;
use rust_core::managers::burst::BurstManager;
use rust_core::managers::swap::SwapManager;
use rust_core::graphics::colors;

#[macroquad::main("PipPop")]
async fn main() {
    // Seed the random number generator using the current system time
    rand::srand(miniquad::date::now() as u64);

    let mut graph = Graph::new();
    graph.init(
        rust_core::graph::bubble::BubbleStyle::Player,
        rust_core::graph::bubble::BubbleStyle::Standard { size: 1, max_size: 5, color: colors::TURQUOISE }
    );

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
    let burst_manager = BurstManager::new(1);
    let mut swap_manager = SwapManager::new();

    loop {
        let dt = get_frame_time();

        // Set up camera to match Android's "Bubble Space" (Y-up)
        let aspect = screen_width() / screen_height();
        let camera = Camera2D {
            target: vec2(0.0, 0.0),
            zoom: vec2(1.0 / 300.0, -aspect / 300.0),
            ..Default::default()
        };

        // Input handling
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let world_pos = camera.screen_to_world(vec2(mouse_pos.0, mouse_pos.1));
            swap_manager.otter_swap(&mut graph, world_pos);
        }
        
        // Physics update (fixed timestep approx 60fps)
        if get_time() - last_physics_time > 0.016 {
            physics::advance_frame(&mut graph);
            last_physics_time = get_time();
        }

        // Managers update
        spawn_manager.update(dt);
        spawn_manager.possibly_spawn(&mut graph);
        slide_manager.slide_slidable_edges(&mut graph, &burst_manager, dt);
        swap_manager.update(&mut graph, dt);

        // Animation update
        renderer.update(dt);

        clear_background(BLACK);

        // Rendering
        renderer.draw(&graph, &camera, &swap_manager);

        draw_text(&format!("FPS: {:03}", get_fps()), 10.0, 30.0, 30.0, WHITE);

        next_frame().await
    }
}
