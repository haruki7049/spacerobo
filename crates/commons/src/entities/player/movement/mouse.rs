use super::super::Player;
use avian3d::prelude::*;
use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};
use crate::configs::GameConfigs;

pub fn update_system(
    mut query: Query<(&mut Transform, &mut AngularVelocity), With<Player>>,
    game_configs: Res<GameConfigs>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
) {
    for (transform, mut angular) in query.iter_mut() {
        // Mouse control
        if accumulated_mouse_motion.delta != Vec2::ZERO {
            let delta = accumulated_mouse_motion.delta;

            let x = -delta.x / 100.;
            let y = -delta.y / 100.;
            let mouse: Vec2 = Vec2::new(x, y);

            let rotation: Quat = transform.rotation;

            let mut velocity: Vec3 = Vec3::ZERO;

            // Mouse X
            {
                let direction: Vec3 = if game_configs.player().mouse().x_reverse() {
                    rotation * Vec3::NEG_Y
                } else {
                    rotation * Vec3::Y
                };

                let x: f32 = mouse.x * direction.x;
                let y: f32 = mouse.x * direction.y;
                let z: f32 = mouse.x * direction.z;
                let result: Vec3 = Vec3::new(x, y, z);

                // Add yaw thruster's info
                velocity += result * game_configs.player().robo().thruster().force().yaw();
            }

            // Mouse Y
            {
                let direction: Vec3 = if game_configs.player().mouse().y_reverse() {
                    rotation * Vec3::NEG_X
                } else {
                    rotation * Vec3::X
                };

                let x: f32 = mouse.y * direction.x;
                let y: f32 = mouse.y * direction.y;
                let z: f32 = mouse.y * direction.z;
                let result: Vec3 = Vec3::new(x, y, z);

                // Add pitch thruster's info
                velocity += result * game_configs.player().robo().thruster().force().pitch();
            }

            angular.0 += velocity;
        }
    }
}
