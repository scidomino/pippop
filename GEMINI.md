# PipPop (Rust + Macroquad)

A high-performance bubble physics game.

## Cross-Platform
Target Native (Desktop) and Web (WebAssembly).

## Architecture
- `rust/`: The main Rust crate containing:
  - `src/graph/`: Trivalent foam topology management.
  - `src/physics/`: Custom Bezier-based pressure and surface tension simulation.
  - `src/game/`: Game logic, state, and "manager" structs for individual systems.
  - `src/graphics/`: Rendering logic and effects.
  - `src/resources.rs`: Encapsulated asset loading and management.
  - `src/main.rs`: The Macroquad game loop and entry point.

## Committing code
- Always run the following before committed:
  - `cargo fmt --manifest-path rust/Cargo.toml`
  - `cargo check --manifest-path rust/Cargo.toml`
  - `cargo test --manifest-path rust/Cargo.toml`
- Always use `--all` instead of naming files one by one
- git commit descriptions should be one concise sentence