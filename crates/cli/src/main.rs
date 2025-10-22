use anyhow::Result;
use clap::{Parser, Subcommand};
use shape_engine_core::{render::TerminalRenderer, time::Clock};

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

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run(args) => {
            // 1. Initialize the renderer
            let mut renderer = TerminalRenderer::new(120, 36)?;
            renderer.init()?;

            let mut clock = Clock::new(args.framerate as f32);
            for i in 0..10 { // Reduced loop for initial test
                let _dt = clock.tick(); // dt is now returned by tick() and handles sleeping
                renderer.clear_screen();
                renderer.draw_text(0, 0, &format!("Running scene from config: {}", args.config));
                renderer.draw_text(0, 1, &format!("Framerate: {}", args.framerate));
                renderer.draw_text(0, 3, &format!("Frame {} - ASCII/Unicode demo incoming...", i));
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
