use macroquad::prelude::*;
use rust::game::controller::GameController;
use rust::game::state::{Interaction, InteractionState};
use rust::game::title::TitleController;
use rust::resources::Resources;

enum Screen {
    Title(TitleController),
    Game(GameController),
}

impl Screen {
    fn interact(&mut self, resources: &Resources, interaction: Interaction) {
        let mut next_screen = None;
        match self {
            Screen::Title(c) => {
                if c.interact(interaction) {
                    next_screen = Some(Screen::Game(GameController::new()));
                }
            }
            Screen::Game(c) => {
                if c.interact(resources, interaction) {
                    next_screen = Some(Screen::Title(TitleController::new()));
                }
            }
        }
        if let Some(new_screen) = next_screen {
            *self = new_screen;
        }
    }

    fn update(&mut self, resources: &Resources, dt: f32) {
        match self {
            Screen::Title(c) => c.update(resources, dt),
            Screen::Game(c) => c.update(resources, dt),
        }
    }

    fn draw(&self, resources: &Resources, camera: &Camera2D) {
        match self {
            Screen::Title(c) => c.draw(resources, camera),
            Screen::Game(c) => c.draw(resources, camera),
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "PipPop".to_owned(),
        high_dpi: true,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    env_logger::init();

    let resources = Resources::load().await;

    let mut screen = Screen::Title(TitleController::new());

    loop {
        let camera = get_camera();

        screen.interact(&resources, get_interaction(&camera));
        screen.update(&resources, get_frame_time());
        screen.draw(&resources, &camera);

        next_frame().await
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
        char_pressed: get_char_pressed(),
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
