# Phase 1: MVP Detailed Steps

**Objective:** Establish the core engine, achieve smooth animation, and run example scenes from configuration files.

## 1. Project Setup & Dependencies

*   **Verify Workspace:** Confirm `morpheus/Cargo.toml` correctly defines the workspace with `crates/engine` and `crates/cli`.
*   **Engine Crate Dependencies:** In `crates/engine/Cargo.toml`, add `crossterm`, `serde`, `toml`, and `anyhow` (for error handling).
*   **CLI Crate Dependencies:** In `crates/cli/Cargo.toml`, add `clap` for argument parsing and `morpheus_engine` (the engine crate) as a dependency.
*   **Basic CLI Commands:**
    *   Implement a `run` command in `crates/cli/src/main.rs` that takes a config file path.
    *   Implement a `list-scenes` command to eventually list available scenes from config.

## 2. Terminal Renderer & Frame Loop

*   **Terminal Abstraction (`crates/engine/src/render.rs`):**
    *   Create a `TerminalRenderer` struct responsible for interacting with `crossterm`.
    *   Implement methods to enter/exit raw mode, hide/show cursor, and clear the screen.
    *   **Double Buffering:** Initialize two `Vec<Cell>` (or similar) buffers: `front_buffer` and `back_buffer`. A `Cell` struct should hold character, foreground color, and background color.
    *   **Diff-based Flushing:** Implement a `flush()` method that compares `front_buffer` with `back_buffer`, generates ANSI escape codes only for changed cells, and writes them to stdout. Then, swap the buffers.
*   **Frame Timing (`crates/engine/src/time.rs`):**
    *   Create a `Clock` struct with a `tick()` method that returns `dt` (delta time) and enforces a target FPS (e.g., 60 FPS).
*   **Main Application Loop (`crates/cli/src/main.rs` or `crates/engine/src/lib.rs`):**
    *   Set up the main `loop` that continuously:
        1.  Calls `clock.tick()` to get `dt`.
        2.  Calls `scene.on_update(ctx, dt)`.
        3.  Calls `scene.on_draw(ctx)`.
        4.  Calls `renderer.flush()`.
        5.  Checks for exit conditions (e.g., keyboard input).

## 3. Canvas & Drawing Primitives

*   **Canvas Abstraction (`crates/engine/src/draw.rs` or `render.rs`):**
    *   Define a `Canvas` trait or struct that provides an interface for drawing operations onto the `back_buffer` of the `TerminalRenderer`.
    *   The `Canvas` should manage the current pixel mode (ASCII, Half-Block, Braille) and color mode.
*   **Pixel Mode Implementation:**
    *   **ASCII Mode:** Directly place characters into the buffer.
    *   **Unicode Half-Block Mode:** Map two vertical cells to a single character (`▄`, `▀`) based on content.
    *   **Unicode Braille Mode:** Map a 2x4 grid of cells to a single Braille character (`⠿`, `⣿`) for higher density. This will require a lookup table or bit manipulation.
*   **Color Support:**
    *   Implement functions to set 16-color, 256-color, and 24-bit Truecolor (RGB) for foreground and background.
*   **Drawing Primitives (`crates/engine/src/draw.rs`):**
    *   `draw_point(x, y, char, color)`
    *   `draw_line(x1, y1, x2, y2, char, color)` (using Bresenham's algorithm)
    *   `draw_rect(x, y, w, h, char, color, filled)`
    *   `draw_circle(cx, cy, r, char, color, filled)`
    *   `draw_text(x, y, text, color)`

## 4. Scene System

*   **`Scene` Trait (`crates/engine/src/lib.rs` or `scene.rs`):**
    ```rust
    pub trait Scene {
        fn on_start(&mut self, ctx: &mut Context);
        fn on_update(&mut self, ctx: &mut Context, dt: f32);
        fn on_draw(&mut self, ctx: &mut Context);
        fn on_exit(&mut self, ctx: &mut Context); // Optional, for cleanup
    }
    ```
*   **`Context` Struct:** Define a `Context` struct to pass mutable references to the `Canvas`, `InputState`, `dt`, and other engine-level data to scene methods.
*   **Scene Manager:** Create a `SceneManager` that can load, activate, and transition between different `Scene` implementations.
*   **Config Loading (`crates/engine/src/config.rs`):**
    *   Define `struct`s using `serde::Deserialize` to represent engine settings and scene configurations from `.toml` files.
    *   Implement a function to load and parse a `.toml` config file into these structs.

## 5. Input Handling & Overlay

*   **Input Handling (`crates/engine/src/input.rs`):**
    *   Use `crossterm::event` to read keyboard input asynchronously.
    *   Create an `InputState` struct to track pressed keys (e.g., `is_key_pressed(KeyCode)`).
    *   Implement logic for pause (e.g., 'P' key), next scene (e.g., 'N' key), and mode toggle (e.g., 'M' key).
*   **On-screen Overlay:**
    *   Add a method to the `Canvas` or `TerminalRenderer` to draw simple text overlays (e.g., current FPS, scene name, debug info) without interfering with the main scene drawing.

## 6. Example Scenes

*   **Implement Scenes (`crates/engine/src/scenes/`):**
    *   Create concrete `struct`s that implement the `Scene` trait for `waves`, `walkers`, and `donut3d`. These will demonstrate the drawing primitives and animation capabilities.
*   **Configuration Files (`examples/`):
    *   Create `waves.toml`, `walkers.toml`, and `donut3d.toml` files that define the settings for each example scene, including their parameters and the desired pixel/color modes.
