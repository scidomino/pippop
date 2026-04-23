use crate::graph::Graph;
use crate::managers::audio::AudioManager;
use crate::managers::burst::BurstManager;
use crate::managers::highlight::HighlightManager;
use crate::managers::pop::PopManager;
use crate::managers::sanity::SanityManager;
use crate::managers::slide::SlideManager;
use crate::managers::spawn::SpawnManager;
use crate::managers::swap::SwapManager;
use macroquad::math::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Normal,
    Popping,
    Swapping,
    Burst,
}

pub struct GameController {
    pub state: GameState,
    pub spawn_manager: SpawnManager,
    pub slide_manager: SlideManager,
    pub burst_manager: BurstManager,
    pub swap_manager: SwapManager,
    pub pop_manager: PopManager,
    pub highlight_manager: HighlightManager,
    pub sanity_manager: SanityManager,
    pub audio_manager: AudioManager,
}

impl GameController {
    pub fn new(spawn_manager: SpawnManager, audio_manager: AudioManager) -> Self {
        Self {
            state: GameState::Normal,
            spawn_manager,
            slide_manager: SlideManager::new(),
            burst_manager: BurstManager::new(1),
            swap_manager: SwapManager::new(),
            pop_manager: PopManager::new(),
            highlight_manager: HighlightManager::new(),
            sanity_manager: SanityManager::new(),
            audio_manager,
        }
    }

    pub fn handle_input(&mut self, graph: &mut Graph, point: Option<Vec2>, clicked: bool) {
        if self.state == GameState::Normal {
            if clicked {
                if let Some(p) = point {
                    if self.swap_manager.otter_swap(graph, p) {
                        self.state = GameState::Swapping;
                        self.highlight_manager.set_point(None);
                    }
                }
            } else {
                self.highlight_manager.set_point(point);
            }
        } else {
            self.highlight_manager.set_point(None);
        }
    }

    pub fn update(&mut self, graph: &mut Graph, dt: f32) {
        // Unconditional updates
        self.spawn_manager.update(dt);

        match self.state {
            GameState::Normal => {
                self.pop_manager.remove_deflated(graph, &self.burst_manager);

                if self.slide_manager.slide_slidable_edges(graph, dt) {
                    // Sliding can create new matches
                    if self.burst_manager.find_and_set_burstable_edge(graph) {
                        self.state = GameState::Burst;
                    }
                }

                if self.spawn_manager.possibly_spawn(graph) {
                    self.audio_manager.play_spawn();
                }
                self.highlight_manager.update(dt);

                if self.state == GameState::Normal && self.pop_manager.deflate_big_bubble(graph) {
                    self.state = GameState::Popping;
                }
            }
            GameState::Popping => {
                if self.pop_manager.update(graph, dt) {
                    self.audio_manager.play_pop();
                }
                if self.pop_manager.pending_pop.is_none() {
                    self.state = GameState::Normal;
                }
            }
            GameState::Swapping => {
                if self.swap_manager.update(graph, dt) {
                    if self.burst_manager.find_and_set_burstable_edge(graph) {
                        self.state = GameState::Burst;
                    } else {
                        self.state = GameState::Normal;
                    }
                }
            }
            GameState::Burst => {
                if self.burst_manager.update(dt) {
                    if let Some(ekey) = self.burst_manager.active_edge {
                        if graph.vertices.contains_key(ekey.vertex) {
                            let bkey = graph.vertices.get_edge(ekey).bubble;
                            self.burst_manager.burst(graph, ekey);
                            self.audio_manager.play_burst();
                            if !self.burst_manager.find_and_set_next_burstable(graph, bkey) {
                                self.burst_manager.active_edge = None;
                                if self.burst_manager.find_and_set_burstable_edge(graph) {
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

        if let Err(e) = self.sanity_manager.check_invariants(graph) {
            let dump = graph.dump_state();
            use std::io::Write;
            if let Ok(mut file) = std::fs::File::create("sanity_fail_dump.txt") {
                let _ = file.write_all(dump.as_bytes());
            }
            log::error!("Graph State Dumped to sanity_fail_dump.txt");
            panic!("Graph Invariant Failure: {}", e);
        }
    }
}
