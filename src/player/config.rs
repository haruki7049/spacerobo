//! Player's Configuration

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Configuration struct
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub keyboard: KeyboardConfig,
    pub mouse: MouseConfig,
    pub controller: ControllerConfig,
    pub robo: RoboConfig,
}

// Configurations about robo
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RoboConfig {
    pub thruster: ThrusterConfig,
}

// Configurations about thrusters
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ThrusterConfig {
    pub force: ForceConfig,
}

// Configuration about force by thrusters
#[derive(Serialize, Deserialize, Debug)]
pub struct ForceConfig {
    // Keyboard
    pub forward: f32,
    pub back: f32,
    pub left: f32,
    pub right: f32,
    pub roll_left: f32,
    pub roll_right: f32,

    // Mouse
    pub pitch: f32,
    pub yaw: f32,
}

impl std::default::Default for ForceConfig {
    fn default() -> Self {
        Self {
            forward: 10.0,
            back: 10.0,
            left: 10.0,
            right: 10.0,
            roll_left: 3.0,
            roll_right: 3.0,
            pitch: 1.0,
            yaw: 1.0,
        }
    }
}

/// Keyboard Configurations. This structure usually contains keymappings.
#[derive(Serialize, Deserialize, Debug)]
pub struct KeyboardConfig {
    // Movements
    pub forward: KeyCode,
    pub back: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub roll_left: KeyCode,
    pub roll_right: KeyCode,

    // Hovering
    pub hover: KeyCode,

    // Gun
    pub toggle_firemode: KeyCode,
}

impl std::default::Default for KeyboardConfig {
    fn default() -> Self {
        Self {
            forward: KeyCode::KeyW,
            back: KeyCode::KeyS,
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
            roll_left: KeyCode::KeyQ,
            roll_right: KeyCode::KeyE,

            hover: KeyCode::ControlLeft,

            toggle_firemode: KeyCode::KeyT,
        }
    }
}

/// Mouse Configurations
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MouseConfig {
    pub x_reverse: bool,
    pub y_reverse: bool,
}

/// Controller (Gamepad) Configurations
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ControllerConfig {}
