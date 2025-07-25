//! Spacerobo

use bevy::prelude::*;
use clap::Parser;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{
    path::PathBuf,
    sync::{LazyLock, Mutex},
};

pub mod player;
pub mod scenes;

/// Includes player configuration
#[derive(Resource, Serialize, Deserialize, Debug, Default)]
pub struct GameConfigs {
    player: player::config::Config,
}

#[derive(Debug, Event)]
pub struct DeathEvent {
    entity: Entity,
}

#[derive(Debug, States, Default, Hash, Eq, PartialEq, Clone)]
#[states(scoped_entities)]
pub enum GameMode {
    #[default]
    Title,
    ShootingRange,
}

impl DeathEvent {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

/// Default Configuration Path, using directories crate to calculate ProjectDirs (~/.config/spacerobo)
static DEFAULT_CONFIG_PATH: LazyLock<Mutex<PathBuf>> = LazyLock::new(|| {
    let proj_dirs = ProjectDirs::from("dev", "haruki7049", "spacerobo")
        .expect("Failed to search ProjectDirs for dev.haruki7049.spacerobo");
    let mut config_path: PathBuf = proj_dirs.config_dir().to_path_buf();
    let filename: &str = "config.toml";

    config_path.push(filename);
    Mutex::new(config_path)
});

/// Command-Line Arguments, using clap crate
/// This structure allows users to set an additional configuration file
#[derive(Parser, Debug, Resource)]
#[clap(version, author, about = env!("CARGO_PKG_DESCRIPTION"))]
pub struct CLIArgs {
    #[arg(short, long, default_value = DEFAULT_CONFIG_PATH.lock().unwrap().display().to_string())]
    config_file: PathBuf,
}

#[derive(Debug, Component)]
pub struct Hp {
    rest: f32,
}

impl std::default::Default for Hp {
    fn default() -> Self {
        Self { rest: 100. }
    }
}

impl Hp {
    pub fn ammo() -> Self {
        Self { rest: 5. }
    }
}
