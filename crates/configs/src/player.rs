//! Player's Configuration

use bevy_input::keyboard::KeyCode;
use serde::{Deserialize, Serialize};

/// Configuration struct
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(default)]
pub struct Config {
    pub keyboard: KeyboardConfig,
    pub mouse: MouseConfig,
    pub robo: RoboConfig,
}

// Configurations about robo
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct RoboConfig {
    pub thruster: ThrusterConfig,
    pub gun: GunConfig,

    /// Linear and angular velocity multiplier applied each frame while the hover key is held.
    pub hover_damping: f32,
}

/// Configuration about the gun's fire-rate cooldown.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct GunConfig {
    /// Minimum time, in seconds, between full-auto shots.
    pub interval_limit: f32,

    /// How much the interval timer decreases per `FixedUpdate` tick.
    pub interval_amount: f32,
}

impl std::default::Default for GunConfig {
    fn default() -> Self {
        Self {
            interval_limit: 0.1,
            interval_amount: 0.01,
        }
    }
}

impl std::default::Default for RoboConfig {
    fn default() -> Self {
        Self {
            thruster: ThrusterConfig::default(),
            gun: GunConfig::default(),
            hover_damping: 0.7,
        }
    }
}

// Configurations about thrusters
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(default)]
pub struct ThrusterConfig {
    pub force: ForceConfig,
}

// Configuration about force by thrusters
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct ForceConfig {
    // Keyboard
    pub accelerate: f32,
    pub dash: f32,

    // Mouse
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

impl std::default::Default for ForceConfig {
    fn default() -> Self {
        Self {
            accelerate: 0.7,
            dash: 3.0,
            pitch: 1.0,
            yaw: 1.0,
            roll: 1.0,
        }
    }
}

/// Keyboard Configurations. This structure usually contains keymappings.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default)]
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
#[serde(default)]
pub struct MouseConfig {
    pub x_reverse: bool,
    pub y_reverse: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_fields_take_default_values() {
        // A file written before `dash`, `quit`, `respawn` and `robo` existed.
        let config: Config = toml::from_str(
            r#"
            [keyboard]
            forward = "ArrowUp"
            back = "KeyS"
            left = "KeyA"
            right = "KeyD"
            hover = "ControlLeft"
            toggle_firemode = "KeyT"

            [mouse]
            x_reverse = true
            y_reverse = false
            "#,
        )
        .unwrap();

        assert_eq!(config.keyboard.forward, KeyCode::ArrowUp);
        assert_eq!(config.keyboard.respawn, KeyboardConfig::default().respawn);
        assert_eq!(config.keyboard.quit, KeyboardConfig::default().quit);
        assert!(config.mouse.x_reverse);
        assert_eq!(config.robo, RoboConfig::default());
    }

    #[test]
    fn empty_file_is_the_default_configuration() {
        let config: Config = toml::from_str("").unwrap();

        assert_eq!(config, Config::default());
    }

    #[test]
    fn partial_table_keeps_the_other_fields() {
        let config: Config = toml::from_str(
            r#"
            [robo.thruster.force]
            dash = 5.0
            "#,
        )
        .unwrap();

        assert_eq!(config.robo.thruster.force.dash, 5.0);
        assert_eq!(
            config.robo.thruster.force.accelerate,
            ForceConfig::default().accelerate
        );
    }

    #[test]
    fn gun_interval_can_be_tuned_independently_of_other_robo_fields() {
        let config: Config = toml::from_str(
            r#"
            [robo.gun]
            interval_limit = 0.2
            "#,
        )
        .unwrap();

        assert_eq!(config.robo.gun.interval_limit, 0.2);
        assert_eq!(
            config.robo.gun.interval_amount,
            GunConfig::default().interval_amount
        );
        assert_eq!(config.robo.thruster, ThrusterConfig::default());
    }

    #[test]
    fn hover_damping_can_be_tuned_independently_of_thruster_force() {
        let config: Config = toml::from_str(
            r#"
            [robo]
            hover_damping = 0.5
            "#,
        )
        .unwrap();

        assert_eq!(config.robo.hover_damping, 0.5);
        assert_eq!(config.robo.thruster, ThrusterConfig::default());
    }
}
