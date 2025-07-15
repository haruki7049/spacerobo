use crate::player::Player;
use avian3d::prelude::*;
use bevy::prelude::*;

pub fn update_system(
    mut commands: Commands,
    mut exit: EventWriter<AppExit>,
    mut query: Query<
        (
            &Transform,
            &mut AngularVelocity,
            &mut LinearVelocity,
            &Player,
        ),
        With<Player>,
    >,
    keyboard: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
) {
    for (transform, mut angular, mut linear, player) in query.iter_mut() {
        // Hovering
        if keyboard.just_pressed(player.config.keyboard.hover) {
            angular.0 = Vec3::ZERO;
            linear.0 = Vec3::ZERO;
        }

        // Exit spacerobo
        if keyboard.just_pressed(KeyCode::Escape) {
            exit.write(AppExit::Success);
        }

        // Moving
        if keyboard.just_pressed(player.config.keyboard.forward) {
            commands.spawn((
                AudioPlayer::new(asset_server.load("SE/engine_dash.ogg")),
                PlaybackSettings::ONCE.with_spatial(false),
            ));

            let mut velocity: Vec3 = Vec3::ZERO;
            const FORCE: f32 = 10.0;

            let rotation: Quat = transform.rotation;
            let direction: Vec3 = rotation * Vec3::NEG_Z;

            let x: f32 = FORCE * direction.x;
            let y: f32 = FORCE * direction.y;
            let z: f32 = FORCE * direction.z;
            let result: Vec3 = Vec3::new(x, y, z);

            velocity += result;

            linear.0 += velocity;
        }

        if keyboard.just_pressed(player.config.keyboard.left) {
            commands.spawn((
                AudioPlayer::new(asset_server.load("SE/engine_dash.ogg")),
                PlaybackSettings::ONCE.with_spatial(false),
            ));

            let mut velocity: Vec3 = Vec3::ZERO;
            const FORCE: f32 = 10.0;

            let rotation: Quat = transform.rotation;
            let direction: Vec3 = rotation * Vec3::NEG_X;

            let x: f32 = FORCE * direction.x;
            let y: f32 = FORCE * direction.y;
            let z: f32 = FORCE * direction.z;
            let result: Vec3 = Vec3::new(x, y, z);

            velocity += result;

            linear.0 += velocity;
        }

        if keyboard.just_pressed(player.config.keyboard.back) {
            commands.spawn((
                AudioPlayer::new(asset_server.load("SE/engine_dash.ogg")),
                PlaybackSettings::ONCE.with_spatial(false),
            ));

            let mut velocity: Vec3 = Vec3::ZERO;
            const FORCE: f32 = 10.0;

            let rotation: Quat = transform.rotation;
            let direction: Vec3 = rotation * Vec3::Z;

            let x: f32 = FORCE * direction.x;
            let y: f32 = FORCE * direction.y;
            let z: f32 = FORCE * direction.z;
            let result: Vec3 = Vec3::new(x, y, z);

            velocity += result;

            linear.0 += velocity;
        }

        if keyboard.just_pressed(player.config.keyboard.right) {
            commands.spawn((
                AudioPlayer::new(asset_server.load("SE/engine_dash.ogg")),
                PlaybackSettings::ONCE.with_spatial(false),
            ));

            let mut velocity: Vec3 = Vec3::ZERO;
            const FORCE: f32 = 10.0;

            let rotation: Quat = transform.rotation;
            let direction: Vec3 = rotation * Vec3::X;

            let x: f32 = FORCE * direction.x;
            let y: f32 = FORCE * direction.y;
            let z: f32 = FORCE * direction.z;
            let result: Vec3 = Vec3::new(x, y, z);

            velocity += result;

            linear.0 += velocity;
        }

        if keyboard.just_pressed(player.config.keyboard.roll_left) {
            commands.spawn((
                AudioPlayer::new(asset_server.load("SE/engine_dash.ogg")),
                PlaybackSettings::ONCE.with_spatial(false),
            ));

            const FORCE: f32 = 3.0;

            let rotation: Quat = transform.rotation;
            let direction: Vec3 = rotation * Vec3::NEG_Z;

            let x: f32 = FORCE * direction.x;
            let y: f32 = FORCE * direction.y;
            let z: f32 = FORCE * direction.z;
            let result: Vec3 = Vec3::new(x, y, z);

            angular.0 += result;
        }

        if keyboard.just_pressed(player.config.keyboard.roll_right) {
            commands.spawn((
                AudioPlayer::new(asset_server.load("SE/engine_dash.ogg")),
                PlaybackSettings::ONCE.with_spatial(false),
            ));

            const FORCE: f32 = 3.0;

            let rotation: Quat = transform.rotation;
            let direction: Vec3 = rotation * Vec3::Z;

            let x: f32 = FORCE * direction.x;
            let y: f32 = FORCE * direction.y;
            let z: f32 = FORCE * direction.z;
            let result: Vec3 = Vec3::new(x, y, z);

            angular.0 += result;
        }
    }
}
