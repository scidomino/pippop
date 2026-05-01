use crate::game::burst::BurstManager;
use crate::game::highlight::HighlightManager;
use crate::game::pop::PopManager;
use crate::game::reap::ReapManager;
use crate::game::sanity::SanityManager;
use crate::game::slide::SlideManager;
use crate::game::spawn::{SpawnManager, SpawnTimer};
use crate::game::swap::SwapManager;
use crate::game::world::WorldManager;
use crate::graph::bubble::BubbleStyle;
use crate::graph::Graph;
use crate::graphics::{colors, RenderContext};
use crate::resources::Resources;
use macroquad::audio::play_sound_once;
use macroquad::math::Vec2;
use macroquad::prelude::*;

use crate::game::state::{GameEvent, GamePhase, GameState};

#[derive(Debug, Clone, Copy)]
pub enum InteractionState {
    Hover,
    Pressed,
    Released,
}

#[derive(Debug, Clone, Copy)]
pub struct Interaction {
    pub position: Vec2,
    pub state: InteractionState,
}

pub struct GameController {
    pub state: GameState,
    pub font: Font,
    pub world_manager: WorldManager,
    pub spawn_manager: SpawnManager,
    pub slide_manager: SlideManager,
    pub burst_manager: BurstManager,
    pub swap_manager: SwapManager,
    pub pop_manager: PopManager,
    pub reap_manager: ReapManager,
    pub highlight_manager: HighlightManager,
    pub sanity_manager: SanityManager,
}

impl GameController {
    pub fn new(resources: &Resources) -> Self {
        Self {
            state: GameState::new(Graph::new(
                BubbleStyle::swappable(5),
                BubbleStyle::colored(colors::TURQUOISE),
            )),
            font: resources.font.clone(),
            world_manager: WorldManager::new(),
            spawn_manager: SpawnManager::new(
                colors::get_group(6),
                SpawnTimer {
                    starting_wait: 2.0,
                    doubling_time: 120.0,
                },
            ),
            slide_manager: SlideManager::new(),
            burst_manager: BurstManager::new(1),
            swap_manager: SwapManager::new(),
            pop_manager: PopManager::new(),
            reap_manager: ReapManager::new(),
            highlight_manager: HighlightManager::new(),
            sanity_manager: SanityManager::new(),
        }
    }

    pub fn interact(&mut self, resources: &Resources, interaction: Interaction) {
        if self.state.phase == GamePhase::Normal {
            if matches!(interaction.state, InteractionState::Released) {
                if self
                    .swap_manager
                    .interact(&mut self.state.graph, interaction.position)
                {
                    self.state.phase = GamePhase::Swapping;
                    if !resources.splash_sounds.is_empty() {
                        let idx = macroquad::rand::gen_range(0, resources.splash_sounds.len());
                        play_sound_once(&resources.splash_sounds[idx]);
                    }
                }
            } else {
                self.highlight_manager.interact(interaction);
            }
        }
    }

    pub fn update(&mut self, resources: &Resources, dt: f32) {
        self.world_manager.update(&mut self.state, dt);
        self.spawn_manager.update(&mut self.state, dt);
        self.reap_manager.update(&mut self.state, dt);
        self.slide_manager.update(&mut self.state, dt);
        self.highlight_manager.update(&mut self.state, dt);
        self.pop_manager.update(&mut self.state, dt);
        self.swap_manager.update(&mut self.state, dt);
        self.burst_manager.update(&mut self.state, dt);

        // Handle events (sounds, etc.)
        for event in self.state.events.drain(..) {
            match event {
                GameEvent::Pop => play_sound_once(&resources.pop_sound),
                GameEvent::Spawn => play_sound_once(&resources.spawn_sound),
                GameEvent::Burst => play_sound_once(&resources.burst_sound),
                GameEvent::Swap => {} // Splash sounds handled in interact for now
            }
        }

        if let Err(e) = self.sanity_manager.check(&self.state) {
            let dump = self.state.graph.dump_state();
            #[cfg(not(target_arch = "wasm32"))]
            {
                use std::io::Write;
                if let Ok(mut file) = std::fs::File::create("sanity_fail_dump.txt") {
                    let _ = file.write_all(dump.as_bytes());
                }
            }
            log::error!("Graph State Dumped to sanity_fail_dump.txt");
            panic!("Graph Invariant Failure: {}", e);
        }
    }

    pub fn draw(&self, camera: &Camera2D) {
        let ctx = RenderContext {
            graph: &self.state.graph,
            font: &self.font,
        };

        // --- Pass 1: World Space (Bubbles, Managers, UI) ---
        set_camera(camera);

        self.world_manager.draw(&self.state.graph, &ctx);
        self.pop_manager.draw(&ctx);
        self.swap_manager.draw(&ctx);
        self.burst_manager.draw(&ctx);
        self.highlight_manager.draw(&ctx);

        // --- Pass 2: Screen Space (UI) ---
        set_default_camera();
        self.spawn_manager.draw(&ctx);
        draw_text(
            &format!("FPS: {:03}", get_fps()),
            10.0,
            30.0,
            30.0,
            colors::WHITE,
        );
    }
}
