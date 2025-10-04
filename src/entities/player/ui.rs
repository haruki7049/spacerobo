//! # UI systems, components & etc...

use super::Player;
use crate::{GameMode, KillCounter};
use bevy::prelude::*;

#[derive(Component)]
pub struct HeadingIndicator;

#[derive(Component)]
pub struct CoordinatesIndicator;

#[derive(Component)]
pub struct KillCounterUI;

pub fn setup_system(mut commands: Commands) {
    // Heading Indicator
    commands
        .spawn((Text::default(), StateScoped(GameMode::ShootingRange)))
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
            (TextFont {
                font_size: 21.0,
                ..default()
            }),
            KillCounterUI,
        ));
}

pub fn update_system(
    mut spans: ParamSet<(
        Query<&mut TextSpan, With<HeadingIndicator>>,
        Query<&mut TextSpan, With<CoordinatesIndicator>>,
        Query<&mut TextSpan, With<KillCounterUI>>,
    )>,
    player_query: Query<&mut Transform, With<Player>>,
    kill_counter: Res<KillCounter>,
) {
    for transform in player_query.iter() {
        debug!("Player's translation: {:?}", transform);
    }

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

    for mut span in &mut spans.p1() {
        **span = format!("Kill Counter: {:.2}\n", kill_counter.get());
    }
}
