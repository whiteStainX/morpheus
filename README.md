# Morpheus CLI

A retro-inspired Ink CLI that helps you stage cinematic screen recordings. Morpheus greets you with an ASCII boot sequence, runs a configurable `genact` fake-busy sequence, and then presents a System 7 flavoured command console for orchestrating your session.

## Getting started

```bash
npm install
npm run build
```

Run the CLI locally with:

```bash
npm run start
```

Once published you can execute the binary directly:

```bash
npx morpheus
```

## Features

- **Welcome Screen** – renders the bundled ASCII art logo and a progress bar while the system initialises.
- **Genact Integration** – launches `genact` in a child process, streaming its output until a timeout (10s by default) or key press.
- **Main Console** – a System 7 inspired layout with command history and an interactive prompt. Includes helper commands (`help`, `record`, `status`, `clear`, `exit`).

## Assets

The ASCII logo lives in `assets/morpheus_logo.txt`. Adjust or replace it to change the branding used throughout the interface.
