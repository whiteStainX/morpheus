use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::style::Color;
use shape_engine_core::{
    load_config, render::TerminalRenderer, time::Clock, Context, EngineSettings, InputState, Scene,
    SceneManager,
};
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
    /// Target framerate (frames per second). Overrides the value in the config file when provided.
    #[arg(short, long, default_value_t = 60)]
    framerate: u32,
}

#[derive(Parser, Debug)]
struct ListScenesArgs {
    /// Path to the configuration file to inspect
    #[arg(short, long)]
    config: String,
}

struct MyTestScene {
    config_path: String,
    frame_count: u64,
    dt: f32,
    x_pos: f32,
    y_pos: f32,
    x_dir: f32,
    y_dir: f32,
    time_elapsed: f32,
}

impl MyTestScene {
    fn new(config_path: impl Into<String>) -> Self {
        Self {
            config_path: config_path.into(),
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
    fn on_start(&mut self, ctx: &mut Context<'_>) {
        ctx.canvas.draw_text(0, 5, "MyTestScene started!");
    }

    fn on_update(&mut self, ctx: &mut Context<'_>) {
        self.frame_count = ctx.frame;
        self.dt = ctx.delta_time;
        self.time_elapsed = ctx.total_time;

        self.x_pos += self.x_dir * 10.0 * ctx.delta_time; // Move 10 units per second
        self.y_pos += self.y_dir * 5.0 * ctx.delta_time; // Move 5 units per second

        if self.x_pos >= (ctx.canvas.width - 1) as f32 || self.x_pos < 0.0 {
            self.x_dir *= -1.0;
        }
        if self.y_pos >= (ctx.canvas.height - 1) as f32 || self.y_pos < 0.0 {
            self.y_dir *= -1.0;
        }
    }

    fn on_draw(&mut self, ctx: &mut Context<'_>) {
        ctx.canvas
            .draw_text(0, 0, &format!("Active config: {}", self.config_path));
        ctx.canvas.draw_text(
            0,
            1,
            &format!(
                "Engine: {}x{} @ {} FPS",
                ctx.engine.width, ctx.engine.height, ctx.engine.framerate
            ),
        );

        let fps = if self.dt > 0.0 { 1.0 / self.dt } else { 0.0 };
        ctx.canvas
            .draw_text(0, 2, &format!("Pixel Mode: {:?}", ctx.engine.mode));
        ctx.canvas.draw_text(
            0,
            3,
            &format!(
                "Frame: {}, Time: {:.2}s",
                self.frame_count, self.time_elapsed
            ),
        );
        ctx.canvas.draw_text(
            0,
            4,
            &format!("Delta Time: {:.4}s, FPS: {:.1}", self.dt, fps),
        );

        ctx.canvas.set_foreground_color(Color::Red);
        ctx.canvas.set_symbol('@');
        ctx.canvas
            .draw_text(self.x_pos as u16, self.y_pos as u16, "@");
        ctx.canvas.set_foreground_color(Color::Reset);

        ctx.canvas.set_background_color(Color::Blue);
        ctx.canvas
            .draw_text(0, 7, "This text has a blue background!");
        ctx.canvas.set_background_color(Color::Reset);

        ctx.canvas.set_foreground_color(Color::Green);
        ctx.canvas.set_symbol('#');
        ctx.canvas.draw_rect(50, 5, 10, 5, false);
        ctx.canvas.set_foreground_color(Color::Reset);

        ctx.canvas.set_foreground_color(Color::DarkYellow);
        ctx.canvas.set_symbol('X');
        ctx.canvas.draw_rect(65, 5, 8, 4, true);
        ctx.canvas.set_foreground_color(Color::Reset);

        ctx.canvas.set_foreground_color(Color::Cyan);
        ctx.canvas.set_symbol('-');
        ctx.canvas.draw_line(50, 15, 70, 10);
        ctx.canvas.set_foreground_color(Color::Reset);

        let circle_radius = (5.0 * (self.time_elapsed.sin() + 1.0) + 2.0) as i32;
        let circle_x =
            (ctx.canvas.width / 2) as i32 + (10.0 * (self.time_elapsed * 0.5).cos()) as i32;
        let circle_y =
            (ctx.canvas.height / 2) as i32 + (5.0 * (self.time_elapsed * 0.8).sin()) as i32;

        ctx.canvas.set_foreground_color(Color::Magenta);
        ctx.canvas.set_symbol('*');
        ctx.canvas
            .draw_circle(circle_x, circle_y, circle_radius, false);
        ctx.canvas.set_foreground_color(Color::Reset);

        let filled_circle_radius = (3.0 * (self.time_elapsed.cos() + 1.0) + 1.0) as i32;
        let filled_circle_x =
            (ctx.canvas.width / 4) as i32 + (5.0 * (self.time_elapsed * 0.7).sin()) as i32;
        let filled_circle_y =
            (ctx.canvas.height / 4) as i32 + (3.0 * (self.time_elapsed * 0.9).cos()) as i32;

        ctx.canvas.set_foreground_color(Color::White);
        ctx.canvas.set_background_color(Color::DarkGrey);
        ctx.canvas.set_symbol('O');
        ctx.canvas
            .draw_circle(filled_circle_x, filled_circle_y, filled_circle_radius, true);
        ctx.canvas.set_foreground_color(Color::Reset);
        ctx.canvas.set_background_color(Color::Reset);

        ctx.canvas.set_symbol(' ');
    }

    fn on_exit(&mut self, ctx: &mut Context<'_>) {
        ctx.canvas.draw_text(0, 15, "MyTestScene exiting. Goodbye!");
        ctx.canvas.set_foreground_color(Color::Reset);
        ctx.canvas.set_background_color(Color::Reset);
        ctx.canvas.set_symbol(' ');
    }
}

fn instantiate_scene(name: &str, config_path: &str) -> Option<Box<dyn Scene>> {
    match name {
        "waves" | "demo" | "test" => Some(Box::new(MyTestScene::new(config_path.to_string()))),
        _ => None,
    }
}

fn run_scene(args: &RunArgs) -> Result<()> {
    let config = load_config(&args.config)?;
    if config.scenes.is_empty() {
        return Err(anyhow!(
            "Config '{}' does not define any scenes to run",
            args.config
        ));
    }

    let mut renderer = TerminalRenderer::new(config.engine.width, config.engine.height)?;
    renderer.init()?;

    let mut engine_settings: EngineSettings = config.engine.clone();
    if args.framerate > 0 {
        engine_settings.framerate = args.framerate;
    }

    let mut clock = Clock::new(engine_settings.framerate as f32);

    let mut scene_manager = SceneManager::new();
    let mut input_state = InputState::new();

    for scene_def in &config.scenes {
        if scene_manager.has_scene(&scene_def.name) {
            continue;
        }
        if let Some(scene) = instantiate_scene(&scene_def.name, &args.config) {
            scene_manager.add_boxed_scene(scene_def.name.clone(), scene);
        } else {
            eprintln!("Warning: scene '{}' is not available", scene_def.name);
        }
    }

    let initial_scene = config.scenes.first().unwrap();
    if !scene_manager.has_scene(&initial_scene.name) {
        return Err(anyhow!(
            "Scene '{}' is not registered in the engine",
            initial_scene.name
        ));
    }

    renderer.clear_screen();
    {
        let mut context = Context::new(renderer.canvas(), engine_settings.clone());
        context.set_input(Some(&input_state));
        context.set_timing(0.0, 0.0, 0);
        context.canvas.current_pixel_mode = engine_settings.mode;
        scene_manager.activate(&initial_scene.name, &mut context)?;
    }
    renderer.flush()?;

    let mut elapsed_time = 0.0f32;
    let mut frame = 0u64;
    let mut exit_requested = false;

    while !exit_requested {
        let dt = clock.tick();
        elapsed_time += dt;
        frame = frame.wrapping_add(1);

        while event::poll(Duration::from_millis(0))? {
            match event::read()? {
                Event::Key(key_event) => match key_event.kind {
                    KeyEventKind::Press | KeyEventKind::Repeat => {
                        input_state.set_key_pressed(key_event.code, true);
                        if key_event.code == KeyCode::Char('q') {
                            exit_requested = true;
                        }
                    }
                    KeyEventKind::Release => {
                        input_state.set_key_pressed(key_event.code, false);
                    }
                },
                _ => {}
            }
        }

        if exit_requested {
            break;
        }

        renderer.clear_screen();
        {
            let mut context = Context::new(renderer.canvas(), engine_settings.clone());
            context.set_input(Some(&input_state));
            context.set_timing(dt, elapsed_time, frame);
            context.canvas.current_pixel_mode = engine_settings.mode;
            scene_manager.update(&mut context)?;
            scene_manager.draw(&mut context);
        }
        renderer.flush()?;
    }

    renderer.clear_screen();
    {
        let mut context = Context::new(renderer.canvas(), engine_settings.clone());
        context.set_input(Some(&input_state));
        context.set_timing(0.0, elapsed_time, frame);
        context.canvas.current_pixel_mode = engine_settings.mode;
        scene_manager.shutdown(&mut context);
    }
    renderer.flush()?;
    std::thread::sleep(Duration::from_millis(500));

    Ok(())
}

fn list_scenes(args: &ListScenesArgs) -> Result<()> {
    let config = load_config(&args.config)?;
    if config.scenes.is_empty() {
        println!("No scenes defined in {}", args.config);
    } else {
        println!("Scenes defined in {}:", args.config);
        for scene in config.scenes {
            if let Some(duration) = scene.duration_seconds() {
                println!("- {} (duration: {:.2}s)", scene.name, duration);
            } else {
                println!("- {}", scene.name);
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run(args) => run_scene(args),
        Commands::ListScenes(args) => list_scenes(args),
    }
}
