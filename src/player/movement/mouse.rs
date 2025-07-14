use crate::player::Player;
use avian3d::prelude::*;
use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

pub fn update_system(
    mut query: Query<(&mut Transform, &mut AngularVelocity, &Player), With<Player>>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
) {
    for (transform, mut angular, player) in query.iter_mut() {
        // Mouse control
        if accumulated_mouse_motion.delta != Vec2::ZERO {
            let delta = accumulated_mouse_motion.delta;

            let x = delta.x * -1. / 100.;
            let y = delta.y * -1. / 100.;
            let mouse: Vec2 = Vec2::new(x, y);

            let rotation: Quat = transform.rotation;

            let mut velocity: Vec3 = Vec3::ZERO;

            // Mouse X
            {
                let direction: Vec3 = if player.config.mouse.x_reverse {
                    rotation * Vec3::NEG_Y
                } else {
                    rotation * Vec3::Y
                };
                // dbg!(direction);

                let x_result: f32 = mouse.x * direction.x;
                let y_result: f32 = mouse.x * direction.y;
                let z_result: f32 = mouse.x * direction.z;

                velocity.x += x_result;
                velocity.y += y_result;
                velocity.z += z_result;
            }

            // Mouse Y
            {
                let direction: Vec3 = if player.config.mouse.y_reverse {
                    rotation * Vec3::NEG_X
                } else {
                    rotation * Vec3::X
                };
                // dbg!(direction);

                let x_result: f32 = mouse.y * direction.x;
                let y_result: f32 = mouse.y * direction.y;
                let z_result: f32 = mouse.y * direction.z;

                velocity.x += x_result;
                velocity.y += y_result;
                velocity.z += z_result;
            }

            angular.x += velocity.x;
            angular.y += velocity.y;
            angular.z += velocity.z;
        }
    }
}
