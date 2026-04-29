use crate::graph::bubble::BubbleStyle;
use crate::graph::Graph;
use crate::graphics::{colors, RenderContext};
use crate::managers::game::{Interaction, InteractionState};
use crate::managers::slide::SlideManager;
use crate::managers::spawn::{RatchetSpawnTimer, SpawnManager};
use crate::managers::world::WorldManager;
use crate::resources::Resources;
use macroquad::prelude::*;

pub struct TitleController {
    pub graph: Graph,
    pub font: Font,
    pub world_manager: WorldManager,
    pub spawn_manager: SpawnManager,
    pub slide_manager: SlideManager,
    pub timer: f32,
}

impl TitleController {
    pub fn new(resources: &Resources) -> Self {
        let graph = Graph::new(
            BubbleStyle::Standard {
                size: 1,
                color: colors::TURQUOISE,
            },
            BubbleStyle::Standard {
                size: 1,
                color: colors::ROSE,
            },
        );

        Self {
            graph,
            font: resources.font.clone(),
            world_manager: WorldManager::new(),
            spawn_manager: SpawnManager::new(
                colors::get_group(6),
                Box::new(RatchetSpawnTimer {
                    starting_wait: 3.0,
                    doubling_time: 120.0,
                }),
            ),
            slide_manager: SlideManager::new(),
            timer: 0.0,
        }
    }

    pub fn interact(&mut self, interaction: Interaction) -> bool {
        matches!(interaction.state, InteractionState::Pressed)
    }

    pub fn update(&mut self, dt: f32) {
        self.timer += dt;
        self.world_manager.update(&mut self.graph, dt);
        self.spawn_manager.update(dt);
        self.slide_manager.update(&mut self.graph, dt);
        self.spawn_manager.possibly_spawn(&mut self.graph);
    }

    pub fn draw(&self, camera: &Camera2D) {
        let ctx = RenderContext {
            graph: &self.graph,
            font: &self.font,
        };

        // --- Pass 1: World Space (Background Bubbles) ---
        set_camera(camera);
        self.world_manager.draw(&self.graph, &ctx);

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
