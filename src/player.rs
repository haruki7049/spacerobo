#![allow(clippy::type_complexity)]

pub mod gun;
pub mod ui;

use crate::player::gun::{Gun, Interval, Muzzle, SelectFire};
use avian3d::prelude::*;
use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

#[derive(Component)]
pub struct Player;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let gap = 4.0;

    // Camera
    commands
        .spawn((
            Camera3d::default(),
            Transform::from_xyz(0., 0., 0.),
            RigidBody::Dynamic,
            GravityScale(0.2),
            Collider::sphere(1.0),
            AngularVelocity(Vec3::ZERO),
            SpatialListener::new(gap),
            Player,
        ))
        // Gun
        .with_child((
            Transform::from_xyz(1., -1., -3.),
            Mesh3d(meshes.add(Extrusion::new(Circle::new(0.125), 2.))),
            MeshMaterial3d(materials.add(Color::BLACK)),
            (Gun {
                select_fire: SelectFire::Full,
                interval: Interval {
                    limit: 0.1,
                    rest: 0.0,
                    amount: 0.01,
                },
            }),
        ))
        // Muzzle
        .with_child((Transform::from_xyz(1., -1., -4.3), Muzzle));
}

pub fn keyboard_mouse_system(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut AngularVelocity, &mut LinearVelocity), With<Player>>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    keyboard: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
) {
    // Mouse control
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
        commands.spawn((
            AudioPlayer::new(asset_server.load("SE/engine_dash.ogg")),
            PlaybackSettings::ONCE.with_spatial(false),
        ));

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
        commands.spawn((
            AudioPlayer::new(asset_server.load("SE/engine_dash.ogg")),
            PlaybackSettings::ONCE.with_spatial(false),
        ));

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
        commands.spawn((
            AudioPlayer::new(asset_server.load("SE/engine_dash.ogg")),
            PlaybackSettings::ONCE.with_spatial(false),
        ));

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
        commands.spawn((
            AudioPlayer::new(asset_server.load("SE/engine_dash.ogg")),
            PlaybackSettings::ONCE.with_spatial(false),
        ));

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
