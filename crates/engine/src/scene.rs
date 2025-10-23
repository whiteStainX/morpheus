use std::collections::HashMap;

use anyhow::{anyhow, Result};

use crate::{config::EngineSettings, draw::Canvas, input::InputState};

pub struct Context<'a> {
    pub canvas: Canvas<'a>,
    pub engine: EngineSettings,
    pub delta_time: f32,
    pub total_time: f32,
    pub frame: u64,
    input: Option<&'a InputState>,
}

impl<'a> Context<'a> {
    pub fn new(canvas: Canvas<'a>, engine: EngineSettings) -> Self {
        Self {
            canvas,
            engine,
            delta_time: 0.0,
            total_time: 0.0,
            frame: 0,
            input: None,
        }
    }

    pub fn set_input(&mut self, input: Option<&'a InputState>) {
        self.input = input;
    }

    pub fn input(&self) -> Option<&InputState> {
        self.input
    }

    pub fn set_timing(&mut self, delta_time: f32, total_time: f32, frame: u64) {
        self.delta_time = delta_time;
        self.total_time = total_time;
        self.frame = frame;
    }
}

pub trait Scene {
    fn on_start(&mut self, ctx: &mut Context<'_>);
    fn on_update(&mut self, ctx: &mut Context<'_>);
    fn on_draw(&mut self, ctx: &mut Context<'_>);
    fn on_exit(&mut self, ctx: &mut Context<'_>);
}

pub struct SceneManager<'scene> {
    scenes: HashMap<String, Box<dyn Scene + 'scene>>,
    active_scene: Option<String>,
    queued_scene: Option<String>,
}

impl<'scene> SceneManager<'scene> {
    pub fn new() -> Self {
        Self {
            scenes: HashMap::new(),
            active_scene: None,
            queued_scene: None,
        }
    }

    pub fn add_scene<S>(&mut self, name: impl Into<String>, scene: S)
    where
        S: Scene + 'scene,
    {
        self.scenes.insert(name.into(), Box::new(scene));
    }

    pub fn add_boxed_scene(&mut self, name: impl Into<String>, scene: Box<dyn Scene + 'scene>) {
        self.scenes.insert(name.into(), scene);
    }

    pub fn has_scene(&self, name: &str) -> bool {
        self.scenes.contains_key(name)
    }

    pub fn queue_transition(&mut self, name: impl Into<String>) {
        self.queued_scene = Some(name.into());
    }

    pub fn activate(&mut self, name: &str, ctx: &mut Context<'_>) -> Result<()> {
        if self.active_scene.as_deref() == Some(name) {
            return Ok(());
        }

        if let Some(current_name) = self.active_scene.take() {
            if let Some(scene) = self.scenes.get_mut(&current_name) {
                scene.on_exit(ctx);
            }
        }

        let scene = self
            .scenes
            .get_mut(name)
            .ok_or_else(|| anyhow!("Scene '{name}' not found"))?;
        scene.on_start(ctx);
        self.active_scene = Some(name.to_string());
        Ok(())
    }

    pub fn update(&mut self, ctx: &mut Context<'_>) -> Result<()> {
        if let Some(next_scene) = self.queued_scene.take() {
            self.activate(&next_scene, ctx)?;
        }

        if let Some(active_name) = self.active_scene.clone() {
            if let Some(scene) = self.scenes.get_mut(&active_name) {
                scene.on_update(ctx);
            }
        }

        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context<'_>) {
        if let Some(active_name) = self.active_scene.clone() {
            if let Some(scene) = self.scenes.get_mut(&active_name) {
                scene.on_draw(ctx);
            }
        }
    }

    pub fn shutdown(&mut self, ctx: &mut Context<'_>) {
        if let Some(current_name) = self.active_scene.take() {
            if let Some(scene) = self.scenes.get_mut(&current_name) {
                scene.on_exit(ctx);
            }
        }
    }

    pub fn current_scene(&self) -> Option<&str> {
        self.active_scene.as_deref()
    }
}
