#![allow(clippy::type_complexity)]

use avian3d::prelude::*;
use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct HeadingIndicator;

#[derive(Component)]
pub struct CoordinatesIndicator;

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

    // Heading Indicator
    commands
        .spawn(Text::default())
        .with_child((
            TextSpan::default(),
            (TextFont {
                font_size: 21.0,
                ..default()
            }),
            HeadingIndicator,
        ))
        .with_child((
            TextSpan::default(),
            (TextFont {
                font_size: 21.0,
                ..default()
            }),
            CoordinatesIndicator,
        ));
}

pub fn keyboard_mouse_system(
    mut query: Query<(&mut Transform, &mut AngularVelocity, &mut LinearVelocity), With<Player>>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if accumulated_mouse_motion.delta != Vec2::ZERO {
        let delta = accumulated_mouse_motion.delta;

        for (transform, mut angular, mut _linear) in &mut query {
            let x = delta.x * -1. / 100.;
            let y = delta.y * -1. / 100.;
            let mouse: Vec2 = Vec2::new(x, y);

            let rotation: Quat = transform.rotation;

            let mut velocity: Vec3 = Vec3::ZERO;

            // Mouse X
            {
                let direction: Vec3 = rotation * Vec3::Y;
                // dbg!(direction);

                let x_result: f32 = mouse.x * direction.x;
                let y_result: f32 = mouse.x * direction.y;
                let z_result: f32 = mouse.x * direction.z;

                velocity.x += x_result;
                velocity.y += y_result;
                velocity.z += z_result;

                // dbg!(velocity);
            }

            // Mouse Y
            {
                let direction: Vec3 = rotation * Vec3::X;
                // dbg!(direction);

                let x_result: f32 = mouse.y * direction.x;
                let y_result: f32 = mouse.y * direction.y;
                let z_result: f32 = mouse.y * direction.z;

                velocity.x += x_result;
                velocity.y += y_result;
                velocity.z += z_result;

                // dbg!(velocity);
            }

            angular.x += velocity.x;
            angular.y += velocity.y;
            angular.z += velocity.z;
        }
    }

    // Hovering
    if keyboard.just_pressed(KeyCode::ControlLeft) {
        for (mut _transform, mut angular, mut linear) in &mut query {
            angular.0 = Vec3::ZERO;
            linear.0 = Vec3::ZERO;
        }
    }

    // Moving
    if keyboard.just_pressed(KeyCode::KeyW) {
        let mut velocity: Vec3 = Vec3::ZERO;
        const FORCE: f32 = 10.0;

        for (transform, mut _angular, mut linear) in &mut query {
            let rotation: Quat = transform.rotation;
            let direction: Vec3 = rotation * Vec3::NEG_Z;

            let x: f32 = FORCE * direction.x;
            let y: f32 = FORCE * direction.y;
            let z: f32 = FORCE * direction.z;
            let result: Vec3 = Vec3::new(x, y, z);

            velocity += result;

            linear.0 += velocity;
        }
    }

    if keyboard.just_pressed(KeyCode::KeyA) {
        let mut velocity: Vec3 = Vec3::ZERO;
        const FORCE: f32 = 10.0;

        for (transform, mut _angular, mut linear) in &mut query {
            let rotation: Quat = transform.rotation;
            let direction: Vec3 = rotation * Vec3::NEG_X;

            let x: f32 = FORCE * direction.x;
            let y: f32 = FORCE * direction.y;
            let z: f32 = FORCE * direction.z;
            let result: Vec3 = Vec3::new(x, y, z);

            velocity += result;

            linear.0 += velocity;
        }
    }

    if keyboard.just_pressed(KeyCode::KeyS) {
        let mut velocity: Vec3 = Vec3::ZERO;
        const FORCE: f32 = 10.0;

        for (transform, mut _angular, mut linear) in &mut query {
            let rotation: Quat = transform.rotation;
            let direction: Vec3 = rotation * Vec3::Z;

            let x: f32 = FORCE * direction.x;
            let y: f32 = FORCE * direction.y;
            let z: f32 = FORCE * direction.z;
            let result: Vec3 = Vec3::new(x, y, z);

            velocity += result;

            linear.0 += velocity;
        }
    }

    if keyboard.just_pressed(KeyCode::KeyD) {
        let mut velocity: Vec3 = Vec3::ZERO;
        const FORCE: f32 = 10.0;

        for (transform, mut _angular, mut linear) in &mut query {
            let rotation: Quat = transform.rotation;
            let direction: Vec3 = rotation * Vec3::X;

            let x: f32 = FORCE * direction.x;
            let y: f32 = FORCE * direction.y;
            let z: f32 = FORCE * direction.z;
            let result: Vec3 = Vec3::new(x, y, z);

            velocity += result;

            linear.0 += velocity;
        }
    }
}

pub fn ui_system(
    mut spans: ParamSet<(
        Query<&mut TextSpan, With<HeadingIndicator>>,
        Query<&mut TextSpan, With<CoordinatesIndicator>>,
    )>,
    player_query: Query<&mut Transform, With<Player>>,
) {
    for mut span in &mut spans.p0() {
        for transform in &player_query {
            let rot: Vec3 = transform.rotation.xyz();
            **span = format!("({rot:.2})\n");
        }
    }

    for mut span in &mut spans.p1() {
        for transform in &player_query {
            **span = format!("[{:.2}]\n", transform.translation);
        }
    }
}

pub fn controller_system(
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
