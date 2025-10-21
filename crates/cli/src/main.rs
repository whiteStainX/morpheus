use anyhow::Result;
use clap::Parser;
use shape_engine_core::{hello, render, time::Clock};
use std::thread;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(name = "shape")]
struct Cli {
    #[arg(short, long, default_value_t = 60)]
    framerate: u32,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    hello();
    render::clear_screen();
    let mut clock = Clock::new(1.0 / cli.framerate as f32);
    for i in 0..120 {
        render::clear_screen();
        println!("Frame {} - ASCII/Unicode demo incoming...", i);
        thread::sleep(Duration::from_secs_f32(clock.dt));
    }
    Ok(())
}
