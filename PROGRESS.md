# Project Progress

## Initial State (2025-10-21)

*   Project structure (workspace, `engine`, `cli` crates) is set up.
*   `README.md`, `DESIGN.md`, and `PLAN.md` provide initial project context and roadmap.
*   `PHASE_ONE.md` details the plan for the MVP.
*   Currently, the project contains placeholder files and basic `Cargo.toml` configurations, awaiting implementation of core features.

## Progress Update (2025-10-21)

*   **Folder Structure:** Validated and created missing `crates/examples` and `assets` directories as per `DESIGN.md`.
*   **Dependency Verification:** Confirmed `crossterm`, `serde`, `toml`, `anyhow`, `clap`, and `shape-engine-core` dependencies are correctly configured in `crates/engine/Cargo.toml` and `crates/cli/Cargo.toml`.
*   **Basic CLI Commands Implemented:**
    *   Refactored `crates/cli/src/main.rs` to use `clap` subcommands: `run` (with `--config` and `--framerate` arguments) and `list-scenes`.
*   **Code Quality:** Resolved compiler warnings related to unused imports and mutable variables.
*   **Test Run:** Successfully executed `cargo run -p shape run --config examples/minimal.toml --framerate 60`. The program ran for 10 frames (0-9) as expected due to the current placeholder loop in `main.rs`.

## Progress Update (2025-10-21) - Phase 1 Core Components Completed

*   **Terminal Abstraction:** Implemented `TerminalRenderer` with `Cell` struct, double buffering, `init()`, `shutdown()`, `clear_screen()`, and a diff-based `flush()` method in `crates/engine/src/render.rs`.
*   **Frame Timing:** Refined `Clock` struct and `tick()` method in `crates/engine/src/time.rs` to accurately provide delta time (`dt`) and enforce target FPS.
*   **Main Application Loop:** Set up the continuous main loop in `crates/cli/src/main.rs`, integrating `Clock`, `TerminalRenderer`, and a placeholder `Scene` (`MyTestScene`). Implemented basic keyboard input handling for exiting the application.
*   **Scene System (Partial):** Defined `Context` struct and `Scene` trait in `crates/engine/src/scene.rs` and exported them from `crates/engine/src/lib.rs`.

## Progress Update (2025-10-21) - Canvas & Drawing Primitives

*   **Canvas Abstraction:** Implemented `Canvas` struct in `crates/engine/src/draw.rs` to manage drawing operations, including current pixel mode, foreground/background colors, and symbol. `TerminalRenderer` now provides a `canvas()` method, and `Context` holds a `Canvas` instance.
*   **Color Support:** Implemented `set_foreground_color` and `set_background_color` methods in `Canvas` to control drawing colors.
*   **Drawing Primitives:** Implemented `draw_point`, `draw_line` (Bresenham's), `draw_rect` (outline/filled), `draw_circle` (outline/filled), and `set_symbol` in `Canvas`.
    *   *Note:* These drawing functions are currently basic and will need further refinement, especially when integrating advanced pixel modes.
*   **Pixel Mode Implementation (Partial):** Defined `PixelMode` enum and integrated it into `Canvas`. ASCII mode is functional. The rendering logic for Unicode Half-Block and Braille modes is pending.

## Progress Update (2025-10-21) - Scene System

*   **Scene Trait:** Completed the `Scene` trait in `crates/engine/src/scene.rs` with `on_start`, `on_update`, `on_draw`, and `on_exit` methods.
*   **Context Struct:** The `Context` struct now provides access to `Canvas`, `EngineSettings`, `InputState`, and timing information (`delta_time`, `total_time`, `frame`).
*   **Scene Manager:** Implemented a `SceneManager` in `crates/engine/src/scene.rs` capable of adding, activating, and managing the lifecycle of multiple scenes.
*   **Config Loading:** Implemented `load_config` function and associated `serde` structs in `crates/engine/src/config.rs` to load engine and scene definitions from a TOML file.
*   **CLI Integration:** The `run` command in `crates/cli/src/main.rs` now uses the `SceneManager` to load and run scenes defined in the specified configuration file.

## Progress Update (2025-10-21) - Input Handling & Overlay

*   **Input System Enhancements:** Added asynchronous input polling in `crates/engine/src/input.rs`, enabling the engine to track pressed keys and expose discrete key events per frame.
*   **Runtime Controls:** Integrated pause (`P`), next scene (`N`), mode toggle (`M`), and quit (`Q`/`Esc`) shortcuts in the CLI loop, including scene cycling logic and pixel mode rotation.
*   **Overlay Rendering:** Extended `TerminalRenderer` with an overlay buffer and helper methods so overlays can be rendered without disturbing the main scene output. The CLI now renders an on-screen HUD with scene details, FPS, and available controls.
