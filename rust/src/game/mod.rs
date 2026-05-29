//! Game Logic and State Management
//!
//! This module orchestrates the gameplay loop. It defines the core [`state::GameState`]
//! and breaks down distinct behaviors (like spawning new bubbles, handling user
//! interactions, or calculating scores) into separate "Manager" structs.
//!
//! The [`controller::GameController`] acts as the central hub, routing `update`, `draw`,
//! and `interact` calls to the various managers based on the current phase of the game.

use std::sync::OnceLock;

/// Returns true if the DEBUG environment variable is set.
/// The result is cached for efficiency.
pub fn is_debug() -> bool {
    static DEBUG_MODE: OnceLock<bool> = OnceLock::new();
    *DEBUG_MODE.get_or_init(|| std::env::var("DEBUG").is_ok())
}

pub mod burst;
pub mod controller;
pub mod debug;
pub mod gameover;
pub mod highlight;
pub mod pause;
pub mod pop;
pub mod reap;
pub mod score;
pub mod slide;
pub mod sound;
pub mod spawn;
pub mod state;
pub mod swap;
pub mod title;
pub mod world;
