//! Player's Configuration

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Configuration struct
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Config {
    pub keyboard: KeyboardConfig,
    pub mouse: MouseConfig,
    pub robo: RoboConfig,
}

// Configurations about robo
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct RoboConfig {
    pub thruster: ThrusterConfig,
}

// Configurations about thrusters
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct ThrusterConfig {
    pub force: ForceConfig,
}

// Configuration about force by thrusters
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ForceConfig {
    // Keyboard
    pub accelerate: f32,
    pub dash: f32,

    // Mouse
    pub pitch: f32,
    pub yaw: f32,
}

impl std::default::Default for ForceConfig {
    fn default() -> Self {
        Self {
            accelerate: 0.7,
            dash: 3.0,
            pitch: 1.0,
            yaw: 1.0,
        }
    }
}

/// Keyboard Configurations. This structure usually contains keymappings.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct KeyboardConfig {
    // Movements
    pub forward: KeyCode,
    pub back: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,

    pub dash: KeyCode,

    // Hovering
    pub hover: KeyCode,

    // Gun
    pub toggle_firemode: KeyCode,

    // Game quit key
    pub quit: KeyCode,

    // Respawn key
    pub respawn: KeyCode,
}

impl std::default::Default for KeyboardConfig {
    fn default() -> Self {
        Self {
            forward: KeyCode::KeyW,
            back: KeyCode::KeyS,
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,

            dash: KeyCode::ShiftLeft,

            hover: KeyCode::ControlLeft,

            toggle_firemode: KeyCode::KeyT,

            quit: KeyCode::Escape,

            respawn: KeyCode::Space,
        }
    }
}

/// Mouse Configurations
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct MouseConfig {
    pub x_reverse: bool,
    pub y_reverse: bool,
}
