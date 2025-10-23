use std::{fs, path::Path};

use anyhow::{Context as AnyhowContext, Result};
use serde::Deserialize;
use toml::value::Table;

use crate::draw::PixelMode;

#[derive(Debug, Clone, Deserialize)]
pub struct EngineSettings {
    pub width: u16,
    pub height: u16,
    pub framerate: u32,
    #[serde(default)]
    pub mode: PixelMode,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SceneConfig {
    pub name: String,
    #[serde(default)]
    pub duration_ms: Option<u64>,
    #[serde(flatten)]
    pub settings: Table,
}

impl SceneConfig {
    pub fn duration_seconds(&self) -> Option<f32> {
        self.duration_ms.map(|ms| ms as f32 / 1_000.0)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EngineConfig {
    pub engine: EngineSettings,
    #[serde(default)]
    pub scenes: Vec<SceneConfig>,
}

pub fn load_config(path: impl AsRef<Path>) -> Result<EngineConfig> {
    let path = path.as_ref();
    let raw = fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file at {}", path.display()))?;

    let config = toml::from_str::<EngineConfig>(&raw)
        .with_context(|| format!("Failed to parse config file at {}", path.display()))?;

    Ok(config)
}
