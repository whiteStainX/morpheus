use anyhow::Result;
use clap::{Parser, Subcommand};
use shape_engine_core::{render::TerminalRenderer, time::Clock, Context, Scene};
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(name = "shape")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run a scene from a configuration file
    Run(RunArgs),
    /// List available scenes from a configuration file
    ListScenes(ListScenesArgs),
}

#[derive(Parser, Debug)]
struct RunArgs {
    /// Path to the configuration file
    #[arg(short, long)]
    config: String,
    #[arg(short, long, default_value_t = 60)]
    framerate: u32,
}

#[derive(Parser, Debug)]
struct ListScenesArgs {
    // No specific arguments for now, but can be extended later
}

struct MyTestScene {
    frame_count: u32,
}

impl MyTestScene {
    fn new() -> Self {
        Self { frame_count: 0 }
    }
}

impl Scene for MyTestScene {
    fn on_start(&mut self, ctx: &mut Context) {
        ctx.renderer.draw_text(0, 5, "MyTestScene started!");
    }

    fn on_update(&mut self, _ctx: &mut Context, _dt: f32) {
        self.frame_count += 1;
    }

    fn on_draw(&mut self, ctx: &mut Context) {
        ctx.renderer.draw_text(0, 7, &format!("Scene Frame: {}", self.frame_count));
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run(args) => {
            // 1. Initialize the renderer
            let mut renderer = TerminalRenderer::new(120, 36)?;
            renderer.init()?;

            let mut clock = Clock::new(args.framerate as f32);
            let mut scene = MyTestScene::new();

            // Call on_start once
            let mut context = Context { renderer: &mut renderer };
            scene.on_start(&mut context);

            loop {
                let dt = clock.tick();

                // Input handling for exit
                if event::poll(Duration::from_millis(0))? {
                    if let Event::Key(key_event) = event::read()? {
                        if key_event.code == KeyCode::Char('q') {
                            break; // Exit loop on 'q'
                        }
                    }
                }

                renderer.clear_screen();
                renderer.draw_text(0, 0, &format!("Running scene from config: {}", args.config));
                renderer.draw_text(0, 1, &format!("Framerate: {}", args.framerate));

                // Update and draw scene
                let mut context = Context { renderer: &mut renderer };
                scene.on_update(&mut context, dt);
                scene.on_draw(&mut context);

                renderer.flush()?;
            }
        }
        Commands::ListScenes(_) => {
            println!("Listing available scenes (not yet implemented)");
            // Placeholder for actual scene listing logic
        }
    }

    Ok(())
}
