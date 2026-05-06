# PipPop (Rust + Macroquad)

A high-performance bubble physics game ported from Android to a pure Rust architecture.

## Project Vision
- **Single Source of Truth:** All game logic, physics, and rendering are handled in Rust.
- **Cross-Platform:** Targets Native (Desktop), Web (WebAssembly), and Android (via Macroquad/miniquad).
- **Historical Context:** The `android/` directory is kept for historical reference only and should not be modified.

## Core Architecture
- `rust_core/`: The main Rust crate containing:
  - `src/graph/`: Trivalent foam topology management.
  - `src/physics/`: Custom Bezier-based pressure and surface tension simulation.
  - `src/game/`: Game logic, state, and "manager" structs for individual systems.
  - `src/graphics/`: Rendering logic and effects.
  - `src/resources.rs`: Encapsulated asset loading and management.
  - `src/main.rs`: The Macroquad game loop and entry point.

## Development Workflow

### Code Styling
- Always run `cargo fmt --manifest-path rust_core/Cargo.toml` before committing changes.

### Run Locally (Native)
```bash
cargo run --manifest-path rust_core/Cargo.toml
```

### Build for Web (Wasm)
```bash
cd rust_core
./build_web.sh
```
The built files will be output to `../web_dist`.

### Build for Android
Macroquad uses `miniquad` to handle Android lifecycle. Refer to Macroquad's documentation for NDK integration.

## Constraints
- Prioritize `rust_core` stability and performance.
- Never launch the game or any graphical application (e.g. `cargo run`), as the agent cannot see or interact with them.
