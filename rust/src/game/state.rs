use crate::graph::bubble::BubbleKey;
use crate::graph::Graph;
use macroquad::math::Vec2;

/// Represents the high-level state machine of the gameplay loop.
///
/// The game can only be in one phase at a time. This dictates which
/// managers are actively processing logic (e.g., you cannot swap while popping).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamePhase {
    /// The standard gameplay state where the physics simulation is running,
    /// bubbles spawn naturally, and the user can initiate interactions.
    Normal,
    /// A temporary state during which a matching cluster of bubbles is animating
    /// its disappearance.
    Popping,
    /// A temporary state during which the player's swappable bubble is
    /// visually exchanging places/styles with a target bubble.
    Swapping,
    /// A temporary state during which adjacent matching bubbles are merging
    /// together into larger bubbles after a swap.
    Bursting,
    /// The end-state of the game, reached when the player runs out of swaps.
    GameOver,
}

/// A decoupled representation of an audio event.
///
/// Managers queue these events during their update logic, and the `SoundManager`
/// consumes them to play the actual audio assets, keeping game logic free of I/O.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundEvent {
    Pop,
    Spawn,
    Burst,
    Swap,
}

/// A decoupled representation of a scoring event.
///
/// Managers queue these events when points should be awarded. The `ScoreManager`
/// consumes them to calculate chains, update the total score, and spawn floating text.
#[derive(Debug, Clone, Copy)]
pub enum ScoreEvent {
    /// Triggered when two adjacent bubbles of the same color merge.
    Burst { position: Vec2 },
    /// Triggered when a large enough merged bubble is popped.
    Pop { position: Vec2, size: i32 },
}

/// The central repository for all mutable game state.
///
/// This struct holds the underlying topological `Graph`, the current `GamePhase`,
/// and event queues. It is passed via context structs to the various `Manager`s
/// so they can read and modify the world.
pub struct GameState {
    pub graph: Graph,
    pub phase: GamePhase,
    pub sound_events: Vec<SoundEvent>,
    pub score_events: Vec<ScoreEvent>,
    /// Used primarily during the `Bursting` phase to track which bubble
    /// initiated the sequence and should be preserved during topological merges.
    pub focus_bubble: Option<BubbleKey>,
}

impl GameState {
    pub fn new(graph: Graph) -> Self {
        Self {
            graph,
            phase: GamePhase::Normal,
            sound_events: Vec::new(),
            score_events: Vec::new(),
            focus_bubble: None,
        }
    }
}

/// Context provided to managers during the update (tick) phase.
pub struct UpdateContext<'a> {
    pub state: &'a mut GameState,
    /// The time delta in seconds since the last frame.
    pub dt: f32,
}

/// Context provided to managers when processing user input.
pub struct InteractContext<'a> {
    pub state: &'a mut GameState,
    pub interaction: Interaction,
}

/// The state of the user's pointer device (mouse or touch).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionState {
    Hover,
    Pressed,
    Released,
}

/// A unified representation of a pointer interaction, abstracting away
/// the difference between mouse and touch inputs.
#[derive(Debug, Clone, Copy)]
pub struct Interaction {
    /// The location of the interaction in world space.
    pub position: Vec2,
    /// The current state of the pointer.
    pub state: InteractionState,
}
