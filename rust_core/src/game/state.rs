use crate::graph::bubble::BubbleKey;
use crate::graph::Graph;
use macroquad::math::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamePhase {
    Normal,
    Popping,
    Swapping,
    Bursting,
    GameOver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundEvent {
    Pop,
    Spawn,
    Burst,
    Swap,
}

// This struct represents the entire state of the game at any given moment.
// This is passed to the update method of the managers, which can modify it as needed.
pub struct GameState {
    pub graph: Graph,
    pub phase: GamePhase,
    pub sound_events: Vec<SoundEvent>,
    pub focus_bubble: Option<BubbleKey>,
}

impl GameState {
    pub fn new(graph: Graph) -> Self {
        Self {
            graph,
            phase: GamePhase::Normal,
            sound_events: Vec::new(),
            focus_bubble: None,
        }
    }
}

pub struct UpdateContext<'a> {
    pub state: &'a mut GameState,
    pub dt: f32,
}

pub struct InteractContext<'a> {
    pub state: &'a mut GameState,
    pub interaction: Interaction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
