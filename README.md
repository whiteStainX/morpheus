# Morpheus

> A terminal-based generative art engine written in **Rust**, designed for **ASCII/Unicode visualization**, **animation**, and **procedural creativity**.

---

## 🌌 Overview

**Morpheus** is a lightweight engine that turns your terminal into a dynamic canvas.  
It focuses on **generative ASCII and Unicode art**, supporting smooth animation, configuration-driven scenes, and scripting extensions.

This project aims to push the boundaries of what’s possible inside a terminal — similar in spirit to *p5.js*, but rendered purely through **ANSI escape codes**, **Unicode half-blocks**, and **Braille pixels**.

---

## 🚀 Features

### Core
- **Frame-based renderer** with double-buffered diff updates.
- **Multiple pixel modes**: ASCII, Unicode half-blocks (`▄ ▀`), and Braille (`⠿ ⣿`).
- **Color support**: 16-color, 256-color, and 24-bit Truecolor.
- **Scene system** for defining procedural visuals and animations.

### Configuration
- Load scene setup from `.toml` / `.yaml` config files.
- Runtime CLI options (`--framerate`, `--config`, etc.).
- Simple command to run any config:
  ```bash
  cargo run -p shape -- --framerate 60
  ```

### Extensible Design
- **Crate-based modularity** (`engine` + `cli` + `examples`).
- Planned support for **QuickJS** or **Lua** scripting for live-coded sketches.
- Optional **Notcurses backend** for rich effects.

---

## 🧩 Project Structure

```
morpheus/
├─ Cargo.toml              # Workspace definition
├─ crates/
│  ├─ engine/              # Core rendering library
│  │  ├─ src/
│  │  │  ├─ render.rs      # Rendering backend and diff system
│  │  │  ├─ time.rs        # Frame clock and timestep
│  │  │  └─ lib.rs
│  ├─ cli/                 # Command-line interface (main binary)
│  │  └─ src/main.rs
├─ examples/
│  └─ minimal.toml         # Example configuration file
└─ assets/                 # (optional) Text/CSV data or future assets
```

---

## 🛠 Quick Start

1. **Clone & Build**
   ```bash
   git clone https://github.com/whiteStainX/morpheus.git
   cd morpheus
   cargo build --release
   ```

2. **Run Example**
   ```bash
   cargo run -p shape -- --framerate 60
   ```

3. **Edit Config**
   Modify `examples/minimal.toml` to change width, height, or frame rate.

---

## 🧱 Future Roadmap

- [ ] Scene system & procedural generators (waves, walkers, donut3D)
- [ ] Unicode Braille renderer for high-fidelity pixel art
- [ ] Scripting via QuickJS or Lua (`draw.circle()`, etc.)
- [ ] Export to GIF/MP4 (optional helper)
- [ ] Audio-reactive and data-driven inputs

---

## 🧠 Philosophy

Morpheus is about creating **order from chaos**, turning plain text into motion and structure.  
It’s built to be an **engine first**, so everything — from scripts to configurations — should be easily extendable, inspectable, and hackable.

> “Shapes dream in symbols, and we render their echoes.”

---

© 2025 whiteStainX — *Morpheus Project*
