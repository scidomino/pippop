use crate::game::burst::BurstManager;
use crate::game::highlight::HighlightManager;
use crate::game::pop::PopManager;
use crate::game::reap::ReapManager;
use crate::game::sanity::SanityManager;
use crate::game::slide::SlideManager;
use crate::game::spawn::{SpawnManager, SpawnTimer};
use crate::game::state::{GameEvent, GameState, Interaction};
use crate::game::swap::SwapManager;
use crate::game::world::WorldManager;
use crate::graph::bubble::BubbleStyle;
use crate::graph::Graph;
use crate::graphics::{colors, RenderContext};
use crate::resources::Resources;
use macroquad::audio::play_sound_once;
use macroquad::prelude::*;

pub struct GameController {
    pub state: GameState,
    pub font: Font,
    pub world: WorldManager,
    pub spawn: SpawnManager,
    pub slide: SlideManager,
    pub burst: BurstManager,
    pub swap: SwapManager,
    pub pop: PopManager,
    pub reap: ReapManager,
    pub highlight: HighlightManager,
    pub sanity: SanityManager,
}

impl GameController {
    pub fn new(resources: &Resources) -> Self {
        Self {
            state: GameState::new(Graph::new(
                BubbleStyle::swappable(5),
                BubbleStyle::colored(colors::TURQUOISE),
            )),
            font: resources.font.clone(),
            world: WorldManager::new(),
            spawn: SpawnManager::new(
                colors::get_group(6),
                SpawnTimer {
                    starting_wait: 2.0,
                    doubling_time: 120.0,
                },
            ),
            slide: SlideManager::new(),
            burst: BurstManager::new(1),
            swap: SwapManager::new(),
            pop: PopManager::new(),
            reap: ReapManager::new(),
            highlight: HighlightManager::new(),
            sanity: SanityManager::new(),
        }
    }

    pub fn interact(&mut self, _resources: &Resources, interaction: Interaction) {
        self.swap.interact(&mut self.state, interaction);
        self.highlight.interact(&mut self.state, interaction);
    }

    pub fn update(&mut self, resources: &Resources, dt: f32) {
        self.world.update(&mut self.state, dt);
        self.spawn.update(&mut self.state, dt);
        self.reap.update(&mut self.state, dt);
        self.slide.update(&mut self.state, dt);
        self.highlight.update(&mut self.state, dt);
        self.pop.update(&mut self.state, dt);
        self.swap.update(&mut self.state, dt);
        self.burst.update(&mut self.state, dt);

        // Handle events (sounds, etc.)
        for event in self.state.events.drain(..) {
            match event {
                GameEvent::Pop => play_sound_once(&resources.pop_sound),
                GameEvent::Spawn => play_sound_once(&resources.spawn_sound),
                GameEvent::Burst => play_sound_once(&resources.burst_sound),
                GameEvent::Swap => {
                    if !resources.splash_sounds.is_empty() {
                        let idx = macroquad::rand::gen_range(0, resources.splash_sounds.len());
                        play_sound_once(&resources.splash_sounds[idx]);
                    }
                }
            }
        }

        if let Err(e) = self.sanity.check(&self.state) {
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

        self.world.draw(&self.state.graph, &ctx);
        self.pop.draw(&ctx);
        self.swap.draw(&ctx);
        self.burst.draw(&ctx);
        self.highlight.draw(&ctx);

        // --- Pass 2: Screen Space (UI) ---
        set_default_camera();
        self.spawn.draw(&ctx);
        draw_text(
            &format!("FPS: {:03}", get_fps()),
            10.0,
            30.0,
            30.0,
            colors::WHITE,
        );
    }
}
