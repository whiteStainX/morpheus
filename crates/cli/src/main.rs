use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use crossterm::event::KeyCode;
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
    frame_count: u64,
    dt: f32,
    x_pos: f32,
    y_pos: f32,
    x_dir: f32,
    y_dir: f32,
    time_elapsed: f32,
    player_x: f32,
    player_y: f32,
}

impl MyTestScene {
    fn new() -> Self {
        Self {
            frame_count: 0,
            dt: 0.0,
            x_pos: 10.0,
            y_pos: 10.0,
            x_dir: 1.0,
            y_dir: 1.0,
            time_elapsed: 0.0,
            player_x: 5.0,
            player_y: 5.0,
        }
    }
}

impl Scene for MyTestScene {
    fn on_start(&mut self, _ctx: &mut Context<'_>) {
        // You can keep start-up logic here if needed.
    }

    fn on_update(&mut self, ctx: &mut Context<'_>) {
        self.frame_count = ctx.frame;
        self.dt = ctx.delta_time;
        self.time_elapsed = ctx.total_time;

        // Automated movement
        self.x_pos += self.x_dir * 10.0 * ctx.delta_time;
        self.y_pos += self.y_dir * 5.0 * ctx.delta_time;
        if self.x_pos >= (ctx.canvas.width - 1) as f32 || self.x_pos < 0.0 {
            self.x_dir *= -1.0;
        }
        if self.y_pos >= (ctx.canvas.height - 1) as f32 || self.y_pos < 0.0 {
            self.y_dir *= -1.0;
        }

        // Player-controlled movement
        if let Some(input) = ctx.input() {
            let speed = 20.0;
            if input.is_key_pressed(KeyCode::Up) {
                self.player_y -= speed * ctx.delta_time;
            }
            if input.is_key_pressed(KeyCode::Down) {
                self.player_y += speed * ctx.delta_time;
            }
            if input.is_key_pressed(KeyCode::Left) {
                self.player_x -= speed * ctx.delta_time;
            }
            if input.is_key_pressed(KeyCode::Right) {
                self.player_x += speed * ctx.delta_time;
            }
        }
    }

    fn on_draw(&mut self, ctx: &mut Context<'_>) {
        // The overlay now handles debug text, so we just draw scene elements here.

        // Draw automated moving character
        ctx.canvas.set_foreground_color(Color::Red);
        ctx.canvas.set_symbol('@');
        ctx.canvas.draw_point(self.x_pos as u16, self.y_pos as u16);
        ctx.canvas.set_foreground_color(Color::Reset);

        // Draw player-controlled rectangle
        ctx.canvas.set_foreground_color(Color::Blue);
        ctx.canvas.set_symbol('■');
        ctx.canvas.draw_rect(self.player_x as u16, self.player_y as u16, 2, 1, true);
        ctx.canvas.set_foreground_color(Color::Reset);

        // Draw some other static and animated primitives from before
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

        ctx.canvas.set_symbol(' ');
    }

    fn on_exit(&mut self, ctx: &mut Context<'_>) {
        ctx.canvas.draw_text(0, 15, "MyTestScene exiting. Goodbye!");
        ctx.canvas.set_foreground_color(Color::Reset);
        ctx.canvas.set_background_color(Color::Reset);
        ctx.canvas.set_symbol(' ');
    }
}

fn instantiate_scene(_name: &str, _config_path: &str) -> Option<Box<dyn Scene>> {
    Some(Box::new(MyTestScene::new()))
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
    let scene_names: Vec<String> = config
        .scenes
        .iter()
        .map(|scene| scene.name.clone())
        .collect();

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
    let mut active_scene_index = scene_names
        .iter()
        .position(|name| name == &initial_scene.name)
        .unwrap_or(0);
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
    let mut paused = false;

    while !exit_requested {
        for key_event in input_state.poll_events()? {
            match key_event.code {
                KeyCode::Char(c) => match c.to_ascii_lowercase() {
                    'q' => exit_requested = true,
                    'p' => paused = !paused,
                    'n' => {
                        if !scene_names.is_empty() {
                            active_scene_index = (active_scene_index + 1) % scene_names.len();
                            if let Some(next_scene) = scene_names.get(active_scene_index) {
                                if scene_manager.has_scene(next_scene) {
                                    scene_manager.queue_transition(next_scene.clone());
                                }
                            }
                        }
                    }
                    'm' => {
                        engine_settings.mode = engine_settings.mode.next();
                    }
                    _ => {}
                },
                KeyCode::Esc => exit_requested = true,
                _ => {}
            }
        }

        if exit_requested {
            break;
        }

        let raw_dt = clock.tick();
        let delta_time = if paused { 0.0 } else { raw_dt };
        if !paused {
            elapsed_time += raw_dt;
            frame = frame.wrapping_add(1);
        }
        let fps = if delta_time > 0.0 {
            1.0 / delta_time
        } else {
            0.0
        };

        renderer.clear_screen();
        {
            let mut context = Context::new(renderer.canvas(), engine_settings.clone());
            context.set_input(Some(&input_state));
            context.set_timing(delta_time, elapsed_time, frame);
            context.canvas.current_pixel_mode = engine_settings.mode;
            scene_manager.update(&mut context)?;
            scene_manager.draw(&mut context);
        }
        {
            let mut overlay = renderer.overlay_canvas();
            overlay.set_foreground_color(Color::Yellow);
            let scene_label = scene_manager.current_scene().unwrap_or("<none>");
            let pause_suffix = if paused { " [Paused]" } else { "" };
            overlay.draw_text(0, 0, &format!("Scene: {}{}", scene_label, pause_suffix));

            overlay.set_foreground_color(Color::White);
            overlay.draw_text(
                0,
                1,
                &format!(
                    "Mode: {:?} | FPS: {:>5.1} | Frame: {}",
                    engine_settings.mode, fps, frame
                ),
            );
            overlay.draw_text(0, 2, "[Q] Quit  [P] Pause  [N] Next Scene  [M] Toggle Mode");
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
