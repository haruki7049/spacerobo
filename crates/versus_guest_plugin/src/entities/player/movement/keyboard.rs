use super::super::Player;
use avian3d::prelude::*;
use bevy::prelude::*;
use spacerobo_commons::{GameMode, configs::GameConfigs};

pub fn update_system(
    mut commands: Commands,
    mut gamemode: ResMut<NextState<GameMode>>,
    mut query: Query<(&Transform, &mut AngularVelocity, &mut LinearVelocity), With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    game_configs: Res<GameConfigs>,
) {
    for (transform, mut angular, mut linear) in query.iter_mut() {
        // Hovering
        if keyboard.just_pressed(game_configs.player().keyboard().hover()) {
            angular.0 = Vec3::ZERO;
            linear.0 = Vec3::ZERO;
        }

        // Exit spacerobo
        if keyboard.just_pressed(game_configs.player().keyboard().quit()) {
            gamemode.set(GameMode::Title);
        }

        // Accelerate
        {
            if keyboard.pressed(game_configs.player().keyboard().forward()) {
                let mut velocity: Vec3 = Vec3::ZERO;
                let force: f32 = game_configs.player().robo().thruster().force().accelerate();

                let rotation: Quat = transform.rotation;
                let direction: Vec3 = rotation * Vec3::NEG_Z;

                let x: f32 = force * direction.x;
                let y: f32 = force * direction.y;
                let z: f32 = force * direction.z;
                let result: Vec3 = Vec3::new(x, y, z);

                velocity += result;

                linear.0 += velocity;
            }

            if keyboard.pressed(game_configs.player().keyboard().left()) {
                let mut velocity: Vec3 = Vec3::ZERO;
                let force: f32 = game_configs.player().robo().thruster().force().accelerate();

                let rotation: Quat = transform.rotation;
                let direction: Vec3 = rotation * Vec3::NEG_X;

                let x: f32 = force * direction.x;
                let y: f32 = force * direction.y;
                let z: f32 = force * direction.z;
                let result: Vec3 = Vec3::new(x, y, z);

                velocity += result;

                linear.0 += velocity;
            }

            if keyboard.pressed(game_configs.player().keyboard().back()) {
                let mut velocity: Vec3 = Vec3::ZERO;
                let force: f32 = game_configs.player().robo().thruster().force().accelerate();

                let rotation: Quat = transform.rotation;
                let direction: Vec3 = rotation * Vec3::Z;

                let x: f32 = force * direction.x;
                let y: f32 = force * direction.y;
                let z: f32 = force * direction.z;
                let result: Vec3 = Vec3::new(x, y, z);

                velocity += result;

                linear.0 += velocity;
            }

            if keyboard.pressed(game_configs.player().keyboard().right()) {
                let mut velocity: Vec3 = Vec3::ZERO;
                let force: f32 = game_configs.player().robo().thruster().force().accelerate();

                let rotation: Quat = transform.rotation;
                let direction: Vec3 = rotation * Vec3::X;

                let x: f32 = force * direction.x;
                let y: f32 = force * direction.y;
                let z: f32 = force * direction.z;
                let result: Vec3 = Vec3::new(x, y, z);

                velocity += result;

                linear.0 += velocity;
            }

            if keyboard.pressed(game_configs.player().keyboard().roll_left()) {
                let force: f32 = game_configs.player().robo().thruster().force().accelerate();

                let rotation: Quat = transform.rotation;
                let direction: Vec3 = rotation * Vec3::NEG_Z;

                let x: f32 = force * direction.x;
                let y: f32 = force * direction.y;
                let z: f32 = force * direction.z;
                let result: Vec3 = Vec3::new(x, y, z);

                angular.0 += result;
            }

            if keyboard.pressed(game_configs.player().keyboard().roll_right()) {
                let force: f32 = game_configs.player().robo().thruster().force().accelerate();

                let rotation: Quat = transform.rotation;
                let direction: Vec3 = rotation * Vec3::Z;

                let x: f32 = force * direction.x;
                let y: f32 = force * direction.y;
                let z: f32 = force * direction.z;
                let result: Vec3 = Vec3::new(x, y, z);

                angular.0 += result;
            }
        }

        // Dash
        {
            if keyboard.pressed(game_configs.player().keyboard().dash())
                && keyboard.just_pressed(game_configs.player().keyboard().forward())
            {
                commands.spawn((
                    AudioPlayer::new(asset_server.load("SE/engine_dash.ogg")),
                    PlaybackSettings::ONCE.with_spatial(false),
                ));

                let mut velocity: Vec3 = Vec3::ZERO;
                let force: f32 = game_configs.player().robo().thruster().force().dash();

                let rotation: Quat = transform.rotation;
                let direction: Vec3 = rotation * Vec3::NEG_Z;

                let x: f32 = force * direction.x;
                let y: f32 = force * direction.y;
                let z: f32 = force * direction.z;
                let result: Vec3 = Vec3::new(x, y, z);

                velocity += result;

                linear.0 += velocity;
            }

            if keyboard.pressed(game_configs.player().keyboard().dash())
                && keyboard.just_pressed(game_configs.player().keyboard().left())
            {
                commands.spawn((
                    AudioPlayer::new(asset_server.load("SE/engine_dash.ogg")),
                    PlaybackSettings::ONCE.with_spatial(false),
                ));

                let mut velocity: Vec3 = Vec3::ZERO;
                let force: f32 = game_configs.player().robo().thruster().force().dash();

                let rotation: Quat = transform.rotation;
                let direction: Vec3 = rotation * Vec3::NEG_X;

                let x: f32 = force * direction.x;
                let y: f32 = force * direction.y;
                let z: f32 = force * direction.z;
                let result: Vec3 = Vec3::new(x, y, z);

                velocity += result;

                linear.0 += velocity;
            }

            if keyboard.pressed(game_configs.player().keyboard().dash())
                && keyboard.just_pressed(game_configs.player().keyboard().back())
            {
                commands.spawn((
                    AudioPlayer::new(asset_server.load("SE/engine_dash.ogg")),
                    PlaybackSettings::ONCE.with_spatial(false),
                ));

                let mut velocity: Vec3 = Vec3::ZERO;
                let force: f32 = game_configs.player().robo().thruster().force().dash();

                let rotation: Quat = transform.rotation;
                let direction: Vec3 = rotation * Vec3::Z;

                let x: f32 = force * direction.x;
                let y: f32 = force * direction.y;
                let z: f32 = force * direction.z;
                let result: Vec3 = Vec3::new(x, y, z);

                velocity += result;

                linear.0 += velocity;
            }

            if keyboard.pressed(game_configs.player().keyboard().dash())
                && keyboard.just_pressed(game_configs.player().keyboard().right())
            {
                commands.spawn((
                    AudioPlayer::new(asset_server.load("SE/engine_dash.ogg")),
                    PlaybackSettings::ONCE.with_spatial(false),
                ));

                let mut velocity: Vec3 = Vec3::ZERO;
                let force: f32 = game_configs.player().robo().thruster().force().dash();

                let rotation: Quat = transform.rotation;
                let direction: Vec3 = rotation * Vec3::X;

                let x: f32 = force * direction.x;
                let y: f32 = force * direction.y;
                let z: f32 = force * direction.z;
                let result: Vec3 = Vec3::new(x, y, z);

                velocity += result;

                linear.0 += velocity;
            }

            if keyboard.pressed(game_configs.player().keyboard().dash())
                && keyboard.just_pressed(game_configs.player().keyboard().roll_left())
            {
                commands.spawn((
                    AudioPlayer::new(asset_server.load("SE/engine_dash.ogg")),
                    PlaybackSettings::ONCE.with_spatial(false),
                ));

                let force: f32 = game_configs.player().robo().thruster().force().dash();

                let rotation: Quat = transform.rotation;
                let direction: Vec3 = rotation * Vec3::NEG_Z;

                let x: f32 = force * direction.x;
                let y: f32 = force * direction.y;
                let z: f32 = force * direction.z;
                let result: Vec3 = Vec3::new(x, y, z);

                angular.0 += result;
            }

            if keyboard.pressed(game_configs.player().keyboard().dash())
                && keyboard.just_pressed(game_configs.player().keyboard().roll_right())
            {
                commands.spawn((
                    AudioPlayer::new(asset_server.load("SE/engine_dash.ogg")),
                    PlaybackSettings::ONCE.with_spatial(false),
                ));

                let force: f32 = game_configs.player().robo().thruster().force().dash();

                let rotation: Quat = transform.rotation;
                let direction: Vec3 = rotation * Vec3::Z;

                let x: f32 = force * direction.x;
                let y: f32 = force * direction.y;
                let z: f32 = force * direction.z;
                let result: Vec3 = Vec3::new(x, y, z);

                angular.0 += result;
            }
        }

        // Boost
        // We can use Boost by {forward, left, right, back, roll_left, roll_right} key with hover key pressing
        {
            if keyboard.pressed(game_configs.player().keyboard().hover())
                && keyboard.just_pressed(game_configs.player().keyboard().forward())
            {
                commands.spawn((
                    AudioPlayer::new(asset_server.load("SE/engine_dash.ogg")),
                    PlaybackSettings::ONCE.with_spatial(false),
                ));

                let mut velocity: Vec3 = Vec3::ZERO;
                let force: f32 = game_configs.player().robo().thruster().force().boost();

                let rotation: Quat = transform.rotation;
                let direction: Vec3 = rotation * Vec3::NEG_Z;

                let x: f32 = force * direction.x;
                let y: f32 = force * direction.y;
                let z: f32 = force * direction.z;
                let result: Vec3 = Vec3::new(x, y, z);

                velocity += result;

                linear.0 += velocity;
            }

            if keyboard.pressed(game_configs.player().keyboard().hover())
                && keyboard.just_pressed(game_configs.player().keyboard().left())
            {
                commands.spawn((
                    AudioPlayer::new(asset_server.load("SE/engine_dash.ogg")),
                    PlaybackSettings::ONCE.with_spatial(false),
                ));

                let mut velocity: Vec3 = Vec3::ZERO;
                let force: f32 = game_configs.player().robo().thruster().force().boost();

                let rotation: Quat = transform.rotation;
                let direction: Vec3 = rotation * Vec3::NEG_X;

                let x: f32 = force * direction.x;
                let y: f32 = force * direction.y;
                let z: f32 = force * direction.z;
                let result: Vec3 = Vec3::new(x, y, z);

                velocity += result;

                linear.0 += velocity;
            }

            if keyboard.pressed(game_configs.player().keyboard().hover())
                && keyboard.just_pressed(game_configs.player().keyboard().back())
            {
                commands.spawn((
                    AudioPlayer::new(asset_server.load("SE/engine_dash.ogg")),
                    PlaybackSettings::ONCE.with_spatial(false),
                ));

                let mut velocity: Vec3 = Vec3::ZERO;
                let force: f32 = game_configs.player().robo().thruster().force().boost();

                let rotation: Quat = transform.rotation;
                let direction: Vec3 = rotation * Vec3::Z;

                let x: f32 = force * direction.x;
                let y: f32 = force * direction.y;
                let z: f32 = force * direction.z;
                let result: Vec3 = Vec3::new(x, y, z);

                velocity += result;

                linear.0 += velocity;
            }

            if keyboard.pressed(game_configs.player().keyboard().hover())
                && keyboard.just_pressed(game_configs.player().keyboard().right())
            {
                commands.spawn((
                    AudioPlayer::new(asset_server.load("SE/engine_dash.ogg")),
                    PlaybackSettings::ONCE.with_spatial(false),
                ));

                let mut velocity: Vec3 = Vec3::ZERO;
                let force: f32 = game_configs.player().robo().thruster().force().boost();

                let rotation: Quat = transform.rotation;
                let direction: Vec3 = rotation * Vec3::X;

                let x: f32 = force * direction.x;
                let y: f32 = force * direction.y;
                let z: f32 = force * direction.z;
                let result: Vec3 = Vec3::new(x, y, z);

                velocity += result;

                linear.0 += velocity;
            }

            if keyboard.pressed(game_configs.player().keyboard().hover())
                && keyboard.just_pressed(game_configs.player().keyboard().roll_left())
            {
                commands.spawn((
                    AudioPlayer::new(asset_server.load("SE/engine_dash.ogg")),
                    PlaybackSettings::ONCE.with_spatial(false),
                ));

                let force: f32 = game_configs.player().robo().thruster().force().boost();

                let rotation: Quat = transform.rotation;
                let direction: Vec3 = rotation * Vec3::NEG_Z;

                let x: f32 = force * direction.x;
                let y: f32 = force * direction.y;
                let z: f32 = force * direction.z;
                let result: Vec3 = Vec3::new(x, y, z);

                angular.0 += result;
            }

            if keyboard.pressed(game_configs.player().keyboard().hover())
                && keyboard.just_pressed(game_configs.player().keyboard().roll_right())
            {
                commands.spawn((
                    AudioPlayer::new(asset_server.load("SE/engine_dash.ogg")),
                    PlaybackSettings::ONCE.with_spatial(false),
                ));

                let force: f32 = game_configs.player().robo().thruster().force().boost();

                let rotation: Quat = transform.rotation;
                let direction: Vec3 = rotation * Vec3::Z;

                let x: f32 = force * direction.x;
                let y: f32 = force * direction.y;
                let z: f32 = force * direction.z;
                let result: Vec3 = Vec3::new(x, y, z);

                angular.0 += result;
            }
        }
    }
}
