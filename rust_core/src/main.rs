use macroquad::prelude::*;
use rust_core::graph::Graph;
use rust_core::graphics::colors;
use rust_core::graphics::Renderer;
use rust_core::managers::audio::AudioManager;
use rust_core::managers::game::GameController;
use rust_core::managers::spawn::{RatchetSpawnTimer, SpawnManager};
use rust_core::physics;

#[macroquad::main("PipPop")]
async fn main() {
    env_logger::init();

    // Seed the random number generator using the current system time
    rand::srand(miniquad::date::now() as u64);

    let mut graph = Graph::new();
    graph.init(
        rust_core::graph::bubble::BubbleStyle::Player,
        rust_core::graph::bubble::BubbleStyle::Standard {
            size: 1,
            color: colors::TURQUOISE,
        },
    );

    let mut renderer = Renderer::new();
    let mut last_physics_time = get_time();

    let spawn_manager = SpawnManager::new(
        colors::get_group(6),
        Box::new(RatchetSpawnTimer {
            starting_wait: 2.0,
            doubling_time: 120.0,
        }),
    );
    let audio_manager = AudioManager::new().await;
    let mut controller = GameController::new(spawn_manager, audio_manager);

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
        let mut point = None;
        let mut clicked = false;

        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = mouse_position();
            point = Some(camera.screen_to_world(vec2(mouse_pos.0, mouse_pos.1)));
        } else if is_mouse_button_released(MouseButton::Left) {
            let mouse_pos = mouse_position();
            point = Some(camera.screen_to_world(vec2(mouse_pos.0, mouse_pos.1)));
            clicked = true;
        }

        controller.handle_input(&mut graph, point, clicked);

        if is_key_pressed(KeyCode::D) {
            let dump = graph.dump_state();
            #[cfg(not(target_arch = "wasm32"))]
            {
                use std::io::Write;
                if let Ok(mut file) = std::fs::File::create("graph_dump.txt") {
                    let _ = file.write_all(dump.as_bytes());
                    log::info!("Graph state dumped to graph_dump.txt");
                }
            }
            #[cfg(target_arch = "wasm32")]
            {
                log::info!("{}", dump);
            }
        }

        // Physics update (fixed timestep approx 60fps)
        if get_time() - last_physics_time > 0.016 {
            physics::advance_frame(&mut graph);
            last_physics_time = get_time();
        }

        // Game Logic Update
        controller.update(&mut graph, dt);

        // Animation update
        renderer.update(dt);

        clear_background(BLACK);

        // Rendering
        renderer.draw(&graph, &camera, &controller);

        draw_text(&format!("FPS: {:03}", get_fps()), 10.0, 30.0, 30.0, WHITE);

        next_frame().await
    }
}
