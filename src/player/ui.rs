use crate::{player::Player, target::Target};
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct HeadingIndicator;

#[derive(Component)]
pub struct CoordinatesIndicator;

#[derive(Component)]
pub struct Timer;

pub fn setup(
    mut commands: Commands,
) {
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
        ))
        .with_child((
            TextSpan::default(),
            TextFont {
                font_size: 21.0,
                ..Default::default()
            },
            Timer,
        ));
}

pub fn ui_system(
    mut spans: ParamSet<(
        Query<&mut TextSpan, With<HeadingIndicator>>,
        Query<&mut TextSpan, With<CoordinatesIndicator>>,
        Query<&mut TextSpan, With<Timer>>,
    )>,
    player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time<Virtual>>,
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

    for mut span in &mut spans.p2() {
        let time: Duration = time.elapsed();
        **span = format!("<{}.{}>\n", time.as_secs(), time.subsec_millis());
    }
}

pub fn time_pause_system(mut time: ResMut<Time<Virtual>>, targets_query: Query<&Target>) {
    // If time was paused then do nothing
    if time.is_paused() {
        return;
    }

    // If you destroy all the targets, the Timer stop.
    if targets_query.is_empty() {
        time.pause();
    }
}

pub fn exit_system(mut exit: EventWriter<AppExit>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
