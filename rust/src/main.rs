use macroquad::prelude::*;
use rust::game::controller::GameController;
use rust::game::state::{Interaction, InteractionState};
use rust::game::title::TitleController;
use rust::graph::Graph;
use rust::resources::Resources;
use std::fs::File;
use std::io::Write;

enum Screen<'a> {
    Title(TitleController),
    Game(GameController<'a>),
}

impl<'a> Screen<'a> {
    fn interact(&mut self, resources: &'a Resources, interaction: Interaction) {
        let mut next_screen = None;
        match self {
            Screen::Title(c) => {
                if c.interact(interaction) {
                    next_screen = Some(Screen::Game(GameController::new(resources)));
                }
            }
            Screen::Game(c) => {
                if c.interact(interaction) {
                    next_screen = Some(Screen::Title(TitleController::new(resources)));
                }
            }
        }
        if let Some(new_screen) = next_screen {
            *self = new_screen;
        }
    }

    fn update(&mut self, dt: f32) {
        match self {
            Screen::Title(c) => c.update(dt),
            Screen::Game(c) => c.update(dt),
        }
    }

    fn draw(&self, camera: &Camera2D) {
        match self {
            Screen::Title(c) => c.draw(camera),
            Screen::Game(c) => c.draw(camera),
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "PipPop".to_owned(),
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    env_logger::init();

    let resources = Resources::load().await;

    let mut screen = Screen::Title(TitleController::new(&resources));

    loop {
        let camera = get_camera();

        screen.interact(&resources, get_interaction(&camera));
        screen.update(get_frame_time());
        screen.draw(&camera);

        if is_key_pressed(KeyCode::D) {
            if let Screen::Game(c) = &screen {
                dump_graph(&c.state.graph);
            }
        }

        next_frame().await
    }
}

fn dump_graph(graph: &Graph) {
    let dump = graph.dump_state();
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Ok(mut file) = File::create("graph_dump.txt") {
            let _ = file.write_all(dump.as_bytes());
            log::info!("Graph state dumped to graph_dump.txt");
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        log::info!("{}", dump);
    }
}

fn get_interaction(camera: &Camera2D) -> Interaction {
    let is_down = is_mouse_button_down(MouseButton::Left);
    let is_released = is_mouse_button_released(MouseButton::Left);
    let (x, y) = mouse_position();

    Interaction {
        position: camera.screen_to_world(vec2(x, y)),
        state: if is_released {
            InteractionState::Released
        } else if is_down {
            InteractionState::Pressed
        } else {
            InteractionState::Hover
        },
    }
}

fn get_camera() -> Camera2D {
    let width = screen_width();
    let height = screen_height();
    let aspect = width / height;

    let target_radius = 200.0;

    let (zoom_x, zoom_y) = if width < height {
        (1.0 / target_radius, aspect / target_radius)
    } else {
        (1.0 / (target_radius * aspect), 1.0 / target_radius)
    };

    Camera2D {
        target: vec2(0.0, 0.0),
        zoom: vec2(zoom_x, zoom_y),
        ..Default::default()
    }
}
