use anyhow::Result;
use clap::{Parser, Subcommand};
use shape_engine_core::{hello, render, time::Clock};
use std::thread;
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

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run(args) => {
            println!("Running scene from config: {}", args.config);
            println!("Framerate: {}", args.framerate);
            // Placeholder for actual engine run logic
            hello();
            render::clear_screen();
            let clock = Clock::new(1.0 / args.framerate as f32);
            for i in 0..10 { // Reduced loop for initial test
                render::clear_screen();
                println!("Frame {} - ASCII/Unicode demo incoming...", i);
                thread::sleep(Duration::from_secs_f32(clock.dt));
            }
        }
        Commands::ListScenes(_) => {
            println!("Listing available scenes (not yet implemented)");
            // Placeholder for actual scene listing logic
        }
    }

    Ok(())
}
