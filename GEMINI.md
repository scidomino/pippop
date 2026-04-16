# PipPop (Rust + Macroquad)

A high-performance bubble physics game ported from Android to a pure Rust architecture.

## Project Vision
- **Single Source of Truth:** All game logic, physics, and rendering are handled in Rust.
- **Cross-Platform:** Targets Native (Desktop), Web (WebAssembly), and Android (via Macroquad/miniquad).
- **Historical Context:** The `android/` directory is kept for historical reference only and should not be modified.

## Core Architecture
- `rust_core/`: The main Rust crate containing:
  - `graph/`: Honeycomb topology management.
  - `physics/`: Custom Bezier-based pressure and surface tension simulation.
  - `main.rs`: The Macroquad game loop and rendering.

## Development Workflow

### Run Locally (Native)
```bash
cargo run --manifest-path rust_core/Cargo.toml
```

### Build for Web (Wasm)
Ensure you have `wasm-pack` or `cargo-quad-wasm` installed.
```bash
# Example using cargo-quad-wasm
cargo-quad-wasm build
```

### Build for Android
Macroquad uses `miniquad` to handle Android lifecycle. Refer to Macroquad's documentation for NDK integration.

## Constraints
- Do not add FFI/JNI logic; we are moving away from the `cdylib` bridge.
- Prioritize `rust_core` stability and performance.
