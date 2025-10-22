//! Player's Configuration

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Configuration struct
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    keyboard: KeyboardConfig,
    mouse: MouseConfig,
    controller: ControllerConfig,
    robo: RoboConfig,
}

impl Config {
    pub fn keyboard(&self) -> KeyboardConfig {
        self.keyboard.clone()
    }

    pub fn mouse(&self) -> MouseConfig {
        self.mouse.clone()
    }

    pub fn controller(&self) -> ControllerConfig {
        self.controller.clone()
    }

    pub fn robo(&self) -> RoboConfig {
        self.robo.clone()
    }
}

// Configurations about robo
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RoboConfig {
    thruster: ThrusterConfig,
}

impl RoboConfig {
    pub fn thruster(&self) -> ThrusterConfig {
        self.thruster.clone()
    }
}

// Configurations about thrusters
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ThrusterConfig {
    force: ForceConfig,
}

impl ThrusterConfig {
    pub fn force(&self) -> ForceConfig {
        self.force.clone()
    }
}

// Configuration about force by thrusters
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForceConfig {
    // Keyboard
    accelerate: f32,
    dash: f32,
    boost: f32,

    // Mouse
    pitch: f32,
    yaw: f32,
}

impl ForceConfig {
    pub fn accelerate(&self) -> f32 {
        self.accelerate
    }

    pub fn dash(&self) -> f32 {
        self.dash
    }

    pub fn boost(&self) -> f32 {
        self.boost
    }

    pub fn pitch(&self) -> f32 {
        self.pitch
    }

    pub fn yaw(&self) -> f32 {
        self.yaw
    }
}

impl std::default::Default for ForceConfig {
    fn default() -> Self {
        Self {
            accelerate: 0.1,
            dash: 10.0,
            boost: 100.0,
            pitch: 1.0,
            yaw: 1.0,
        }
    }
}

/// Keyboard Configurations. This structure usually contains keymappings.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyboardConfig {
    // Movements
    forward: KeyCode,
    back: KeyCode,
    left: KeyCode,
    right: KeyCode,
    roll_left: KeyCode,
    roll_right: KeyCode,

    dash: KeyCode,

    // Hovering
    hover: KeyCode,

    // Gun
    toggle_firemode: KeyCode,

    // Game quit key
    quit: KeyCode,

    // Respawn key
    respawn: KeyCode,
}

impl KeyboardConfig {
    pub fn forward(&self) -> KeyCode {
        self.forward.clone()
    }

    pub fn back(&self) -> KeyCode {
        self.back.clone()
    }

    pub fn left(&self) -> KeyCode {
        self.left.clone()
    }

    pub fn right(&self) -> KeyCode {
        self.right.clone()
    }

    pub fn roll_left(&self) -> KeyCode {
        self.roll_left.clone()
    }

    pub fn roll_right(&self) -> KeyCode {
        self.roll_right.clone()
    }

    pub fn dash(&self) -> KeyCode {
        self.dash.clone()
    }

    pub fn hover(&self) -> KeyCode {
        self.hover.clone()
    }

    pub fn toggle_firemode(&self) -> KeyCode {
        self.toggle_firemode.clone()
    }

    pub fn quit(&self) -> KeyCode {
        self.quit.clone()
    }

    pub fn respawn(&self) -> KeyCode {
        self.respawn.clone()
    }
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

            dash: KeyCode::ShiftLeft,

            hover: KeyCode::ControlLeft,

            toggle_firemode: KeyCode::KeyT,

            quit: KeyCode::Escape,

            respawn: KeyCode::Space,
        }
    }
}

/// Mouse Configurations
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MouseConfig {
    x_reverse: bool,
    y_reverse: bool,
}

impl MouseConfig {
    pub fn x_reverse(&self) -> bool {
        self.x_reverse
    }

    pub fn y_reverse(&self) -> bool {
        self.y_reverse
    }
}

/// Controller (Gamepad) Configurations
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ControllerConfig {}
