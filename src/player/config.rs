use bevy::prelude::*;

pub struct Config {
    pub keyboard: KeyboardConfig,
    pub mouse: MouseConfig,
    pub controller: ControllerConfig,
}

pub struct KeyboardConfig {
    // Movements
    pub forward: KeyCode,
    pub back: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,

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

            hover: KeyCode::ControlLeft,

            toggle_firemode: KeyCode::KeyT,
        }
    }
}

pub struct MouseConfig {}

impl std::default::Default for MouseConfig {
    fn default() -> Self {
        Self {}
    }
}

pub struct ControllerConfig {}

impl std::default::Default for ControllerConfig {
    fn default() -> Self {
        Self {}
    }
}
