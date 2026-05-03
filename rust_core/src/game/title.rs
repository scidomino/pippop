use crate::game::slide::SlideManager;
use crate::game::spawn::SpawnManager;
use crate::game::state::{GameState, Interaction, InteractionState};
use crate::game::world::WorldManager;
use crate::graph::bubble::BubbleStyle;
use crate::graph::Graph;
use crate::graphics::{colors, RenderContext};
use crate::resources::Resources;
use macroquad::prelude::*;

pub struct TitleController {
    pub state: GameState,
    pub font: Font,
    pub world_manager: WorldManager,
    pub spawn_manager: SpawnManager,
    pub slide_manager: SlideManager,
    pub timer: f32,
}

impl TitleController {
    pub fn new(resources: &Resources) -> Self {
        let graph = Graph::new(
            BubbleStyle::colored(colors::TURQUOISE),
            BubbleStyle::colored(colors::ROSE),
        );

        Self {
            state: GameState::new(graph),
            font: resources.font.clone(),
            world_manager: WorldManager::new(),
            spawn_manager: SpawnManager::new(colors::get_group(6)),
            slide_manager: SlideManager::new(),
            timer: 0.0,
        }
    }

    pub fn interact(&mut self, interaction: Interaction) -> bool {
        matches!(interaction.state, InteractionState::Pressed)
    }

    pub fn update(&mut self, dt: f32) {
        self.timer += dt;
        self.world_manager.update(&mut self.state, dt);
        self.spawn_manager.update(&mut self.state, dt);
        self.slide_manager.update(&mut self.state, dt);

        // Drain events so they don't accumulate
        self.state.events.clear();
    }

    pub fn draw(&self, camera: &Camera2D) {
        let ctx = RenderContext {
            graph: &self.state.graph,
            font: &self.font,
            camera,
        };
        self.world_manager.draw(&ctx);

        // --- Pass 2: Screen Space (UI) ---
        set_default_camera();

        let screen_width = screen_width();
        let screen_height = screen_height();

        // Dim the background graph
        draw_rectangle(
            0.0,
            0.0,
            screen_width,
            screen_height,
            Color::new(0.0, 0.0, 0.0, 0.7),
        );

        let screen_center_x = screen_width / 2.0;
        let screen_center_y = screen_height / 2.0;

        // Draw Title "PipPop"
        let title_text = "PipPop";
        let title_size = 120;
        let title_dims = measure_text(title_text, Some(&self.font), title_size, 1.0);

        // Shadow
        draw_text_ex(
            title_text,
            screen_center_x - title_dims.width / 2.0 - 4.0,
            screen_center_y - 40.0 + 4.0,
            TextParams {
                font: Some(&self.font),
                font_size: title_size,
                color: colors::WHITE,
                ..Default::default()
            },
        );
        // Foreground
        draw_text_ex(
            title_text,
            screen_center_x - title_dims.width / 2.0,
            screen_center_y - 40.0,
            TextParams {
                font: Some(&self.font),
                font_size: title_size,
                color: colors::TURQUOISE,
                ..Default::default()
            },
        );

        // Draw "Gioca!" text with pulsing animation
        let play_text = "Gioca!";
        let play_base_size = 60;
        let pulse = (self.timer * 3.0).sin() * 0.1 + 1.0; // 0.9 to 1.1 pulse
        let play_size = (play_base_size as f32 * pulse) as u16;

        let play_dims = measure_text(play_text, Some(&self.font), play_size, 1.0);

        // Shadow
        draw_text_ex(
            play_text,
            screen_center_x - play_dims.width / 2.0 - 2.0,
            screen_center_y + 80.0 + 2.0,
            TextParams {
                font: Some(&self.font),
                font_size: play_size,
                color: colors::WHITE,
                ..Default::default()
            },
        );
        // Foreground
        draw_text_ex(
            play_text,
            screen_center_x - play_dims.width / 2.0,
            screen_center_y + 80.0,
            TextParams {
                font: Some(&self.font),
                font_size: play_size,
                color: colors::RED,
                ..Default::default()
            },
        );
    }
}
