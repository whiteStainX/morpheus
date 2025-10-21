# DESIGN.md

## 🎯 Design Overview

Morpheus is structured as a **modular Rust workspace** consisting of multiple crates.  
The goal is to separate concerns cleanly between the rendering engine and the CLI application.

---

## 🧩 Workspace Layout

```
morpheus/
├─ Cargo.toml               # Workspace root
├─ crates/
│  ├─ engine/               # Core engine (library crate)
│  │  ├─ src/
│  │  │  ├─ lib.rs
│  │  │  ├─ render.rs
│  │  │  ├─ time.rs
│  │  │  └─ (future modules)
│  ├─ cli/                  # Executable crate (main CLI entrypoint)
│  │  ├─ src/main.rs
│  └─ examples/             # Example scenes and configs
├─ examples/                # Example .toml configuration files
└─ assets/                  # Future asset or data directory
```

---

## 🧱 Core Architectural Layers

### 1. **Render Layer**
- Uses `crossterm` for terminal control (cursor, clear, raw mode).
- Implements **double buffering** and **diff-based flushing** to reduce flicker.
- Supports **three pixel modes**:
  - ASCII (1 cell = 1 pixel)
  - Unicode Half-Block (`▄`, `▀`)
  - Unicode Braille (`⠿`, `⣿`) for high-density output.

### 2. **Draw Layer**
- Provides geometric primitives: point, line, rectangle, circle, polygon, text.
- Uses integer-based math for fast rasterization (Bresenham algorithms).
- Eventually will support brightness → charset mapping for visual density.

### 3. **Scene System**
- Each scene implements a trait:
  ```rust
  trait Scene {
      fn on_start(&mut self, ctx: &mut Context);
      fn on_update(&mut self, ctx: &mut Context, dt: f32);
      fn on_draw(&mut self, ctx: &mut Context);
  }
  ```
- The engine manages multiple scenes (`SceneManager`) and transitions.

### 4. **Config Layer**
- Reads `.toml` or `.yaml` files defining engine settings and scenes.
- Provides palette customization, symbol maps, and frame rate control.

### 5. **CLI Layer**
- Binary interface that ties everything together.
- Parses options with `clap`, initializes the engine, and runs scenes.

### 6. **Scripting (Future)**
- Embedded scripting runtime (QuickJS / Lua) for defining procedural sketches.
- Exposes safe bindings (`draw`, `rand`, `time`, etc.) to scripts.

---

## 🧠 Module Overview

| Module | Description |
|--------|--------------|
| `render` | Terminal backend, buffers, and flushing logic |
| `time` | Frame timing and fixed-step loop |
| `scene` | Scene management and lifecycle hooks |
| `draw` | Drawing primitives and geometric algorithms |
| `config` | Config parsing via Serde (TOML/YAML) |
| `input` | Keyboard handling and real-time control |
| `cli` | Main executable: loads configs, runs engine |

---

## 🔄 Render Loop (simplified)

```rust
loop {
    let dt = clock.tick();
    scene.on_update(&mut ctx, dt);
    scene.on_draw(&mut ctx);
    renderer.flush()?;

    if input.should_exit() { break; }
}
```

---

## ⚙️ Future Backends & Features

| Feature | Purpose |
|----------|----------|
| **Notcurses backend** | richer glyph effects and smoother color transitions |
| **QuickJS/Lua scripting** | runtime generative logic |
| **Audio input** | sound-reactive visuals |
| **Export module** | text → GIF/MP4 renderer |
| **Network mode** | multiplayer / distributed ASCII canvas |

---

## ✅ Summary

Morpheus combines **system-level performance** with **expressive creativity**, designed to be:
- **Fast** – diff-based rendering and low-level control.
- **Flexible** – modular crates and config-based scenes.
- **Fun** – an experimental playground for ASCII and Unicode art.

> The architecture should *never constrain imagination*, only guide it.
