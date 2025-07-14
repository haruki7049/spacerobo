use bevy::prelude::*;
use clap::Parser;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{
    path::PathBuf,
    sync::{LazyLock, Mutex},
};

pub mod player;
pub mod target;

#[derive(Resource, Serialize, Deserialize, Debug, Default)]
pub struct GameConfigs {
    player: player::config::Config,
}

static DEFAULT_CONFIG_PATH: LazyLock<Mutex<PathBuf>> = LazyLock::new(|| {
    let proj_dirs = ProjectDirs::from("dev", "haruki7049", "spacerobo")
        .expect("Failed to search ProjectDirs for dev.haruki7049.spacerobo");
    let mut config_path: PathBuf = proj_dirs.config_dir().to_path_buf();
    let filename: &str = "config.toml";

    config_path.push(filename);
    Mutex::new(config_path)
});

#[derive(Parser, Debug, Resource)]
#[clap(version, author, about)]
pub struct CLIArgs {
    #[arg(short, long, default_value = DEFAULT_CONFIG_PATH.lock().unwrap().display().to_string())]
    config_file: PathBuf,
}
