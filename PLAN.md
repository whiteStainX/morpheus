# Shape Engine – PLAN

> **Goal:**  
> Build a terminal-based **generative ASCII/Unicode art engine**, written in **Rust**, designed for **animation, configurability**, and **creative scripting**.  
> The engine will run in MacOS and Linux terminals, rendering animated visualizations frame by frame using **ANSI control sequences** and **Unicode pixel modes**.

---

## 🧠 Concept Overview

This engine aims to **push the limits of ASCII art**.  
It will serve as a *visual playground* for procedural generation, sound-reactive effects, and data-driven shapes — all rendered inside the terminal.

**Core principles:**
- Pure CLI experience — no GUI dependencies.
- Every frame = full control of characters, color, and timing.
- Configurable with `.toml` / `.yaml` + optional scripting layer.
- Portable, fast, and open to extension.

---

## 🧱 Core Features (by Layers)

### 1. Frame Loop & Rendering
- **Double-buffered rendering** → redraw only changed cells (diff-based).
- **Frame timing control** → fixed or adaptive timestep.
- **Cursor control** via ANSI sequences (`\x1b[H`, hide/show cursor, etc.).
- **Resolution modes**:
  - **ASCII mode**: basic character grid.
  - **Unicode half-blocks mode** (`▄`, `▀`) for 2× vertical fidelity.
  - **Unicode Braille mode** (`⠿`, `⣿`) for ultra-fine density.
- **Color modes**:
  - 16-color
  - 256-color
  - 24-bit Truecolor (where terminal supports it)

---

### 2. Shape Primitives
Fundamental drawing tools:
- Point (x, y)
- Line (Bresenham algorithm)
- Rectangle, Circle, Ellipse
- Polygon (filled or wireframe)
- Text elements (for labels or motion typography)
- Curves (quadratic/cubic Bézier)
- 3D projection demos (like the classic spinning donut)

---

### 3. Scene System
- Each scene contains shapes, parameters, and logic.
- Scene lifecycle: `on_start`, `on_update(dt)`, `on_draw(frame)`, `on_exit`.
- Loadable from configuration files (`*.toml` / `*.yaml`).
- Scenes can be chained, looped, or triggered interactively.

---

### 4. Input & Output
- Keyboard input via `crossterm` (pause, switch scenes, toggle display modes).
- File-based data inputs (text, csv, json) to drive visual parameters.
- Optional frame export (save rendered frames to text or video).

---

### 5. Procedural & Generative Toolkit
Creative algorithmic layer:
- Random walkers
- Noise functions (Perlin / Simplex)
- L-systems / fractals
- Particle systems
- Mathematical field visualizations
- Audio-reactive mapping (amplitude → parameter)

---

### 6. Scripting Layer
Allows creative sketches without recompilation.
- **Backed by QuickJS (JavaScript)** or **Lua (via `mlua`)**
- Script API provides:
  - `draw.*` (create shapes)
  - `math.*`, `rand.*`, `time.*`, `input.*`
  - Transformations (`rotate`, `scale`, `translate`)
- Live reload support during runtime.

Example script:
```js
function update(t) {
  draw.circle(40 + 10 * Math.sin(t), 12, 6, "@");
}
```

---

### 7. Config & Themes
Configuration defines engine behavior and visuals.

```toml
[engine]
width = 120
height = 36
framerate = 60
mode = "braille"

[palette]
charset = "@%#*+=-:. "
color = "truecolor"

[[scenes]]
name = "waves"
duration_ms = 8000
[scenes.params]
amp = 8.0
freq = 0.12
speed = 1.4
symbol = "@"
```

Features:
- Switchable color palettes & character sets.
- Themed symbol maps (light, dark, neon, minimal).
- Optional frame overlay (FPS, scene name).

---

### 8. Performance & System Layer
- Dirty-region redraw optimization.
- Buffered output writing.
- Frame profiler (draw time, diff ratio, FPS).
- Graceful exit restoring terminal state.
- Runs smoothly on 80×24 → 160×48 terminals.

---

### 9. Extensibility
- **Shape plugins**: add new drawing primitives.
- **Effect plugins**: add filters (blur, distort, morph).
- **I/O plugins**: external data or network streams.
- Optional integration with **Notcurses** backend for richer visual effects.

---

## 🚀 Phase-by-Phase Development Plan

### 🩻 Phase 1 – MVP: “It draws, it moves, it’s configurable”

**Objectives**
- Establish engine core.
- Achieve smooth animation.
- Run example scenes from configuration.

**Deliverables**
1. **Project Setup**
   - Rust workspace: `engine/`, `cli/`, `examples/`
   - Libraries: `crossterm`, `serde`, `toml`, `clap`
   - CLI command:
     ```bash
     shape run config.toml
     shape list-scenes
     ```
2. **Renderer & Frame Loop**
   - Double buffer & diff-based rendering.
   - Frame timing at 30–60 FPS.
3. **Canvas & Primitives**
   - Implement point, line, rectangle, circle, text.
   - Support for ASCII / half-block / Braille modes.
   - 16/256/truecolor ANSI color.
4. **Scene System**
   - Load scenes from config.
   - Scene lifecycle with start/update/draw/exit.
5. **Input & Overlay**
   - Handle keys for pause, next scene, mode toggle.
   - On-screen overlay for FPS and debug info.
6. **Examples**
   - `waves`, `walkers`, `donut3d`.

**Exit Criteria**
- Stable FPS (60±5%) on 120×36 terminal.
- Clean exit (terminal restored).
- No flicker, no panics after 10-minute run.

---

### 🛠 Phase 1.5 – Polish & Ergonomics
- Theme palettes (`dark`, `light`, etc.)
- Frame export to file.
- Frame profiler overlay (draw time, dirty ratio).
- Live theme switch.

---

### 🎨 Phase 2 – Generative Toolkit
- Implement procedural generators:
  - Perlin/Simplex noise
  - Particle systems
  - L-systems
- Transformations (rotate/scale/translate)
- Post-filters: brightness → charset mapping, motion blur, warp.

---

### 🔊 Phase 3 – Scripting
- Integrate scripting runtime (QuickJS or Lua).
- Expose engine API to scripts (`draw`, `rand`, `time`, `input`).
- Support live reload & error overlays.
- Script-defined scenes and behaviors.

---

### ⚙️ Phase 4 – Performance & Backends
- Dirty-rect compression & subregion flush.
- Character atlas optimization.
- Optional Notcurses backend.
- Headless render mode (for exports/CI).

---

### 🧩 Phase 5 – Plugins & IO
- Registerable effect/shape/data plugins.
- Example plugin crate (compiled separately).
- External data feed (CSV, JSON, UDP).

---

### 🌐 Phase 6 – Extended Features
- Networked multi-user ASCII canvas.
- GIF/MP4 export helpers.
- Interactive REPL for live parameter changes.
- Minimal DSL for animation scripting.

---

## ✅ Summary

| Phase | Title | Focus | Deliverable |
|-------|--------|--------|-------------|
| 1 | MVP | Core rendering & scenes | Smooth animation & basic shapes |
| 1.5 | Polish | UX & profiling | Themes, FPS overlay |
| 2 | Generative Toolkit | Procedural visuals | Noise, particles, filters |
| 3 | Scripting | Extensibility | QuickJS/Lua integration |
| 4 | Performance | Optimizations | Notcurses backend |
| 5 | Plugins | Modularity | Extensible effect system |
| 6 | Extended | Experimental ideas | Audio/network/reactive features |

---

**Next step:** define the **architecture** (modules, traits, and folder structure) for the MVP implementation.
