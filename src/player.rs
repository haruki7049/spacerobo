use avian3d::prelude::*;
use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

#[derive(Component)]
pub struct Player;

pub fn setup(mut commands: Commands) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-5.0, 3.5, 5.5).looking_at(Vec3::ZERO, Vec3::Y),
        RigidBody::Dynamic,
        GravityScale(0.2),
        Collider::sphere(1.0),
        AngularVelocity(Vec3::ZERO),
        Player,
    ));
}

pub fn keyboard_mouse_system(
    mut angular_query: Query<&mut AngularVelocity, With<Player>>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    key_input: Res<ButtonInput<KeyCode>>,
) {
    if accumulated_mouse_motion.delta != Vec2::ZERO {
        let delta = accumulated_mouse_motion.delta;

        for mut angular_velocity in &mut angular_query {
            let velocity_z = delta.y * -1. / 10.;
            let velocity_y = delta.x * -1. / 10.;

            angular_velocity.z += velocity_z;
            angular_velocity.y += velocity_y;
        }
    }

    // Hovering
    if key_input.just_pressed(KeyCode::KeyH) {
        for mut angular_velocity in &mut angular_query {
            angular_velocity.z = 0.;
            angular_velocity.y = 0.;
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
