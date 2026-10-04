use super::Controllable;
use crate::configs::GameConfigs;
use avian3d::prelude::*;
use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};
use spacerobo_math::rotate_camera;

/// Flight camera rotation
/// Mouse control
fn flight_camera(
    game_configs: &GameConfigs,
    accumulated_mouse_motion: &AccumulatedMouseMotion,
    transform: &Transform,
    angular: &mut AngularVelocity,
) {
    angular.0 += rotate_camera(
        accumulated_mouse_motion.delta,
        transform.rotation,
        Vec3::Z,
        Vec3::X,
        game_configs.player.mouse.x_reverse,
        game_configs.player.mouse.y_reverse,
        game_configs.player.robo.thruster.force.roll,
        game_configs.player.robo.thruster.force.pitch,
    );
}

/// Normal camera rotation
/// Mouse control
fn normal_camera(
    game_configs: &GameConfigs,
    accumulated_mouse_motion: &AccumulatedMouseMotion,
    transform: &Transform,
    angular: &mut AngularVelocity,
) {
    angular.0 += rotate_camera(
        accumulated_mouse_motion.delta,
        transform.rotation,
        Vec3::Y,
        Vec3::X,
        game_configs.player.mouse.x_reverse,
        game_configs.player.mouse.y_reverse,
        game_configs.player.robo.thruster.force.yaw,
        game_configs.player.robo.thruster.force.pitch,
    );
}

pub fn update_system(
    mut query: Query<(&Transform, &mut AngularVelocity), With<Controllable>>,
    game_configs: Res<GameConfigs>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    for (transform, mut angular) in query.iter_mut() {
        if !mouse_button.pressed(MouseButton::Right) {
            normal_camera(
                &game_configs,
                &accumulated_mouse_motion,
                transform,
                &mut angular,
            );
        } else {
            flight_camera(
                &game_configs,
                &accumulated_mouse_motion,
                transform,
                &mut angular,
            );
        }
    }
}
