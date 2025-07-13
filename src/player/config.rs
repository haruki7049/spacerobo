use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub keyboard: KeyboardConfig,
    pub mouse: MouseConfig,
    pub controller: ControllerConfig,
}

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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MouseConfig {
    pub x_reverse: bool,
    pub y_reverse: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ControllerConfig {}
