use anyhow::Result;
use clap::{Parser, Subcommand};
use shape_engine_core::{render::TerminalRenderer, time::Clock, Context, Scene};
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;
use crossterm::style::Color;
use std::thread;

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
    dt: f32,
    x_pos: f32,
    y_pos: f32,
    x_dir: f32,
    y_dir: f32,
    time_elapsed: f32,
}

impl MyTestScene {
    fn new() -> Self {
        Self {
            frame_count: 0,
            dt: 0.0,
            x_pos: 0.0,
            y_pos: 0.0,
            x_dir: 1.0,
            y_dir: 1.0,
            time_elapsed: 0.0,
        }
    }
}

impl Scene for MyTestScene {
    fn on_start(&mut self, ctx: &mut Context) {
        ctx.canvas.draw_text(0, 5, "MyTestScene started!");
    }

    fn on_update(&mut self, ctx: &mut Context, dt: f32) {
        self.frame_count += 1;
        self.dt = dt;
        self.time_elapsed += dt;

        // Simple movement logic for '@' character
        self.x_pos += self.x_dir * 10.0 * dt; // Move 10 units per second
        self.y_pos += self.y_dir * 5.0 * dt;  // Move 5 units per second

        // Bounce off edges
        if self.x_pos >= (ctx.canvas.width - 1) as f32 || self.x_pos < 0.0 {
            self.x_dir *= -1.0;
        }
        if self.y_pos >= (ctx.canvas.height - 1) as f32 || self.y_pos < 0.0 {
            self.y_dir *= -1.0;
        }
    }

    fn on_draw(&mut self, ctx: &mut Context) {
        // Display info
        ctx.canvas.draw_text(0, 0, &format!("Running scene from config: {}", "examples/minimal.toml"));
        ctx.canvas.draw_text(0, 1, &format!("Framerate: {}", (1.0 / self.dt) as u32));
        ctx.canvas.draw_text(0, 2, &format!("Pixel Mode: {:?}, Time: {:.2}", &ctx.canvas.current_pixel_mode, self.time_elapsed));
        ctx.canvas.draw_text(0, 7, &format!("Scene Frame: {}", self.frame_count));
        ctx.canvas.draw_text(0, 8, &format!("Delta Time (dt): {:.4}", self.dt));
        ctx.canvas.draw_text(0, 9, &format!("FPS: {:.2}", 1.0 / self.dt));

        // Draw moving character in red
        ctx.canvas.set_foreground_color(Color::Red);
        ctx.canvas.set_symbol('@');
        ctx.canvas.draw_text(self.x_pos as u16, self.y_pos as u16, "@");
        ctx.canvas.set_foreground_color(Color::Reset);

        // Draw some colored text
        ctx.canvas.set_background_color(Color::Blue);
        ctx.canvas.draw_text(0, 12, "This text has a blue background!");
        ctx.canvas.set_background_color(Color::Reset);

        // --- Demonstrate new primitives ---

        // Static Rectangle
        ctx.canvas.set_foreground_color(Color::Green);
        ctx.canvas.set_symbol('#');
        ctx.canvas.draw_rect(50, 5, 10, 5, false); // Outline rect
        ctx.canvas.set_foreground_color(Color::Reset);

        // Filled Rectangle
        ctx.canvas.set_foreground_color(Color::DarkYellow);
        ctx.canvas.set_symbol('X');
        ctx.canvas.draw_rect(65, 5, 8, 4, true); // Filled rect
        ctx.canvas.set_foreground_color(Color::Reset);

        // Static Line
        ctx.canvas.set_foreground_color(Color::Cyan);
        ctx.canvas.set_symbol('-');
        ctx.canvas.draw_line(50, 15, 70, 10);
        ctx.canvas.set_foreground_color(Color::Reset);

        // Animated Circle
        let circle_radius = (5.0 * (self.time_elapsed.sin() + 1.0) + 2.0) as i32; // Radius oscillates
        let circle_x = (ctx.canvas.width / 2) as i32 + (10.0 * (self.time_elapsed * 0.5).cos()) as i32;
        let circle_y = (ctx.canvas.height / 2) as i32 + (5.0 * (self.time_elapsed * 0.8).sin()) as i32;

        ctx.canvas.set_foreground_color(Color::Magenta);
        ctx.canvas.set_symbol('*');
        ctx.canvas.draw_circle(circle_x, circle_y, circle_radius, false); // Outline circle
        ctx.canvas.set_foreground_color(Color::Reset);

        // Animated Filled Circle
        let filled_circle_radius = (3.0 * (self.time_elapsed.cos() + 1.0) + 1.0) as i32;
        let filled_circle_x = (ctx.canvas.width / 4) as i32 + (5.0 * (self.time_elapsed * 0.7).sin()) as i32;
        let filled_circle_y = (ctx.canvas.height / 4) as i32 + (3.0 * (self.time_elapsed * 0.9).cos()) as i32;

        ctx.canvas.set_foreground_color(Color::White);
        ctx.canvas.set_background_color(Color::DarkGrey);
        ctx.canvas.set_symbol('O');
        ctx.canvas.draw_circle(filled_circle_x, filled_circle_y, filled_circle_radius, true); // Filled circle
        ctx.canvas.set_foreground_color(Color::Reset);
        ctx.canvas.set_background_color(Color::Reset);

        // Reset symbol to default after drawing primitives
        ctx.canvas.set_symbol(' ');
    }

    fn on_exit(&mut self, ctx: &mut Context) {
        ctx.canvas.draw_text(0, 15, "MyTestScene exiting. Goodbye!");
        // Ensure exit message is displayed before terminal cleanup
        ctx.canvas.set_foreground_color(Color::Reset);
        ctx.canvas.set_background_color(Color::Reset);
        ctx.canvas.set_symbol(' ');
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
            let mut context = Context { canvas: renderer.canvas() };
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
                let mut context = Context { canvas: renderer.canvas() };

                // Update and draw scene
                scene.on_update(&mut context, dt);
                scene.on_draw(&mut context);

                renderer.flush()?;
            }
            // Call on_exit when loop breaks
            let mut context = Context { canvas: renderer.canvas() };
            scene.on_exit(&mut context);
            renderer.flush()?; // Added: Flush after on_exit to display message
            std::thread::sleep(Duration::from_secs(1)); // Added: Delay to see exit message
        }
        Commands::ListScenes(_) => {
            println!("Listing available scenes (not yet implemented)");
            // Placeholder for actual scene listing logic
        }
    }

    Ok(())
}
