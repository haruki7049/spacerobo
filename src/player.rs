use avian3d::prelude::*;
use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

#[derive(Component)]
pub struct Player;

pub fn setup(mut commands: Commands) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        // Transform::from_xyz(0., 0., 0.),
        // Transform::from_xyz(0., 0., 0.).looking_at(Vec3::Y, Dir3::Y),
        // Transform::from_xyz(0., 0., 0.).looking_at(Vec3::Y, Dir3::X),
        Transform::from_xyz(0., 0., 0.).looking_at(Vec3::Y, Dir3::NEG_X),
        RigidBody::Dynamic,
        GravityScale(0.2),
        Collider::sphere(1.0),
        AngularVelocity(Vec3::ZERO),
        Player,
    ));
}

pub fn keyboard_mouse_system(
    mut query: Query<(&mut Transform, &mut AngularVelocity), With<Player>>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if accumulated_mouse_motion.delta != Vec2::ZERO {
        let delta = accumulated_mouse_motion.delta;

        for (transform, mut angular) in &mut query {
            let x = delta.x * -1. / 100.;
            let y = delta.y * -1. / 100.;
            let mouse: Vec2 = Vec2::new(x, y);

            let rotation: Quat = transform.rotation;

            let mut velocity: Vec3 = Vec3::ZERO;

            // Mouse X
            {
                let direction: Vec3 = rotation * Vec3::Y;
                dbg!(direction);

                // [X, 0, 0] * [0, Y, 0] => Z
                let x_result: f32 = mouse.x * direction.x;
                let y_result: f32 = mouse.x * direction.y;
                let z_result: f32 = mouse.x * direction.z;

                velocity.x += x_result;
                velocity.y += y_result;
                velocity.z += z_result;

                dbg!(velocity);
            }

            // Mouse Y
            {
                let direction: Vec3 = rotation * Vec3::X;
                dbg!(direction);

                // [X, 0, 0] * [0, Y, 0] => Z
                let x_result: f32 = mouse.y * direction.x;
                let y_result: f32 = mouse.y * direction.y;
                let z_result: f32 = mouse.y * direction.z;

                velocity.x += x_result;
                velocity.y += y_result;
                velocity.z += z_result;

                dbg!(velocity);
            }

            angular.x += velocity.x;
            angular.y += velocity.y;
            angular.z += velocity.z;
        }
    }

    // Hovering
    if keyboard.just_pressed(KeyCode::KeyH) {
        for (mut _transform, mut angular) in &mut query {
            angular.x = 0.;
            angular.y = 0.;
            angular.z = 0.;
        }
    }
}

pub fn controller_system(
    mut angular_query: Query<&mut AngularVelocity, With<Player>>,
    gamepads: Query<(Entity, &Gamepad)>,
) {
    for (entity, gamepad) in &gamepads {
        let left_stick = gamepad.left_stick();
        let right_stick = gamepad.right_stick();

        debug!("{} right_stick: {}", entity, right_stick);
        debug!("{} left_stick: {}", entity, left_stick);

        for mut angular_velocity in &mut angular_query {
            let velocity_z = (right_stick.x * -1.0) / 3.;
            let velocity_y = (right_stick.y * -1.0) / 3.;

            angular_velocity.z += velocity_z;
            angular_velocity.y += velocity_y;
        }
    }
}
