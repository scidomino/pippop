use crate::graph::bubble::BubbleStyle;
use crate::graph::Graph;
use crate::graphics::{colors, Renderer};
use crate::managers::audio::AudioManager;
use crate::managers::burst::BurstManager;
use crate::managers::highlight::HighlightManager;
use crate::managers::pop::PopManager;
use crate::managers::reap::ReapManager;
use crate::managers::sanity::SanityManager;
use crate::managers::slide::SlideManager;
use crate::managers::spawn::{RatchetSpawnTimer, SpawnManager};
use crate::managers::swap::SwapManager;
use crate::resources::Resources;
use macroquad::math::Vec2;
use macroquad::prelude::Camera2D;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Normal,
    Popping,
    Swapping,
    Burst,
}

#[derive(Debug, Clone, Copy)]
pub struct Interaction {
    pub position: Vec2,
    pub is_clicked: bool,
}

pub struct GameController {
    pub graph: Graph,
    pub renderer: Renderer,
    pub state: GameState,
    pub physics_accumulator: f32,
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
        let mut graph = Graph::new();
        graph.init(
            BubbleStyle::Player,
            BubbleStyle::Standard {
                size: 1,
                color: colors::TURQUOISE,
            },
        );

        Self {
            graph,
            renderer: Renderer::new(resources),
            state: GameState::Normal,
            physics_accumulator: 0.0,
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

    pub fn handle_input(&mut self, interaction: Option<Interaction>) {
        let mut new_highlight = None;
        if self.state == GameState::Normal {
            if let Some(i) = interaction {
                if i.is_clicked {
                    if self.swap_manager.swap(&mut self.graph, i.position) {
                        self.state = GameState::Swapping;
                    }
                } else {
                    new_highlight = Some(i.position);
                }
            }
        }
        self.highlight_manager.set_point(new_highlight);
    }

    pub fn update(&mut self, dt: f32) {
        // Physics update (fixed timestep)
        self.physics_accumulator += dt;
        while self.physics_accumulator >= 0.016 {
            crate::physics::advance_frame(&mut self.graph);
            self.physics_accumulator -= 0.016;
        }

        // Unconditional updates
        self.spawn_manager.update(dt);

        match self.state {
            GameState::Normal => {
                self.reap_manager.reap_popped(&mut self.graph);

                // Check if any reaped bubbles caused a new match
                if self.burst_manager.find_and_set_burstable_edge(&self.graph) {
                    self.state = GameState::Burst;
                }

                if self.slide_manager.slide_slidable_edges(&mut self.graph, dt) {
                    // Sliding can create new matches
                    if self.burst_manager.find_and_set_burstable_edge(&self.graph) {
                        self.state = GameState::Burst;
                    }
                }

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
                if self.swap_manager.update(&mut self.graph, dt) {
                    if self.burst_manager.find_and_set_burstable_edge(&self.graph) {
                        self.state = GameState::Burst;
                    } else {
                        self.state = GameState::Normal;
                    }
                }
            }
            GameState::Burst => {
                if self.burst_manager.update(dt) {
                    if let Some(ekey) = self.burst_manager.active_edge {
                        if self.graph.vertices.contains_key(ekey.vertex) {
                            let bkey = self.graph.vertices.get_edge(ekey).bubble;
                            self.burst_manager.burst(&mut self.graph, ekey);
                            self.audio_manager.play_burst();
                            if !self
                                .burst_manager
                                .find_and_set_next_burstable(&self.graph, bkey)
                            {
                                self.burst_manager.active_edge = None;
                                if self.burst_manager.find_and_set_burstable_edge(&self.graph) {
                                    self.state = GameState::Burst;
                                } else {
                                    self.state = GameState::Normal;
                                }
                            } else {
                                self.state = GameState::Burst;
                            }
                        } else {
                            self.burst_manager.active_edge = None;
                            self.state = GameState::Normal;
                        }
                    } else {
                        self.state = GameState::Normal;
                    }
                }
            }
        }

        if let Err(e) = self.sanity_manager.check_invariants(&self.graph) {
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
        self.renderer.draw(&self.graph, camera, self);
    }
}
