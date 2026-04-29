use crate::graph::bubble::BubbleStyle;
use crate::graph::Graph;
use crate::graphics::{colors, RenderContext};
use crate::managers::audio::AudioManager;
use crate::managers::burst::BurstManager;
use crate::managers::highlight::HighlightManager;
use crate::managers::pop::PopManager;
use crate::managers::reap::ReapManager;
use crate::managers::sanity::SanityManager;
use crate::managers::slide::SlideManager;
use crate::managers::spawn::{RatchetSpawnTimer, SpawnManager};
use crate::managers::swap::SwapManager;
use crate::managers::world::WorldManager;
use crate::resources::Resources;
use macroquad::math::Vec2;
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Normal,
    Popping,
    Swapping,
    Burst,
}

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
    pub graph: Graph,
    pub font: Font,
    pub state: GameState,
    pub world_manager: WorldManager,
    pub spawn_manager: SpawnManager,
    pub slide_manager: SlideManager,
    pub burst_manager: BurstManager,
    pub swap_manager: SwapManager,
    pub pop_manager: PopManager,
    pub reap_manager: ReapManager,
    pub highlight_manager: HighlightManager,
    pub sanity_manager: SanityManager,
    pub audio_manager: AudioManager,
}

impl GameController {
    pub fn new(resources: &Resources) -> Self {
        let graph = Graph::new(
            BubbleStyle::Player { swaps_left: 5 },
            BubbleStyle::Standard {
                size: 1,
                color: colors::TURQUOISE,
            },
        );

        Self {
            graph,
            font: resources.font.clone(),
            state: GameState::Normal,
            world_manager: WorldManager::new(),
            spawn_manager: SpawnManager::new(
                colors::get_group(6),
                Box::new(RatchetSpawnTimer {
                    starting_wait: 2.0,
                    doubling_time: 120.0,
                }),
            ),
            slide_manager: SlideManager::new(),
            burst_manager: BurstManager::new(1),
            swap_manager: SwapManager::new(),
            pop_manager: PopManager::new(),
            reap_manager: ReapManager::new(),
            highlight_manager: HighlightManager::new(),
            sanity_manager: SanityManager::new(),
            audio_manager: AudioManager::new(resources),
        }
    }

    pub fn interact(&mut self, interaction: Interaction) {
        if self.state == GameState::Normal {
            if matches!(interaction.state, InteractionState::Released) {
                if self
                    .swap_manager
                    .interact(&mut self.graph, interaction.position)
                {
                    self.state = GameState::Swapping;
                }
            } else {
                self.highlight_manager.interact(interaction);
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.world_manager.update(&mut self.graph, dt);
        self.spawn_manager.update(dt);

        match self.state {
            GameState::Normal => {
                self.reap_manager.update(&mut self.graph);
                self.slide_manager.update(&mut self.graph, dt);

                if self.spawn_manager.possibly_spawn(&mut self.graph) {
                    self.audio_manager.play_spawn();
                }
                self.highlight_manager.update(dt);

                if self.state == GameState::Normal
                    && self.pop_manager.start_pop_if_ready(&mut self.graph)
                {
                    self.state = GameState::Popping;
                }
            }
            GameState::Popping => {
                if self.pop_manager.update(&mut self.graph, dt) {
                    self.audio_manager.play_pop();
                }
                if self.pop_manager.pending_pop.is_none() {
                    self.state = GameState::Normal;
                }
            }
            GameState::Swapping => {
                if let Some(bkey) = self.swap_manager.update(&mut self.graph, dt) {
                    self.burst_manager.set_focus_bubble(bkey);
                    if self.burst_manager.find_and_set_next_burstable(&self.graph) {
                        self.state = GameState::Burst;
                    } else {
                        self.burst_manager.focus_bubble = None;
                        self.state = GameState::Normal;
                    }
                }
            }
            GameState::Burst => {
                if self.burst_manager.update(dt) {
                    if let Some(ekey) = self.burst_manager.active_edge {
                        if self.graph.vertices.contains_key(ekey.vertex) {
                            self.burst_manager.burst(&mut self.graph, ekey);
                            self.audio_manager.play_burst();
                            if !self.burst_manager.find_and_set_next_burstable(&self.graph) {
                                self.burst_manager.active_edge = None;
                                self.burst_manager.focus_bubble = None;
                                self.state = GameState::Normal;
                            } else {
                                self.state = GameState::Burst;
                            }
                        } else {
                            self.burst_manager.active_edge = None;
                            self.burst_manager.focus_bubble = None;
                            self.state = GameState::Normal;
                        }
                    } else {
                        self.state = GameState::Normal;
                    }
                }
            }
        }

        if let Err(e) = self.sanity_manager.check(&self.graph) {
            let dump = self.graph.dump_state();
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
            graph: &self.graph,
            font: &self.font,
        };

        // --- Pass 1: World Space (Bubbles, Managers, UI) ---
        set_camera(camera);

        self.world_manager.draw(&self.graph, &ctx);
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
