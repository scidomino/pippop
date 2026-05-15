use crate::game::slide::SlideManager;
use crate::game::spawn::SpawnManager;
use crate::game::state::{GameState, Interaction, InteractionState, UpdateContext};
use crate::game::world::WorldManager;
use crate::graph::bubble::BubbleStyle;
use crate::graph::Graph;
use crate::graphics::{colors, RenderContext};
use crate::resources::Resources;
use macroquad::prelude::*;

pub struct TitleController {
    pub state: GameState,
    pub world_manager: WorldManager,
    pub spawn_manager: SpawnManager,
    pub slide_manager: SlideManager,
    pub timer: f32,
}

impl TitleController {
    pub fn new() -> Self {
        let graph = Graph::new(
            BubbleStyle::colored(colors::TURQUOISE),
            BubbleStyle::colored(colors::ROSE),
        );

        Self {
            state: GameState::new(graph),
            world_manager: WorldManager::new(),
            spawn_manager: SpawnManager::new(colors::get_group(6)),
            slide_manager: SlideManager::new(),
            timer: 0.0,
        }
    }
}

impl Default for TitleController {
    fn default() -> Self {
        Self::new()
    }
}

impl TitleController {
    pub fn interact(&mut self, interaction: Interaction) -> bool {
        matches!(interaction.state, InteractionState::Released)
    }

    pub fn update(&mut self, resources: &Resources, dt: f32) {
        self.timer += dt;
        let mut ctx = UpdateContext {
            state: &mut self.state,
            resources: Some(resources),
            dt,
        };
        self.world_manager.update(&mut ctx);
        self.spawn_manager.update(&mut ctx);
        self.slide_manager.update(&mut ctx);

        // Drain events so they don't accumulate
        self.state.sound_events.clear();
    }

    pub fn draw(&self, resources: &Resources, camera: &Camera2D) {
        // --- HACK: Font Atlas Priming ---
        // On macOS (Apple Silicon), macroquad's dynamic font atlas resizing can cause
        // an OpenGL driver crash ("texture unloadable") if it happens mid-game.
        // We force the atlas to allocate and resize immediately on the title screen
        // by drawing all digits and common characters at all used sizes (32, 64).
        if self.timer < 0.5 {
            let chars = "0123456789GameOverTccapricmntz!S";
            let sizes = [32, 64];
            for &size in &sizes {
                draw_text_ex(
                    chars,
                    -1000.0,
                    -1000.0,
                    TextParams {
                        font: Some(&resources.font),
                        font_size: size,
                        color: Color::new(0.0, 0.0, 0.0, 0.0),
                        ..Default::default()
                    },
                );
            }
        }

        let ctx = RenderContext {
            state: &self.state,
            resources,
            camera,
        };
        self.world_manager.draw(&ctx);

        // --- Pass 2: Screen Space (UI) ---
        set_default_camera();

        let screen_width = screen_width();
        let screen_height = screen_height();

        draw_rectangle(
            0.0,
            0.0,
            screen_width,
            screen_height,
            Color::new(0.0, 0.0, 0.0, 0.5),
        );

        let screen_center_x = screen_width / 2.0;
        let screen_center_y = screen_height / 2.0;

        let title_text = "PipPop";
        let title_base = 64;
        let title_scale = 2.0;
        let title_dims = measure_text(title_text, Some(&resources.font), title_base, title_scale);

        draw_text_ex(
            title_text,
            screen_center_x - title_dims.width / 2.0 - 4.0,
            screen_center_y - 40.0 + 4.0,
            TextParams {
                font: Some(&resources.font),
                font_size: title_base,
                font_scale: title_scale,
                color: colors::WHITE,
                ..Default::default()
            },
        );
        draw_text_ex(
            title_text,
            screen_center_x - title_dims.width / 2.0,
            screen_center_y - 40.0,
            TextParams {
                font: Some(&resources.font),
                font_size: title_base,
                font_scale: title_scale,
                color: colors::TURQUOISE,
                ..Default::default()
            },
        );

        let play_text = "Gioca!";
        let play_base_size = 64;
        let play_scale = (self.timer * 3.0).sin() * 0.1 + 1.0;

        let play_dims = measure_text(play_text, Some(&resources.font), play_base_size, play_scale);

        draw_text_ex(
            play_text,
            screen_center_x - play_dims.width / 2.0 - 2.0,
            screen_center_y + 80.0 + 2.0,
            TextParams {
                font: Some(&resources.font),
                font_size: play_base_size,
                font_scale: play_scale,
                color: colors::WHITE,
                ..Default::default()
            },
        );
        draw_text_ex(
            play_text,
            screen_center_x - play_dims.width / 2.0,
            screen_center_y + 80.0,
            TextParams {
                font: Some(&resources.font),
                font_size: play_base_size,
                font_scale: play_scale,
                color: colors::RED,
                ..Default::default()
            },
        );
    }
}
