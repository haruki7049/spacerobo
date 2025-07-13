use bevy::prelude::*;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub mod player;
pub mod target;

#[derive(Resource, Serialize, Deserialize, Debug, Default)]
pub struct GameConfigs {
    player: player::config::Config,
}

#[derive(Parser, Debug, Resource)]
#[clap(version, author, about)]
pub struct CLIArgs {
    config: Option<PathBuf>,
}
