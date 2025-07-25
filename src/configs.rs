use crate::player;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Includes player configuration
#[derive(Resource, Serialize, Deserialize, Debug, Default)]
pub struct GameConfigs {
    pub player: player::config::Config,
}

