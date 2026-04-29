use macroquad::prelude::*;
use rust_core::graph::Graph;
use rust_core::managers::game::{GameController, Interaction};
use rust_core::managers::title::TitleController;
use rust_core::resources::Resources;

enum Screen {
    Title(TitleController),
    Game(GameController),
}

impl Screen {
    fn handle_input(&mut self, interaction: Option<Interaction>) {
        match self {
            Screen::Title(_) => (),
            Screen::Game(c) => c.handle_input(interaction),
        }
    }

    fn update(&mut self, resources: &Resources, dt: f32) {
        let mut next_screen = None;

        match self {
            Screen::Title(c) => {
                if c.update(dt) {
                    next_screen = Some(Screen::Game(GameController::new(resources)));
                }
            }
            Screen::Game(c) => {
                c.update(dt);
            }
        }

        if let Some(new_screen) = next_screen {
            *self = new_screen;
        }
    }

    fn draw(&self, camera: &Camera2D) {
        match self {
            Screen::Title(c) => c.draw(camera),
            Screen::Game(c) => c.draw(camera),
        }
    }
}

#[macroquad::main("PipPop")]
async fn main() {
    env_logger::init();

    let resources = Resources::load().await;

    let mut screen = Screen::Title(TitleController::new(&resources));

    loop {
        let camera = get_camera();

        screen.handle_input(get_interaction(&camera));
        screen.update(&resources, get_frame_time());
        screen.draw(&camera);

        if is_key_pressed(KeyCode::D) {
            if let Screen::Game(c) = &screen {
                dump_graph(&c.graph);
            }
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
