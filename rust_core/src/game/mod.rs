//! Game Logic and State Management
//!
//! This module orchestrates the gameplay loop. It defines the core [`state::GameState`]
//! and breaks down distinct behaviors (like spawning new bubbles, handling user
//! interactions, or calculating scores) into separate "Manager" structs.
//!
//! The [`game::GameController`] acts as the central hub, routing `update`, `draw`,
//! and `interact` calls to the various managers based on the current phase of the game.

pub mod burst;
pub mod game;
pub mod gameover;
pub mod highlight;
pub mod pop;
pub mod reap;
pub mod sanity;
pub mod score;
pub mod slide;
pub mod sound;
pub mod spawn;
pub mod state;
pub mod swap;
pub mod title;
pub mod world;
