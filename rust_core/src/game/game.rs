use crate::game::burst::BurstManager;
use crate::game::gameover::GameOverManager;
use crate::game::highlight::HighlightManager;
use crate::game::pop::PopManager;
use crate::game::reap::ReapManager;
use crate::game::sanity::SanityManager;
use crate::game::score::ScoreManager;
use crate::game::slide::SlideManager;
use crate::game::sound::SoundManager;
use crate::game::spawn::SpawnManager;
use crate::game::state::{GameState, InteractContext, Interaction, UpdateContext};
use crate::game::swap::SwapManager;
use crate::game::world::WorldManager;
use crate::graph::bubble::BubbleStyle;
use crate::graph::Graph;
use crate::graphics::{colors, RenderContext};
use crate::resources::Resources;
use macroquad::prelude::*;

pub struct GameController<'a> {
    pub state: GameState,
    pub font: Font,

    pub world: WorldManager,
    pub spawn: SpawnManager,
    pub slide: SlideManager,
    pub burst: BurstManager,
    pub swap: SwapManager,
    pub pop: PopManager,
    pub score: ScoreManager,
    pub reap: ReapManager,
    pub highlight: HighlightManager,
    pub gameover: GameOverManager,
    pub sanity: SanityManager,
    pub sound: SoundManager<'a>,
}

impl<'a> GameController<'a> {
    pub fn new(resources: &'a Resources) -> Self {
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
            score: ScoreManager::new(),
            reap: ReapManager::new(),
            highlight: HighlightManager::new(),
            gameover: GameOverManager::new(),
            sanity: SanityManager::new(),
            sound: SoundManager::new(resources),
        }
    }

    pub fn interact(&mut self, interaction: Interaction) -> bool {
        let mut ctx = InteractContext {
            state: &mut self.state,
            interaction,
        };
        let should_exit = self.gameover.interact(&mut ctx);
        self.swap.interact(&mut ctx);
        self.highlight.interact(&mut ctx);
        should_exit
    }

    pub fn update(&mut self, _resources: &Resources, dt: f32) {
        let mut ctx = UpdateContext {
            state: &mut self.state,
            dt,
        };

        self.world.update(&mut ctx);
        self.spawn.update(&mut ctx);
        self.reap.update(&mut ctx);
        self.slide.update(&mut ctx);
        self.highlight.update(&mut ctx);
        self.pop.update(&mut ctx);
        self.score.update(&mut ctx);
        self.swap.update(&mut ctx);
        self.burst.update(&mut ctx);
        self.gameover.update(&mut ctx);
        self.sound.update(&mut ctx);

        self.sanity.update(&self.state);
    }

    pub fn draw(&self, camera: &Camera2D) {
        let ctx = RenderContext {
            graph: &self.state.graph,
            phase: &self.state.phase,
            font: &self.font,
            camera,
        };

        self.world.draw(&ctx);
        self.pop.draw(&ctx);
        self.swap.draw(&ctx);
        self.burst.draw(&ctx);
        self.highlight.draw(&ctx);
        self.score.draw(&ctx);
        self.gameover.draw(&ctx);
    }
}
