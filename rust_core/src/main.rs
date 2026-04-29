use macroquad::prelude::*;
use rust_core::graph::Graph;
use rust_core::managers::game::{GameController, Interaction};
use rust_core::resources::Resources;

#[macroquad::main("PipPop")]
async fn main() {
    env_logger::init();

    // Seed the random number generator using the current system time
    rand::srand(miniquad::date::now() as u64);

    let resources = Resources::load().await;

    let mut controller = GameController::new(&resources);

    loop {
        let camera = get_camera();
        controller.handle_input(get_interaction(&camera));
        controller.update(get_frame_time());
        controller.draw(&camera);
        if is_key_pressed(KeyCode::D) {
            dump_graph(&controller.graph);
        }

        next_frame().await
    }
}

fn dump_graph(graph: &Graph) {
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

fn get_interaction(camera: &Camera2D) -> Option<Interaction> {
    if is_mouse_button_down(MouseButton::Left) {
        let (x, y) = mouse_position();
        Some(Interaction {
            position: camera.screen_to_world(vec2(x, y)),
            is_clicked: false,
        })
    } else if is_mouse_button_released(MouseButton::Left) {
        let (x, y) = mouse_position();
        Some(Interaction {
            position: camera.screen_to_world(vec2(x, y)),
            is_clicked: true,
        })
    } else {
        None
    }
}

fn get_camera() -> Camera2D {
    let aspect = screen_width() / screen_height();
    Camera2D {
        target: vec2(0.0, 0.0),
        zoom: vec2(1.0 / 300.0, aspect / 300.0),
        ..Default::default()
    }
}
