# PipPop (Rust + Macroquad)

A high-performance bubble physics game.

## Project Vision
- **Single Source of Truth:** All game logic, physics, and rendering are handled in Rust.
- **Cross-Platform:** Targets Native (Desktop) and Web (WebAssembly).

## Core Architecture
- `rust/`: The main Rust crate containing:
  - `src/graph/`: Trivalent foam topology management.
  - `src/physics/`: Custom Bezier-based pressure and surface tension simulation.
  - `src/game/`: Game logic, state, and "manager" structs for individual systems.
  - `src/graphics/`: Rendering logic and effects.
  - `src/resources.rs`: Encapsulated asset loading and management.
  - `src/main.rs`: The Macroquad game loop and entry point.

## Development Workflow

### Code Styling
- Always run `cargo fmt --manifest-path rust/Cargo.toml` before committing changes.

### Run Locally (Native)
```bash
cargo run --manifest-path rust/Cargo.toml
```

### Build for Web (Wasm)
```bash
cd rust
./build_web.sh
```
The built files will be output to `../web_dist`.

## Constraints
- Prioritize `rust` stability and performance.
- Never launch the game or any graphical application (e.g. `cargo run`), as the agent cannot see or interact with them.
