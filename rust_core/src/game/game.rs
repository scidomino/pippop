use crate::game::burst::BurstManager;
use crate::game::highlight::HighlightManager;
use crate::game::pop::PopManager;
use crate::game::reap::ReapManager;
use crate::game::sanity::SanityManager;
use crate::game::slide::SlideManager;
use crate::game::spawn::SpawnManager;
use crate::game::state::{GameEvent, GameState, Interaction};
use crate::game::swap::SwapManager;
use crate::game::world::WorldManager;
use crate::graph::bubble::BubbleStyle;
use crate::graph::Graph;
use crate::graphics::{colors, RenderContext};
use crate::resources::Resources;
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
            spawn: SpawnManager::new(colors::get_group(6)),
            slide: SlideManager::new(),
            burst: BurstManager::new(),
            swap: SwapManager::new(),
            pop: PopManager::new(),
            reap: ReapManager::new(),
            highlight: HighlightManager::new(),
            sanity: SanityManager::new(),
        }
    }

    pub fn interact(&mut self, interaction: Interaction) {
        self.swap.interact(&mut self.state, interaction);
        self.highlight.interact(&mut self.state, interaction);
    }

    pub fn update(&mut self, resources: &Resources, dt: f32) {
        self.world.update(&mut self.state, dt);
        self.spawn.update(&mut self.state, dt);
        self.reap.update(&mut self.state);
        self.slide.update(&mut self.state, dt);
        self.highlight.update(dt);
        self.pop.update(&mut self.state, dt);
        self.swap.update(&mut self.state, dt);
        self.burst.update(&mut self.state, dt);
        self.sanity.update(&self.state);

        self.play_sounds(resources);
    }

    pub fn draw(&self, camera: &Camera2D) {
        let ctx = RenderContext {
            graph: &self.state.graph,
            font: &self.font,
            camera,
        };

        self.world.draw(&ctx);
        self.pop.draw(&ctx);
        self.swap.draw(&ctx);
        self.burst.draw(&ctx);
        self.highlight.draw(&ctx);
        self.spawn.draw(&ctx);
    }

    fn play_sounds(&mut self, resources: &Resources) {
        // Handle events (sounds, etc.)
        for event in self.state.events.drain(..) {
            match event {
                GameEvent::Pop => resources.play_pop(),
                GameEvent::Spawn => resources.play_spawn(),
                GameEvent::Burst => resources.play_burst(),
                GameEvent::Swap => resources.play_swap(),
            }
        }
    }
}
