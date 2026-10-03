use super::Controllable;
use crate::{GameMode, configs::GameConfigs};
use avian3d::prelude::*;
use bevy::prelude::*;

/// Thrust applied along a local `direction`, rotated into world space by `rotation` and scaled by `force`.
fn apply_thrust(rotation: Quat, direction: Vec3, force: f32) -> Vec3 {
    force * (rotation * direction)
}

pub fn update_system(
    mut gamemode: ResMut<NextState<GameMode>>,
    mut query: Query<(&Transform, &mut AngularVelocity, &mut LinearVelocity), With<Controllable>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    game_configs: Res<GameConfigs>,
) {
    for (transform, mut angular, mut linear) in query.iter_mut() {
        // Hovering
        if keyboard.pressed(game_configs.player.keyboard.hover) {
            angular.0 *= Vec3::new(0.7, 0.7, 0.7);
            linear.0 *= Vec3::new(0.7, 0.7, 0.7);
        }

        // Exit spacerobo
        if keyboard.just_pressed(game_configs.player.keyboard.quit) {
            gamemode.set(GameMode::Title);
        }

        let rotation: Quat = transform.rotation;

        // Accelerate
        {
            let force: f32 = game_configs.player.robo.thruster.force.accelerate;

            if keyboard.pressed(game_configs.player.keyboard.forward) {
                linear.0 += apply_thrust(rotation, Vec3::NEG_Z, force);
            }

            if keyboard.pressed(game_configs.player.keyboard.left) {
                linear.0 += apply_thrust(rotation, Vec3::NEG_X, force);
            }

            if keyboard.pressed(game_configs.player.keyboard.back) {
                linear.0 += apply_thrust(rotation, Vec3::Z, force);
            }

            if keyboard.pressed(game_configs.player.keyboard.right) {
                linear.0 += apply_thrust(rotation, Vec3::X, force);
            }
        }

        // Dash
        if keyboard.pressed(game_configs.player.keyboard.dash) {
            let force: f32 = game_configs.player.robo.thruster.force.dash;

            if keyboard.pressed(game_configs.player.keyboard.forward) {
                linear.0 += apply_thrust(rotation, Vec3::NEG_Z, force);
            }

            if keyboard.pressed(game_configs.player.keyboard.left) {
                linear.0 += apply_thrust(rotation, Vec3::NEG_X, force);
            }

            if keyboard.pressed(game_configs.player.keyboard.back) {
                linear.0 += apply_thrust(rotation, Vec3::Z, force);
            }

            if keyboard.pressed(game_configs.player.keyboard.right) {
                linear.0 += apply_thrust(rotation, Vec3::X, force);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    /// `apply_thrust`'s unit tests
    mod apply_thrust {
        use super::super::apply_thrust;
        use bevy::prelude::*;
        use std::f32::consts::FRAC_PI_2;

        /// With no rotation, thrust is the direction scaled by force.
        #[test]
        fn identity_rotation_scales_direction() {
            let result: Vec3 = apply_thrust(Quat::IDENTITY, Vec3::NEG_Z, 0.7);
            assert_eq!(result, Vec3::new(0., 0., -0.7));
        }

        /// Zero force yields a zero vector regardless of rotation or direction.
        #[test]
        fn zero_force_yields_zero_vector() {
            let rotation: Quat = Quat::from_rotation_y(FRAC_PI_2);
            let result: Vec3 = apply_thrust(rotation, Vec3::X, 0.);
            assert_eq!(result, Vec3::ZERO);
        }

        /// The direction is rotated into world space before being scaled by force.
        #[test]
        fn rotation_rotates_direction_before_scaling() {
            // A 90-degree rotation around Y maps NEG_Z to NEG_X.
            let rotation: Quat = Quat::from_rotation_y(FRAC_PI_2);
            let result: Vec3 = apply_thrust(rotation, Vec3::NEG_Z, 2.0);
            let expected: Vec3 = Vec3::new(-2., 0., 0.);

            assert!(
                (result - expected).length() < 1e-4,
                "expected {expected:?}, got {result:?}"
            );
        }
    }
}
