use crate::player::Player;
use avian3d::prelude::*;
use bevy::prelude::*;

pub fn update_system(
    mut query: Query<(&mut Transform, &mut AngularVelocity, &mut LinearVelocity), With<Player>>,
    gamepads: Query<(Entity, &Gamepad)>,
) {
    for (entity, gamepad) in &gamepads {
        let left_stick = gamepad.left_stick();
        let right_stick = gamepad.right_stick();

        debug!("{} right_stick: {}", entity, right_stick);
        debug!("{} left_stick: {}", entity, left_stick);

        // Hovering
        if gamepad.just_pressed(GamepadButton::DPadDown) {
            for (_transform, mut angular, mut linear) in &mut query {
                angular.0 = Vec3::ZERO;
                linear.0 = Vec3::ZERO;
            }
        }

        // Moving
        for (transform, _angular, mut linear) in &mut query {
            let rotation: Quat = transform.rotation;
            let mut velocity: Vec3 = Vec3::ZERO;

            // LeftStick X
            if left_stick.x.abs() > 0.1 {
                let direction: Vec3 = rotation * Vec3::X;

                let x_result: f32 = left_stick.x * direction.x;
                let y_result: f32 = left_stick.x * direction.y;
                let z_result: f32 = left_stick.x * direction.z;

                velocity.x += x_result;
                velocity.y += y_result;
                velocity.z += z_result;
            }

            // LeftStick Y
            if left_stick.y.abs() > 0.1 {
                let direction: Vec3 = rotation * Vec3::NEG_Z;

                let x_result: f32 = left_stick.y * direction.x;
                let y_result: f32 = left_stick.y * direction.y;
                let z_result: f32 = left_stick.y * direction.z;

                velocity.x += x_result;
                velocity.y += y_result;
                velocity.z += z_result;
            }

            linear.0 += velocity;
        }

        for (transform, mut angular, mut _linear) in &mut query {
            let rotation: Quat = transform.rotation;

            let mut velocity: Vec3 = Vec3::ZERO;

            // RightStick X
            if right_stick.x.abs() > 0.1 {
                let direction: Vec3 = rotation * Vec3::Y;

                let x_result: f32 = -right_stick.x * direction.x;
                let y_result: f32 = -right_stick.x * direction.y;
                let z_result: f32 = -right_stick.x * direction.z;

                velocity.x += x_result;
                velocity.y += y_result;
                velocity.z += z_result;
            }

            // RightStick Y
            if right_stick.y.abs() > 0.1 {
                let direction: Vec3 = rotation * Vec3::X;

                let x_result: f32 = right_stick.y * direction.x;
                let y_result: f32 = right_stick.y * direction.y;
                let z_result: f32 = right_stick.y * direction.z;

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
