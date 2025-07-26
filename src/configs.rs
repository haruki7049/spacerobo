use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub mod player;

/// Includes player configuration
#[derive(Resource, Serialize, Deserialize, Debug, Default)]
pub struct GameConfigs {
    pub player: crate::configs::player::Config,
}
